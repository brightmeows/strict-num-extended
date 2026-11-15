//! # Trigonometric Operation Tests
//!
//! Comprehensive test of `sin()`, `cos()`, and `tan()` operations
//!
//! These tests are only compiled when the `std` feature is enabled, as
//! trigonometric functions are not available in `no_std` environments.

#![cfg(feature = "std")]
// Strict floating-point comparisons and unwrap usage in test code are justified
#![expect(clippy::no_effect_underscore_binding)]

use strict_num_extended::*;

// ============================================================================
// sin() Operation Tests
// ============================================================================

#[test]
fn test_sin_basic() {
    let angle = FinF64::new(std::f64::consts::PI / 2.0).unwrap();
    let result: SymmetricF64 = angle.sin();

    // Explicitly check the return type
    let _typed: SymmetricF64 = result;
    assert_eq!(result.get(), 1.0);
}

#[test]
fn test_sin_zero() {
    let zero = FinF64::new(0.0).unwrap();
    let result: SymmetricF64 = zero.sin();

    // Explicitly check the return type
    let _typed: SymmetricF64 = result;
    assert_eq!(result.get(), 0.0);
}

#[test]
fn test_sin_pi() {
    let pi = FinF64::new(std::f64::consts::PI).unwrap();
    let result: SymmetricF64 = pi.sin();

    // Explicitly check the return type
    let _typed: SymmetricF64 = result;
    assert!((result.get() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_sin_various_types() {
    let fin = FinF64::new(std::f64::consts::PI / 6.0).unwrap();
    let nonnegative = NonNegativeF64::new_const(0.0);
    let sym = SymmetricF64::new_const(0.5);

    // Test with FinF64
    let sin_fin: SymmetricF64 = fin.sin();
    let _typed1: SymmetricF64 = sin_fin;
    assert!((sin_fin.get() - 0.5).abs() < f64::EPSILON);

    // Test with NonNegativeF64
    let sin_nonnegative: SymmetricF64 = nonnegative.sin();
    let _typed2: SymmetricF64 = sin_nonnegative;
    assert_eq!(sin_nonnegative.get(), 0.0);

    // Test with SymmetricF64
    let sin_sym: SymmetricF64 = sym.sin();
    let _typed3: SymmetricF64 = sin_sym;
    assert!((sin_sym.get() - 0.479_425_538_604_203).abs() < f64::EPSILON);
}

// ============================================================================
// cos() Operation Tests
// ============================================================================

#[test]
fn test_cos_basic() {
    let angle = FinF64::new(0.0).unwrap();
    let result: SymmetricF64 = angle.cos();

    // Explicitly check the return type
    let _typed: SymmetricF64 = result;
    assert_eq!(result.get(), 1.0);
}

#[test]
fn test_cos_pi_half() {
    let pi_half = FinF64::new(std::f64::consts::PI / 2.0).unwrap();
    let result: SymmetricF64 = pi_half.cos();

    // Explicitly check the return type
    let _typed: SymmetricF64 = result;
    assert!((result.get() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_cos_pi() {
    let pi = FinF64::new(std::f64::consts::PI).unwrap();
    let result: SymmetricF64 = pi.cos();

    // Explicitly check the return type
    let _typed: SymmetricF64 = result;
    assert!((result.get() - (-1.0)).abs() < f64::EPSILON);
}

#[test]
fn test_cos_various_types() {
    let fin = FinF64::new(std::f64::consts::PI / 3.0).unwrap();
    let nonnegative = NonNegativeF64::new_const(0.0);

    // Test with FinF64
    let cos_fin: SymmetricF64 = fin.cos();
    let _typed1: SymmetricF64 = cos_fin;
    assert!((cos_fin.get() - 0.5).abs() < f64::EPSILON);

    // Test with NonNegativeF64
    let cos_nonnegative: SymmetricF64 = nonnegative.cos();
    let _typed2: SymmetricF64 = cos_nonnegative;
    assert_eq!(cos_nonnegative.get(), 1.0);
}

// ============================================================================
// tan() Operation Tests
// ============================================================================

#[test]
fn test_tan_basic() {
    let angle = FinF64::new(0.0).unwrap();
    let result: Result<FinF64, FloatError> = angle.tan();

    // Explicitly check the return type and value
    assert!(result.is_ok());
    let value = result.unwrap();
    let _typed: FinF64 = value;
    assert_eq!(value.get(), 0.0);
}

#[test]
fn test_tan_pi_four() {
    let pi_over_4 = FinF64::new(std::f64::consts::PI / 4.0).unwrap();
    let result: Result<FinF64, FloatError> = pi_over_4.tan();

    // Explicitly check the return type and value
    assert!(result.is_ok());
    let value = result.unwrap();
    let _typed: FinF64 = value;
    assert!((value.get() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_tan_various_types() {
    let fin = FinF64::new(std::f64::consts::PI / 6.0).unwrap();
    let nonnegative = NonNegativeF64::new_const(0.0);

    // Test with FinF64
    let tan_fin: Result<FinF64, FloatError> = fin.tan();
    assert!(tan_fin.is_ok());
    let fin_value = tan_fin.unwrap();
    let _typed1: FinF64 = fin_value;
    // tan(π/6) = 1/√3 ≈ 0.5773502691896257
    let expected = 0.577_350_269_189_625_7f64;
    assert!((fin_value.get() - expected).abs() < f64::EPSILON);

    // Test with NonNegativeF64
    let tan_nonnegative: Result<FinF64, FloatError> = nonnegative.tan();
    assert!(tan_nonnegative.is_ok());
    let nonnegative_value = tan_nonnegative.unwrap();
    let _typed2: FinF64 = nonnegative_value;
    assert_eq!(nonnegative_value.get(), 0.0);
}

#[test]
fn test_tan_singular_point() {
    // tan produces very large values near singular points
    let close_to_singular = FinF64::new((std::f64::consts::PI / 2.0) - 0.0001).unwrap();
    let result: Result<FinF64, FloatError> = close_to_singular.tan();

    // Very close to singular point, should return a large finite value
    assert!(result.is_ok());
    let value = result.unwrap();
    let _typed: FinF64 = value;
    assert!(value.get().abs() > 1000.0);

    // Use a value that makes tan result very large but not infinite
    let very_close = FinF64::new((std::f64::consts::PI / 2.0) - 1e-10).unwrap();
    let result2: Result<FinF64, FloatError> = very_close.tan();
    // This may produce infinity or a very large finite value
    // We only check that it doesn't panic
    let _ = result2;
}

// ============================================================================
// Combined Tests
// ============================================================================

#[test]
fn test_trig_identities() {
    let angle = FinF64::new(std::f64::consts::PI / 4.0).unwrap();

    let sin_val: SymmetricF64 = angle.sin();
    let _typed_sin: SymmetricF64 = sin_val;
    let cos_val: SymmetricF64 = angle.cos();
    let _typed_cos: SymmetricF64 = cos_val;

    // sin²(x) + cos²(x) = 1
    let sum_of_squares = sin_val.get() * sin_val.get() + cos_val.get() * cos_val.get();
    assert!((sum_of_squares - 1.0).abs() < f64::EPSILON * 10.0);
}

#[test]
fn test_trig_with_negation() {
    let angle = FinF64::new(std::f64::consts::PI / 6.0).unwrap();

    let sin_pos: SymmetricF64 = angle.sin();
    let _typed_sin_pos: SymmetricF64 = sin_pos;
    let sin_neg: SymmetricF64 = (-angle).sin();
    let _typed_sin_neg: SymmetricF64 = sin_neg;

    // sin(-x) = -sin(x)
    assert!((sin_pos.get() + sin_neg.get()).abs() < f64::EPSILON);

    let cos_pos: SymmetricF64 = angle.cos();
    let _typed_cos_pos: SymmetricF64 = cos_pos;
    let cos_neg: SymmetricF64 = (-angle).cos();
    let _typed_cos_neg: SymmetricF64 = cos_neg;

    // cos(-x) = cos(x)
    assert!((cos_pos.get() - cos_neg.get()).abs() < f64::EPSILON);
}
