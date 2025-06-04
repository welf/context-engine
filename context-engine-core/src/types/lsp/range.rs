//! Extensions for working with code ranges.
//!
//! This module provides extension methods for the [`lsp_types::Range`] type,
//! adding functionality specific to the Context Engine's needs.

use lsp_types::{Position, Range};

use crate::types::{LocationError, PositionExt};

/// Extension methods for Range
///
/// This trait adds helpful methods to the standard LSP Range type.
///
/// # Examples
///
/// ```
/// use context_engine_core::types::{Position, Range, PositionExt, RangeExt};
///
/// // Create a range
/// let start = Position::new(10, 0);
/// let end = Position::new(10, 20);
/// let range = Range::validated(start, end).unwrap();
///
/// // Check if a position is within the range
/// let pos = Position::new(10, 10);
/// assert!(range.contains_position(&pos));
///
/// // Invalid ranges will return an error
/// let invalid = Range::validated(end, start);
/// assert!(invalid.is_err());
/// ```
pub trait RangeExt {
    /// Returns true if the range contains the given position.
    ///
    /// A position is contained in a range if:
    /// - It is equal to or after the start position, AND
    /// - It is equal to or before the end position
    ///
    /// # Arguments
    ///
    /// * `position` - The position to check
    ///
    /// # Returns
    ///
    /// `true` if the position is contained within the range, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Position, Range, RangeExt};
    ///
    /// let range = Range::validated(
    ///     Position::new(10, 5),   // Start position
    ///     Position::new(10, 20)   // End position
    /// ).unwrap();
    ///
    /// // Position within range
    /// let within = Position::new(10, 10);
    /// assert!(range.contains_position(&within));
    ///
    /// // Position at start boundary
    /// let at_start = Position::new(10, 5);
    /// assert!(range.contains_position(&at_start));
    ///
    /// // Position at end boundary
    /// let at_end = Position::new(10, 20);
    /// assert!(range.contains_position(&at_end));
    ///
    /// // Position before range
    /// let before = Position::new(10, 4);
    /// assert!(!range.contains_position(&before));
    ///
    /// // Position after range
    /// let after = Position::new(10, 21);
    /// assert!(!range.contains_position(&after));
    /// ```
    fn contains_position(&self, position: &Position) -> bool;

    /// Returns true if the range is empty.
    ///
    /// The [`Range`] is considered empty if its start and end
    /// positions are equal.
    ///
    /// # Returns
    /// * `true` if the range is empty
    /// * `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Position, Range, RangeExt};
    ///
    /// // Empty range
    /// let empty_range = Range::validated(
    ///     Position::new(10, 5),
    ///     Position::new(10, 5)
    /// ).unwrap();
    /// assert!(empty_range.is_empty());
    ///
    /// // Non-empty range
    /// let non_empty_range = Range::new(Position::new(10, 5), Position::new(10, 20));
    /// assert!(!non_empty_range.is_empty());
    /// ```
    fn is_empty(&self) -> bool;

    /// Creates a new [`Range`], validating that start comes
    /// before end.
    ///
    /// This method ensures that the start position is before or equal to
    /// the end position, according to the Position ordering rules.
    ///
    /// # Arguments
    ///
    /// * `start` - The start position of the range
    /// * `end` - The end position of the range
    ///
    /// # Returns
    ///
    /// * `Ok(Range)` - If the range is valid
    /// * `Err(LocationError)` - If the range is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Position, Range, RangeExt};
    ///
    /// // Valid range - start is before end
    /// let start = Position::new(10, 5);
    /// let end = Position::new(10, 20);
    /// let valid = Range::validated(start, end);
    /// assert!(valid.is_ok());
    ///
    /// // Invalid range - start is after end
    /// let invalid = Range::validated(end, start);
    /// assert!(invalid.is_err());
    /// ```
    fn validated(start: Position, end: Position) -> Result<Range, LocationError>;

    /// Returns true if the range contains the given range
    ///
    /// A range is contained in another range if:
    /// 1. Its start position is contained in the other range, AND
    /// 2. Its end position is contained in the other range
    ///
    /// # Arguments
    ///
    /// * `range` - The range to check
    ///
    /// # Returns
    ///
    /// * `bool` - True if the range contains the given range, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use context_engine_core::types::{Position, Range, RangeExt};
    ///
    /// let range = Range::validated(Position::new(10, 5), Position::new(10, 20)).unwrap();
    /// let contained = Range::validated(Position::new(10, 10), Position::new(10, 15)).unwrap();
    /// let not_contained = Range::validated(Position::new(10, 10), Position::new(10, 25)).unwrap();
    ///
    /// assert!(range.contains_range(&contained));
    /// assert!(!range.contains_range(&not_contained));
    /// ```
    fn contains_range(&self, range: &Range) -> bool {
        self.contains_position(&range.start) && self.contains_position(&range.end)
    }
}

impl RangeExt for Range {
    fn contains_position(&self, position: &Position) -> bool {
        // A position is contained in a range if:
        // 1. It comes after or at the start position, AND
        // 2. It comes before or at the end position
        !position.is_before(&self.start) && !position.is_after(&self.end)
    }

    fn is_empty(&self) -> bool {
        self.start == self.end
    }

    fn validated(start: Position, end: Position) -> Result<Range, LocationError> {
        if start.is_after(&end) {
            return Err(LocationError::InvalidRange);
        }

        Ok(Range::new(start, end))
    }
}

#[cfg(test)]
#[path = "tests/range.rs"]
mod tests;
