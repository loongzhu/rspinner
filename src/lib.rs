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
    sender: Option<Sender<(Instant, State, String)>>,
    join: Option<JoinHandle<()>>,
    message: String,
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if self.join.is_some() && self.sender.is_some() {
            self.sender
                .as_mut()
                .unwrap()
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

        Self {
            sender: None,
            join: None,
            message: message.to_string(),
        }
    }

    /// Starts the spinner with the message provided
    ///
    /// If no message is provided, it defaults to the message provided in the constructor
    ///
    /// # Example
    ///
    /// ```
    /// use spinner_rs::Spinner;
    ///
    /// let mut spinner = Spinner::new(Some("Loading..."));
    ///
    /// spinner.start(None);
    ///
    /// spinner.start(Some("Loading..."));
    ///
    /// ```
    pub fn start(&mut self, message: Option<&str>) {
        let _message = match message {
            Some(message) => message.to_string(),
            None => self.message.to_string(),
        };

        let stream = Stream::default();

        let (sender, recv) = channel::<(Instant, State, String)>();

        let _thread: JoinHandle<()> = thread::spawn(move || 'outer: loop {
            for frame in FRAMES.iter() {
                let (do_stop, _stop_time, state, message) = match recv.try_recv() {
                    Ok((stop_time, state, message)) => (true, Some(stop_time), state, message),
                    Err(TryRecvError::Disconnected) => {
                        (false, None, State::Loading, _message.clone())
                    }
                    Err(TryRecvError::Empty) => (false, None, State::Loading, _message.clone()),
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

        self.sender = Some(sender);
        self.join = Some(_thread);

        if message.is_some() {
            self.message = message.unwrap().to_string();
        }
    }

    fn wirte(&mut self, state: State, message: Option<&str>) {
        let m = match message {
            Some(message) => message.to_string(),
            None => self.message.to_string(),
        };
        self.sender
            .as_mut()
            .unwrap()
            .send((Instant::now(), state, m))
            .unwrap();

        self.join.take().unwrap().join().unwrap();

        self.sender = None;
        self.join = None;
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
