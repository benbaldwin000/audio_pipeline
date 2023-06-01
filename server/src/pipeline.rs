use crate::{library::AudioLibrary};
use std::sync::{RwLock};

pub struct AudioPipeline{
    pub library: RwLock<AudioLibrary>,
}

impl AudioPipeline {
    pub fn new(library: RwLock<AudioLibrary>) -> Self {
        Self {
            library,
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        if let Err(err) = self.library.write().unwrap().init() {
            return Err(format!("error initializing library: {err}"));
        }

        Ok(())
    }
}
