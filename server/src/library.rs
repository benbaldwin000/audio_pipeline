use std::collections::HashMap;
use symphonia::core::{io::MediaSourceStream, audio::{AudioBufferRef, SampleBuffer, AudioBuffer, SignalSpec}, sample::{Sample, u24, i24}};

pub enum SampleBufferRef {
    U8(Box<SampleBuffer<u8>>),
    U16(Box<SampleBuffer<u16>>),
    U24(Box<SampleBuffer<u24>>),
    U32(Box<SampleBuffer<u32>>),
    S8(Box<SampleBuffer<i8>>),
    S16(Box<SampleBuffer<i16>>),
    S24(Box<SampleBuffer<i24>>),
    S32(Box<SampleBuffer<i32>>),
    F64(Box<SampleBuffer<f64>>),
    F32(Box<SampleBuffer<f32>>),
}

pub struct Track {
    id: String,
    title: String,
    duration_ms: u32,
    artists: Vec<ArtistSummary>,
    release: ReleaseSummary,
}

pub struct TrackSummary {
    id: String,
    title: String,
    duration_ms: u32,
}

pub struct TracksQuery {}

pub struct TrackMutation {}

pub struct Release {
    id: String,
    title: String,
    artists: Vec<ArtistSummary>,
    tracks: Vec<TrackSummary>,
}

pub struct ReleaseSummary {
    id: String,
    title: String,
}

pub struct ReleasesQuery {}

pub struct ReleseMutation {}

pub struct Artist {
    id: String,
    name: String,
    releases: Vec<ReleaseSummary>,
}

pub struct ArtistSummary {
    id: String,
    name: String,
}

pub struct ArtistsQuery {}

pub struct ArtistMutation {}

pub trait AudioProvider {
    fn init(&mut self) -> Result<(), String>;
}

pub trait ReadableStructuredProvider: AudioProvider {
    fn get_track(&self, id: &str) -> Result<Track, String>;
    fn get_many_tracks(&self, query: &TracksQuery) -> Result<Vec<TrackSummary>, String>;

    fn get_release(&self, id: &str) -> Result<Release, String>;
    fn get_many_releases(&self, query: &ReleasesQuery) -> Result<Vec<ReleaseSummary>, String>;

    fn get_artist(&self, id: &str) -> Result<Artist, String>;
    fn get_many_artists(&self, query: &ArtistsQuery) -> Result<Vec<ArtistSummary>, String>;
}

pub trait WriteableStructuredProvider: AudioProvider {
    fn create_track(&self, value: &Track) -> Result<String, String>;
    fn update_track(&self, mutation: &TrackMutation) -> Result<(), String>;
    fn delete_track(&self, id: &str) -> Result<(), String>;

    fn create_release(&self, value: &Release) -> Result<String, String>;
    fn update_release(&self, mutation: &ReleseMutation) -> Result<(), String>;
    fn delete_release(&self, id: &str) -> Result<(), String>;

    fn create_artist(&self, value: &Artist) -> Result<String, String>;
    fn update_artist(&self, mutation: &ArtistMutation) -> Result<(), String>;
    fn delete_artist(&self, id: &str) -> Result<(), String>;
}

pub trait ListenableBlobProvider: AudioProvider {}

pub trait ReadableBlobProvider: AudioProvider {
    fn get_audio(&self, id: &str) -> Result<MediaSourceStream, String>;
    fn get_release_art(&self, id: &str) -> Result<Vec<u8>, String>;
}

pub trait WriteableBlobProvider: AudioProvider {
    fn create_audio(&self, data: SampleBufferRef, spec: &SignalSpec) -> Result<String, String>;
    fn delete_audio(&self, id: &str) -> Result<(), String>;

    fn create_release_art(&self, value: &Vec<u8>) -> Result<String, String>;
    fn delete_release_art(&self, id: &str) -> Result<(), String>;
}

pub struct AudioLibrary<'a> {
    readable_structured_providers: HashMap<String, &'a dyn ReadableStructuredProvider>,
    writeable_structured_providers: HashMap<String, &'a dyn WriteableStructuredProvider>,
    readable_blob_providers: HashMap<String, &'a dyn ReadableBlobProvider>,
    writeable_blob_providers: HashMap<String, &'a dyn WriteableBlobProvider>,
}

