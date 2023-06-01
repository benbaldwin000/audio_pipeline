pub mod gateway;
pub mod library;
pub mod pipeline;
pub mod playback;
pub mod storage;

use gateway::start_http_audio_gateway;
use library::AudioLibrary;
use pipeline::AudioPipeline;
use std::sync::RwLock;
use storage::FSAudioStorage;

#[tokio::main]
async fn main() {
    let mut library = AudioLibrary::new();
    library.register_storage("fs", FSAudioStorage::new("/home/ben/Desktop/audio_pipeline/server/public"));
    if let Err(err) = library.init() {
        panic!("error initializing audio library: {}", err);
    };

    let library_lock = RwLock::new(library);
    let pipeline = AudioPipeline::new(library_lock);
    start_http_audio_gateway(8080, pipeline).await;
}
