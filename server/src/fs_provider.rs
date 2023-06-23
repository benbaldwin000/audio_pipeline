use crate::core::{audio::AudioReader, provider::{ReadableProvider, ProviderError, WriteableProvider}};
use std::{
    fs::{File, self},
    path::PathBuf,
};
use hound::{WavSpec, WavWriter};
use symphonia::{core::{
    io::MediaSourceStream, meta::MetadataOptions, formats::FormatOptions, probe::Hint, audio::SampleBuffer,
}, default::{get_probe, get_codecs}};

pub struct FsAudioProvider {
    path: PathBuf,
}

impl FsAudioProvider {
    const AUDIO_DIR: &str = "audio";

    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        if let Err(e) = fs::create_dir_all(self.path.join(Self::AUDIO_DIR)) {
            return Err(format!("error creating audio dir: {e}"));
        }

        Ok(())
    }
}

impl ReadableProvider<AudioReader> for FsAudioProvider {
    fn get(&self, id: &str) -> Result<AudioReader, ProviderError> {
        let source_path = self.path.join(Self::AUDIO_DIR).join(format!("{id}.wav"));
        let source_file = match File::open(source_path) {
            Ok(file) => file,
            Err(e) => return Err(ProviderError::Other("error reading source file")),
        };

        let media = MediaSourceStream::new(Box::new(source_file), Default::default());
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();
        let hint = Hint::new();
        let format_reader =  match get_probe().format(&hint, media, &fmt_opts, &meta_opts) {
            Err(e) => return Err(ProviderError::Other("error formatting audio")),
            Ok(result) => result.format
        };

        return match AudioReader::new(format_reader, get_codecs(), &Default::default()) {
            Ok(reader) => Ok(reader),
            Err(_) => Err(ProviderError::Other("error constructing reader")),
        };
    }
}

impl WriteableProvider<AudioReader> for FsAudioProvider {
    fn set(&self, id: &str, mut audio: AudioReader) -> Result<(), ProviderError> {
        let signal_spec = audio.signal_spec();
        let wav_spec = WavSpec {
            channels: signal_spec.channels.count() as u16,
            sample_rate: signal_spec.rate,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let file_path = self.path.join(Self::AUDIO_DIR).join(format!("{id}.wav"));
        let mut writer = WavWriter::create(file_path, wav_spec).unwrap();
        let mut sample_buffer = SampleBuffer::new(64 * 1024, signal_spec);

        loop {
            match audio.read_next_as_samples::<f32>(&mut sample_buffer) {
                Ok(_) => (),
                Err(e) if e == "EOF" => break,
                Err(e) => return Err(ProviderError::Other("error reading samples")),
            }

            for sample in sample_buffer.samples() {
                if let Err(e) = writer.write_sample(*sample) {
                    return Err(ProviderError::Other("error writing sample"));
                }
            }
        }

        return match writer.finalize() {
            Ok(_) => Ok(()),
            Err(_) => Err(ProviderError::Other("error closing file"))
        };
    }

    // fn delete_audio(&self, id: &str) -> Result<(), String> {
    //     let file_path = self.path.join(format!("./audio/{id}.wav"));
    //     if let Err(e) = fs::remove_file(file_path) {
    //         return Err(ProviderError::Other(format!("error deleting audio file: {e}").as_str()));
    //     }

    //     Ok(())
    // }
}
