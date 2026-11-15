//! Comprehensive error validation tests
//!
//! Test objectives:
//! 1. Cover all error types
//! 2. Test edge cases and special values
//! 3. Verify error message accuracy
//! 4. Ensure compile-time and runtime consistency

#![expect(clippy::uninlined_format_args, clippy::panic)]

use strict_num_extended::*;

// ========== NaN Error Tests ==========

mod test_nan_errors {
    use super::*;

    #[test]
    fn test_standard_nan() {
        assert!(matches!(FinF32::new(f32::NAN), Err(FloatError::NaN)));
        assert!(matches!(FinF64::new(f64::NAN), Err(FloatError::NaN)));
    }

    #[test]
    fn test_negative_nan() {
        assert!(matches!(FinF32::new(-f32::NAN), Err(FloatError::NaN)));
        assert!(matches!(FinF64::new(-f64::NAN), Err(FloatError::NaN)));
    }

    #[test]
    fn test_arithmetic_nan() {
        // ∞ - ∞ = NaN
        let nan_inf = f32::INFINITY - f32::INFINITY;
        assert!(matches!(FinF32::new(nan_inf), Err(FloatError::NaN)));
    }

    #[test]
    fn test_nan_in_all_types() {
        // NonNegativeF64 should reject NaN
        assert!(matches!(
            NonNegativeF64::new(f64::NAN),
            Err(FloatError::NaN)
        ));

        // NonPositiveF64 should reject NaN
        assert!(matches!(
            NonPositiveF64::new(f64::NAN),
            Err(FloatError::NaN)
        ));

        // NonZeroF64 should reject NaN
        assert!(matches!(NonZeroF64::new(f64::NAN), Err(FloatError::NaN)));

        // NormalizedF64 should reject NaN
        assert!(matches!(NormalizedF64::new(f64::NAN), Err(FloatError::NaN)));

        // SymmetricF64 should reject NaN
        assert!(matches!(SymmetricF64::new(f64::NAN), Err(FloatError::NaN)));
    }
}

// ========== Infinity Error Tests ==========

mod test_infinity_errors {
    use super::*;

    #[test]
    fn test_positive_infinity() {
        assert!(matches!(
            FinF32::new(f32::INFINITY),
            Err(FloatError::PosInf)
        ));
        assert!(matches!(
            FinF64::new(f64::INFINITY),
            Err(FloatError::PosInf)
        ));
    }

    #[test]
    fn test_negative_infinity() {
        assert!(matches!(
            FinF32::new(f32::NEG_INFINITY),
            Err(FloatError::NegInf)
        ));
        assert!(matches!(
            FinF64::new(f64::NEG_INFINITY),
            Err(FloatError::NegInf)
        ));
    }

    #[test]
    fn test_arithmetic_infinity() {
        // 1.0 / 0.0 = +∞
        let inf = 1.0f32 / 0.0;
        assert!(matches!(FinF32::new(inf), Err(FloatError::PosInf)));

        // -1.0 / 0.0 = -∞
        let neg_inf = -1.0f32 / 0.0;
        assert!(matches!(FinF32::new(neg_inf), Err(FloatError::NegInf)));
    }

    #[test]
    fn test_overflow_infinity() {
        // f32::MAX * 2.0 overflows to +∞
        let overflow = f32::MAX * 2.0;
        assert!(matches!(FinF32::new(overflow), Err(FloatError::PosInf)));

        // f32::MIN * 2.0 underflows to -∞
        let underflow = f32::MIN * 2.0;
        assert!(matches!(FinF32::new(underflow), Err(FloatError::NegInf)));
    }

    #[test]
    fn test_infinity_in_all_types() {
        // NonNegativeF64 should reject +∞
        assert!(matches!(
            NonNegativeF64::new(f64::INFINITY),
            Err(FloatError::PosInf)
        ));

        // NonPositiveF64 should reject -∞
        assert!(matches!(
            NonPositiveF64::new(f64::NEG_INFINITY),
            Err(FloatError::NegInf)
        ));

        // NonZeroF64 should reject +∞
        assert!(matches!(
            NonZeroF64::new(f64::INFINITY),
            Err(FloatError::PosInf)
        ));

        // NormalizedF64 should reject +∞
        assert!(matches!(
            NormalizedF64::new(f64::INFINITY),
            Err(FloatError::PosInf)
        ));

        // SymmetricF64 should reject +∞
        assert!(matches!(
            SymmetricF64::new(f64::INFINITY),
            Err(FloatError::PosInf)
        ));
    }
}

// ========== Zero Variant Tests ==========

mod test_zero_variants {
    use super::*;

