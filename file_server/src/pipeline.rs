use std::collections::HashMap;

pub struct AudioPipeline {
  audio_sources: HashMap<String, AudioSource>,
  compression_codecs: HashMap<String, AudioCompression>,
  pipes: Vec<_>,
}

impl AudioPipeline {
  
}