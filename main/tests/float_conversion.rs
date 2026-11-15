//! F32/F64 type conversion tests

#![expect(clippy::shadow_unrelated)]

use strict_num_extended::{FloatError, *};

// ========== as_f32/as_f64 primitive tests ==========

#[test]
fn test_as_f32_primitive() {
    let val = NonNegativeF32::new_const(5.0);
    assert_eq!(val.as_f32(), 5.0);

    let val = NormalizedF32::new_const(0.75);
    assert_eq!(val.as_f32(), 0.75);
}

#[test]
fn test_as_f64_primitive() {
    let val = NonNegativeF64::new_const(5.0);
    assert_eq!(val.as_f64(), 5.0);

    let val = NormalizedF64::new_const(0.75);
    assert_eq!(val.as_f64(), 0.75);
}

// ========== as_f32_type tests ==========

#[test]
fn test_as_f32_type_clone() {
    let val = NonNegativeF32::new_const(5.0);
    let cloned = val.as_f32_type();
    assert_eq!(val.as_f32(), cloned.as_f32());

    let val = NormalizedF32::new_const(0.75);
    let cloned = val.as_f32_type();
    assert_eq!(val.as_f32(), cloned.as_f32());
}

// ========== try_into_f32_type tests ==========

#[test]
fn test_try_into_f32_type_precision_loss() {
    // Precision loss is now allowed
    let value_f64 = FinF64::new_const(1.234_567_890_123_456_7);
    let result: Result<FinF32, FloatError> = value_f64.try_into_f32_type();
    assert!(result.is_ok(), "Should succeed (precision loss is allowed)");
}

#[test]
fn test_try_into_f32_type_range_overflow() {
    // Range overflow results in infinity, which is rejected
    let value_f64 = FinF64::new_const(1e40);
    let result: Result<FinF32, FloatError> = value_f64.try_into_f32_type();
    assert!(result.is_err(), "Should fail due to infinity");
    assert!(
        matches!(result, Err(FloatError::PosInf)),
        "Should be PosInf error"
    );
}

#[test]
fn test_try_into_f32_type_constraint_violation() {
    // Test constraint violation (NonNegativeF64 -> NonNegativeF32)
    let positive_f64 = NonNegativeF64::new_const(2.0);
    let positive_result: Result<NonNegativeF32, FloatError> = positive_f64.try_into_f32_type();
    assert!(
        positive_result.is_ok(),
        "Should succeed with valid positive value"
    );

    // Test constraint after conversion
    let normalized_f64 = NormalizedF64::new_const(0.5);
    let normalized_result: Result<NormalizedF32, FloatError> = normalized_f64.try_into_f32_type();
    assert!(
        normalized_result.is_ok(),
        "Should succeed with normalized value"
    );

    // Test value outside constraint range
    let large_f64 = NonNegativeF64::new_const(1000.0);
    let large_result: Result<NonNegativeF32, FloatError> = large_f64.try_into_f32_type();
    // This may succeed or fail, depending on whether 1000.0 is within F32 range and satisfies NonNegativeF32 constraint
    if let Ok(f32_val) = large_result {
        assert!(f32_val.get() > 0.0, "Should maintain positive constraint");
    }
}

#[test]
fn test_try_into_f32_type_normal_conversion() {
    // Test normal conversion (using exactly representable values)
    let value_f64 = FinF64::new_const(3.0);
    let result: Result<FinF32, FloatError> = value_f64.try_into_f32_type();
    assert!(result.is_ok(), "Should succeed for F32-representable value");

    let f32_value = result.unwrap();
    assert_eq!(f32_value.get(), 3.0);
}

#[test]
fn test_try_into_f32_type_different_constraints() {
    // Test conversion of different constraint types
    let positive = NonNegativeF64::new_const(10.0);
    let pos_result: Result<NonNegativeF32, FloatError> = positive.try_into_f32_type();
    assert!(pos_result.is_ok());
    assert_eq!(pos_result.unwrap().get(), 10.0);

    let negative = NonPositiveF64::new_const(-10.0);
    let neg_result: Result<NonPositiveF32, FloatError> = negative.try_into_f32_type();
    assert!(neg_result.is_ok());
    assert_eq!(neg_result.unwrap().get(), -10.0);

    let nonzero = NonZeroF64::new_const(5.0);
    let nz_result: Result<NonZeroF32, FloatError> = nonzero.try_into_f32_type();
    assert!(nz_result.is_ok());
    assert_eq!(nz_result.unwrap().get(), 5.0);

    let symmetric = SymmetricF64::new_const(0.5);
    let sym_result: Result<SymmetricF32, FloatError> = symmetric.try_into_f32_type();
    assert!(sym_result.is_ok());
    assert_eq!(sym_result.unwrap().get(), 0.5);

    let normalized = NormalizedF64::new_const(0.75);
    let norm_result: Result<NormalizedF32, FloatError> = normalized.try_into_f32_type();
    assert!(norm_result.is_ok());
    assert_eq!(norm_result.unwrap().get(), 0.75);

    let neg_normalized = NegativeNormalizedF64::new_const(-0.5);
    let neg_norm_result: Result<NegativeNormalizedF32, FloatError> =
        neg_normalized.try_into_f32_type();
    assert!(neg_norm_result.is_ok());
    assert_eq!(neg_norm_result.unwrap().get(), -0.5);

    let nonzero_pos = PositiveF64::new_const(1.0);
    let nz_pos_result: Result<PositiveF32, FloatError> = nonzero_pos.try_into_f32_type();
    assert!(nz_pos_result.is_ok());
    assert_eq!(nz_pos_result.unwrap().get(), 1.0);

    let nonzero_neg = NegativeF64::new_const(-1.0);
    let nz_neg_result: Result<NegativeF32, FloatError> = nonzero_neg.try_into_f32_type();
    assert!(nz_neg_result.is_ok());
    assert_eq!(nz_neg_result.unwrap().get(), -1.0);
}

