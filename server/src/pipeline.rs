use std::collections::HashMap;
use crate::{storage::AudioStorage, gateway::AudioGateway, playback::PlaybackState};


pub struct AudioPipeline<'a> {
  pub storage: HashMap<String, &'a mut dyn AudioStorage>,
  pub gateway: &'a mut dyn AudioGateway,
  pub playback: PlaybackState<'a>
}

impl<'a> AudioPipeline<'a> {
  pub fn init(&mut self) -> Result<(), String> {
    for (key, storage) in self.storage.iter_mut() {
      match storage.init() {
        Err(err) => return Err(format!("error initializing audio source \"{key}\": {err}")),
        _ => continue,
      }
    }

    match self.gateway.init() {
        Err(err) => return Err(format!("error initializing gateway: {err}")),
        _ => ()
    };

    Ok(())
  }

  pub fn open(&mut self) -> Result<(), String> {
    Ok(())
  }
}