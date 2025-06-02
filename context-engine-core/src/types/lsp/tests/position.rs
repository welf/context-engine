#![allow(clippy::unwrap_used)]

use super::*;
use proptest::prelude::*;
use serde_json::{from_str, to_string};

#[test]
fn test_position_extension_methods() {
    let pos1 = Position::new(10, 5);
    let pos2 = Position::new(10, 10);
    let pos3 = Position::new(11, 0);

    // Test is_before
    assert!(pos1.is_before(&pos2));
    assert!(pos1.is_before(&pos3));
    assert!(pos2.is_before(&pos3));
    assert!(!pos2.is_before(&pos1));
    assert!(!pos3.is_before(&pos1));
    assert!(!pos3.is_before(&pos2));

    // Test is_after
    assert!(pos2.is_after(&pos1));
    assert!(pos3.is_after(&pos1));
    assert!(pos3.is_after(&pos2));
    assert!(!pos1.is_after(&pos2));
    assert!(!pos1.is_after(&pos3));
    assert!(!pos2.is_after(&pos3));
}

#[test]
fn test_position_comparison_consistency() {
    let test_positions = [
        Position::new(0, 0),
        Position::new(0, 5),
        Position::new(1, 0),
        Position::new(10, 20),
    ];

    for (i, pos_a) in test_positions.iter().enumerate() {
        for (j, pos_b) in test_positions.iter().enumerate() {
            if i < j {
                assert!(pos_a.is_before(pos_b));
                assert!(pos_b.is_after(pos_a));
            } else if i > j {
                assert!(pos_a.is_after(pos_b));
                assert!(pos_b.is_before(pos_a));
            } else {
                assert!(!pos_a.is_before(pos_b));
                assert!(!pos_a.is_after(pos_b));
            }
        }
    }
}

// Property-based tests
proptest! {
    #[test]
    fn prop_position_ordering_is_transitive(
        a_line in 0u32..1000,
        a_char in 0u32..1000,
        b_line in 0u32..1000,
        b_char in 0u32..1000,
        c_line in 0u32..1000,
        c_char in 0u32..1000
    ) {
        let pos_a = Position::new(a_line, a_char);
        let pos_b = Position::new(b_line, b_char);
        let pos_c = Position::new(c_line, c_char);

        // Test transitivity of is_before
        if pos_a.is_before(&pos_b) && pos_b.is_before(&pos_c) {
            prop_assert!(pos_a.is_before(&pos_c));
        }

        // Test transitivity of is_after
        if pos_a.is_after(&pos_b) && pos_b.is_after(&pos_c) {
            prop_assert!(pos_a.is_after(&pos_c));
        }
    }

    #[test]
    fn prop_position_binary_serialization_roundtrip(
        line in 0u32..1000,
        ch in 0u32..1000,
    ) {
        let pos = Position::new(line, ch);

        let config = bincode::config::standard();

        let encoded = bincode::serde::encode_to_vec(pos, config).unwrap();
        let (decoded, _): (Position, usize) =
            bincode::serde::decode_from_slice(&encoded, config).unwrap();

        prop_assert_eq!(pos, decoded);
    }

    #[test]
    fn prop_position_json_serialization_roundtrip(
        line in 0u32..1000,
        ch in 0u32..1000,
    ) {
        let pos = Position::new(line, ch);


        // Serialize to JSON
        let json_str = to_string(&pos).unwrap();
        // Deserialize from JSON
        let deserialized: Position = from_str(&json_str).unwrap();

        prop_assert_eq!(pos, deserialized);
    }
}
