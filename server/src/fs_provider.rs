use crate::{library::{AudioProvider, ReadableBlobProvider, WriteableBlobProvider}, audio::{SampleBufferRef, self, AudioReader}};
use std::{
    fs::{self, File},
    path::PathBuf,
};
use hound::{WavSpec, WavWriter};
use symphonia::core::{audio::{SignalSpec, SampleBuffer}, io::{MediaSourceStream, MediaSourceStreamOptions}};

pub struct FsAudioProvider {
    path: PathBuf,
}

impl FsAudioProvider {
    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
        }
    }
}

impl AudioProvider for FsAudioProvider {
    fn init(&mut self) -> Result<(), String> {
        if let Err(e) = fs::create_dir_all(self.path.join("audio")) {
            return Err(format!("error creating audio dir: {e}"));
        }

        if let Err(e) = fs::create_dir_all(self.path.join("release_art")) {
            return Err(format!("error creating release art dir: {e}"));
        }

        Ok(())
    }
}

impl ReadableBlobProvider for FsAudioProvider {
    fn get_audio(&self, id: &str, media_opts: MediaSourceStreamOptions) -> Result<MediaSourceStream, String> {
        let source_path = self.path.join("audio").join(format!("{id}.wav"));
        let source_file = match File::open(source_path) {
            Ok(file) => file,
            Err(e) => return Err(format!("error reading source file: {e}")),
        };

        return Ok(MediaSourceStream::new(
            Box::new(source_file),
            media_opts,
        ));
    }

    fn get_release_art(&self, id: &str) -> Result<Vec<u8>, String> {
        todo!()
    }
}

impl WriteableBlobProvider for FsAudioProvider {
    fn create_audio(
        &self,
        audio: &mut AudioReader
    ) -> Result<String, String> {
        let id = 0;
        let signal_spec = audio.signal_spec();
        let wav_spec = WavSpec {
            channels: signal_spec.channels.count() as u16,
            sample_rate: signal_spec.rate,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float
        };

        let file_path = self.path.join("audio").join(format!("{id}.wav"));
        let mut writer = WavWriter::create(file_path, wav_spec).unwrap();
        let mut sample_buffer = SampleBuffer::new(64 * 1024, signal_spec);

        loop {
            match audio.read_next_as_samples::<f32>(&mut sample_buffer) {
                Ok(_) => (),
                Err(e) if e == "EOF" => break,
                Err(e) => return  Err(format!("error reading samples: {e}"))
            }

            for sample in sample_buffer.samples() {
                if let Err(e) = writer.write_sample(*sample) {
                    return Err(format!("error writing sample: {e}"))
                }
            }
        };

        if let Err(e) = writer.finalize() {
            return Err(format!("error closing file: {e}"))
        }

        Ok(id.to_string())
    }

    fn delete_audio(&self, id: &str) -> Result<(), String> {
        let file_path = self.path.join(format!("./audio/{id}.wav"));
        if let Err(e) = fs::remove_file(file_path) {
            return Err(format!("error deleting audio file: {e}"));
        }

        Ok(())
    }

    fn create_release_art(&self, value: &Vec<u8>) -> Result<String, String> {
        todo!()
    }

    fn delete_release_art(&self, id: &str) -> Result<(), String> {
        todo!()
    }
}
