/// Basic error type for the meter
#[derive(Debug)]
pub enum MeterError {
    /// Failed to get elapsed time since last [`DataPoint`]
    Elapsed,

    /// A calculation error
    Calculation,
}

impl std::fmt::Display for MeterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Elapsed => write!(f, "Failed to get elapsed tiem since last datapoint"),
            Self::Calculation => write!(f, "Calculation error"),
        }
    }
}
