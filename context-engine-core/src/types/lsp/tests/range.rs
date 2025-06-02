#![allow(clippy::unwrap_used)]

use proptest::prelude::*;

use super::*;

#[test]
fn test_range_extension_methods() {
    let before_start = Position::new(9, 0);
    let start = Position::new(10, 5);
    let next_to_start = Position::new(10, 6);
    let mid = Position::new(10, 10);
    let end = Position::new(11, 0);
    let next_to_end = Position::new(11, 1);

    // Create a valid range
    let range = Range::validated(start, end).unwrap();
    let empty_range = Range::validated(start, start).unwrap();
    let contained_range_1 = Range::validated(start, mid).unwrap();
    let contained_range_2 = Range::validated(next_to_start, end).unwrap();
    let contained_range_3 = Range::validated(next_to_start, mid).unwrap();
    let not_contained_range_1 = Range::validated(before_start, start).unwrap();
    let not_contained_range_2 = Range::validated(start, next_to_end).unwrap();
    let not_contained_range_3 = Range::validated(next_to_start, next_to_end).unwrap();
    let not_contained_range_4 = Range::validated(before_start, next_to_end).unwrap();

    // Tests emptiness
    assert!(!range.is_empty());
    assert!(empty_range.is_empty());

    // Test contains_position
    assert!(range.contains_position(&start));
    assert!(range.contains_position(&mid));
    assert!(range.contains_position(&end));

    let before = Position::new(9, 0);
    let after = Position::new(11, 1);

    assert!(!range.contains_position(&before));
    assert!(!range.contains_position(&after));

    // Test contains_range
    assert!(range.contains_range(&empty_range));
    assert!(range.contains_range(&contained_range_1));
    assert!(range.contains_range(&contained_range_2));
    assert!(range.contains_range(&contained_range_3));
    assert!(!range.contains_range(&not_contained_range_1));
    assert!(!range.contains_range(&not_contained_range_2));
    assert!(!range.contains_range(&not_contained_range_3));
    assert!(!range.contains_range(&not_contained_range_4));

    // Test range validation
    assert!(Range::validated(start, end).is_ok());
    assert!(Range::validated(start, mid).is_ok());
    assert!(Range::validated(mid, end).is_ok());

    // Invalid ranges (start after end)
    assert!(Range::validated(end, start).is_err());
    assert!(Range::validated(end, mid).is_err());
    assert!(Range::validated(mid, start).is_err());
}

// Property-based tests
proptest! {
    #[test]
    fn prop_range_contains_all_intermediate_positions(
        start_line in 0u32..1000,
        start_char in 0u32..1000,
        end_line in 0u32..1000,
        end_char in 0u32..1000
    ) {
        // Ensure start is before end
        let (start_line, end_line, start_char, end_char) = if start_line > end_line {
            (end_line, start_line, end_char, start_char)
        } else if start_line == end_line && start_char > end_char {
            (start_line, end_line, end_char, start_char)
        } else {
            (start_line, end_line, start_char, end_char)
        };

        let start = Position::new(start_line, start_char);
        let end = Position::new(end_line, end_char);

        if let Ok(range) = Range::validated(start, end) {
            // Check that start and end are contained
            prop_assert!(range.contains_position(&start));
            prop_assert!(range.contains_position(&end));

            // Check a few points in between
            if start_line == end_line {
                // Same line, check middle character
                if start_char < end_char {
                    let mid_char = start_char + (end_char - start_char) / 2;
                    let mid = Position::new(start_line, mid_char);
                    prop_assert!(range.contains_position(&mid));
                }
            }

            // If there is a line between start and end, check middle line
            if end_line - start_line > 1 {
                // Different lines, check middle line
                let mid_line = start_line + (end_line - start_line) / 2;
                let mid = Position::new(mid_line, 0);
                prop_assert!(range.contains_position(&mid));
            }
        }
    }

    #[test]
    fn prop_range_serialization_roundtrip(
        start_line in 0u32..1000,
        start_char in 0u32..1000,
        end_line in 0u32..1000,
        end_char in 0u32..1000
    ) {
        // Ensure start is before end
        let (start_line, end_line, start_char, end_char) = if start_line > end_line {
            (end_line, start_line, end_char, start_char)
        } else if start_line == end_line && start_char > end_char {
            (start_line, end_line, end_char, start_char)
        } else {
            (start_line, end_line, start_char, end_char)
        };

        let start = Position::new(start_line, start_char);
        let end = Position::new(end_line, end_char);

        if let Ok(range) = Range::validated(start, end) {
            // Binary serialization
            let config = bincode::config::standard();
            let serialized = bincode::serde::encode_to_vec(range, config).unwrap();
            let (deserialized, _): (Range, _) = bincode::serde::decode_from_slice(&serialized, config).unwrap();
            assert_eq!(range, deserialized);

            // JSON serialization
            let serialized = serde_json::to_string(&range).unwrap();
            let deserialized = serde_json::from_str(&serialized).unwrap();
            assert_eq!(range, deserialized);
        }
    }
}
