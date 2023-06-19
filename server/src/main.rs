mod library;
// mod pipeline;
mod fs_provider;

use std::{fs::File, io};
use fs_provider::FsAudioProvider;
use library::{ReadableBlobProvider, AudioProvider};
use wav::{BitDepth, WAV_FORMAT_IEEE_FLOAT};
use symphonia::{
    core::{
        audio::{AudioBufferRef, Channels, SignalSpec},
        errors::Error,
        formats::FormatOptions,
        meta::MetadataOptions,
        probe::Hint,
    },
    default::{get_codecs, get_probe},
};

fn main() {
    let mut fs_provider = FsAudioProvider::new("./public");
    fs_provider.init().unwrap();

    let media = fs_provider.get_audio("sine").unwrap();

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();
    let hint = Hint::new();
    let formatted = get_probe()
        .format(&hint, media, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    let mut format = formatted.format;
    let track = format.default_track().unwrap();
    let track_id = track.id;
    let mut decoder = get_codecs()
        .make(&track.codec_params, &Default::default())
        .expect("unsupported codec");

    let mut out_samples = BitDepth::Empty;
    let mut signal_spec = SignalSpec::new(0, Channels::empty());
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::IoError(ref e)) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => panic!("{}", e),
        };

        if packet.track_id() != track_id {
            continue;
        }

        let buffer = match decoder.decode(&packet) {
            Ok(buffer) => buffer,
            Err(Error::DecodeError(_)) => continue,
            Err(e) => panic!("error decoding packet: {}", e),
        };

        if signal_spec.rate == 0 {
            signal_spec = buffer.spec().clone();
        }

        match buffer {
            AudioBufferRef::F32(buf) => {
                if out_samples.is_empty() {
                    out_samples = BitDepth::ThirtyTwoFloat(Vec::new());
                }

                let planes = buf.planes();
                let channels = planes.planes();
                let channel_count = channels.len();
                assert!(channel_count == signal_spec.channels.count());

                if let BitDepth::ThirtyTwoFloat(ref mut samples) = out_samples {
                    let sample_count = channels.iter().map(|c| c.len()).sum::<usize>();
                    samples.reserve(sample_count);

                    for i in 0..sample_count {
                        samples.push(channels[i % channel_count][i / channel_count])
                    }
                } else {
                    panic!("mismatched sample types");
                }
            }
            _ => panic!("unsupported sample type"),
        }
    }

    let mut file = File::create(format!("./test.wav")).unwrap();
    let sample_bits = match out_samples {
        BitDepth::Eight(_) => 8,
        BitDepth::Sixteen(_) => 16,
        BitDepth::TwentyFour(_) => 24,
        BitDepth::ThirtyTwoFloat(_) => 32,
        BitDepth::Empty => 0,
    };

    let wav_header = wav::Header::new(
        WAV_FORMAT_IEEE_FLOAT,
        signal_spec.channels.count() as u16,
        signal_spec.rate,
        sample_bits,
    );

    wav::write(wav_header, &out_samples, &mut file).unwrap();
}
