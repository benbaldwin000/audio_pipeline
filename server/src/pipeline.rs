use crate::{gateway::AudioGateway, library::AudioLibrary, playback::ObservablePlaybackState};
use std::sync::{Arc, RwLock};

pub struct AudioPipeline<'a, G: AudioGateway<'a> + 'static> {
    gateway: G,
    library: Arc<RwLock<AudioLibrary>>,
    playback: Arc<RwLock<ObservablePlaybackState<'a>>>,
}

impl<'a, G: AudioGateway<'a> + 'static> AudioPipeline<'a, G> {
    pub fn new(library: AudioLibrary, gateway: G) -> Self {
        Self {
            gateway,
            library: Arc::new(RwLock::new(library)),
            playback: Arc::new(RwLock::new(ObservablePlaybackState::new())),
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        if let Err(err) = self.library.write().unwrap().init() {
            return Err(format!("error initializing library: {err}"));
        }

        if let Err(err) = self.gateway.init(&self.library, &self.playback) {
            return Err(format!("error initializing gateway: {err}"));
        }

        Ok(())
    }

    pub fn open(&mut self) -> Result<(), String> {
        Ok(())
    }
}
