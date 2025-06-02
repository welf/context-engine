//! Error types for location-related operations.
//!
//! This module defines error types that can occur when working with
//! positions, ranges, locations, and URIs.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur when working with locations and URIs.
#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationError {
    /// Error that occurs when an invalid URI format is provided
    #[error("Invalid URI format: {0}")]
    InvalidUri(String),

    /// Error that occurs when an invalid position is created
    #[error("Invalid position: line={line}, character={character}, reason={reason}")]
    InvalidPosition {
        /// The line number that caused the error
        line: u32,
        /// The character position that caused the error
        character: u32,
        /// Reason why the position is invalid
        reason: String,
    },

    /// Error that occurs when a range's start position is after its end
    /// position
    #[error("Invalid range: start position occurs after end position")]
    InvalidRange,

    /// Error that occurs when a position is out of bounds
    #[error("Position out of bounds: {0}")]
    PositionOutOfBounds(String),
}
