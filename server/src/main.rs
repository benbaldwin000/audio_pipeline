mod library;
mod pipeline;

use std::fs::File;

use library::AudioLibrary;

fn main() {
    let library = AudioLibrary::builder().build();
    let audio = library.get_audio("").unwrap();
    let samples = wav::BitDepth::ThirtyTwoFloat(audio.samples.to_vec());
    let fmt_header = wav::Header::new(
        wav::header::WAV_FORMAT_IEEE_FLOAT,
        1,
        44100,
        32,
    );

    let mut file = File::create("test.wav").unwrap();
    wav::write(fmt_header, &samples, &mut file).unwrap();
}

