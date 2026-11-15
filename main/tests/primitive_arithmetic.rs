//! Tests for arithmetic operations between constrained types and primitive types

// Strict floating-point comparisons, unwrap usage, and legacy numeric constants in test code are justified

use strict_num_extended::*;

#[cfg(test)]
mod tests {
    use super::*;
    const F32_INFINITY: f32 = f32::INFINITY;
    const F32_NAN: f32 = f32::NAN;
    const F32_NEG_INFINITY: f32 = f32::NEG_INFINITY;
    const INFINITY: f64 = f64::INFINITY;
    const NAN: f64 = f64::NAN;
    const NEG_INFINITY: f64 = f64::NEG_INFINITY;

    // ========================================================================
    // Basic Arithmetic Tests with f64
    // ========================================================================

    #[test]
    fn test_finf64_add_f64() {
        let a = FinF64::new(2.0).unwrap();
        let result = a + 3.0f64;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_finf64_sub_f64() {
        let a = FinF64::new(5.0).unwrap();
        let result = a - 2.0f64;
        assert_eq!(result.unwrap().get(), 3.0);
    }

    #[test]
    fn test_finf64_mul_f64() {
        let a = FinF64::new(3.0).unwrap();
        let result = a * 4.0f64;
        assert_eq!(result.unwrap().get(), 12.0);
    }

    #[test]
    fn test_finf64_div_f64() {
        let a = FinF64::new(10.0).unwrap();
        let result = a / 2.0f64;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_finf64_div_zero_f64() {
        let a = FinF64::new(5.0).unwrap();
        assert!(matches!(a / 0.0f64, Err(FloatError::NaN)));
    }

    #[test]
    fn test_f64_add_finf64() {
        let a = 2.0f64;
        let b = FinF64::new(3.0).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_f64_sub_finf64() {
        let a = 5.0f64;
        let b = FinF64::new(2.0).unwrap();
        let result = a - b;
        assert_eq!(result.unwrap().get(), 3.0);
    }

    #[test]
    fn test_f64_mul_finf64() {
        let a = 3.0f64;
        let b = FinF64::new(4.0).unwrap();
        let result = a * b;
        assert_eq!(result.unwrap().get(), 12.0);
    }

    #[test]
    fn test_f64_div_finf64() {
        let a = 10.0f64;
        let b = FinF64::new(2.0).unwrap();
        let result = a / b;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    // ========================================================================
    // Basic Arithmetic Tests with f32
    // ========================================================================

    #[test]
    fn test_finf32_add_f32() {
        let a = FinF32::new(2.0).unwrap();
        let result = a + 3.0f32;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_finf32_sub_f32() {
        let a = FinF32::new(5.0).unwrap();
        let result = a - 2.0f32;
        assert_eq!(result.unwrap().get(), 3.0);
    }

    #[test]
    fn test_finf32_mul_f32() {
        let a = FinF32::new(3.0).unwrap();
        let result = a * 4.0f32;
        assert_eq!(result.unwrap().get(), 12.0);
    }

    #[test]
    fn test_finf32_div_f32() {
        let a = FinF32::new(10.0).unwrap();
        let result = a / 2.0f32;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_finf32_div_zero_f32() {
        let a = FinF32::new(5.0).unwrap();
        assert!(matches!(a / 0.0f32, Err(FloatError::NaN)));
    }

    #[test]
    fn test_f32_add_finf32() {
        let a = 2.0f32;
        let b = FinF32::new(3.0).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_f32_sub_finf32() {
        let a = 5.0f32;
        let b = FinF32::new(2.0).unwrap();
        let result = a - b;
        assert_eq!(result.unwrap().get(), 3.0);
    }

    #[test]
    fn test_f32_mul_finf32() {
        let a = 3.0f32;
        let b = FinF32::new(4.0).unwrap();
        let result = a * b;
        assert_eq!(result.unwrap().get(), 12.0);
    }

    #[test]
    fn test_f32_div_finf32() {
        let a = 10.0f32;
        let b = FinF32::new(2.0).unwrap();
        let result = a / b;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    // ========================================================================
    // NonNegative Type Tests
    // ========================================================================

    #[test]
    fn test_nonnegativef64_add_f64() {
        let a = NonNegativeF64::new(2.0).unwrap();
        let result = a + 3.0f64;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_nonnegativef64_mul_f64() {
        let a = NonNegativeF64::new(2.0).unwrap();
        let result = a * 3.0f64;
        assert_eq!(result.unwrap().get(), 6.0);
    }

    #[test]
    fn test_f64_add_nonnegativef64() {
        let a = 2.0f64;
        let b = NonNegativeF64::new(3.0).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_nonnegativef32_add_f32() {
        let a = NonNegativeF32::new(2.0).unwrap();
        let result = a + 3.0f32;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_f32_add_nonnegativef32() {
        let a = 2.0f32;
        let b = NonNegativeF32::new(3.0).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    // ========================================================================
    // Normalized Type Tests
    // ========================================================================

    #[test]
    fn test_normalizedf64_add_f64() {
        let a = NormalizedF64::new(0.5).unwrap();
        let result = a + 0.3f64;
        assert_eq!(result.unwrap().get(), 0.8);
    }

    #[test]
    fn test_f64_add_normalizedf64() {
        let a = 0.5f64;
        let b = NormalizedF64::new(0.3).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), 0.8);
    }

    #[test]
    fn test_normalizedf32_add_f32() {
        let a = NormalizedF32::new(0.5).unwrap();
        let result = a + 0.3f32;
        assert_eq!(result.unwrap().get(), 0.8);
    }

    #[test]
    fn test_f32_add_normalizedf32() {
        let a = 0.5f32;
        let b = NormalizedF32::new(0.3).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), 0.8);
    }

    // ========================================================================
    // NonZero Type Tests
    // ========================================================================

    #[test]
    fn test_nonzero_f64_add_f64() {
        let a = NonZeroF64::new(2.0).unwrap();
        let result = a + 3.0f64;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_f64_add_nonzero_f64() {
        let a = 2.0f64;
        let b = NonZeroF64::new(3.0).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_nonzero_f32_add_f32() {
        let a = NonZeroF32::new(2.0).unwrap();
        let result = a + 3.0f32;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_f32_add_nonzero_f32() {
        let a = 2.0f32;
        let b = NonZeroF32::new(3.0).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    // ========================================================================
    // NonPositive Type Tests
    // ========================================================================

    #[test]
    fn test_nonpositivef64_add_f64() {
        let a = NonPositiveF64::new(-2.0).unwrap();
        let result = a + (-3.0f64);
        assert_eq!(result.unwrap().get(), -5.0);
    }

    #[test]
    fn test_f64_add_nonpositivef64() {
        let a = -2.0f64;
        let b = NonPositiveF64::new(-3.0).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), -5.0);
    }

    #[test]
    fn test_nonpositivef32_add_f32() {
        let a = NonPositiveF32::new(-2.0).unwrap();
        let result = a + (-3.0f32);
        assert_eq!(result.unwrap().get(), -5.0);
    }

    #[test]
    fn test_f32_add_nonpositivef32() {
        let a = -2.0f32;
        let b = NonPositiveF32::new(-3.0).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), -5.0);
    }

    // ========================================================================
    // Positive Type Tests
    // ========================================================================

    #[test]
    fn test_positivef64_add_f64() {
        let a = PositiveF64::new(2.0).unwrap();
        let result = a + 3.0f64;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_f64_add_positivef64() {
        let a = 2.0f64;
        let b = PositiveF64::new(3.0).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_positivef32_add_f32() {
        let a = PositiveF32::new(2.0).unwrap();
        let result = a + 3.0f32;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    #[test]
    fn test_f32_add_positivef32() {
        let a = 2.0f32;
        let b = PositiveF32::new(3.0).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), 5.0);
    }

    // ========================================================================
    // Symmetric Type Tests
    // ========================================================================

    #[test]
    fn test_symmetricf64_add_f64() {
        let a = SymmetricF64::new(0.5).unwrap();
        let result = a + 0.3f64;
        assert_eq!(result.unwrap().get(), 0.8);
    }

    #[test]
    fn test_f64_add_symmetricf64() {
        let a = 0.5f64;
        let b = SymmetricF64::new(0.3).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), 0.8);
    }

    #[test]
    fn test_symmetricf32_add_f32() {
        let a = SymmetricF32::new(0.5).unwrap();
        let result = a + 0.3f32;
        assert_eq!(result.unwrap().get(), 0.8);
    }

    #[test]
    fn test_f32_add_symmetricf32() {
        let a = 0.5f32;
        let b = SymmetricF32::new(0.3).unwrap();
        let result = a + b;
        assert_eq!(result.unwrap().get(), 0.8);
    }

    // ========================================================================
    // Error Handling Tests - NaN, Infinity
    // ========================================================================

    #[test]
    fn test_finf64_add_nan() {
        let a = FinF64::new(2.0).unwrap();
        assert!(matches!(a + NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_finf64_add_infinity() {
        let a = FinF64::new(2.0).unwrap();
        assert!(matches!(a + INFINITY, Err(FloatError::NaN)));
    }

    #[test]
    fn test_finf64_add_neg_infinity() {
        let a = FinF64::new(2.0).unwrap();
        assert!(matches!(a + NEG_INFINITY, Err(FloatError::NaN)));
    }

    #[test]
    fn test_finf64_sub_nan() {
        let a = FinF64::new(2.0).unwrap();
        assert!(matches!(a - NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_finf64_mul_nan() {
        let a = FinF64::new(2.0).unwrap();
        assert!(matches!(a * NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_finf64_div_nan() {
        let a = FinF64::new(2.0).unwrap();
        assert!(matches!(a / NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_f64_add_finf64_nan() {
        let b = FinF64::new(3.0).unwrap();
        assert!(matches!(NAN + b, Err(FloatError::NaN)));
    }

    #[test]
    fn test_f64_sub_finf64_nan() {
        let b = FinF64::new(3.0).unwrap();
        assert!(matches!(NAN - b, Err(FloatError::NaN)));
    }

    #[test]
    fn test_f64_mul_finf64_nan() {
        let b = FinF64::new(3.0).unwrap();
        assert!(matches!(NAN * b, Err(FloatError::NaN)));
    }

    #[test]
    fn test_f64_div_finf64_nan() {
        let b = FinF64::new(3.0).unwrap();
        assert!(matches!(NAN / b, Err(FloatError::NaN)));
    }

    #[test]
    fn test_finf32_add_nan() {
        let a = FinF32::new(2.0).unwrap();
        assert!(matches!(a + F32_NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_finf32_add_infinity() {
        let a = FinF32::new(2.0).unwrap();
        assert!(matches!(a + F32_INFINITY, Err(FloatError::NaN)));
    }

    #[test]
    fn test_finf32_add_neg_infinity() {
        let a = FinF32::new(2.0).unwrap();
        assert!(matches!(a + F32_NEG_INFINITY, Err(FloatError::NaN)));
    }

    #[test]
    fn test_f32_add_finf32_nan() {
        let b = FinF32::new(3.0).unwrap();
        assert!(matches!(F32_NAN + b, Err(FloatError::NaN)));
    }

    #[test]
    fn test_f32_add_finf32_infinity() {
        let b = FinF32::new(3.0).unwrap();
        assert!(matches!(F32_INFINITY + b, Err(FloatError::NaN)));
    }

    #[test]
    fn test_nonnegativef64_add_nan() {
        let a = NonNegativeF64::new(2.0).unwrap();
        assert!(matches!(a + NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_nonnegativef32_add_nan() {
        let a = NonNegativeF32::new(2.0).unwrap();
        assert!(matches!(a + F32_NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_normalizedf64_add_nan() {
        let a = NormalizedF64::new(0.5).unwrap();
        assert!(matches!(a + NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_nonzero_f64_add_nan() {
        let a = NonZeroF64::new(2.0).unwrap();
        assert!(matches!(a + NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_nonpositivef64_add_nan() {
        let a = NonPositiveF64::new(-2.0).unwrap();
        assert!(matches!(a + NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_positivef64_add_nan() {
        let a = PositiveF64::new(2.0).unwrap();
        assert!(matches!(a + NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_symmetricf64_add_nan() {
        let a = SymmetricF64::new(0.5).unwrap();
        assert!(matches!(a + NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_negative_normalizedf64_add_nan() {
        let a = NegativeNormalizedF64::new(-0.5).unwrap();
        assert!(matches!(a + NAN, Err(FloatError::NaN)));
    }

    #[test]
    fn test_negativef64_add_nan() {
        let a = NegativeF64::new(-2.0).unwrap();
        assert!(matches!(a + NAN, Err(FloatError::NaN)));
    }
}
