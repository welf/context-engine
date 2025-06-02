#![allow(clippy::unwrap_used)]

use lsp_types::Position;
use proptest::prelude::*;

use super::*;

#[test]
fn test_uri_extension_methods() {
    // Test file URI detection
    let file_uri = Uri::from_str("file:///src/main.rs").unwrap();
    assert!(file_uri.is_file_uri());

    let http_uri = Uri::from_str("http://example.com/main.rs").unwrap();
    assert!(!http_uri.is_file_uri());

    // Test filename extraction
    assert_eq!(file_uri.filename(), Some("main.rs".to_string()));

    // Test to_location conversion
    let range = Range::new(Position::new(10, 0), Position::new(11, 0));
    let location = file_uri.to_location(range);
    assert_eq!(location.uri, file_uri);
    assert_eq!(location.range, range);

    // Test file path extraction
    let path_result = file_uri.to_file_path();
    assert!(path_result.is_ok());

    let http_path_result = http_uri.to_file_path();
    assert!(http_path_result.is_err());

    // Test new_file_uri validation
    let valid_uri = Uri::new_file_uri("file:///src/test.rs");
    assert!(valid_uri.is_ok());

    let invalid_scheme = Uri::new_file_uri("http://example.com/test.rs");
    assert!(invalid_scheme.is_err());

    let invalid_syntax = Uri::new_file_uri("not a uri");
    assert!(invalid_syntax.is_err());
}

// Property-based tests
proptest! {
    #[test]
    fn prop_uri_roundtrip(filename in "[a-zA-Z0-9_]+\\.rs") {
        let uri_str = format!("file:///{filename}");

        if let Ok(url) = Uri::from_str(&uri_str) {
            // Check filename extraction
            prop_assert_eq!(url.filename(), Some(filename));

            // Check to_string format
            prop_assert_eq!(url.to_string(), uri_str.clone());

            // Check range to location conversion
            let range = Range::new(Position::new(0, 0), Position::new(1, 0));
            let location = url.to_location(range);
            prop_assert_eq!(location.uri.as_str(), uri_str);
            prop_assert_eq!(location.range, range);
        }
    }

    #[test]
    fn prop_uri_serialization_roundtrip(filename in "[a-zA-Z0-9_]+\\.rs") {
        let uri_str = format!("file:///{filename}");

        if Uri::from_str(&uri_str).is_ok() {
            // Binary serialization / deserialization
            let config = bincode::config::standard();
            let range = Range::new(Position::new(0, 0), Position::new(1, 0));
            let serialized = bincode::serde::encode_to_vec(range, config).unwrap();
            let (deserialized_range, _): (Range, _) = bincode::serde::decode_from_slice(&serialized, config).unwrap();
            prop_assert_eq!(deserialized_range, range);

            // JSON serialization / deserialization
            let json = serde_json::to_string(&range).unwrap();
            let deserialized_range: Range = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(deserialized_range, range);
        }
    }

    #[test]
    fn prop_uri_filename(filename in "[a-zA-Z0-9_]*") {
        let uri_str = format!("file:///path/to/{filename}");

        if let Ok(uri) = Uri::from_str(&uri_str) {
            // Check filename extraction
            if filename.is_empty() {
                prop_assert_eq!(uri.filename(), None);
            } else {
                prop_assert_eq!(uri.filename(), Some(filename));
                // Check to_string format
                prop_assert_eq!(uri.to_string(), uri_str.clone());
            }


            // Check range to location conversion
            let range = Range::new(Position::new(0, 0), Position::new(1, 0));
            let location = uri.to_location(range);
            prop_assert_eq!(location.uri.as_str(), uri_str);
            prop_assert_eq!(location.range, range);
        }
    }
}
