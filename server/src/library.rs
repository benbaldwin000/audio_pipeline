use crate::audio::AudioReader;
use std::collections::HashMap;
use symphonia::{
    core::{
        formats::FormatOptions,
        io::{MediaSourceStream, MediaSourceStreamOptions},
        meta::MetadataOptions,
        probe::Hint,
    },
    default::{get_codecs, get_probe},
};

pub trait AudioProvider {
    fn init(&mut self) -> Result<(), String>;
}

pub trait ReadableBlobProvider: AudioProvider {
    fn get_audio(
        &self,
        id: &str,
        media_opts: MediaSourceStreamOptions,
    ) -> Result<MediaSourceStream, String>;
}

pub trait WriteableBlobProvider: AudioProvider {
    fn create_audio(&self, audio: &mut AudioReader) -> Result<String, String>;
    fn delete_audio(&self, id: &str) -> Result<(), String>;
}

pub struct AudioLibrary<'a> {
    readable_blob_providers: HashMap<&'static str, &'a dyn ReadableBlobProvider>,
    writeable_blob_providers: HashMap<&'static str, &'a dyn WriteableBlobProvider>,
}

impl<'a> AudioLibrary<'a> {
    pub fn builder() -> AudioLibraryBuilder<'a> {
        AudioLibraryBuilder::new()
    }

    pub fn get_audio(
        &self,
        id: &str,
        media_opts: MediaSourceStreamOptions,
    ) -> Result<AudioReader, String> {
        for (_, provider) in self.readable_blob_providers.iter() {
            match provider.get_audio(id, MediaSourceStreamOptions { ..media_opts }) {
                Err(_) => continue,
                Ok(media) => {
                    let meta_opts: MetadataOptions = Default::default();
                    let fmt_opts: FormatOptions = Default::default();
                    let hint = Hint::new();

                    return match get_probe().format(&hint, media, &fmt_opts, &meta_opts) {
                        Err(e) => Err(format!("error probing media: {e}")),
                        Ok(result) => {
                            AudioReader::new(result.format, get_codecs(), &Default::default())
                        }
                    };
                }
            };
        }

        Err("no providers returned audio".to_string())
    }

    pub fn create_audio(&self, source: MediaSourceStream) -> Result<String, String> {
        todo!()
    }

    pub fn delete_audio(&self, id: &str) -> Result<(), String> {
        todo!()
    }

    pub fn get_cover_art(&self, id: &str) -> Result<Vec<u8>, String> {
        todo!()
    }

    pub fn create_cover_art(&self, value: &Vec<u8>) -> Result<String, String> {
        todo!()
    }

    pub fn delete_cover_art(&self, id: &str) -> Result<(), String> {
        todo!()
    }
}

pub struct AudioLibraryBuilder<'a> {
    readable_blob_providers: HashMap<&'static str, &'a mut dyn ReadableBlobProvider>,
    writeable_blob_providers: HashMap<&'static str, &'a mut dyn WriteableBlobProvider>,
}

impl<'a> AudioLibraryBuilder<'a> {
    pub fn new() -> Self {
        Self {
            readable_blob_providers: HashMap::new(),
            writeable_blob_providers: HashMap::new(),
        }
    }

    pub fn add_readable_blob_provider(
        mut self,
        name: &'static str,
        provider: &'a mut impl ReadableBlobProvider,
    ) -> Self {
        self.readable_blob_providers.insert(name, provider);

        self
    }

    pub fn add_writeable_blob_provider(
        mut self,
        name: &'static str,
        provider: &'a mut impl WriteableBlobProvider,
    ) -> Self {
        self.writeable_blob_providers.insert(name, provider);

        self
    }

    pub fn build(self) -> Result<AudioLibrary<'a>, String> {
        let mut library = AudioLibrary {
            readable_blob_providers: HashMap::with_capacity(self.readable_blob_providers.len()),
            writeable_blob_providers: HashMap::with_capacity(self.writeable_blob_providers.len()),
        };

        for (id, provider) in self.readable_blob_providers.into_iter() {
            if let Err(e) = provider.init() {
                return Err(format!("error initializing provider \"{id}\": {e}"));
            }

            library.readable_blob_providers.insert(id, &(*provider));
        }

        for (id, provider) in self.writeable_blob_providers.into_iter() {
            if let Err(e) = provider.init() {
                return Err(format!("error initializing provider \"{id}\": {e}"));
            }

            library.writeable_blob_providers.insert(id, provider);
        }

        Ok(library)
    }
}
