pub mod gateway;
pub mod storage;
pub mod  pipeline;
pub mod playback;
pub mod  library;

use std::collections::HashMap;
use gateway::HttpAudioGateway;
use pipeline::{AudioPipeline};
use playback::PlaybackState;
use storage::{AudioStorage, FSAudioStorage};

fn main() {
    let mut storage = HashMap::<String, &mut dyn AudioStorage>::new();
    let mut fs_storage = FSAudioStorage::new("./public");
    storage.insert("fs".to_string(), &mut fs_storage);

    let mut pipeline = AudioPipeline {
        storage,
        gateway: &mut HttpAudioGateway::new(8080),
        playback: PlaybackState::new(),
    };

    pipeline.init().unwrap();
    pipeline.open().unwrap();
}