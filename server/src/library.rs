use std::collections::HashMap;

pub type Sample = f32;

pub struct Audio {
    pub samples: Box<[Sample]>,
}

pub struct AudioStream {}

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

pub trait ReadableStructuredStorage {
    fn get_track(&self, id: &str) -> Result<Track, String>;
    fn get_many_tracks(&self, query: &TracksQuery) -> Result<Vec<TrackSummary>, String>;

    fn get_release(&self, id: &str) -> Result<Release, String>;
    fn get_many_releases(&self, query: &ReleasesQuery) -> Result<Vec<ReleaseSummary>, String>;

    fn get_artist(&self, id: &str) -> Result<Artist, String>;
    fn get_many_artists(&self, query: &ArtistsQuery) -> Result<Vec<ArtistSummary>, String>;
}

pub trait WriteableStructuredStorage: ReadableStructuredStorage {
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

pub trait ListenableBlobStorage {}

pub trait ReadableBlobStorage {
    fn get_audio(&self, id: &str) -> Result<Audio, String>;
    fn stream_audio(&self, id: &str) -> Result<AudioStream, String>;

    fn get_cover_art(&self, id: &str) -> Result<Vec<u8>, String>;
}

pub trait WriteableBlobStorage: ReadableBlobStorage {
    fn create_audio(&self, value: &Audio) -> Result<String, String>;
    fn delete_audio(&self, id: &str) -> Result<(), String>;

    fn create_cover_art(&self, value: &Vec<u8>) -> Result<String, String>;
    fn delete_cover_art(&self, id: &str) -> Result<(), String>;
}

pub struct AudioLibrary {
    readable_structured_storage: HashMap<String, Box<dyn ReadableStructuredStorage>>,
    writeable_structured_storage: HashMap<String, Box<dyn WriteableStructuredStorage>>,
    readable_blob_storage: HashMap<String, Box<dyn ReadableBlobStorage>>,
    writeable_blob_storage: HashMap<String, Box<dyn WriteableBlobStorage>>,
}

impl AudioLibrary {
    pub fn builder() -> AudioLibraryBuilder {
        AudioLibraryBuilder {
            readable_structured_storage: HashMap::new(),
            writeable_structured_storage: HashMap::new(),
            readable_blob_storage: HashMap::new(),
            writeable_blob_storage: HashMap::new(),
        }
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

    pub fn get_audio(&self, id: &str) -> Result<Audio, String> {
        const SAMPLE_RATE: u32 = 44100;
        const DURATION: u32 = 3000;
        const SAMPLE_COUNT: u32 = SAMPLE_RATE * DURATION / 1000;
        const FREQUENCY: f32 = 440.0;
        const AMPLITUDE: f32 = 0.25;

        let mut samples = Vec::<Sample>::with_capacity((SAMPLE_COUNT) as usize);
        for i in 0..SAMPLE_COUNT {
            let t = i as f32 / SAMPLE_RATE as f32;
            let sample = AMPLITUDE * (2.0 * std::f32::consts::PI * FREQUENCY * t).sin();
            samples.push(sample);
        }

        Ok(Audio {
            samples: samples.into_boxed_slice(),
        })
    }

    pub fn stream_audio(&self, id: &str) -> Result<AudioStream, String> {
        todo!()
    }

    pub fn create_audio(&self, value: &Audio) -> Result<String, String> {
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

pub struct AudioLibraryBuilder {
    readable_structured_storage: HashMap<String, Box<dyn ReadableStructuredStorage>>,
    writeable_structured_storage: HashMap<String, Box<dyn WriteableStructuredStorage>>,
    readable_blob_storage: HashMap<String, Box<dyn ReadableBlobStorage>>,
    writeable_blob_storage: HashMap<String, Box<dyn WriteableBlobStorage>>,
}

impl AudioLibraryBuilder {
    pub fn add_readable_structured_storage(
        mut self,
        name: &str,
        storage: impl ReadableStructuredStorage + 'static,
    ) -> Self {
        self.readable_structured_storage
            .insert(name.to_string(), Box::new(storage));

        self
    }

    pub fn add_writeable_structured_storage(
        mut self,
        name: &str,
        storage: impl WriteableStructuredStorage + 'static,
    ) -> Self {
        self.writeable_structured_storage
            .insert(name.to_string(), Box::new(storage));

        self
    }

    pub fn add_readable_blob_storage(
        mut self,
        name: &str,
        storage: impl ReadableBlobStorage + 'static,
    ) -> Self {
        self.readable_blob_storage
            .insert(name.to_string(), Box::new(storage));

        self
    }

    pub fn add_writeable_blob_storage(
        mut self,
        name: &str,
        storage: impl WriteableBlobStorage + 'static,
    ) -> Self {
        self.writeable_blob_storage
            .insert(name.to_string(), Box::new(storage));

        self
    }

    pub fn build(self) -> AudioLibrary {
        AudioLibrary {
            readable_structured_storage: self.readable_structured_storage,
            writeable_structured_storage: self.writeable_structured_storage,
            readable_blob_storage: self.readable_blob_storage,
            writeable_blob_storage: self.writeable_blob_storage,
        }
    }
}