// ========== as_f64_type tests ==========

#[test]
fn test_as_f64_type_const_context() {
    // Test const context
    const VALUE_F32: FinF32 = FinF32::new_const(3.0);
    const VALUE_F64: FinF64 = VALUE_F32.as_f64_type();

    assert_eq!(VALUE_F64.get(), 3.0);
}

#[test]
fn test_as_f64_type_different_constraints() {
    // Test conversion of different constraint types
    let f32_val = NonNegativeF32::new_const(10.0);
    let f64_val: NonNegativeF64 = f32_val.as_f64_type();
    assert_eq!(f64_val.get(), 10.0);
    assert!(f64_val.get() > 0.0);

    let f32_val = NonPositiveF32::new_const(-10.0);
    let f64_val: NonPositiveF64 = f32_val.as_f64_type();
    assert_eq!(f64_val.get(), -10.0);
    assert!(f64_val.get() < 0.0);

    let f32_val = NonZeroF32::new_const(5.0);
    let f64_val: NonZeroF64 = f32_val.as_f64_type();
    assert_eq!(f64_val.get(), 5.0);
    assert_ne!(f64_val.get(), 0.0);

    let f32_val = SymmetricF32::new_const(0.5);
    let f64_val: SymmetricF64 = f32_val.as_f64_type();
    assert_eq!(f64_val.get(), 0.5);

    let f32_val = NormalizedF32::new_const(0.75);
    let f64_val: NormalizedF64 = f32_val.as_f64_type();
    assert_eq!(f64_val.get(), 0.75);

    let f32_val = NegativeNormalizedF32::new_const(-0.5);
    let f64_val: NegativeNormalizedF64 = f32_val.as_f64_type();
    assert_eq!(f64_val.get(), -0.5);

    let f32_val = PositiveF32::new_const(1.0);
    let f64_val: PositiveF64 = f32_val.as_f64_type();
    assert_eq!(f64_val.get(), 1.0);

    let f32_val = NegativeF32::new_const(-1.0);
    let f64_val: NegativeF64 = f32_val.as_f64_type();
    assert_eq!(f64_val.get(), -1.0);
}

#[test]
fn test_as_f64_type_precision_preservation() {
    // Test precision preservation (from F32 to F64 should always preserve precision)
    let f32_val = FinF32::new_const(1.5);
    let f64_val: FinF64 = f32_val.as_f64_type();
    assert_eq!(f64_val.get(), 1.5);

    let f32_val = FinF32::new_const(std::f32::consts::PI);
    let f64_val: FinF64 = f32_val.as_f64_type();
    assert!((f64_val.get() - std::f64::consts::PI).abs() < 1e-6);
}

#[test]
#[allow(clippy::approx_constant)]
fn test_roundtrip_conversion() {
    // Test roundtrip conversion: F32 -> F64 -> F32
    let original_f32 = FinF32::new_const(3.14159);
    let f64_val: FinF64 = original_f32.as_f64_type();
    let back_to_f32: Result<FinF32, FloatError> = f64_val.try_into_f32_type();

    assert!(
        back_to_f32.is_ok(),
        "Roundtrip F32->F64->F32 should succeed"
    );
    let final_f32 = back_to_f32.unwrap();
    assert!((final_f32.get() - original_f32.get()).abs() < 1e-6);
}

// ========== Comprehensive tests ==========

#[test]
fn test_combined_conversion_with_arithmetic() {
    // Test combination of conversion and arithmetic operations
    let a_f64 = NonNegativeF64::new_const(10.0);
    let b_f64 = NonNegativeF64::new_const(20.0);

    // Convert to F32 for computation
    let a_f32: Result<NonNegativeF32, FloatError> = a_f64.try_into_f32_type();
    let b_f32: Result<NonNegativeF32, FloatError> = b_f64.try_into_f32_type();

    assert!(a_f32.is_ok() && b_f32.is_ok());

    let sum_f32 = a_f32.unwrap() + b_f32.unwrap();
    let sum_f32 = sum_f32.unwrap();

    // Convert back to F64
    let sum_f64: NonNegativeF64 = sum_f32.as_f64_type();
    assert_eq!(sum_f64.get(), 30.0);
}

#[cfg(test)]
mod test_construction {
    use super::*;

    #[test]
    fn test_construction_with_conversion() {
        // Test using conversion in const context (as_f64_type)
        const F32_VAL: NonNegativeF32 = NonNegativeF32::new_const(42.0);
        const F64_VAL: NonNegativeF64 = F32_VAL.as_f64_type();

        assert_eq!(F64_VAL.get(), 42.0);
    }

    #[test]
    fn test_runtime_conversion() {
        // Test runtime conversion (try_into_f32_type)
        let f64_val = NonNegativeF64::new(42.0).unwrap();
        let f32_val: Result<NonNegativeF32, FloatError> = f64_val.try_into_f32_type();

        assert!(f32_val.is_ok());
        assert_eq!(f32_val.unwrap().get(), 42.0);
    }
}
