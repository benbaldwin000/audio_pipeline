mod library;
// mod pipeline;
mod audio;
mod fs_provider;

use audio::AudioReader;
use fs_provider::FsAudioProvider;
use library::{AudioProvider, ReadableBlobProvider, WriteableBlobProvider};
use symphonia::{
    core::{formats::FormatOptions, meta::MetadataOptions, probe::Hint},
    default::{get_codecs, get_probe},
};

fn main() {
    let mut fs_provider = FsAudioProvider::new("./public");
    fs_provider.init().unwrap();

    let media = fs_provider.get_audio("sine", Default::default()).unwrap();
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();
    let hint = Hint::new();
    let formatted = get_probe()
        .format(&hint, media, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    let format = formatted.format;
    let mut audio_reader = AudioReader::new(format, get_codecs(), &Default::default()).unwrap();
    fs_provider.create_audio(&mut audio_reader).unwrap();
}
