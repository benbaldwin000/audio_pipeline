pub mod core;
pub mod fs_provider;

use crate::core::provider::{ReadableProvider, WriteableProvider};
use fs_provider::FsAudioProvider;

fn main() {
    let mut fs_provider = FsAudioProvider::new("./public");
    fs_provider.init().unwrap();

    let audio_reader = fs_provider.get("sine").unwrap();
    fs_provider.set("sine_copy", audio_reader).unwrap();
}
