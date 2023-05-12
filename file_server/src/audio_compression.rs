use std::io::BufReader;

pub trait AudioCompression {
  fn compress(source: &BufReader) -> BufReader;
  fn decompress(source: &BufReader) -> BufReader;
}