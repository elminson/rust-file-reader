//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use cli_utils::config::Logging;
//! let config = Logging::new();
//! ```
//! 
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// ```
/// 
/// Creating a new instance of the Logging struct:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,   
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }

    // Getter method for the 'enabled' field
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    // Setter method for the 'enabled' field
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    // Getter method for the 'level' field
    pub fn get_level(&self) -> &LogLevel {
        &self.level
    }

    // Setter method for the 'level' field
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    // Getter method for the 'destination' field
    pub fn get_destination(&self) -> &LogOutput {
        &self.destination
    }

    // Setter method for the 'destination' field
    pub fn set_destination(&mut self, destination: LogOutput) {
        self.destination = destination;
    }
}