    #[test]
    fn test_positive_zero() {
        // NonNegative should accept +0.0
        assert!(NonNegativeF32::new(0.0).is_ok());
        assert!(NonNegativeF64::new(0.0).is_ok());

        // NonPositive should accept +0.0
        assert!(NonPositiveF32::new(0.0).is_ok());
        assert!(NonPositiveF64::new(0.0).is_ok());

        // NonZero should reject +0.0
        assert!(matches!(NonZeroF32::new(0.0), Err(FloatError::OutOfRange)));
        assert!(matches!(NonZeroF64::new(0.0), Err(FloatError::OutOfRange)));
    }

    #[test]
    fn test_negative_zero() {
        // IEEE 754: -0.0 == 0.0
        assert_eq!(-0.0f32, 0.0f32);
        assert_eq!(-0.0f64, 0.0f64);

        // NonNegative should accept -0.0 (since -0.0 == 0.0)
        assert!(NonNegativeF32::new(-0.0).is_ok());
        assert!(NonNegativeF64::new(-0.0).is_ok());

        // NonPositive should accept -0.0
        assert!(NonPositiveF32::new(-0.0).is_ok());
        assert!(NonPositiveF64::new(-0.0).is_ok());

        // NonZero should reject -0.0 (since -0.0 == 0.0, and val != 0.0 is false)
        assert!(matches!(NonZeroF32::new(-0.0), Err(FloatError::OutOfRange)));
        assert!(matches!(NonZeroF64::new(-0.0), Err(FloatError::OutOfRange)));
    }

    #[test]
    fn test_zero_equality() {
        // Verify that +0.0 and -0.0 are equal within the type
        let pos_zero = NonNegativeF32::new(0.0).unwrap();
        let neg_zero = NonNegativeF32::new(-0.0).unwrap();
        assert_eq!(pos_zero.get(), neg_zero.get());
    }

    #[test]
    fn test_nonzero_rejects_both_zeros() {
        // Positive should reject both +0.0 and -0.0
        assert!(matches!(PositiveF32::new(0.0), Err(FloatError::OutOfRange)));
        assert!(matches!(
            PositiveF32::new(-0.0),
            Err(FloatError::OutOfRange)
        ));

        assert!(matches!(NegativeF32::new(0.0), Err(FloatError::OutOfRange)));
        assert!(matches!(
            NegativeF32::new(-0.0),
            Err(FloatError::OutOfRange)
        ));
    }
}

// ========== Division by Zero Error Tests ==========

mod test_division_by_zero_errors {
    use super::*;

    #[test]
    fn test_division_by_positive_zero() {
        let a = NonNegativeF64::new(10.0).unwrap();
        let zero = NonNegativeF64::new(0.0).unwrap();

        let result = a / zero;
        assert!(matches!(result, Err(FloatError::NaN)));
    }

    #[test]
    fn test_division_by_negative_zero() {
        let a = NonNegativeF64::new(10.0).unwrap();
        // Use unsafe to create -0.0
        let zero_neg = unsafe { NonNegativeF64::new_unchecked(-0.0) };

        let result = a / zero_neg;
        assert!(matches!(result, Err(FloatError::NaN)));
    }

    #[test]
    fn test_division_by_zero_all_types() {
        // FinF64
        let fin_a = FinF64::new(10.0).unwrap();
        let fin_zero = FinF64::new(0.0).unwrap();
        assert!(matches!(fin_a / fin_zero, Err(FloatError::NaN)));

        // NormalizedF64
        let norm_a = NormalizedF64::new(0.5).unwrap();
        let norm_zero = NormalizedF64::new(0.0).unwrap();
        assert!(matches!(norm_a / norm_zero, Err(FloatError::NaN)));
    }

    #[test]
    fn test_nonzero_types_no_division_by_zero() {
        // NonZero types theoretically cannot divide by zero, since zero values
        // are rejected at creation time
        let a = NonNegativeF64::new(10.0).unwrap();
        let b = PositiveF64::new(2.0).unwrap();

        let result = a / b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 5.0);
    }
}

// ========== Precision Loss Tests ==========

mod test_precision_loss {
    use super::*;

    #[test]
    fn test_exact_conversion() {
        // Integers and simple fractions should convert exactly
        assert!(FinF64::new(3.0).unwrap().try_into_f32_type().is_ok());
        assert!(FinF64::new(0.5).unwrap().try_into_f32_type().is_ok());
        assert!(FinF64::new(-1.5).unwrap().try_into_f32_type().is_ok());
        assert!(FinF64::new(2.0).unwrap().try_into_f32_type().is_ok());
    }

    #[test]
    fn test_precision_loss_allowed() {
        // Precision loss is now allowed - only constraint validation is performed
        let precise = FinF64::new(1.234_567_890_123_456_7).unwrap();
        // Conversion succeeds (though precision is lost)
        assert!(precise.try_into_f32_type().is_ok());
    }

