mod library;
// mod pipeline;
mod audio;
mod fs_provider;

use fs_provider::FsAudioProvider;
use library::{AudioLibrary, WriteableBlobProvider};

fn main() {
    let mut fs_provider = FsAudioProvider::new("./public");
    let library = AudioLibrary::builder()
        .add_readable_blob_provider("fs", &mut fs_provider)
        .build()
        .unwrap();

    let mut audio_reader = library.get_audio("sine", Default::default()).unwrap();
    fs_provider.create_audio(&mut audio_reader).unwrap();
}
