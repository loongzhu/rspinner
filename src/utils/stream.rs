use colored::*;
use std::io::{stderr, stdout, Result, Write};

/// Handles the Printing logic for the Spinner
#[derive(Default, Copy, Clone)]
pub enum Stream {
    #[default]
    Stderr,
    Stdout,
}

pub enum State {
    Loading,
    Info,
    Success,
    Warn,
    Error,
}

#[allow(dead_code)]
#[warn(unreachable_code)]
impl Stream {
    /// Matches on self and returns the internal writer
    fn match_target(&self) -> Box<dyn Write> {
        match self {
            Self::Stderr => Box::new(stderr()),
            Self::Stdout => Box::new(stdout()),
        }
    }

    /// Writes the current message and optionally prints the durations
    pub fn write(&self, frame: &str, message: &str, state: State) -> Result<()> {
        let mut writer = self.match_target();

        let icon = match state {
            State::Loading => frame.blue(),
            State::Info => "ℹ️".blue(),
            State::Success => "✔".green(),
            State::Warn => "⚠".yellow(),
            State::Error => "✖".red(),
        };

        let end = match state {
            State::Loading => "",
            _ => "\n",
        };

        write!(writer, "\r{} {}{end}", icon, message)?;

        writer.flush()
    }
}
