use crate::library::{AudioProvider, ReadableBlobProvider, SampleBufferRef, WriteableBlobProvider};
use std::{
    fs::{self, File},
    path::PathBuf,
};
use symphonia::core::{audio::SignalSpec, io::MediaSourceStream};
use wav::{BitDepth, WAV_FORMAT_IEEE_FLOAT};

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
    fn get_audio(&self, id: &str) -> Result<MediaSourceStream, String> {
        let source_path = self.path.join("audio").join(format!("{id}.wav"));
        let source_file = match File::open(source_path) {
            Ok(file) => file,
            Err(e) => return Err(format!("error reading source file: {e}")),
        };

        return Ok(MediaSourceStream::new(
            Box::new(source_file),
            Default::default(),
        ));
    }

    fn get_release_art(&self, id: &str) -> Result<Vec<u8>, String> {
        todo!()
    }
}

impl WriteableBlobProvider for FsAudioProvider {
    fn create_audio(
        &self,
        audio_ref: SampleBufferRef,
        spec: &SignalSpec,
    ) -> Result<String, String> {
        let id = 0;
        let mut file = match File::create(format!("audio/{id}.wav")) {
            Ok(file) => file,
            Err(e) => return Err(format!("error creating file: {e}")),
        };

        let mut wav_header = wav::Header::new(0, spec.channels.count() as u16, spec.rate, 0);

        match audio_ref {
            SampleBufferRef::F32(buffer) => {
                wav_header.audio_format = WAV_FORMAT_IEEE_FLOAT;
                wav_header.bits_per_sample = 32;
                if let Err(e) = wav::write(
                    wav_header,
                    &BitDepth::ThirtyTwoFloat(Vec::from(buffer.samples())),
                    &mut file,
                ) {
                    return Err(format!("error writing wav file: {e}"));
                }
            }
            _ => return Err("unsupported format".to_string()),
        };

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