    #[test]
    fn test_range_overflow() {
        // Large numbers outside f32 range become infinity
        let huge = FinF64::new(1e40).unwrap();
        // Conversion fails because infinity is rejected by FinF32::new()
        assert!(huge.try_into_f32_type().is_err());

        // Verify the returned error type is PosInf
        let result = huge.try_into_f32_type();
        assert!(matches!(result, Err(FloatError::PosInf)));
    }

    #[test]
    fn test_range_underflow() {
        // Small numbers outside f32 range become negative infinity
        let tiny = FinF64::new(-1e40).unwrap();
        // Conversion fails because negative infinity is rejected by FinF32::new()
        assert!(tiny.try_into_f32_type().is_err());

        // Verify the returned error type is NegInf
        let result = tiny.try_into_f32_type();
        assert!(matches!(result, Err(FloatError::NegInf)));
    }

    #[test]
    fn test_roundtrip_conversion() {
        // Exact values should be able to round-trip
        let val_f64 = FinF64::new(2.5).unwrap();
        let val_f32 = val_f64.try_into_f32_type().unwrap();
        let back_f64: FinF64 = val_f32.into();
        assert_eq!(back_f64.get(), 2.5);
    }

    #[test]
    fn test_pi_conversion() {
        // f64::π to f32 conversion succeeds (precision loss is allowed)
        let pi_f64 = FinF64::new(std::f64::consts::PI).unwrap();
        assert!(pi_f64.try_into_f32_type().is_ok());

        // f32::π itself is valid (no conversion needed)
        let pi_f32 = FinF32::new(std::f32::consts::PI).unwrap();
        assert!(pi_f32.get().is_finite());
    }

    #[test]
    fn test_f32_boundary_conversion() {
        // f32::MAX can convert exactly
        let max = FinF64::new(f64::from(f32::MAX)).unwrap();
        assert!(max.try_into_f32_type().is_ok());

        // f32::MIN can convert exactly
        let min = FinF64::new(f64::from(f32::MIN)).unwrap();
        assert!(min.try_into_f32_type().is_ok());
    }
}

// ========== Overflow/Underflow Tests ==========

mod test_overflow_underflow {
    use super::*;

    #[test]
    fn test_addition_overflow() {
        let a = NonNegativeF64::new(1e308).unwrap();
        let b = NonNegativeF64::new(1e308).unwrap();

        let result = a + b;
        // Should return PosInf error
        assert!(matches!(result, Err(FloatError::PosInf)));
    }

    #[test]
    fn test_subtraction_underflow() {
        let a = NonPositiveF64::new(-1e308).unwrap();
        let b = NonNegativeF64::new(1e308).unwrap();

        // NonPositive - NonNegative result type is Fin (deduced via operator overloading)
        let result = a - b;
        // Should return NegInf error
        assert!(matches!(result, Err(FloatError::NegInf)));
    }

    #[test]
    fn test_multiplication_overflow() {
        let a = NonNegativeF64::new(1e200).unwrap();
        let b = NonNegativeF64::new(1e200).unwrap();

        let result = a * b;
        // Should return PosInf error
        assert!(matches!(result, Err(FloatError::PosInf)));
    }

    #[test]
    fn test_f32_overflow() {
        // f32 version of overflow test
        // f32::MAX ≈ 3.4e38, use large enough values to cause overflow
        let a = NonNegativeF32::new(2e38).unwrap();
        let b = NonNegativeF32::new(2e38).unwrap();

        let result = a + b;
        // 2e38 + 2e38 = 4e38 > f32::MAX, should overflow
        assert!(matches!(result, Err(FloatError::PosInf)));
    }
}

// ========== Boundary Value Tests ==========

mod test_boundary_values {
    use super::*;

    #[test]
    fn test_normalized_boundaries() {
        // Normalized: [0.0, 1.0]
        assert!(NormalizedF32::new(0.0).is_ok());
        assert!(NormalizedF32::new(1.0).is_ok());
        assert!(matches!(
            NormalizedF32::new(-0.001),
            Err(FloatError::OutOfRange)
        ));
        assert!(matches!(
            NormalizedF32::new(1.001),
            Err(FloatError::OutOfRange)
        ));
    }

    #[test]
    fn test_symmetric_boundaries() {
        // Symmetric: [-1.0, 1.0]
        assert!(SymmetricF32::new(-1.0).is_ok());
        assert!(SymmetricF32::new(1.0).is_ok());
        assert!(matches!(
            SymmetricF32::new(-1.001),
            Err(FloatError::OutOfRange)
        ));
        assert!(matches!(
            SymmetricF32::new(1.001),
            Err(FloatError::OutOfRange)
        ));
    }

    #[test]
    fn test_finite_boundaries() {
        // Fin should accept all finite values
        assert!(FinF32::new(f32::MIN).is_ok());
        assert!(FinF32::new(f32::MAX).is_ok());
        assert!(FinF64::new(f64::MIN).is_ok());
        assert!(FinF64::new(f64::MAX).is_ok());
    }

