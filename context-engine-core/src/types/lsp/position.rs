//! Extensions for working with code positions.
//!
//! This module provides extension methods for the [`lsp_types::Position`] type,
//! adding functionality specific to the Context Engine's needs.

use lsp_types::Position;

/// Extension methods for Position
///
/// This trait adds helpful methods to the standard LSP Position type.
///
/// # Examples
///
/// ```
/// use context_engine_core::types::{Position, PositionExt};
///
/// // Create two positions
/// let pos1 = Position::new(10, 5);
/// let pos2 = Position::new(10, 20);
///
/// // Check if one is before the other
/// assert!(pos1.is_before(&pos2));
/// assert!(pos2.is_after(&pos1));
/// ```
pub trait PositionExt {
    /// Returns true if this position is before the other position.
    ///
    /// A position is considered "before" another if:
    /// - It has a smaller line number, or
    /// - The line numbers are equal, but it has a smaller character position
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Position, PositionExt};
    ///
    /// let pos1 = Position::new(10, 5);  // Line 10, character 5
    /// let pos2 = Position::new(10, 10); // Line 10, character 10
    /// let pos3 = Position::new(11, 0);  // Line 11, character 0
    ///
    /// assert!(pos1.is_before(&pos2)); // Same line, earlier character
    /// assert!(pos1.is_before(&pos3)); // Earlier line
    /// assert!(pos2.is_before(&pos3)); // Earlier line
    /// assert!(!pos2.is_before(&pos1)); // Not before
    /// ```
    fn is_before(&self, other: &Position) -> bool;

    /// Returns true if this position is after the other position.
    ///
    /// A position is considered "after" another if:
    /// - It has a larger line number, or
    /// - The line numbers are equal, but it has a larger character position
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Position, PositionExt};
    ///
    /// let pos1 = Position::new(10, 5);  // Line 10, character 5
    /// let pos2 = Position::new(10, 10); // Line 10, character 10
    ///
    /// assert!(pos2.is_after(&pos1)); // Same line, later character
    /// assert!(!pos1.is_after(&pos2)); // Not after
    /// ```
    fn is_after(&self, other: &Position) -> bool;
}

impl PositionExt for Position {
    fn is_before(&self, other: &Position) -> bool {
        self.line < other.line || (self.line == other.line && self.character < other.character)
    }

    fn is_after(&self, other: &Position) -> bool {
        self.line > other.line || (self.line == other.line && self.character > other.character)
    }
}

#[cfg(test)]
#[path = "tests/position.rs"]
mod tests;
