use crate::storage::{AudioStorage, Track, TrackQuery};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct AudioLibrary {
    storage: HashMap<String, Box<dyn AudioStorage>>,
    session_cache: Mutex<HashMap<u64, Arc<Track>>>,
}

impl AudioLibrary {
    pub fn new() -> Self {
        Self {
            session_cache: Mutex::new(HashMap::new()),
            storage: HashMap::new()
        }
    }

    pub fn register_storage(&mut self, key: &str, storage: impl AudioStorage + 'static) {
        self.storage.insert(key.to_string(), Box::from(storage));
    } 

    pub fn init(&mut self) -> Result<(), String> {
        for (key, storage) in self.storage.iter_mut() {
            match storage.init() {
                Err(err) => return Err(format!("error initializing \"{key}\" storage: {err}")),
                _ => continue,
            }
        }

        Ok(())
    }

    pub fn query(&self, query: &LibraryQuery) -> Vec<Arc<Track>> {
        let mut result = Vec::new();
        let track_query = TrackQuery {
            title: query.title.clone(),
            read_cursor: query.read_cursor,
            max_response: query.max_response,
        };

        for (_, storage) in self.storage.iter() {
            match storage.query_tracks(&track_query) {
                Ok(tracks) => result.extend(tracks),
                _ => continue,
            };
        }

        let arc_iter = result.into_iter().map(|track| (track.id, Arc::new(track)));
        let mut cache = self.session_cache.lock().unwrap();
        cache.extend(arc_iter);
        cache
            .iter()
            .filter(|(_, track)| track.matches(&track_query))
            .map(|(_, track)| Arc::clone(track))
            .collect()
    }

    pub fn get_track(&self, id: u64) -> Result<Arc<Track>, String> {
        let mut cache = self.session_cache.lock().unwrap();
        if let Some(track) = cache.get(&id) {
            return Ok(Arc::clone(track));
        }

        for (_, storage) in self.storage.iter() {
            if let Ok(track) = storage.get_track(id) {
                let rc = Arc::new(track);
                let rc_clone = Arc::clone(&rc);
                cache.insert(id, rc);
                return Ok(rc_clone);
            }
        }

        Err("track not found".to_string())
    }

    pub fn get_track_source(&self, id: u64) -> Result<Vec<u8>, String> {
        for (_, storage) in self.storage.iter() {
            let res = storage.get_track_source(id);
            if let Ok(src) = res {
                return Ok(src);
            }
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
