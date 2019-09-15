use sdl2::event::Event;
use std::time::Duration;

#[derive(Debug)]
pub struct KeyEvents(pub Vec<Event>);

impl From<Vec<Event>> for KeyEvents {
    fn from(t: Vec<Event>) -> Self {
        Self(t)
    }
}

#[derive(Debug)]
pub struct Delta(pub Duration);

impl From<Duration> for Delta {
    fn from(t: Duration) -> Self {
        Self(t)
    }
}
