use std::path::PathBuf;

use crate::library::{AudioProvider, ReadableBlobProvider, WriteableBlobProvider};

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

impl AudioProvider for FsAudioProvider {}

impl ReadableBlobProvider for FsAudioProvider {
    fn get_audio(&self, id: &str) -> Result<crate::library::Audio, String> {
        todo!()
    }

    fn stream_audio(&self, id: &str) -> Result<crate::library::AudioStream, String> {
        todo!()
    }

    fn get_cover_art(&self, id: &str) -> Result<Vec<u8>, String> {
        todo!()
    }
}

impl WriteableBlobProvider for FsAudioProvider {
    fn create_audio(&self, value: &crate::library::Audio) -> Result<String, String> {
        todo!()
    }

    fn delete_audio(&self, id: &str) -> Result<(), String> {
        todo!()
    }

    fn create_cover_art(&self, value: &Vec<u8>) -> Result<String, String> {
        todo!()
    }

    fn delete_cover_art(&self, id: &str) -> Result<(), String> {
        todo!()
    }
}