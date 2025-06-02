//! Types for working with source code locations, positions, ranges, and URIs.
//!
//! This module provides extensions to the standard LSP types from the
//! `lsp_types` crate, adding functionality specific to the Context Engine's
//! needs. Rather than implementing custom versions of these types, we leverage
//! the well-tested implementations from `lsp_types` and extend them with
//! additional capabilities through extension traits.
//!
//! ## Traits
//!
//! * [`PositionExt`] - Extension trait for [`lsp_types::Position`]. It extends
//!   the `Position` with the following methods:
//!     * [`is_before`](PositionExt::is_before): Checks if the position is
//!       before another position.
//!     * [`is_after`](PositionExt::is_after): Checks if the position is after
//!       another position.
//! * [`RangeExt`] - Extension trait for [`lsp_types::Range`]. It extends the
//!   `Range` with the following methods:
//!     * [`validated`](RangeExt::validated): Constructs a validated range.
//!     * [`is_empty`](RangeExt::is_empty): Checks if the range is empty.
//!     * [`contains_position`](RangeExt::contains_position): Checks if the
//!       range contains a given position.
//!     * [`contains_range`](RangeExt::contains_range): Checks if the range
//!       contains another range.
//! * [`UriExt`] - Extension trait for [`lsp_types::Uri`]. It extends the `Uri`
//!   with the following methods:
//!     * [`is_file_uri`](UriExt::is_file_uri): Returns true if this is a file
//!       `Uri` (has 'file' scheme).
//!     * [`filename`](UriExt::filename): Gets the filename component of the
//!       `Uri`.
//!     * [`to_location`](UriExt::to_location): Converts to a standard
//!       `Location` with the given `Range`.
//!     * [`to_file_path`](UriExt::to_file_path): Extracts the file path from
//!       the `Uri`.
//!     * [`new_file_uri`](UriExt::new_file_uri): Creates a new `Uri` from a
//!       string, validating it's a file `Uri`.
//! * [`LocationExt`] - Extension trait for [`lsp_types::Location`]. It extends
//!   the `Location` with the following methods:
//!     * [`validated`](LocationExt::validated): Constructs a validated
//!       `Location`.
//!     * [`filename`](LocationExt::filename): Returns the filename of the
//!       `Location`.
//! * [`LocationError`] - Error types for location-related operations
//!
//! ## Usage Example
//!
//! ```
//! use context_engine_core::types::{
//!     Position,
//!     Range,
//!     Uri,
//!     PositionExt,
//!     RangeExt,
//!     UriExt,
//!     LocationExt
//! };
//! use std::str::FromStr;
//!
//! // Create a position
//! let position = Position::new(10, 20);
//!
//! // Create a validated range. Use Range::validated to create a validated range.
//! let range = Range::validated(
//!     Position::new(10, 0),
//!     Position::new(10, 30)
//! ).unwrap();
//!
//! // Check if position is within range
//! assert!(range.contains_position(&position));
//!
//! // Create a file URI and convert to location
//! let uri = Uri::from_str("file:///src/main.rs").unwrap();
//! let location = uri.to_location(range);
//!
//! // Extract filename
//! assert_eq!(location.filename(), Some("main.rs".to_string()));
//! ```

// Re-export the standard LSP types for convenience
pub use lsp_types::{Location, Position, Range, Uri};

// mod error;
mod lsp;

// Re-export extension traits for easy access
pub use lsp::error::LocationError;
pub use lsp::location::LocationExt;
pub use lsp::position::PositionExt;
pub use lsp::range::RangeExt;
pub use lsp::uri::UriExt;