impl<'a> AudioLibrary<'a> {
    pub fn builder() -> AudioLibraryBuilder<'a> {
        AudioLibraryBuilder::new()
    }

    pub fn get_track(&self, id: &str) -> Result<Track, String> {
        todo!()
    }

    pub fn get_many_tracks(&self, query: &TracksQuery) -> Result<Vec<TrackSummary>, String> {
        todo!()
    }

    pub fn create_track(&self, value: &Track) -> Result<String, String> {
        todo!()
    }

    pub fn update_track(&self, mutation: &TrackMutation) -> Result<(), String> {
        todo!()
    }

    pub fn delete_track(&self, id: &str) -> Result<(), String> {
        todo!()
    }

    pub fn get_release(&self, id: &str) -> Result<Release, String> {
        todo!()
    }

    pub fn get_many_releases(&self, query: &ReleasesQuery) -> Result<Vec<ReleaseSummary>, String> {
        todo!()
    }

    pub fn create_release(&self, value: &Release) -> Result<String, String> {
        todo!()
    }

    pub fn update_release(&self, mutation: &ReleseMutation) -> Result<(), String> {
        todo!()
    }

    pub fn delete_release(&self, id: &str) -> Result<(), String> {
        todo!()
    }

    pub fn get_artist(&self, id: &str) -> Result<Artist, String> {
        todo!()
    }

    pub fn get_many_artists(&self, query: &ArtistsQuery) -> Result<Vec<ArtistSummary>, String> {
        todo!()
    }

    pub fn create_artist(&self, value: &Artist) -> Result<String, String> {
        todo!()
    }

    pub fn update_artist(&self, mutation: &ArtistMutation) -> Result<(), String> {
        todo!()
    }

    pub fn delete_artist(&self, id: &str) -> Result<(), String> {
        todo!()
    }

    pub fn get_audio(&self, id: &str) -> Result<MediaSourceStream, String> {
        for (_, provider) in self.readable_blob_providers.iter() {
            let audio_result = provider.get_audio(id);
            if audio_result.is_ok() {
                return audio_result
            }
        }

        Err("no providers returned audio".to_string())
    }

    pub fn stream_audio(&self, id: &str) -> Result<MediaSourceStream, String> {
        todo!()
    }

    pub fn create_audio(&self, value: &MediaSourceStream) -> Result<String, String> {
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
    readable_structured_providers: HashMap<String, &'a dyn ReadableStructuredProvider>,
    writeable_structured_providers: HashMap<String, &'a dyn WriteableStructuredProvider>,
    readable_blob_providers: HashMap<String, &'a dyn ReadableBlobProvider>,
    writeable_blob_providers: HashMap<String, &'a dyn WriteableBlobProvider>,
}

impl<'a> AudioLibraryBuilder<'a> {
    pub fn new() -> Self {
        Self {
            readable_structured_providers: HashMap::new(),
            writeable_structured_providers: HashMap::new(),
            readable_blob_providers: HashMap::new(),
            writeable_blob_providers: HashMap::new(),
        }
    }

    pub fn add_readable_structured_provider(
        mut self,
        name: &str,
        provider: &'a impl ReadableStructuredProvider,
    ) -> Self {
        self.readable_structured_providers
            .insert(name.to_string(), provider);

        self
    }

    pub fn add_writeable_structured_provider(
        mut self,
        name: &str,
        provider: &'a impl WriteableStructuredProvider,
    ) -> Self {
        self.writeable_structured_providers
            .insert(name.to_string(), provider);

        self
    }

    pub fn add_readable_blob_provider(
        mut self,
        name: &str,
        provider: &'a impl ReadableBlobProvider,
    ) -> Self {
        self.readable_blob_providers
            .insert(name.to_string(), provider);

        self
    }

    pub fn add_writeable_blob_provider(
        mut self,
        name: &str,
        provider: &'a impl WriteableBlobProvider,
    ) -> Self {
        self.writeable_blob_providers
            .insert(name.to_string(), provider);

        self
    }

    pub fn build(self) -> AudioLibrary<'a> {
        AudioLibrary {
            readable_structured_providers: self.readable_structured_providers,
            writeable_structured_providers: self.writeable_structured_providers,
            readable_blob_providers: self.readable_blob_providers,
            writeable_blob_providers: self.writeable_blob_providers,
        }
    }
}
