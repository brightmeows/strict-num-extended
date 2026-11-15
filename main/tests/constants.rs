//! Constant tests

use strict_num_extended::*;

#[test]
fn test_fin_f64_constants() {
    assert_eq!(FinF64::ZERO.get(), 0.0);
    assert_eq!(FinF64::ONE.get(), 1.0);
    assert_eq!(FinF64::NEG_ONE.get(), -1.0);
    assert_eq!(FinF64::PI.get(), core::f64::consts::PI);
    assert_eq!(FinF64::NEG_PI.get(), -core::f64::consts::PI);
    assert_eq!(FinF64::E.get(), core::f64::consts::E);
    assert_eq!(FinF64::NEG_E.get(), -core::f64::consts::E);
}

#[test]
fn test_fin_f32_constants() {
    assert_eq!(FinF32::ZERO.get(), 0.0);
    assert_eq!(FinF32::ONE.get(), 1.0);
    assert_eq!(FinF32::NEG_ONE.get(), -1.0);
    assert_eq!(FinF32::PI.get(), core::f32::consts::PI);
    assert_eq!(FinF32::E.get(), core::f32::consts::E);
}

#[test]
fn test_positive_f64_constants() {
    // Positive type should have positive constants
    assert_eq!(PositiveF64::ONE.get(), 1.0);
    assert_eq!(PositiveF64::PI.get(), core::f64::consts::PI);
    assert_eq!(PositiveF64::E.get(), core::f64::consts::E);
    assert_eq!(PositiveF64::TWO.get(), 2.0);
    assert_eq!(PositiveF64::HALF.get(), 0.5);

    // Check PI fraction constants
    assert_eq!(PositiveF64::FRAC_1_PI.get(), core::f64::consts::FRAC_1_PI);
    assert_eq!(PositiveF64::FRAC_2_PI.get(), core::f64::consts::FRAC_2_PI);
    assert_eq!(PositiveF64::FRAC_PI_2.get(), core::f64::consts::FRAC_PI_2);
    assert_eq!(PositiveF64::FRAC_PI_3.get(), core::f64::consts::FRAC_PI_3);
    assert_eq!(PositiveF64::FRAC_PI_4.get(), core::f64::consts::FRAC_PI_4);
    assert_eq!(PositiveF64::FRAC_PI_6.get(), core::f64::consts::FRAC_PI_6);
    assert_eq!(PositiveF64::FRAC_PI_8.get(), core::f64::consts::FRAC_PI_8);
}

#[test]
fn test_negative_f64_constants() {
    // Negative type should have negative constants
    assert_eq!(NegativeF64::NEG_ONE.get(), -1.0);
    assert_eq!(NegativeF64::NEG_PI.get(), -core::f64::consts::PI);
    assert_eq!(NegativeF64::NEG_E.get(), -core::f64::consts::E);
    assert_eq!(NegativeF64::NEG_TWO.get(), -2.0);
    assert_eq!(NegativeF64::NEG_HALF.get(), -0.5);
}

#[test]
fn test_non_negative_f64_constants() {
    // NonNegative type should have non-negative constants
    assert_eq!(NonNegativeF64::ZERO.get(), 0.0);
    assert_eq!(NonNegativeF64::ONE.get(), 1.0);
    assert_eq!(NonNegativeF64::PI.get(), core::f64::consts::PI);
    assert_eq!(NonNegativeF64::E.get(), core::f64::consts::E);
    assert_eq!(NonNegativeF64::HALF.get(), 0.5);
}

#[test]
fn test_non_positive_f64_constants() {
    // NonPositive type should have non-positive constants
    assert_eq!(NonPositiveF64::ZERO.get(), 0.0);
    assert_eq!(NonPositiveF64::NEG_ONE.get(), -1.0);
    assert_eq!(NonPositiveF64::NEG_PI.get(), -core::f64::consts::PI);
    assert_eq!(NonPositiveF64::NEG_E.get(), -core::f64::consts::E);
    assert_eq!(NonPositiveF64::NEG_HALF.get(), -0.5);
}

#[test]
fn test_normalized_f32_constants() {
    // Normalized [0, 1] should have constants in range
    assert_eq!(NormalizedF32::ZERO.get(), 0.0);
    assert_eq!(NormalizedF32::ONE.get(), 1.0);
    assert_eq!(NormalizedF32::HALF.get(), 0.5);

    // PI fraction constants in [0, 1] range
    assert_eq!(NormalizedF32::FRAC_1_PI.get(), core::f32::consts::FRAC_1_PI);
    assert_eq!(NormalizedF32::FRAC_2_PI.get(), core::f32::consts::FRAC_2_PI);
    assert_eq!(NormalizedF32::FRAC_PI_4.get(), core::f32::consts::FRAC_PI_4);
    assert_eq!(NormalizedF32::FRAC_PI_6.get(), core::f32::consts::FRAC_PI_6);
    assert_eq!(NormalizedF32::FRAC_PI_8.get(), core::f32::consts::FRAC_PI_8);
}

