use std::io::BufReader;

pub struct Audio {
  metadata: AudioMetadata,
  stream: BufReader
}