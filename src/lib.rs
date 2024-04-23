use std::{
    sync::mpsc::{channel, Sender, TryRecvError},
    thread,
    thread::JoinHandle,
    time::{Duration, Instant},
};

pub use crate::utils::stream::{State, Stream};

mod utils;

#[warn(dead_code)]
pub struct Spinner {
    sender: Sender<(Instant, State, String)>,
    join: Option<JoinHandle<()>>,
    message: String,
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if self.join.is_some() {
            self.sender
                .send((Instant::now(), State::Loading, "self.message".to_string()))
                .unwrap();
            self.join.take().unwrap().join().unwrap();
        }
    }
}

const FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

impl Spinner {
    /// Creates a new Spinner with a message
    /// If no message is provided, it defaults to "Loading..."
    ///
    /// # Example
    ///
    /// ```
    /// use spinner_rs::Spinner;
    ///
    /// let mut spinner = Spinner::new(Some("Loading..."));
    ///
    /// ```
    pub fn new(message: Option<&str>) -> Self {
        let message = match message {
            Some(message) => message,
            None => "Loading...",
        };

        Self::start_in(message)
    }

    pub fn start_in(m: &str) -> Self {
        let stream = Stream::default();

        let (sender, recv) = channel::<(Instant, State, String)>();

        let join = thread::spawn(move || 'outer: loop {
            for frame in FRAMES.iter() {
                let (do_stop, _stop_time, state, message) = match recv.try_recv() {
                    Ok((stop_time, state, message)) => (true, Some(stop_time), state, message),
                    Err(TryRecvError::Disconnected) => {
                        (true, None, State::Loading, "Loading...".to_string())
                    }
                    Err(TryRecvError::Empty) => {
                        (false, None, State::Loading, "Loading...".to_string())
                    }
                };

                let frame = frame.to_string();

                let message = message.to_string();

                stream.write(&frame, &message, state).expect("IO Error");

                if do_stop {
                    break 'outer;
                }

                thread::sleep(Duration::from_millis(80));
            }
        });

        Self {
            sender,
            join: Some(join),
            message: m.to_string(),
        }
    }

    fn wirte(&mut self, state: State, message: Option<&str>) {
        let m = match message {
            Some(message) => message.to_string(),
            None => self.message.to_string(),
        };
        self.sender.send((Instant::now(), state, m)).unwrap();

        self.join.take().unwrap().join().unwrap();
    }

    /// Writes the message with the Loading state
    /// If no message is provided, it defaults to the message provided in the constructor
    ///
    /// # Example
    ///
    /// ```
    /// use spinner_rs::Spinner;
    ///
    /// let mut spinner = Spinner::new(Some("Loading..."));
    ///
    /// spinner.info(None);
    ///
    /// spinner.info(Some("Loading..."));
    ///
    /// ```
    pub fn info(&mut self, message: Option<&str>) {
        self.wirte(State::Info, message)
    }

    /// Writes the message with the Loading state
    /// If no message is provided, it defaults to the message provided in the constructor
    ///
    /// # Example
    ///
    /// ```
    /// use spinner_rs::Spinner;
    ///
    /// let mut spinner = Spinner::new(Some("Loading..."));
    ///
    /// spinner.success(None);
    ///
    /// spinner.success(Some("Loading..."));
    ///
    /// ```
    pub fn success(&mut self, message: Option<&str>) {
        self.wirte(State::Success, message)
    }

    /// Writes the message with the Loading state
    /// If no message is provided, it defaults to the message provided in the constructor
    ///
    /// # Example
    ///
    /// ```
    /// use spinner_rs::Spinner;
    ///
    /// let mut spinner = Spinner::new(Some("Loading..."));
    ///
    /// spinner.warning(None);
    ///
    /// spinner.warning(Some("Loading..."));
    ///
    /// ```
    pub fn warning(&mut self, message: Option<&str>) {
        self.wirte(State::Warn, message)
    }

    /// Writes the message with the Loading state
    /// If no message is provided, it defaults to the message provided in the constructor
    ///
    /// # Example
    ///
    /// ```
    /// use spinner_rs::Spinner;
    ///
    /// let mut spinner = Spinner::new(Some("Loading..."));
    ///
    /// spinner.error(None);
    ///
    /// spinner.error(Some("Loading..."));
    ///
    /// ```
    pub fn error(&mut self, message: Option<&str>) {
        self.wirte(State::Error, message)
    }
}
