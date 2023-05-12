use std::io::BufWriter;

pub trait AudioSource {
  fn find() -> Future<Option<AudioMetadata>>;
  fn get() -> Future<Option<Audio>>;
}