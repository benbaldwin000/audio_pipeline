pub mod gateway;
pub mod library;
pub mod pipeline;
pub mod playback;
pub mod storage;

use gateway::HttpAudioGateway;
use library::AudioLibrary;
use pipeline::AudioPipeline;
use storage::FSAudioStorage;

fn main() {
    let mut library = AudioLibrary::new();
    library.register_storage("fs", FSAudioStorage::new("./public"));

    let gateway = HttpAudioGateway::new(8080);
    let mut pipeline = AudioPipeline::new(library, gateway);

    pipeline.init().unwrap();
    pipeline.open().unwrap();
}
