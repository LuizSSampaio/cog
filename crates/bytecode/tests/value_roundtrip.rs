use bytecode::types::Type;
use bytecode::values::{Value, ValueError};
use proptest::prelude::*;
use std::convert::TryFrom;

/// Helper function to perform a roundtrip: Value -> Vec<u8> -> Value
fn roundtrip(value: Value) -> Result<Value, ValueError> {
    let bytes: Vec<u8> = value.into();
    Value::try_from(bytes)
}

/// Assert that two values are equal with special handling for floats.
/// Floats are compared by their bit representation to handle correctly.
fn assert_value_eq_roundtrip(original: &Value, roundtripped: &Value) {
    match (original, roundtripped) {
        (Value::Int(i1), Value::Int(i2)) => assert_eq!(i1, i2, "Int values differ"),
        (Value::Float(f1), Value::Float(f2)) => {
            assert_eq!(
                f1.to_bits(),
                f2.to_bits(),
                "Float values differ (comparing bits)"
            )
        }
        (Value::Bool(b1), Value::Bool(b2)) => assert_eq!(b1, b2, "Bool values differ"),
        (Value::Str(s1), Value::Str(s2)) => assert_eq!(s1, s2, "String values differ"),
        (Value::Char(c1), Value::Char(c2)) => assert_eq!(c1, c2, "Char values differ"),
        _ => panic!(
            "Type mismatch: original = {:?}, roundtripped = {:?}",
            original, roundtripped
        ),
    }
}

// Strategy for generating chars that are compatible with current encoding.
// Current implementation stores char as a single u8, so we limit to 0..=255.
fn char_strategy() -> impl Strategy<Value = char> {
    (0u8..=255u8).prop_map(|b| b as char)
}

// Strategy for generating valid UTF-8 strings of reasonable length.
// We use a restricted char set to ensure valid UTF-8 and reasonable test performance.
fn string_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(char_strategy(), 0..=256).prop_map(|chars| chars.into_iter().collect())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(2000))]

    /// Test that Int values roundtrip correctly through Vec<u8> serialization
    #[test]
    #[allow(clippy::expect_used)]
    fn test_int_roundtrip(value in any::<isize>()) {
        let original = Value::Int(value);
        let roundtripped = roundtrip(original.clone())
            .expect("Int roundtrip should succeed");
        assert_value_eq_roundtrip(&original, &roundtripped);
    }

    /// Test that Float values roundtrip correctly through Vec<u8> serialization
    /// This includes special values like NaN, Infinity, -Infinity, and -0.0
    #[test]
    #[allow(clippy::expect_used)]
    fn test_float_roundtrip(value in any::<f64>()) {
        let original = Value::Float(value);
        let roundtripped = roundtrip(original.clone())
            .expect("Float roundtrip should succeed");
        assert_value_eq_roundtrip(&original, &roundtripped);
    }

    /// Test that Bool values roundtrip correctly through Vec<u8> serialization
    #[test]
    #[allow(clippy::expect_used)]
    fn test_bool_roundtrip(value in any::<bool>()) {
        let original = Value::Bool(value);
        let roundtripped = roundtrip(original.clone())
            .expect("Bool roundtrip should succeed");
        assert_value_eq_roundtrip(&original, &roundtripped);
    }

    /// Test that String values roundtrip correctly through Vec<u8> serialization
    #[test]
    #[allow(clippy::expect_used)]
    fn test_string_roundtrip(value in string_strategy()) {
        let original = Value::Str(value);
        let roundtripped = roundtrip(original.clone())
            .expect("String roundtrip should succeed");
        assert_value_eq_roundtrip(&original, &roundtripped);
    }

    /// Test that Char values roundtrip correctly through Vec<u8> serialization
    /// Limited to u8 range (0..=255) due to current implementation constraint
    #[test]
    #[allow(clippy::expect_used)]
    fn test_char_roundtrip(value in char_strategy()) {
        let original = Value::Char(value);
        let roundtripped = roundtrip(original.clone())
            .expect("Char roundtrip should succeed");
        assert_value_eq_roundtrip(&original, &roundtripped);
    }
}

// Negative tests for malformed buffers

#[test]
fn test_empty_buffer_returns_no_tag_error() {
    let result = Value::try_from(Vec::new());
    assert!(result.is_err(), "Empty buffer should return error");
    match result {
        Err(ValueError::NoTag) => {} // Expected
        other => panic!("Expected NoTag error, got: {:?}", other),
    }
}

#[test]
fn test_int_with_wrong_size_returns_error() {
    // Int type tag but only 4 bytes instead of required 8
    let buffer = vec![Type::Int as u8, 1, 2, 3, 4];
    let result = Value::try_from(buffer);
    assert!(result.is_err(), "Int with wrong size should return error");
    match result {
        Err(ValueError::IncompatibleSize) => {} // Expected
        other => panic!("Expected IncompatibleSize error, got: {:?}", other),
    }
}

