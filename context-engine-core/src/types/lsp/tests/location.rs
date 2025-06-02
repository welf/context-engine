#![allow(clippy::unwrap_used)]

use lsp_types::Position;
use proptest::prelude::*;

use super::*;

#[test]
fn test_location_extension_methods() {
    // Test valid location creation
    let range = Range::new(Position::new(10, 5), Position::new(11, 0));
    let location_result = Location::validated("file:///test.rs", range);
    assert!(location_result.is_ok());

    let location = location_result.unwrap();

    // Test filename extraction
    assert_eq!(location.filename(), Some("test.rs".to_string()));

    // Test invalid URI
    assert!(Location::validated("invalid-uri", range).is_err());
    let path = "http://test.rs";
    let invalid_location = Location::validated(path, range);
    assert!(invalid_location.is_err());
    assert!(
        invalid_location
            .err()
            .unwrap()
            .to_string()
            .contains("URI must use 'file' scheme")
    );
    let invalid_range = Range::new(Position::new(10, 5), Position::new(9, 0));
    let invalid_location = Location::validated("file:///test.rs", invalid_range);
    assert!(invalid_location.is_err());
    assert!(
        invalid_location
            .err()
            .unwrap()
            .to_string()
            .contains("Invalid range: start position occurs after end position")
    );
}

// Property-based tests
proptest! {
    #[test]
    fn prop_location_roundtrip(
        filename in "[a-zA-Z0-9_]+\\.rs",
        start_line in 0u32..1000,
        end_line in 0u32..1000,
        start_char in 0u32..1000,
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

        let start_position = Position::new(start_line, start_char);
        let end_position = Position::new(end_line, end_char);

        let uri_str = format!("file:///{filename}");
        let location_result = Location::validated(&uri_str, Range::validated(start_position, end_position).unwrap());
        assert!(location_result.is_ok());

        let location = location_result.unwrap();
        assert_eq!(location.filename().unwrap().as_str(), filename);
    }

    #[test]
    fn prop_location_serialization_roundtrip(
        filename in "[a-zA-Z0-9_]+\\.rs",
        start_line in 0u32..1000,
        end_line in 0u32..1000,
        start_char in 0u32..1000,
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

        let start_position = Position::new(start_line, start_char);
        let end_position = Position::new(end_line, end_char);

        let uri_str = format!("file:///{filename}");
        let location_result = Location::validated(&uri_str, Range::validated(start_position, end_position).unwrap()).unwrap();

        // Test binary serialization and deserialization
        let config = bincode::config::standard();
        let serialized = bincode::serde::encode_to_vec(&location_result, config).unwrap();
        let (deserialized, _len): (Location, usize) = bincode::serde::decode_from_slice(&serialized, config).unwrap();
        assert_eq!(location_result, deserialized);

        // Test JSON serialization and deserialization
        let json_str = serde_json::to_string(&location_result).unwrap();
        let deserialized_json: Location = serde_json::from_str(&json_str).unwrap();
        assert_eq!(location_result, deserialized_json);
    }
}
