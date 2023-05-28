pub mod gateway;
pub mod storage;
pub mod  pipeline;

use std::collections::HashMap;
use gateway::HttpAudioGateway;
use pipeline::{AudioPipeline, PlaybackState};
use storage::{AudioStorage, FSAudioStorage, Track};

fn main() {
    let mut storage = HashMap::<String, &dyn AudioStorage>::new();
    let fs_storage = FSAudioStorage::new("./public");
    storage.insert("fs".to_string(), &fs_storage);

    let pipeline = AudioPipeline {
        storage,
        gateway: &HttpAudioGateway::new(8080),
        playback: PlaybackState::new(),
    };

}