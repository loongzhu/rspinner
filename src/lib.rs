use std::{
    sync::mpsc::{channel, Sender, TryRecvError},
    thread,
    thread::JoinHandle,
    time::{Duration, Instant},
};

pub use crate::utils::stream::Stream;

mod utils;

pub struct Spinner {
    sender: Sender<(Instant, Option<String>)>,
    join: Option<JoinHandle<()>>,
    stream: Stream,
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if self.join.is_some() {
            self.sender.send((Instant::now(), None)).unwrap();
            self.join.take().unwrap().join().unwrap();
        }
    }
}

const FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

impl Spinner {
    pub fn new(message: Option<String>) -> Self {
        let message = match message {
            Some(message) => message,
            None => "Loading...".to_string(),
        };

        Self::start(message)
    }

    fn start(message: String) -> Self {
        let stream = Stream::default();

        let (sender, recv) = channel::<(Instant, Option<String>)>();

        let join = thread::spawn(move || 'outer: loop {
            for frame in FRAMES.iter() {
                let (do_stop, _stop_time, stop_symbol) = match recv.try_recv() {
                    Ok((stop_time, stop_symbol)) => (true, Some(stop_time), stop_symbol),
                    Err(TryRecvError::Disconnected) => (true, None, None),
                    Err(TryRecvError::Empty) => (false, None, None),
                };

                let frame = stop_symbol.unwrap_or_else(|| frame.to_string());

                stream.write(&frame, &message).expect("IO Error");

                if do_stop {
                    break 'outer;
                }

                thread::sleep(Duration::from_millis(80));
            }
        });

        Self {
            sender,
            join: Some(join),
            stream,
        }
    }

    pub fn info(&self) {
        self.sender
            .send((Instant::now(), Some("\n".to_string())))
            .unwrap();

        self.stream.write("✔", "Info").unwrap();
    }

    pub fn success(&self) {
        self.sender
            .send((Instant::now(), Some("✖".to_string())))
            .unwrap();
    }

    pub fn warning(&self) {
        self.sender
            .send((Instant::now(), Some("⚠".to_string())))
            .unwrap();
    }

    pub fn error(&self) {
        self.sender
            .send((Instant::now(), Some("🛈".to_string())))
            .unwrap();
    }
}
