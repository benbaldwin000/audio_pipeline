mod library;
mod pipeline;
mod providers;

use axum::{
    body::Body,
    extract::{BodyStream, Query, State},
    routing::{get, post},
    Router, Server,
};
use hyper::{body::Bytes, StatusCode};
use library::{AudioLibrary, ReadableBlobProvider};
use providers::{FsAudioProvider, SineAudioProvider};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, Cursor},
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use symphonia::{
    core::{
        audio::{AudioBufferRef, Channels, SignalSpec},
        codecs::{DecoderOptions, CODEC_TYPE_NULL},
        errors::Error,
        formats::FormatOptions,
        io::{MediaSourceStream, ReadOnlySource},
        meta::MetadataOptions,
        probe::Hint,
    },
    default::{get_codecs, get_probe},
};
use vorbis_rs::{VorbisBitrateManagementStrategy::Vbr, VorbisEncoder};
use wav::{BitDepth, WAV_FORMAT_PCM, WAV_FORMAT_IEEE_FLOAT};

struct HandlerCtx {
    provider: FsAudioProvider,
}

type SharedHandlerState = State<Arc<HandlerCtx>>;

#[tokio::main]
async fn main() {
    // let sine_provider = SineAudioProvider::new(440.0, 0.25, 2000.0);
    // let fs_provider = FsAudioProvider::new("./audio");
    // let handler_ctx = Arc::new(HandlerCtx { provider: fs_provider });

    let address = SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 8080);
    let service = Router::new()
        .route("/track", get(http_get_audio))
        .route("/track", post(http_create_audio))
        .into_make_service();

    println!("starting server at: {:?}", address);
    Server::bind(&address).serve(service).await.unwrap();
}

#[derive(Serialize, Deserialize)]
struct TrackOptions {}

async fn http_create_audio(body: Bytes) -> Result<(), StatusCode> {
    let meta_opts: MetadataOptions = Default::default();
    let mut fmt_opts: FormatOptions = Default::default();
    fmt_opts.enable_gapless = true;

    let mut hint = Hint::new();
    hint.with_extension("mp3");

    let media = MediaSourceStream::new(Box::new(Cursor::new(body)), Default::default());
    let formatted = match get_probe().format(&hint, media, &fmt_opts, &meta_opts) {
        Ok(formatted) => formatted,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let mut format = formatted.format;
    let tracks = format.tracks();
    let track = match tracks
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
    {
        Some(track) => track,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let decoder_opts: DecoderOptions = Default::default();
    let mut decoder = match get_codecs().make(&track.codec_params, &decoder_opts) {
        Ok(decoder) => decoder,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let track_id = track.id;
    let mut samples = BitDepth::Empty;
    let mut signal_spec = SignalSpec {
        rate: 0,
        channels: Channels::empty(),
    };

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::IoError(e)) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    break;
                }
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            Err(e) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        while !format.metadata().is_latest() {
            format.metadata().pop();
        }

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(buffer) => match buffer {
                AudioBufferRef::U8(buf) => {
                    if samples.is_empty() {
                        samples = BitDepth::Eight(Vec::new());
                        signal_spec = buf.spec().clone();
                    }

                    if let BitDepth::Eight(ref mut v) = samples {
                        v.extend_from_slice(buf.planes().planes()[0]);
                    } else {
                        return Err(StatusCode::BAD_REQUEST);
                    }
                }
                AudioBufferRef::S16(buf) => {
                    if samples.is_empty() {
                        samples = BitDepth::Sixteen(Vec::new());
                        signal_spec = buf.spec().clone();
                    }

                    if let BitDepth::Sixteen(ref mut v) = samples {
                        v.extend_from_slice(buf.planes().planes()[0]);
                    } else {
                        return Err(StatusCode::BAD_REQUEST);
                    }
                }
                AudioBufferRef::S24(buf) => {
                    if samples.is_empty() {
                        samples = BitDepth::TwentyFour(Vec::new());
                        signal_spec = buf.spec().clone();
                    }

                    if let BitDepth::TwentyFour(ref mut v) = samples {
                        v.extend(buf.planes().planes()[0].iter().map(|s| s.inner()));
                    } else {
                        return Err(StatusCode::BAD_REQUEST);
                    }
                }
                AudioBufferRef::F32(buf) => {
                    if samples.is_empty() {
                        samples = BitDepth::ThirtyTwoFloat(Vec::new());
                        signal_spec = buf.spec().clone();
                    }
                    if let BitDepth::ThirtyTwoFloat(ref mut v) = samples {
                        v.extend_from_slice(buf.planes().planes()[0]);
                    } else {
                        return Err(StatusCode::BAD_REQUEST);
                    }
                }
                _ => return Err(StatusCode::BAD_REQUEST),
            },

            Err(Error::DecodeError(_)) => continue,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    let mut file = match File::create(format!("./audio/{track_id}.wav")) {
        Ok(file) => file,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let sample_bits = match samples {
        BitDepth::Eight(_) => 8,
        BitDepth::Sixteen(_) => 16,
        BitDepth::TwentyFour(_) => 24,
        BitDepth::ThirtyTwoFloat(_) => 32,
        BitDepth::Empty => 0,
    };

    let wav_header = wav::Header::new(
        WAV_FORMAT_PCM,
        signal_spec.channels.count() as u16,
        signal_spec.rate,
        sample_bits,
    );

    return match wav::write(wav_header, &samples, &mut file) {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Serialize, Deserialize)]
struct UniqueTrackQuery {
    id: String,
}

async fn http_get_audio(Query(params): Query<UniqueTrackQuery>) -> Result<Vec<u8>, StatusCode> {
    // let encoder = VorbisEncoder::new(id, [], 44100, 2, Vbr {}, None, ());

    todo!()
}