#[test]
fn test_float_with_wrong_size_returns_error() {
    // Float type tag but only 3 bytes instead of required 8
    let buffer = vec![Type::Float as u8, 1, 2, 3];
    let result = Value::try_from(buffer);
    assert!(result.is_err(), "Float with wrong size should return error");
    match result {
        Err(ValueError::IncompatibleSize) => {} // Expected
        other => panic!("Expected IncompatibleSize error, got: {:?}", other),
    }
}

#[test]
fn test_bool_with_wrong_size_returns_error() {
    // Bool type tag but 3 bytes instead of required 1
    let buffer = vec![Type::Bool as u8, 1, 2, 3];
    let result = Value::try_from(buffer);
    assert!(result.is_err(), "Bool with wrong size should return error");
    match result {
        Err(ValueError::IncompatibleSize) => {} // Expected
        other => panic!("Expected IncompatibleSize error, got: {:?}", other),
    }
}

#[test]
fn test_char_with_wrong_size_returns_error() {
    // Char type tag but 3 bytes instead of required 1
    let buffer = vec![Type::Char as u8, 1, 2, 3];
    let result = Value::try_from(buffer);
    assert!(result.is_err(), "Char with wrong size should return error");
    match result {
        Err(ValueError::IncompatibleSize) => {} // Expected
        other => panic!("Expected IncompatibleSize error, got: {:?}", other),
    }
}

#[test]
fn test_string_with_insufficient_data_returns_error() {
    // String type tag but less than 4 bytes for length prefix
    let buffer = vec![Type::Str as u8, 1, 2];
    let result = Value::try_from(buffer);
    assert!(
        result.is_err(),
        "String with insufficient data should return error"
    );
    match result {
        Err(ValueError::IncompatibleSize) => {} // Expected
        other => panic!("Expected IncompatibleSize error, got: {:?}", other),
    }
}

#[test]
fn test_string_with_mismatched_length_returns_error() {
    // String type tag with length=10 but only 3 bytes of data
    let mut buffer = vec![Type::Str as u8];
    buffer.extend_from_slice(&10u32.to_le_bytes()); // Claim 10 bytes
    buffer.extend_from_slice(&[65, 66, 67]); // But only provide 3 bytes
    let result = Value::try_from(buffer);
    assert!(
        result.is_err(),
        "String with mismatched length should return error"
    );
    match result {
        Err(ValueError::IncompatibleSize) => {} // Expected
        other => panic!("Expected IncompatibleSize error, got: {:?}", other),
    }
}

#[test]
fn test_invalid_type_tag_returns_error() {
    // Invalid type tag (not in range 0x20..=0x24)
    let buffer = vec![0xFF, 1, 2, 3, 4, 5, 6, 7, 8];
    let result = Value::try_from(buffer);
    assert!(result.is_err(), "Invalid type tag should return error");
    match result {
        Err(ValueError::Type(_)) => {} // Expected
        other => panic!("Expected TypeError, got: {:?}", other),
    }
}

// Edge case tests for specific values

#[test]
#[allow(clippy::expect_used)]
fn test_float_special_values_roundtrip() {
    let special_values = vec![
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        0.0,
        -0.0,
        f64::MIN,
        f64::MAX,
        f64::EPSILON,
    ];

    for &value in &special_values {
        let original = Value::Float(value);
        let roundtripped = roundtrip(original.clone()).expect("Float roundtrip should succeed");
        assert_value_eq_roundtrip(&original, &roundtripped);
    }
}

#[test]
#[allow(clippy::expect_used)]
fn test_string_edge_cases_roundtrip() {
    let test_cases = vec![
        String::from(""),              // Empty string
        String::from("a"),             // Single char
        String::from("Hello, World!"), // ASCII
        String::from("🦀"),            // Emoji (multi-byte UTF-8)
        String::from("日本語"),        // Non-ASCII characters
        "a".repeat(1000),              // Long string
    ];

    for value in test_cases {
        let original = Value::Str(value);
        let roundtripped = roundtrip(original.clone()).expect("String roundtrip should succeed");
        assert_value_eq_roundtrip(&original, &roundtripped);
    }
}

#[test]
#[allow(clippy::expect_used)]
fn test_int_edge_cases_roundtrip() {
    let edge_cases = vec![isize::MIN, isize::MAX, 0, -1, 1, -1000, 1000];

    for &value in &edge_cases {
        let original = Value::Int(value);
        let roundtripped = roundtrip(original.clone()).expect("Int roundtrip should succeed");
        assert_value_eq_roundtrip(&original, &roundtripped);
    }
}
