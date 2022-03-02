use crate::error::macros::quick_impl;

/// Basic error type for the meter
#[derive(Debug)]
pub enum MeterError {
    /// Failed to get elapsed time since last [`DataPoint`]
    Elapsed,

    /// A calculation error
    Calculation,

    /// Time error
    Time,
}

impl std::fmt::Display for MeterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Elapsed => write!(f, "Failed to get elapsed tiem since last datapoint"),
            Self::Calculation => write!(f, "Calculation error"),
            Self::Time => write!(f, "Time error"),
        }
    }
}

impl std::error::Error for MeterError {}

quick_impl!(From<std::time::SystemTimeError> for MeterError, MeterError::Time);
