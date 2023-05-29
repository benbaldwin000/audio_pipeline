use crate::storage::Track;

struct PlaybackState<'a> {
    pub is_playing: bool,
    pub current_track: usize,
    pub session: Vec<&'a Track>,
}

impl<'a> PlaybackState<'a> {
    pub fn new() -> Self {
        Self {
            is_playing: false,
            current_track: 0,
            session: Vec::new(),
        }
    }
}

type Listeners<P> = Vec<Box<dyn Fn(&P) -> ()>>;

pub struct ObservablePlaybackState<'a> {
    state: PlaybackState<'a>,

    is_playing_listeners: Listeners<bool>,
    current_track_listeners: Listeners<Option<&'a Track>>,
    enqueue_listeners: Listeners<(&'a Track, usize)>,
    dequeue_listeners: Listeners<(&'a Track, usize)>,
}

impl<'a> ObservablePlaybackState<'a> {
    pub fn new() -> Self {
        Self {
            state: PlaybackState::new(),
            is_playing_listeners: Vec::new(),
            current_track_listeners: Vec::new(),
            enqueue_listeners: Vec::new(),
            dequeue_listeners: Vec::new(),
        }
    }

    pub fn on_is_playing_changed<F>(&mut self, callback: F)
    where
        F: Fn(&bool) -> () + 'static,
    {
        self.is_playing_listeners.push(Box::new(callback));
    }

    pub fn on_current_track_changed<F>(&mut self, callback: F)
    where
        F: Fn(&Option<&'a Track>) -> () + 'static,
    {
        self.current_track_listeners.push(Box::new(callback));
    }

    pub fn on_enqueue<F>(&mut self, callback: F)
    where
        F: Fn(&(&'a Track, usize)) -> () + 'static,
    {
        self.enqueue_listeners.push(Box::new(callback))
    }

    pub fn on_dequeue<F>(&mut self, callback: F)
    where
        F: Fn(&(&'a Track, usize)) -> () + 'static,
    {
        self.dequeue_listeners.push(Box::new(callback))
    }

    fn notify_listeners<P>(listeners: &Listeners<P>, args: P) {
        for callback in listeners.iter() {
            (*callback)(&args)
        }
    }

    pub fn is_playing(&self) -> bool {
        self.state.is_playing
    }

    pub fn set_playing(&mut self, playing: bool) {
        if self.state.is_playing {
            return;
        }
        self.state.is_playing = playing;
        Self::notify_listeners(&self.is_playing_listeners, playing)
    }

    pub fn queue(&self) -> &[&'a Track] {
        let i = (self.state.current_track + 1).min(self.state.session.len());
        &self.state.session[i..]
    }

    pub fn current_track(&self) -> Option<&'a Track> {
        match self.state.session.get(self.state.current_track) {
            Some(track) => Some(*track),
            _ => None,
        }
    }

    pub fn history(&self) -> &[&'a Track] {
        let i = self.state.current_track.min(self.state.session.len() - 1);
        &self.state.session[..i]
    }

    pub fn session(&self) -> &[&'a Track] {
        &self.state.session
    }

    pub fn play_now(&mut self, track: &'a Track) {
        self.enqueue(track, 0);
        self.skip(1);
    }

    pub fn skip(&mut self, n: i32) {
        self.state.current_track = (self.state.current_track as i32 + n)
            .min(self.state.session.len() as i32)
            .max(0) as usize;

        Self::notify_listeners(&self.current_track_listeners, self.current_track());
    }

    pub fn enqueue(&mut self, track: &'a Track, offset: usize) {
        let i = (self.state.current_track + offset + 1).min(self.state.session.len());
        self.state.session.insert(i, track);
        Self::notify_listeners(&self.enqueue_listeners, (track, i));
    }

    pub fn dequeue(&mut self, offset: usize) {
        let i = self.state.current_track + offset + 1;
        if i >= self.state.session.len() {
            return;
        }
        
        let track = self.state.session.remove(i);
        Self::notify_listeners(&self.dequeue_listeners, (track, i))
    }
}
