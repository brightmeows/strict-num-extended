//! Comprehensive serde serialization/deserialization tests
//!
//! These tests only run when the "serde" feature is enabled.
//! Tests cover all generated types and verify actual error types.

#![cfg(feature = "serde")]
#![expect(clippy::shadow_unrelated, clippy::uninlined_format_args)] // Test functions commonly use the same local variable names

use strict_num_extended::*;

// ==================== Serialization/Deserialization for all types ====================

#[test]
fn test_serialize_deserialize_fin_f32() {
    let original = FinF32::new(std::f32::consts::PI).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: FinF32 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_fin_f64() {
    let original = FinF64::new(std::f64::consts::PI).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: FinF64 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_nonnegative_f32() {
    let original = NonNegativeF32::new(1.5).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NonNegativeF32 = serde_json::from_str(&json).unwrap();
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
fn test_serialize_deserialize_nonpositive_f32() {
    let original = NonPositiveF32::new(-1.5).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NonPositiveF32 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_nonpositive_f64() {
    let original = NonPositiveF64::new(-42.0).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NonPositiveF64 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_nonzero_f32() {
    let original = NonZeroF32::new(1.5).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NonZeroF32 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_nonzero_f64() {
    let original = NonZeroF64::new(42.5).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NonZeroF64 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_positive_f32() {
    let original = PositiveF32::new(1.5).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PositiveF32 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_positive_f64() {
    let original = PositiveF64::new(42.5).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PositiveF64 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_negative_f32() {
    let original = NegativeF32::new(-1.5).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NegativeF32 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_negative_f64() {
    let original = NegativeF64::new(-42.5).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NegativeF64 = serde_json::from_str(&json).unwrap();
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
fn test_serialize_deserialize_normalized_f64() {
    let original = NormalizedF64::new(0.25).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NormalizedF64 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_negative_normalized_f32() {
    let original = NegativeNormalizedF32::new(-0.75).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NegativeNormalizedF32 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_negative_normalized_f64() {
    let original = NegativeNormalizedF64::new(-0.25).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NegativeNormalizedF64 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_symmetric_f32() {
    let original = SymmetricF32::new(0.5).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: SymmetricF32 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_symmetric_f64() {
    let original = SymmetricF64::new(-0.5).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: SymmetricF64 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

// ==================== Error type tests ====================

#[test]
fn test_serialize_deserialize_error() {
    for error_variant in [
        FloatError::NaN,
        FloatError::PosInf,
        FloatError::NegInf,
        FloatError::OutOfRange,
    ] {
        let json = serde_json::to_string(&error_variant).unwrap();
        let deserialized: FloatError = serde_json::from_str(&json).unwrap();
        assert_eq!(error_variant, deserialized);
    }
}

// ==================== Type-specific validation failure tests ====================

#[test]
fn test_nonnegative_f32_rejects_negative() {
    let json = "-1.0";
    let result: Result<NonNegativeF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_nonnegative_f64_rejects_negative() {
    let json = "-1.0";
    let result: Result<NonNegativeF64, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_nonpositive_f32_rejects_positive() {
    let json = "1.0";
    let result: Result<NonPositiveF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_nonpositive_f64_rejects_positive() {
    let json = "1.0";
    let result: Result<NonPositiveF64, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_nonzero_f32_rejects_zero() {
    let json = "0.0";
    let result: Result<NonZeroF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_nonzero_f64_rejects_zero() {
    let json = "0.0";
    let result: Result<NonZeroF64, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_positive_f32_rejects_zero() {
    let json = "0.0";
    let result: Result<PositiveF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_positive_f64_rejects_negative() {
    let json = "-1.0";
    let result: Result<PositiveF64, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_negative_f32_rejects_zero() {
    let json = "0.0";
    let result: Result<NegativeF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_negative_f64_rejects_positive() {
    let json = "1.0";
    let result: Result<NegativeF64, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_normalized_f32_rejects_above_one() {
    let json = "1.5";
    let result: Result<NormalizedF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_normalized_f32_rejects_below_zero() {
    let json = "-0.5";
    let result: Result<NormalizedF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_normalized_f64_rejects_out_of_range() {
    // Test upper bound
    let json = "1.1";
    let result: Result<NormalizedF64, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // Test lower bound
    let json = "-0.1";
    let result: Result<NormalizedF64, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_negative_normalized_f32_rejects_positive() {
    let json = "0.5";
    let result: Result<NegativeNormalizedF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_negative_normalized_f32_rejects_below_minus_one() {
    let json = "-1.5";
    let result: Result<NegativeNormalizedF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_symmetric_f32_rejects_above_one() {
    let json = "1.5";
    let result: Result<SymmetricF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_symmetric_f32_rejects_below_minus_one() {
    let json = "-1.5";
    let result: Result<SymmetricF32, _> = serde_json::from_str(json);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.to_lowercase().contains("range"),
        "Error should mention out of range: {}",
        err_str
    );
}

#[test]
fn test_symmetric_f64_rejects_out_of_range() {
    // Test upper bound
    let json = "1.1";
    let result: Result<SymmetricF64, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // Test lower bound
    let json = "-1.1";
    let result: Result<SymmetricF64, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// ==================== Boundary value tests ====================

#[test]
fn test_deserialize_nonnegative_boundary_zero() {
    let json = "0.0";
    let value: NonNegativeF32 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), 0.0);
}

#[test]
fn test_deserialize_nonpositive_boundary_zero() {
    let json = "0.0";
    let value: NonPositiveF32 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), 0.0);
}

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
fn test_deserialize_negative_normalized_boundary_zero() {
    let json = "0.0";
    let value: NegativeNormalizedF32 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), 0.0);
}

#[test]
fn test_deserialize_negative_normalized_boundary_minus_one() {
    let json = "-1.0";
    let value: NegativeNormalizedF32 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), -1.0);
}

#[test]
fn test_deserialize_symmetric_boundary_one() {
    let json = "1.0";
    let value: SymmetricF32 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), 1.0);
}

#[test]
fn test_deserialize_symmetric_boundary_minus_one() {
    let json = "-1.0";
    let value: SymmetricF32 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), -1.0);
}

#[test]
fn test_deserialize_symmetric_boundary_zero() {
    let json = "0.0";
    let value: SymmetricF32 = serde_json::from_str(json).unwrap();
    assert_eq!(value.get(), 0.0);
}
