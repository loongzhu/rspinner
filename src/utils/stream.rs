use colored::*;
use std::io::{stderr, stdout, Result, Write};

/// Handles the Printing logic for the Spinner
#[derive(Default, Copy, Clone)]
pub enum Stream {
    #[default]
    Stderr,
    Stdout,
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

    /// Writes the message with the duration
    fn print_message_with_duration(
        writer: &mut Box<dyn Write>,
        frame: &str,
        message: &str,
    ) -> Result<()> {
        write!(writer, "\r{} {}", frame.blue(), message)?;
        writer.flush()
    }

    /// Writes the current message and optionally prints the durations
    pub fn write(&self, frame: &str, message: &str) -> Result<()> {
        let mut writer = self.match_target();
        Self::print_message_with_duration(&mut writer, frame, message)?;

        Ok(())
    }
}
