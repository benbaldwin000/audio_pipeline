use dotenv::dotenv;
use librespot::{
    connect::spirc::Spirc,
    core::{
        authentication::Credentials,
        config::{ConnectConfig, SessionConfig},
        session::Session,
    },
    playback::{
        audio_backend::{self, Sink},
        config::PlayerConfig,
        convert::Converter,
        decoder::AudioPacket,
        mixer::{softmixer::SoftMixer, Mixer, NoOpVolume},
        player::Player,
    },
};
use std::env;

#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let mut connect_config = ConnectConfig::default();
    connect_config.name = "crushr".to_string();

    dotenv().unwrap();
    let credentials =
        Credentials::with_password(env::var("USERNAME").unwrap(), env::var("PASSWORD").unwrap());

    println!("Connecting...");
    let (session, _) = match Session::connect(session_config, credentials, None, true).await {
        Ok(res) => res,
        Err(e) => panic!("Error connecting: {}", e),
    };

    let (player, _reciever) = Player::new(
        player_config,
        session.clone(),
        Box::new(NoOpVolume),
        move || Box::new(Middleware::new()),
    );

    let (spirc, spirc_task) = Spirc::new(
        connect_config,
        session.clone(),
        player,
        Box::new(SoftMixer::open(Default::default())),
    );

    spirc_task.await
}

struct Middleware {
    sink: Box<dyn Sink>,
}

impl Middleware {
    fn new() -> Self {
        let sink = audio_backend::find(None).unwrap()(None, Default::default());
        Self { sink }
    }
}

impl Sink for Middleware {
    fn write(
        &mut self,
        packet: AudioPacket,
        converter: &mut Converter,
    ) -> audio_backend::SinkResult<()> {
        let resampled = packet
            .samples()
            .unwrap()
            .iter()
            .map(|s| (*s * (i8::MAX as f64)) as i8)
            .map(|s| s as f32);
        
        let new_packet = AudioPacket::samples_from_f32(resampled.collect());
        self.sink.write(new_packet, converter)
    }
}