#[test]
fn test_normalized_f64_constants() {
    // Normalized [0, 1] should have constants in range
    assert_eq!(NormalizedF64::ZERO.get(), 0.0);
    assert_eq!(NormalizedF64::ONE.get(), 1.0);
    assert_eq!(NormalizedF64::HALF.get(), 0.5);

    // PI fraction constants in [0, 1] range
    assert_eq!(NormalizedF64::FRAC_1_PI.get(), core::f64::consts::FRAC_1_PI);
    assert_eq!(NormalizedF64::FRAC_2_PI.get(), core::f64::consts::FRAC_2_PI);
    assert_eq!(NormalizedF64::FRAC_PI_4.get(), core::f64::consts::FRAC_PI_4);
    assert_eq!(NormalizedF64::FRAC_PI_6.get(), core::f64::consts::FRAC_PI_6);
    assert_eq!(NormalizedF64::FRAC_PI_8.get(), core::f64::consts::FRAC_PI_8);
}

#[test]
fn test_negative_normalized_f64_constants() {
    // NegativeNormalized [-1, 0] should have constants in range
    assert_eq!(NegativeNormalizedF64::ZERO.get(), 0.0);
    assert_eq!(NegativeNormalizedF64::NEG_ONE.get(), -1.0);
    assert_eq!(NegativeNormalizedF64::NEG_HALF.get(), -0.5);
}

#[test]
fn test_symmetric_f64_constants() {
    // Symmetric [-1, 1] should have constants in range
    assert_eq!(SymmetricF64::ZERO.get(), 0.0);
    assert_eq!(SymmetricF64::ONE.get(), 1.0);
    assert_eq!(SymmetricF64::NEG_ONE.get(), -1.0);
    assert_eq!(SymmetricF64::HALF.get(), 0.5);
    assert_eq!(SymmetricF64::NEG_HALF.get(), -0.5);

    // PI fraction constants in [-1, 1] range
    assert_eq!(SymmetricF64::FRAC_1_PI.get(), core::f64::consts::FRAC_1_PI);
    assert_eq!(SymmetricF64::FRAC_2_PI.get(), core::f64::consts::FRAC_2_PI);
    assert_eq!(SymmetricF64::FRAC_PI_4.get(), core::f64::consts::FRAC_PI_4);
    assert_eq!(SymmetricF64::FRAC_PI_6.get(), core::f64::consts::FRAC_PI_6);
    assert_eq!(SymmetricF64::FRAC_PI_8.get(), core::f64::consts::FRAC_PI_8);
}

#[test]
fn test_const_context() {
    // Constants should be usable in const context
    const ONE: PositiveF64 = PositiveF64::ONE;
    const HALF: NormalizedF32 = NormalizedF32::HALF;
    const ZERO: FinF64 = FinF64::ZERO;

    assert_eq!(ONE.get(), 1.0);
    assert_eq!(HALF.get(), 0.5);
    assert_eq!(ZERO.get(), 0.0);
}

#[test]
fn test_non_zero_f64_constants() {
    // NonZero type should not have ZERO constant
    assert_eq!(NonZeroF64::ONE.get(), 1.0);
    assert_eq!(NonZeroF64::NEG_ONE.get(), -1.0);
    assert_eq!(NonZeroF64::PI.get(), core::f64::consts::PI);
}

#[test]
fn test_pibounded_f64_constants() {
    // PiBounded [-PI, PI] should have constants in range
    assert_eq!(PiBoundedF64::ZERO.get(), 0.0);
    assert_eq!(PiBoundedF64::ONE.get(), 1.0);
    assert_eq!(PiBoundedF64::NEG_ONE.get(), -1.0);
    assert_eq!(PiBoundedF64::HALF.get(), 0.5);
    assert_eq!(PiBoundedF64::NEG_HALF.get(), -0.5);

    // PI-related constants
    assert_eq!(PiBoundedF64::PI.get(), core::f64::consts::PI);
    assert_eq!(PiBoundedF64::NEG_PI.get(), -core::f64::consts::PI);
    assert_eq!(PiBoundedF64::FRAC_PI_2.get(), core::f64::consts::FRAC_PI_2);
    assert_eq!(PiBoundedF64::FRAC_PI_3.get(), core::f64::consts::FRAC_PI_3);
    assert_eq!(PiBoundedF64::FRAC_PI_4.get(), core::f64::consts::FRAC_PI_4);
    assert_eq!(PiBoundedF64::FRAC_PI_6.get(), core::f64::consts::FRAC_PI_6);
    assert_eq!(PiBoundedF64::FRAC_PI_8.get(), core::f64::consts::FRAC_PI_8);
}