    #[test]
    fn test_positive_boundaries() {
        // NonNegative: [0.0, +∞)
        assert!(NonNegativeF32::new(0.0).is_ok());
        assert!(NonNegativeF32::new(f32::MAX).is_ok());
        assert!(matches!(
            NonNegativeF32::new(-0.001),
            Err(FloatError::OutOfRange)
        ));
    }

    #[test]
    fn test_negative_boundaries() {
        // NonPositive: (-∞, 0.0]
        assert!(NonPositiveF32::new(0.0).is_ok());
        assert!(NonPositiveF32::new(f32::MIN).is_ok());
        assert!(matches!(
            NonPositiveF32::new(0.001),
            Err(FloatError::OutOfRange)
        ));
    }

    #[test]
    fn test_nonzero_boundaries() {
        // NonZero: (-∞, 0.0) ∪ (0.0, +∞)
        assert!(NonZeroF32::new(0.001).is_ok());
        assert!(NonZeroF32::new(-0.001).is_ok());
        assert!(matches!(NonZeroF32::new(0.0), Err(FloatError::OutOfRange)));
        assert!(matches!(NonZeroF32::new(-0.0), Err(FloatError::OutOfRange)));
    }
}

// ========== Error Message Tests ==========

mod test_error_messages {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", FloatError::NaN),
            "value is NaN (Not a Number)"
        );
        assert_eq!(
            format!("{}", FloatError::PosInf),
            "value is positive infinity"
        );
        assert_eq!(
            format!("{}", FloatError::NegInf),
            "value is negative infinity"
        );
        assert_eq!(
            format!("{}", FloatError::OutOfRange),
            "value is outside the valid range for this type"
        );
        assert_eq!(
            format!("{}", FloatError::NoneOperand),
            "right-hand side operand is None in Option arithmetic"
        );
    }

    #[test]
    fn test_error_debug() {
        assert!(format!("{:?}", FloatError::NaN).contains("NaN"));
        assert!(format!("{:?}", FloatError::PosInf).contains("PosInf"));
        assert!(format!("{:?}", FloatError::NegInf).contains("NegInf"));
        assert!(format!("{:?}", FloatError::OutOfRange).contains("OutOfRange"));
        assert!(format!("{:?}", FloatError::NoneOperand).contains("NoneOperand"));
    }
}

// ========== NoneOperand Error Tests ==========

mod test_none_operand_errors {
    use super::*;

    #[test]
    fn test_option_none_operand_all_operations() {
        const A: NonNegativeF64 = NonNegativeF64::new_const(5.0);
        let none: Option<NonPositiveF64> = None;

        // Addition (safe operation) returns None
        let add_result: Option<FinF64> = A + none;
        assert!(add_result.is_none());

        // Multiplication (fallible operation) returns Err
        let none_pos: Option<NonNegativeF64> = None;
        let mul_result: Result<NonNegativeF64, FloatError> = A * none_pos;
        assert!(matches!(mul_result, Err(FloatError::NoneOperand)));

        // Division (fallible operation) returns Err
        let div_result: Result<NonNegativeF64, FloatError> = A / none_pos;
        assert!(matches!(div_result, Err(FloatError::NoneOperand)));
    }

    #[test]
    fn test_none_operand_error_message() {
        const A: NonNegativeF64 = NonNegativeF64::new_const(5.0);
        let none: Option<NonNegativeF64> = None;

        let result: Result<NonNegativeF64, FloatError> = A * none;
        assert!(matches!(result, Err(FloatError::NoneOperand)));

        if let Err(e) = result {
            assert_eq!(
                format!("{}", e),
                "right-hand side operand is None in Option arithmetic"
            );
        }
    }
}

// ========== TryFrom Consistency Tests ==========

mod test_tryfrom_consistency {
    use super::*;

    #[test]
    fn test_try_from_primitive_consistency() {
        // TryFrom<f32> should return the same error as new()
        let result_new = NonNegativeF32::new(-1.0);
        let result_try: Result<NonNegativeF32, _> = NonNegativeF32::try_from(-1.0f32);

        match (result_new, result_try) {
            (Err(FloatError::OutOfRange), Err(FloatError::OutOfRange)) => {}
            _ => panic!("TryFrom and new() should return the same error type"),
        }
    }

    #[test]
    fn test_try_from_f64_to_f32_consistency() {
        // TryFrom<f32> for F32 types should call new() and return the same error
        // Use f32::MAX, which is a valid value
        let result_new = NonNegativeF32::new(f32::MAX);
        let result_try: Result<NonNegativeF32, _> = NonNegativeF32::try_from(f32::MAX);

        // Both should succeed
        assert!(result_new.is_ok());
        assert!(result_try.is_ok());
    }
}
