use std::sync::{RwLock, Arc};

use crate::{library::AudioLibrary, playback::ObservablePlaybackState};

pub trait AudioGateway<'a> {
    fn init(
        &mut self,
        library: &Arc<RwLock<AudioLibrary>>,
        playback: &Arc<RwLock<ObservablePlaybackState<'a>>>,
    ) -> Result<(), String>;
    fn run(&mut self) -> Result<(), String>;
}

pub struct HttpAudioGateway<'a> {
    port: u32,
    library: Option<Arc<RwLock<AudioLibrary>>>,
    playback: Option<Arc<RwLock<ObservablePlaybackState<'a>>>>,
}

impl<'a> HttpAudioGateway<'a> {
    pub fn new(port: u32) -> Self {
        Self {
            port,
            playback: None,
            library: None,
        }
    }
}

impl<'a> AudioGateway<'a> for HttpAudioGateway<'a> {
    fn init(
        &mut self,
        library: &Arc<RwLock<AudioLibrary>>,
        playback: &Arc<RwLock<ObservablePlaybackState<'a>>>,
    ) -> Result<(), String> {
        self.library = Some(Arc::clone(library));
        self.playback = Some(Arc::clone(playback));
        Ok(())
    }

    fn run(&mut self) -> Result<(), String> {
        todo!()
    }
}
