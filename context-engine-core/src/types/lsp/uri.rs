//! Extensions for working with URIs.
//!
//! This module provides extension methods for the [`lsp_types::Uri`] type,
//! which is used as the URI type in the LSP protocol. It adds functionality
//! specific to the Context Engine's needs, particularly for file URIs.

use std::path::PathBuf;
use std::str::FromStr;

use lsp_types::{Location, Range, Uri};

use crate::types::LocationError;

/// Extension methods for URI handling
///
/// This trait adds helpful methods to the standard LSP URI type (which is
/// actually Uri).
///
/// # Examples
///
/// ```
/// use context_engine_core::types::{Uri, Range, Position, UriExt};
///
/// // Create a file URI
/// let uri = Uri::new_file_uri("file:///src/main.rs").unwrap();
///
/// // Check if it's a file URI
/// assert!(uri.is_file_uri());
///
/// // Extract the filename
/// assert_eq!(uri.filename(), Some("main.rs".to_string()));
///
/// // Convert to a location
/// let range = Range::new(Position::new(10, 0), Position::new(10, 20));
/// let location = uri.to_location(range);
/// ```
pub trait UriExt {
    /// Returns true if this is a file URI (has 'file' scheme).
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Uri, UriExt};
    /// use std::str::FromStr;
    ///
    /// let file_uri = Uri::from_str("file:///src/main.rs").unwrap();
    /// assert!(file_uri.is_file_uri());
    ///
    /// let http_uri = Uri::from_str("http://example.com/main.rs").unwrap();
    /// assert!(!http_uri.is_file_uri());
    /// ```
    fn is_file_uri(&self) -> bool;

    /// Gets the filename component of the URI.
    ///
    /// # Returns
    ///
    /// * `Some(String)` - The filename if present
    /// * `None` - If the URI doesn't have a path or filename
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Uri, UriExt};
    /// use std::str::FromStr;
    ///
    /// let uri = Uri::from_str("file:///src/main.rs").unwrap();
    /// assert_eq!(uri.filename(), Some("main.rs".to_string()));
    ///
    /// let uri = Uri::from_str("file:///src/").unwrap();
    /// assert_eq!(uri.filename(), None); // No filename component
    /// ```
    fn filename(&self) -> Option<String>;

    /// Converts to a standard lsp_types::Location with the given range.
    ///
    /// # Arguments
    ///
    /// * `range` - The range within the document identified by this URI
    ///
    /// # Returns
    ///
    /// A Location object combining this URI with the provided range
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Uri, Range, Position, UriExt};
    /// use std::str::FromStr;
    ///
    /// let uri = Uri::from_str("file:///src/main.rs").unwrap();
    /// let range = Range::new(Position::new(10, 0), Position::new(10, 20));
    ///
    /// let location = uri.to_location(range.clone());
    /// assert_eq!(location.uri, uri);
    /// assert_eq!(location.range, range);
    /// ```
    fn to_location(&self, range: Range) -> Location;

    /// Extracts the file path from the URI.
    ///
    /// This only works for file:// URIs.
    ///
    /// # Returns
    ///
    /// * `Ok(PathBuf)` - The file path if it could be extracted
    /// * `Err(LocationError)` - If the URI is not a file URI or the path is
    ///   invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Uri, UriExt};
    /// use std::str::FromStr;
    ///
    /// let uri = Uri::from_str("file:///src/main.rs").unwrap();
    /// let path = uri.to_file_path();
    /// assert!(path.is_ok());
    ///
    /// let http_uri = Uri::from_str("http://example.com/main.rs").unwrap();
    /// let path = http_uri.to_file_path();
    /// assert!(path.is_err());
    /// ```
    fn to_file_path(&self) -> Result<PathBuf, LocationError>;

    /// Creates a new URI from a string, validating it's a file URI.
    ///
    /// # Arguments
    ///
    /// * `uri_str` - The URI string to parse
    ///
    /// # Returns
    ///
    /// * `Ok(Uri)` - If the URI is valid and has the file scheme
    /// * `Err(LocationError)` - If the URI is invalid or not a file URI
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Uri, UriExt};
    ///
    /// // Valid file URI
    /// let valid = Uri::new_file_uri("file:///src/main.rs");
    /// assert!(valid.is_ok());
    ///
    /// // Not a file URI
    /// let invalid = Uri::new_file_uri("http://example.com/main.rs");
    /// assert!(invalid.is_err());
    ///
    /// // Invalid URI syntax
    /// let invalid = Uri::new_file_uri("not a uri");
    /// assert!(invalid.is_err());
    /// ```
    fn new_file_uri(uri_str: &str) -> Result<Uri, LocationError>;
}

impl UriExt for Uri {
    fn is_file_uri(&self) -> bool {
        self.scheme().map(|scheme| scheme.as_str() == "file") == Some(true)
    }

    fn filename(&self) -> Option<String> {
        let filename = self.path().segments().next_back().map(|s| s.to_string());

        // Sometimes, segments can be empty strings
        match filename {
            Some(name) if name.is_empty() => None,
            _ => filename,
        }
    }

    fn to_location(&self, range: Range) -> Location {
        Location::new(self.clone(), range)
    }

    fn to_file_path(&self) -> Result<PathBuf, LocationError> {
        if !self.is_file_uri() {
            return Err(LocationError::InvalidUri(format!(
                "URI must use 'file' scheme, got {}",
                self.scheme()
                    .map(|scheme| scheme.as_str())
                    .unwrap_or("None")
            )));
        }

        Ok(PathBuf::from(self.as_str()))
    }

    fn new_file_uri(uri_str: &str) -> Result<Uri, LocationError> {
        let url =
            Uri::from_str(uri_str).map_err(|_| LocationError::InvalidUri(uri_str.to_string()))?;

        // Ensure this is a file URI
        if !url.is_file_uri() {
            return Err(LocationError::InvalidUri(format!(
                "URI must use 'file' scheme, got: {}",
                url.scheme().map(|scheme| scheme.as_str()).unwrap_or("None")
            )));
        }

        Ok(url)
    }
}

#[cfg(test)]
#[path = "tests/uri.rs"]
mod tests;
