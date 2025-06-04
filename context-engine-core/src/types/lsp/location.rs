//! Extensions for working with code locations.
//!
//! This module provides extension methods for the [`lsp_types::Location`] type,
//! adding functionality specific to the Context Engine's needs.

use std::str::FromStr;

use lsp_types::{Location, Range, Uri};

use crate::types::{LocationError, RangeExt, UriExt};

/// Extension methods for Location
///
/// This trait adds helpful methods to the standard LSP Location type.
///
/// # Examples
///
/// ```
/// use context_engine_core::types::{Range, Position, Location, LocationExt};
///
/// // Create a range
/// let range = Range::new(
///     Position::new(10, 0),
///     Position::new(10, 20)
/// );
///
/// // Create a location with validation
/// let location = Location::validated("file:///src/main.rs", range);
/// assert!(location.is_ok());
///
/// // Get the filename from a location
/// let loc = location.unwrap();
/// assert_eq!(loc.filename(), Some("main.rs".to_string()));
///
/// // Invalid URIs will return an error
/// assert!(Location::validated("invalid-uri", range).is_err());
/// ```
pub trait LocationExt {
    /// Creates a new Location, validating the URI and range.
    ///
    /// This method parses and validates the URI string before creating
    /// a Location object.
    ///
    /// # Arguments
    ///
    /// * `uri_str` - The URI string
    /// * `range` - The range within the document
    ///
    /// # Returns
    ///
    /// * `Ok(Location)` - If the location is valid
    /// * `Err(LocationError)` - If the URI is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Range, Position, Location, LocationExt, RangeExt};
    ///
    /// let range = Range::validated(Position::new(10, 0), Position::new(10, 20)).unwrap();
    ///
    /// // Valid location
    /// let valid = Location::validated("file:///src/main.rs", range.clone());
    /// assert!(valid.is_ok());
    ///
    /// // Invalid URI
    /// let invalid = Location::validated("not a uri", range);
    /// assert!(invalid.is_err());
    /// ```
    fn validated(uri_str: &str, range: Range) -> Result<Location, LocationError>;

    /// Gets the filename component of the location's URI.
    ///
    /// This extracts just the filename from the URI path.
    ///
    /// # Returns
    ///
    /// * `Some(String)` - The filename if it could be extracted
    /// * `None` - If the URI doesn't have a filename component
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Location, Range, Position, LocationExt, RangeExt};
    /// use std::str::FromStr;
    ///
    /// let range = Range::validated(Position::new(0, 0), Position::new(1, 0)).unwrap();
    ///
    /// let location = Location::validated("file:///src/main.rs", range).unwrap();
    ///
    /// assert_eq!(location.filename(), Some("main.rs".to_string()));
    /// ```
    fn filename(&self) -> Option<String>;
}

impl LocationExt for Location {
    fn validated(uri_str: &str, range: Range) -> Result<Location, LocationError> {
        let uri =
            Uri::from_str(uri_str).map_err(|_| LocationError::InvalidUri(uri_str.to_string()))?;

        if !uri.is_file_uri() {
            return Err(LocationError::InvalidUri(format!(
                "URI must use 'file' scheme, got {}",
                uri.scheme().map(|scheme| scheme.as_str()).unwrap_or("None")
            )));
        }

        // Validate the range
        let start_position = range.start;
        let end_position = range.end;
        // Use the Range::validated method to validate the range
        match Range::validated(start_position, end_position) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }

        Ok(Location::new(uri, range))
    }

    fn filename(&self) -> Option<String> {
        self.uri.filename()
    }
}

#[cfg(test)]
#[path = "tests/location.rs"]
mod tests;
