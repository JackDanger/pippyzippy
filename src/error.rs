use std::io;
use thiserror::Error;

/// All errors produced by pippyzippy.
#[derive(Error, Debug)]
pub enum PippyZippyError {
    /// Wraps an underlying IO error.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Error from the ppmd-rust backend (Phase 1).
    #[error("PPMd backend error: {0}")]
    Backend(String),

    /// The input was truncated or otherwise invalid.
    #[error("truncated or invalid PPMd stream: {0}")]
    Truncated(String),
}

/// Convenience alias used throughout pippyzippy.
pub type PippyZippyResult<T> = Result<T, PippyZippyError>;
