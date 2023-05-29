use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: u64,
    pub title: String,
    pub duration_ms: i32,
    pub source_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackQuery {
    pub title: String,
    pub read_cursor: u32,
    pub max_response: u32,
}

impl Track {
    pub fn matches(&self, query: &TrackQuery) -> bool {
        self.title.to_lowercase().contains(query.title.to_lowercase().as_str())
    }
}

pub trait AudioStorage {
    fn init(&mut self) -> Result<(), String>;
    fn query_tracks(&self, query: &TrackQuery) -> Result<Vec<Track>, String>;
    fn get_track(&self, id: u64) -> Result<Track, String>;
    fn get_track_source(&self, id: u64) -> Result<Vec<u8>, String>;
}

pub struct FSAudioStorage {
    dir: String,
    library_cache: HashMap<u64, Track>,
}

impl FSAudioStorage {
    pub fn new(path: &str) -> Self {
        Self {
            dir: path.to_string(),
            library_cache: HashMap::new(),
        }
    }
}

impl AudioStorage for FSAudioStorage {
    fn init(&mut self) -> Result<(), String> {
        let library_path = Path::new(&self.dir).join("/library.json");
        if !library_path.is_file() {
            return Ok(());
        }

        let json = match fs::read_to_string(library_path) {
            Ok(json) => json,
            Err(err) => return Err(format!("error reading library: {err}")),
        };

        let library = match serde_json::from_str::<Vec<Track>>(&json) {
            Ok(library) => library,
            Err(err) => return Err(format!("error deserializing library: {err}")),
        };

        for track in library.iter() {
            self.library_cache.insert(track.id, track.clone());
        }

        Ok(())
    }

    fn query_tracks(&self, query: &TrackQuery) -> Result<Vec<Track>, String> {
        Ok(self
            .library_cache
            .values()
            .filter(|track| {
                track
                    .title
                    .to_lowercase()
                    .find(query.title.to_lowercase().as_str())
                    .is_some()
            })
            .skip(query.read_cursor as usize)
            .take(query.max_response as usize)
            .map(|track| track.clone())
            .collect::<Vec<_>>())
    }

    fn get_track(&self, id: u64) -> Result<Track, String> {
        match self.library_cache.get(&id) {
            Some(track) => Ok(track.clone()),
            None => Err("track not found".to_string()),
        }
    }

    fn get_track_source(&self, id: u64) -> Result<Vec<u8>, String> {
        let track = match self.get_track(id) {
            Ok(track) => track,
            Err(err) => return Err(format!("error getting track: {err}")),
        };

        let src_path = Path::new(&self.dir).join("tracks").join(track.source_path);
        return match fs::read(src_path) {
            Ok(source) => Ok(source),
            Err(_) => Err("error reading source file".to_string()),
        };
    }
}
