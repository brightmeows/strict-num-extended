//! Serde serialization/deserialization tests
//!
//! These tests only run when the "serde" feature is enabled.

#![cfg(feature = "serde")]
#![expect(clippy::shadow_unrelated)] // Test functions commonly use the same local variable names

use strict_num_extended::*;

#[test]
fn test_serialize_deserialize_fin_f32() {
    let original = FinF32::new(std::f32::consts::PI).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: FinF32 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_nonnegative_f64() {
    let original = NonNegativeF64::new(42.0).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NonNegativeF64 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_normalized_f32() {
    let original = NormalizedF32::new(0.75).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NormalizedF32 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_error() {
    let error = FloatError::NaN;
    let json = serde_json::to_string(&error).unwrap();
    let deserialized: FloatError = serde_json::from_str(&json).unwrap();
    assert_eq!(error, deserialized);
}

#[test]
fn test_serialize_deserialize_symmetric_f64() {
    let original = SymmetricF64::new(0.5).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: SymmetricF64 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

// ==================== Deserialization validation tests ====================

#[test]
fn test_deserialize_fin_f32_from_json() {
    let json = "123.456";
    let value: FinF32 = serde_json::from_str(json).unwrap();
    assert!((value.get() - 123.456).abs() < 0.001);
}

#[test]
fn test_deserialize_nonnegative_f64_from_json() {
    let json = "42.5";
    let value: NonNegativeF64 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), 42.5);
}

#[test]
fn test_deserialize_normalized_f32_from_json() {
    let json = "0.75";
    let value: NormalizedF32 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), 0.75);
}

#[test]
fn test_deserialize_symmetric_f64_from_json() {
    let json = "-0.5";
    let value: SymmetricF64 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), -0.5);
}

// ==================== Deserialization failure cases ====================

// Note: JSON standard does not support NaN and Infinity as literals
// These values are rejected before being deserialized into f32/f64

#[test]
fn test_deserialize_nonnegative_f64_negative_fails() {
    let json = "-1.0";
    let result: Result<NonNegativeF64, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.to_lowercase().contains("range"));
}

#[test]
fn test_deserialize_normalized_f32_out_of_range_fails() {
    // Test upper bound exceeded
    let json = "1.5";
    let result: Result<NormalizedF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.to_lowercase().contains("range"));

    // Test lower bound exceeded
    let json = "-0.5";
    let result: Result<NormalizedF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.to_lowercase().contains("range"));
}

#[test]
fn test_deserialize_symmetric_f64_out_of_range_fails() {
    // Test upper bound exceeded
    let json = "1.5";
    let result: Result<SymmetricF64, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.to_lowercase().contains("range"));

    // Test lower bound exceeded
    let json = "-1.5";
    let result: Result<SymmetricF64, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.to_lowercase().contains("range"));
}

#[test]
fn test_deserialize_positive_f32_zero_fails() {
    let json = "0.0";
    let result: Result<PositiveF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.to_lowercase().contains("range"));
}

#[test]
fn test_deserialize_negative_f64_negative_zero_fails() {
    let json = "-0.0";
    let result: Result<NegativeF64, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.to_lowercase().contains("range"));
}

// ==================== Boundary value tests ====================

#[test]
fn test_deserialize_normalized_boundary_zero() {
    let json = "0.0";
    let value: NormalizedF32 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), 0.0);
}

#[test]
fn test_deserialize_normalized_boundary_one() {
    let json = "1.0";
    let value: NormalizedF32 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), 1.0);
}

#[test]
fn test_deserialize_symmetric_boundary_one() {
    let json = "1.0";
    let value: SymmetricF64 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), 1.0);
}

#[test]
fn test_deserialize_symmetric_boundary_negative_one() {
    let json = "-1.0";
    let value: SymmetricF64 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), -1.0);
}

#[test]
fn test_deserialize_nonnegative_boundary_zero() {
    let json = "0.0";
    let value: NonNegativeF64 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), 0.0);
}

#[test]
fn test_deserialize_negative_boundary_zero() {
    let json = "-0.00001";
    let value: NegativeF32 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), -0.00001);
}

// ==================== Complex data structure tests ====================

#[test]
fn test_deserialize_struct_with_finite_floats() {
    #[derive(serde::Deserialize)]
    struct Data {
        x: FinF32,
        y: NonNegativeF64,
    }

    let json = r#"{"x": 987.654, "y": 42.0}"#;
    let data: Data = serde_json::from_str(json).unwrap();
    assert!((data.x.get() - 987.654).abs() < 0.001);
    assert_eq!(data.y.get(), 42.0);
}

#[test]
fn test_deserialize_struct_with_validation_failure() {
    #[derive(serde::Deserialize)]
    struct Data {
        _x: FinF32,
        _y: NonNegativeF64,
    }

    // y field is negative, should fail
    let json = r#"{"x": 3.14, "y": -1.0}"#;
    let result: Result<Data, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_vec_of_normalized() {
    let json = "[0.0, 0.5, 1.0]";
    let values: Vec<NormalizedF32> = serde_json::from_str(json).unwrap();
    assert_eq!(values.len(), 3);
    let mut iter = values.iter();
    assert_eq!(iter.next().unwrap().get(), 0.0);
    assert_eq!(iter.next().unwrap().get(), 0.5);
    assert_eq!(iter.next().unwrap().get(), 1.0);
}

#[test]
fn test_deserialize_vec_of_normalized_with_invalid() {
    let json = "[0.0, 0.5, 1.5]";
    let result: Result<Vec<NormalizedF32>, _> = serde_json::from_str(json);
    assert!(result.is_err());
}
