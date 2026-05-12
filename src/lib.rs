//! pippyzippy — Pure-Rust PPMd compression, part of the 7zippy umbrella.
//!
//! Phase 1: thin wrapper around the `ppmd-rust` crate (pure-Rust PPMd7 implementation).
//! Phase 2: native PPMd implementation.
//!
//! # PPMd7 properties
//!
//! The 7z PPMd coder stores 5 bytes of properties:
//! - byte 0: `order` (model order; valid range 2..=64)
//! - bytes 1–4: `mem_size` as a little-endian u32 (in bytes; typically `1 << 16` to `1 << 28`)

#![deny(unsafe_op_in_unsafe_fn)]

pub mod error;

#[cfg(test)]
mod tests;

use std::io::{Read, Write};

use ppmd_rust::{Ppmd7Decoder, Ppmd7Encoder};

pub use error::{PippyZippyError, PippyZippyResult};

/// Default PPMd7 model order. Matches 7zz's default (`-mo=6`).
pub const DEFAULT_ORDER: u32 = 6;

/// Default memory size for the PPMd7 model in bytes (16 MiB).
///
/// 7zz picks the memory size based on file size; 16 MiB is a safe default for
/// typical test payloads. Larger files benefit from larger model memory.
pub const DEFAULT_MEM_SIZE: u32 = 16 * 1024 * 1024;

/// Compress `input` bytes using PPMd7 (the variant used by the 7z format).
///
/// `order` must be in `2..=64`. `mem_size` must be in `2048..=u32::MAX` (bytes).
///
/// # Errors
/// Returns [`PippyZippyError::Backend`] if the ppmd-rust encoder fails.
pub fn encode(input: &[u8], order: u32, mem_size: u32) -> PippyZippyResult<Vec<u8>> {
    let mut out = Vec::new();
    let mut encoder = Ppmd7Encoder::new(&mut out, order, mem_size)
        .map_err(|e| PippyZippyError::Backend(e.to_string()))?;
    encoder
        .write_all(input)
        .map_err(|e| PippyZippyError::Backend(e.to_string()))?;
    // Finish without end marker — the 7z format stores the uncompressed size
    // separately and uses read_exact rather than end-of-stream detection.
    encoder
        .finish(false)
        .map_err(|e| PippyZippyError::Backend(e.to_string()))?;
    Ok(out)
}

/// Decompress PPMd7-compressed `input` bytes, reading exactly `uncompressed_size` bytes.
///
/// # Errors
/// Returns [`PippyZippyError::Backend`] if the stream is corrupt or truncated.
pub fn decode(
    input: &[u8],
    uncompressed_size: u64,
    order: u32,
    mem_size: u32,
) -> PippyZippyResult<Vec<u8>> {
    let cursor = std::io::Cursor::new(input);
    let mut decoder = Ppmd7Decoder::new(cursor, order, mem_size)
        .map_err(|e| PippyZippyError::Backend(e.to_string()))?;
    let mut out = vec![0u8; uncompressed_size as usize];
    decoder
        .read_exact(&mut out)
        .map_err(|e| PippyZippyError::Backend(e.to_string()))?;
    Ok(out)
}
