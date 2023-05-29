use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::storage::{AudioStorage, Track, TrackQuery};

pub struct AudioLibrary<'a> {
  storage: HashMap::<String, &'a mut dyn AudioStorage>,
  track_cache: HashMap::<u64, Track>,
}

impl<'a> AudioLibrary<'a> {
  pub fn query(&mut self, query: &LibraryQuery) -> Vec<&Track> {
    let mut result = Vec::new();
    let track_query = TrackQuery {
      title: query.title,
      read_cursor: query.read_cursor,
      max_response: query.max_response
    };

    for (_, storage) in self.storage.iter() {
      match storage.query_tracks(&track_query) {
        Ok(tracks) => {
          self.track_cache.extend(tracks.into_iter().map(|track| (track.id, track)));
          for track in tracks.iter() {
            match self.track_cache.get(&track.id) {
              Some(track_ref) => result.push(track_ref),
              _ => continue
            }
          }
        },
        _ => continue
      };
    }

    result
  }

  pub fn get_track(&mut self, id: u64) -> Result<&Track, String> {
    let cached = self.track_cache.get(&id);
    if cached.is_some() { return Ok(cached.unwrap()) }
    for (_, storage) in self.storage.iter() {
      let res = storage.get_track(id);
      if !res.is_ok() { continue }
      let track = res.unwrap();
      self.track_cache.insert(track.id, track);
    };

    Err("track not found".to_string())
  }

  pub fn get_track_source(&mut self, id: u64) -> Result<Vec<u8>, String> {
    for (_, storage) in self.storage.iter() {
      let res = storage.get_track_source(id);
      if res.is_ok() { return Ok(res.unwrap()) }
    }

    Err("track not found".to_string())
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryQuery {
    pub title: String,
    pub read_cursor: u32,
    pub max_response: u32,
}