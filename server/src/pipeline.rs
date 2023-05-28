use std::collections::HashMap;
use crate::{storage::{AudioStorage, Track}, gateway::AudioGateway};

pub struct PlaybackState<'a> {
  is_playing: bool,
  current_track: Option<&'a Track>,
  queue: Vec<&'a Track>,
}

impl<'a> PlaybackState<'a> {
  pub fn new() -> Self {
    Self {
      is_playing: false,
      current_track: None,
      queue: Vec::new()
    }
  }
}

pub struct AudioPipeline<'a> {
  pub storage: HashMap<String, &'a dyn AudioStorage>,
  pub gateway: &'a dyn AudioGateway,
  pub playback: PlaybackState<'a>
}