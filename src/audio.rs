use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;

const CHIME_AUDIO: &[u8] = include_bytes!("../assets/chime.wav");

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default()
            .expect("Failed to create audio output stream");

        Self {
            _stream: stream,
            stream_handle,
        }
    }

    pub fn play_chime(&self) {
        let cursor = Cursor::new(CHIME_AUDIO);

        if let Ok(source) = Decoder::new(cursor) {
            if let Ok(sink) = Sink::try_new(&self.stream_handle) {
                sink.append(source);
                sink.detach();
            }
        }
    }
}
