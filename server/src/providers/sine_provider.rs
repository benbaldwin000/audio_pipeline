use crate::library::{Audio, AudioProvider, AudioStream, ReadableBlobProvider, Sample};

pub struct SineAudioProvider {
    frequency: f32,
    amplitude: f32,
    duration: f32,
}

impl SineAudioProvider {
    pub fn new(frequency: f32, amplitude: f32, duration: f32) -> Self {
        Self {
            frequency,
            amplitude,
            duration,
        }
    }
}

impl AudioProvider for SineAudioProvider {}

impl ReadableBlobProvider for SineAudioProvider {
    fn get_audio(&self, _: &str) -> Result<Audio, String> {
        const SAMPLE_RATE: u32 = 44100;
        let sample_count = ((SAMPLE_RATE as f32) * self.duration / 1000.0) as u32;

        let mut samples = Vec::<Sample>::with_capacity((sample_count) as usize);
        for i in 0..sample_count {
            let t = i as f32 / SAMPLE_RATE as f32;
            let sample = self.amplitude * (2.0 * std::f32::consts::PI * self.frequency * t).sin();
            samples.push(sample);
        }

        Ok(Audio {
            samples: samples.into_boxed_slice(),
        })
    }

    fn stream_audio(&self, id: &str) -> Result<AudioStream, String> {
        todo!()
    }

    fn get_cover_art(&self, _: &str) -> Result<Vec<u8>, String> {
        Err("cannot provide cover art".to_string())
    }
}

