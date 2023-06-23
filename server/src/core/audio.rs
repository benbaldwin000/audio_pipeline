use std::io;
use axum::extract::BodyStream;
use symphonia::core::{
    audio::{AudioBufferRef, SampleBuffer, SignalSpec},
    codecs::{CodecParameters, CodecRegistry, Decoder, DecoderOptions},
    conv::ConvertibleSample,
    errors::Error,
    formats::FormatReader,
};

use super::provider::ProviderObject;

pub struct AudioReader {
    format_reader: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track_id: u32,
}

impl AudioReader {
    pub fn new(
        format_reader: Box<dyn FormatReader>,
        codecs: &CodecRegistry,
        opts: &DecoderOptions,
    ) -> Result<Self, String> {
        let track = match format_reader.default_track() {
            Some(track) => track,
            None => return Err("no default track".to_string()),
        };

        let decoder = match codecs.make(&track.codec_params, opts) {
            Ok(decoder) => decoder,
            Err(e) => return Err(format!("error making decoder: {e}")),
        };

        let track_id = track.id;

        Ok(Self {
            format_reader,
            decoder,
            track_id,
        })
    }

    pub fn codec_params(&self) -> &CodecParameters {
        self.decoder.codec_params()
    }

    pub fn signal_spec(&self) -> SignalSpec {
        let codec_params = self.codec_params();
        SignalSpec {
            rate: codec_params.sample_rate.unwrap_or(44100),
            channels: codec_params.channels.unwrap_or_default(),
        }
    }

    pub fn consume_next<F>(&mut self, mut callback: F) -> Result<(), String>
    where
        F: FnMut(AudioBufferRef) -> Result<(), String>,
    {
        loop {
            let packet = match self.format_reader.next_packet() {
                Ok(p) if p.track_id() == self.track_id => p,
                Ok(_) => continue,
                Err(Error::IoError(ref e)) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    return Err("EOF".to_string())
                }
                Err(e) => return Err(format!("error reading packet: {e}")),
            };

            let buffer = match self.decoder.decode(&packet) {
                Ok(buffer) => buffer,
                Err(Error::DecodeError(_)) => continue,
                Err(e) => panic!("error decoding packet: {}", e),
            };

            if let Err(e) = callback(buffer) {
                return Err(format!("error consuming buffer: {e}"));
            }

            return Ok(());
        }
    }

    pub fn read_next_as_samples<S: ConvertibleSample>(
        &mut self,
        dst: &mut SampleBuffer<S>,
    ) -> Result<(), String> {
        return match self.consume_next(|buffer| {
            dst.copy_interleaved_ref(buffer);
            Ok(())
        }) {
            Err(e) if e == "EOF" => Err(e),
            Err(e) => Err(format!("error consuming reader: {e}")),
            ok => ok,
        };
    }
}

impl ProviderObject for AudioReader {}

impl From<BodyStream> for AudioReader {
    fn from(value: BodyStream) -> Self {
        

        todo!()
    }
}