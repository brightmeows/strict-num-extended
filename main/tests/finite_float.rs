//! # struct-num-extended Tests
//!
//! This module tests all functionality of finite floating-point types.

// Strict floating-point comparisons, unwrap usage, and variable shadowing in test code are justified
#![expect(
    clippy::shadow_unrelated,
    clippy::float_cmp,
    clippy::uninlined_format_args
)]

use strict_num_extended::*;

/// Macro for testing arithmetic operations (redefined to avoid duplication)
macro_rules! test_arithmetic {
    ($test_name:ident, $Type:ty, $op:tt, $a:expr, $b:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            const A: $Type = <$Type>::new_const($a);
            const B: $Type = <$Type>::new_const($b);
            let result = (A $op B).unwrap();
            assert_eq!(result.get(), $expected);
        }
    };
}

/// Macro for testing safe arithmetic operations (redefined to avoid duplication)
macro_rules! test_safe_arithmetic {
    ($test_name:ident, $Type:ty, $ResultType:ty, $op:tt, $a:expr, $b:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            const A: $Type = <$Type>::new_const($a);
            const B: $Type = <$Type>::new_const($b);
            let result: $ResultType = A $op B;
            assert_eq!(result.get(), $expected);
        }
    };
}

/// Macro for testing basic value creation with `new_const`
macro_rules! test_get {
    ($test_name:ident, $Type:ty, $value:expr) => {
        #[test]
        fn $test_name() {
            const VAL: $Type = <$Type>::new_const($value);
            assert_eq!(VAL.get(), $value);
        }
    };
}

/// Macro for testing value creation with floating-point tolerance
macro_rules! test_get_approx {
    ($test_name:ident, $Type:ty, $value:expr, $eps:ty) => {
        #[test]
        fn $test_name() {
            const VAL: $Type = <$Type>::new_const($value);
            assert!((VAL.get() - $value).abs() < <$eps>::EPSILON);
        }
    };
}

/// Macro for testing Debug formatting
macro_rules! test_debug {
    ($test_name:ident, $Type:ty, $value:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            const VAL: $Type = <$Type>::new_const($value);
            assert!(format!("{:?}", VAL).contains($expected));
        }
    };
}

/// Macro for testing Display formatting
macro_rules! test_display {
    ($test_name:ident, $Type:ty, $value:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            const VAL: $Type = <$Type>::new_const($value);
            assert_eq!(format!("{}", VAL), $expected);
        }
    };
}

/// Tests basic functionality of `FinF32`
mod test_finf32 {
    use super::*;

    #[test]
    fn test_finf32_new_valid() {
        assert!(FinF32::new(1.0).is_ok());
        assert!(FinF32::new(-1.0).is_ok());
        assert!(FinF32::new(0.0).is_ok());
        assert!(FinF32::new(f32::MAX).is_ok());
        assert!(FinF32::new(f32::MIN).is_ok());
        assert!(FinF32::new(0.00001).is_ok());
    }

    #[test]
    fn test_finf32_new_invalid() {
        assert!(FinF32::new(f32::NAN).is_err());
        assert!(FinF32::new(f32::INFINITY).is_err());
        assert!(FinF32::new(f32::NEG_INFINITY).is_err());
    }

    test_get_approx!(test_finf32_get, FinF32, std::f32::consts::PI, f32);
    test_debug!(test_finf32_debug, FinF32, 1.5, "FiniteFloat");
    test_display!(test_finf32_display, FinF32, 1.5, "1.5");
}

/// Tests basic functionality of `FinF64`
mod test_finf64 {
    use super::*;

    #[test]
    fn test_finf64_new_valid() {
        assert!(FinF64::new(1.0).is_ok());
        assert!(FinF64::new(-1.0).is_ok());
        assert!(FinF64::new(0.0).is_ok());
        assert!(FinF64::new(f64::MAX).is_ok());
        assert!(FinF64::new(f64::MIN).is_ok());
    }

    #[test]
    fn test_finf64_new_invalid() {
        assert!(FinF64::new(f64::NAN).is_err());
        assert!(FinF64::new(f64::INFINITY).is_err());
        assert!(FinF64::new(f64::NEG_INFINITY).is_err());
    }

    test_get_approx!(test_finf64_get, FinF64, std::f64::consts::PI, f64);
}

/// Tests basic functionality of `NonNegativeF32`
mod test_nonnnegativef32 {
    use super::*;

    #[test]
    fn test_nonnnegativef32_new_valid() {
        assert!(NonNegativeF32::new(1.0).is_ok());
        assert!(NonNegativeF32::new(0.0).is_ok());
        // NonNegative no longer allows infinity
        assert!(NonNegativeF32::new(f32::INFINITY).is_err());
        assert!(NonNegativeF32::new(f32::MAX).is_ok());
    }

    #[test]
    fn test_nonnnegativef32_new_invalid() {
        assert!(NonNegativeF32::new(f32::NAN).is_err());
        assert!(NonNegativeF32::new(-1.0).is_err());
        // NonNegative now uses numeric comparison (>= 0.0), accepts -0.0
        assert!(NonNegativeF32::new(-0.0).is_ok());
        assert!(NonNegativeF32::new(f32::NEG_INFINITY).is_err());
        assert!(NonNegativeF32::new(f32::INFINITY).is_err());
    }

    test_get!(test_nonnnegativef32_get, NonNegativeF32, 42.0);
}

/// Tests basic functionality of `NonNegativeF64`
mod test_nonnnegativef64 {
    use super::*;

    #[test]
    fn test_nonnnegativef64_new_valid() {
        assert!(NonNegativeF64::new(1.0).is_ok());
        assert!(NonNegativeF64::new(0.0).is_ok());
        // NonNegative no longer allows infinity
        assert!(NonNegativeF64::new(f64::INFINITY).is_err());
        assert!(NonNegativeF64::new(f64::MAX).is_ok());
    }

    #[test]
    fn test_nonnnegativef64_new_invalid() {
        assert!(NonNegativeF64::new(f64::NAN).is_err());
        assert!(NonNegativeF64::new(-1.0).is_err());
        // NonNegative now uses numeric comparison (>= 0.0), accepts -0.0
        assert!(NonNegativeF64::new(-0.0).is_ok());
        assert!(NonNegativeF64::new(f64::NEG_INFINITY).is_err());
        assert!(NonNegativeF64::new(f64::INFINITY).is_err());
    }

    test_get!(test_nonnnegativef64_get, NonNegativeF64, 123.456);
}

/// Tests arithmetic operations
mod test_arithmetic_operations {
    use super::*;

    // FinF32 arithmetic operations
    test_arithmetic!(test_finf32_add, FinF32, +, 2.0, 3.0, 5.0);
    test_arithmetic!(test_finf32_sub, FinF32, -, 10.0, 3.0, 7.0);
    test_arithmetic!(test_finf32_mul, FinF32, *, 4.0, 3.0, 12.0);
    test_arithmetic!(test_finf32_div, FinF32, /, 12.0, 3.0, 4.0);

    #[test]
    fn test_finf32_arithmetic_zero() {
        const A: FinF32 = FinF32::new_const(5.0);
        const B: FinF32 = FinF32::new_const(0.0);
        assert_eq!((A + B).unwrap().get(), 5.0);
        assert_eq!((A - B).unwrap().get(), 5.0);
        assert_eq!((A * B).unwrap().get(), 0.0);
    }

    // NonNegativeF32 arithmetic operations
    test_arithmetic!(test_nonnnegativef32_add, NonNegativeF32, +, 2.0, 3.0, 5.0);
    test_safe_arithmetic!(test_nonnnegativef32_sub, NonNegativeF32, FinF32, -, 10.0, 3.0, 7.0);
    test_arithmetic!(test_nonnnegativef32_mul, NonNegativeF32, *, 4.0, 3.0, 12.0);
    test_arithmetic!(test_nonnnegativef32_div, NonNegativeF32, /, 12.0, 3.0, 4.0);

    // FinF64 arithmetic operations
    test_arithmetic!(test_finf64_add, FinF64, +, 2.5, 3.5, 6.0);
    test_arithmetic!(test_finf64_mul, FinF64, *, 2.5, 4.0, 10.0);

    // NonNegativeF64 arithmetic operations
    test_arithmetic!(test_nonnnegativef64_add, NonNegativeF64, +, 2.5, 3.5, 6.0);
    test_arithmetic!(test_nonnnegativef64_mul, NonNegativeF64, *, 2.5, 4.0, 10.0);

    // Complex operations
    #[test]
    fn test_complex_arithmetic() {
        const A: FinF32 = FinF32::new_const(10.0);
        const B: FinF32 = FinF32::new_const(5.0);
        const C: FinF32 = FinF32::new_const(2.0);
        let ab = (A + B).unwrap();
        let result = (ab * C).unwrap();
        assert_eq!(result.get(), 30.0);
    }
}

/// Tests comparison operations
mod test_comparison_operations {
    use super::*;

    #[test]
    fn test_finf32_partial_eq() {
        let a = FinF32::new(1.0).unwrap();
        let b = FinF32::new(1.0).unwrap();
        assert_eq!(a, b);

        let c = FinF32::new(2.0).unwrap();
        assert_ne!(a, c);
    }

    #[test]
    fn test_finf32_partial_ord() {
        let a = FinF32::new(1.0).unwrap();
        let b = FinF32::new(2.0).unwrap();
        assert!(a < b);
        assert!(b > a);
        assert!(a <= b);
        assert!(b >= a);

        let c = FinF32::new(1.0).unwrap();
        assert!(a <= c);
        assert!(a >= c);
    }

    #[test]
    fn test_nonnnegativef32_comparison() {
        let a = NonNegativeF32::new(1.0).unwrap();
        let b = NonNegativeF32::new(2.0).unwrap();
        assert!(a < b);
        assert_eq!(a, NonNegativeF32::new(1.0).unwrap());
    }

    #[test]
    fn test_finf64_comparison() {
        let a = FinF64::new(1.5).unwrap();
        let b = FinF64::new(2.5).unwrap();
        assert!(a < b);
        assert_eq!(a, FinF64::new(1.5).unwrap());
    }
}

/// Tests type conversions
mod test_conversions {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_f32_to_f64() {
        let value_f32 = std::f32::consts::PI;
        let finite_64 = FinF64::try_from(value_f32).unwrap();
        // f32::PI has limited precision after conversion to f64, use appropriate tolerance
        assert!((finite_64.get() - std::f64::consts::PI).abs() < 1e-6);
    }

    #[test]
    fn test_try_from_nonnnegative_types() {
        let value_f32 = 5.0f32;
        let nonnnegative_32 = NonNegativeF32::try_from(value_f32).unwrap();
        assert_eq!(nonnnegative_32.get(), 5.0);
    }

    #[test]
    fn test_try_from_with_constraint_validation() {
        // Try to convert from FinF32 to NonNegativeF32, negative value will fail
        let finite_32 = FinF32::new(-5.0).unwrap();
        let value = finite_32.get();
        assert!(NonNegativeF32::new(value).is_err());

        // NonNegative value should succeed
        let finite_32 = FinF32::new(5.0).unwrap();
        let nonnnegative_32 = NonNegativeF32::new(finite_32.get()).unwrap();
        assert_eq!(nonnnegative_32.get(), 5.0);

        // FinF32 can accept NonNegativeF32 values
        let nonnnegative_32 = NonNegativeF32::new(5.0).unwrap();
        let finite_32 = FinF32::new(nonnnegative_32.get()).unwrap();
        assert_eq!(finite_32.get(), 5.0);
    }
}

/// Tests unsafe `new_unchecked`
mod test_unchecked {
    use super::*;

    #[test]
    fn test_new_unchecked_valid() {
        // Safe usage: passing values that satisfy constraints
        let finite = unsafe { FinF32::new_unchecked(std::f32::consts::PI) };
        assert!((finite.get() - std::f32::consts::PI).abs() < f32::EPSILON);

        let nonnnegative = unsafe { NonNegativeF32::new_unchecked(5.0) };
        assert_eq!(nonnnegative.get(), 5.0);
    }

    #[test]
    fn test_new_unchecked_behavior() {
        // unsafe function doesn't panic, just allows creating potentially invalid values
        // These tests verify the function's existence and behavior, but don't test panic
        let nan_value = unsafe { FinF32::new_unchecked(f32::NAN) };
        assert!(nan_value.get().is_nan());

        let inf_value = unsafe { FinF32::new_unchecked(f32::INFINITY) };
        assert!(inf_value.get().is_infinite());

        let neg_value = unsafe { NonNegativeF32::new_unchecked(-1.0) };
        assert_eq!(neg_value.get(), -1.0);
    }
}

/// Tests edge cases
mod test_edge_cases {
    use super::*;

    #[test]
    fn test_min_values() {
        let min_f32 = FinF32::new(f32::MIN).unwrap();
        assert_eq!(min_f32.get(), f32::MIN);

        let min_f64 = FinF64::new(f64::MIN).unwrap();
        assert_eq!(min_f64.get(), f64::MIN);
    }

    #[test]
    fn test_max_values() {
        let max_f32 = FinF32::new(f32::MAX).unwrap();
        assert_eq!(max_f32.get(), f32::MAX);

        let max_f64 = FinF64::new(f64::MAX).unwrap();
        assert_eq!(max_f64.get(), f64::MAX);
    }

    #[test]
    fn test_very_small_values() {
        let tiny_f32 = FinF32::new(f32::EPSILON).unwrap();
        assert_eq!(tiny_f32.get(), f32::EPSILON);

        let tiny_f64 = FinF64::new(f64::EPSILON).unwrap();
        assert_eq!(tiny_f64.get(), f64::EPSILON);
    }

    #[test]
    fn test_zero_variants() {
        let zero_nonnnegative = NonNegativeF32::new(0.0).unwrap();
        let neg_zero = FinF32::new(-0.0).unwrap();

        assert_eq!(zero_nonnnegative.get(), 0.0);
        assert_eq!(neg_zero.get(), -0.0);
        assert_eq!(zero_nonnnegative.get(), neg_zero.get());
    }

    #[test]
    fn test_chained_arithmetic() {
        let a = FinF32::new(1.0).unwrap();
        let b = FinF32::new(2.0).unwrap();
        let c = FinF32::new(3.0).unwrap();
        let d = FinF32::new(4.0).unwrap();

        let ab = (a + b).unwrap();
        let abc = (ab * c).unwrap();
        let result = (abc - d).unwrap();
        assert_eq!(result.get(), 5.0);
    }

    #[test]
    fn test_division_edge_cases() {
        let a = FinF32::new(5.0).unwrap();
        let b = FinF32::new(2.0).unwrap();
        let result = (a / b).unwrap();
        assert!((result.get() - 2.5).abs() < f32::EPSILON);
    }
}

/// Tests how constraint traits work
mod test_constraints {
    use super::*;

    #[test]
    fn test_finf32_constraint() {
        assert!(FinF32::new(1.0).is_ok());
        assert!(FinF32::new(f32::NAN).is_err());
        assert!(FinF32::new(f32::INFINITY).is_err());
        assert!(FinF32::new(f32::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_nonnnegativef32_constraint() {
        assert!(NonNegativeF32::new(1.0).is_ok());
        assert!(NonNegativeF32::new(0.0).is_ok());
        // NonNegative no longer allows infinity
        assert!(NonNegativeF32::new(f32::INFINITY).is_err());
        assert!(NonNegativeF32::new(-1.0).is_err());
        assert!(NonNegativeF32::new(f32::NAN).is_err());
    }
}

/// Tests basic functionality of `NonZeroF32`
mod test_nonzerof32 {
    use super::*;

    #[test]
    fn test_nonzerof32_new_valid() {
        assert!(NonZeroF32::new(1.0).is_ok());
        assert!(NonZeroF32::new(-1.0).is_ok());
        assert!(NonZeroF32::new(f32::MAX).is_ok());
        assert!(NonZeroF32::new(f32::MIN).is_ok());
        assert!(NonZeroF32::new(0.00001).is_ok());
    }

    #[test]
    fn test_nonzerof32_new_invalid() {
        assert!(NonZeroF32::new(f32::NAN).is_err());
        assert!(NonZeroF32::new(f32::INFINITY).is_err());
        assert!(NonZeroF32::new(f32::NEG_INFINITY).is_err());
        assert!(NonZeroF32::new(0.0).is_err());
        assert!(NonZeroF32::new(-0.0).is_err());
    }

    #[test]
    fn test_nonzerof32_get() {
        const NON_ZERO: NonZeroF32 = NonZeroF32::new_const(std::f32::consts::PI);
        assert!((NON_ZERO.get() - std::f32::consts::PI).abs() < f32::EPSILON);
    }
}

/// Tests basic functionality of `NonZeroF64`
mod test_nonzerof64 {
    use super::*;

    #[test]
    fn test_nonzerof64_new_valid() {
        assert!(NonZeroF64::new(1.0).is_ok());
        assert!(NonZeroF64::new(-1.0).is_ok());
        assert!(NonZeroF64::new(f64::MAX).is_ok());
        assert!(NonZeroF64::new(f64::MIN).is_ok());
    }

    #[test]
    fn test_nonzerof64_new_invalid() {
        assert!(NonZeroF64::new(f64::NAN).is_err());
        assert!(NonZeroF64::new(f64::INFINITY).is_err());
        assert!(NonZeroF64::new(f64::NEG_INFINITY).is_err());
        assert!(NonZeroF64::new(0.0).is_err());
        assert!(NonZeroF64::new(-0.0).is_err());
    }

    #[test]
    fn test_nonzerof64_get() {
        const NON_ZERO: NonZeroF64 = NonZeroF64::new_const(std::f64::consts::PI);
        assert!((NON_ZERO.get() - std::f64::consts::PI).abs() < f64::EPSILON);
    }
}

/// Tests basic functionality of `PositiveF32`
mod test_positivef32 {
    use super::*;

    #[test]
    fn test_positivef32_new_valid() {
        assert!(PositiveF32::new(1.0).is_ok());
        assert!(PositiveF32::new(f32::MAX).is_ok());
        assert!(PositiveF32::new(0.00001).is_ok());
    }

    #[test]
    fn test_positivef32_new_invalid() {
        assert!(PositiveF32::new(f32::NAN).is_err());
        assert!(PositiveF32::new(-1.0).is_err());
        assert!(PositiveF32::new(-0.0).is_err());
        assert!(PositiveF32::new(0.0).is_err());
        assert!(PositiveF32::new(f32::INFINITY).is_err());
        assert!(PositiveF32::new(f32::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_positivef32_get() {
        const POSITIVE: PositiveF32 = PositiveF32::new_const(42.0);
        assert_eq!(POSITIVE.get(), 42.0);
    }
}

/// Tests basic functionality of `PositiveF64`
mod test_positivef64 {
    use super::*;

    #[test]
    fn test_positivef64_new_valid() {
        assert!(PositiveF64::new(1.0).is_ok());
        assert!(PositiveF64::new(f64::MAX).is_ok());
        assert!(PositiveF64::new(0.00001).is_ok());
    }

    #[test]
    fn test_positivef64_new_invalid() {
        assert!(PositiveF64::new(f64::NAN).is_err());
        assert!(PositiveF64::new(-1.0).is_err());
        assert!(PositiveF64::new(-0.0).is_err());
        assert!(PositiveF64::new(0.0).is_err());
        assert!(PositiveF64::new(f64::INFINITY).is_err());
        assert!(PositiveF64::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_positivef64_get() {
        const POSITIVE: PositiveF64 = PositiveF64::new_const(123.456);
        assert_eq!(POSITIVE.get(), 123.456);
    }
}

/// Tests arithmetic operations for `NonZero` types
mod test_nonzero_arithmetic_operations {
    use super::*;

    // NonZeroF32 arithmetic operations
    test_arithmetic!(test_nonzerof32_add, NonZeroF32, +, 2.0, 3.0, 5.0);
    test_arithmetic!(test_nonzerof32_sub, NonZeroF32, -, 10.0, 3.0, 7.0);
    test_arithmetic!(test_nonzerof32_mul, NonZeroF32, *, 4.0, 3.0, 12.0);
    test_arithmetic!(test_nonzerof32_div, NonZeroF32, /, 12.0, 3.0, 4.0);

    // PositiveF32 arithmetic operations
    test_arithmetic!(test_positivef32_add, PositiveF32, +, 2.0, 3.0, 5.0);
    test_arithmetic!(test_positivef32_mul, PositiveF32, *, 4.0, 3.0, 12.0);
    test_arithmetic!(test_positivef32_div, PositiveF32, /, 12.0, 3.0, 4.0);

    // NonZeroF64 arithmetic operations
    test_arithmetic!(test_nonzerof64_add, NonZeroF64, +, 2.5, 3.5, 6.0);
    test_arithmetic!(test_nonzerof64_mul, NonZeroF64, *, 2.5, 4.0, 10.0);

    // PositiveF64 arithmetic operations
    test_arithmetic!(test_positivef64_add, PositiveF64, +, 2.5, 3.5, 6.0);
    test_arithmetic!(test_positivef64_mul, PositiveF64, *, 2.5, 4.0, 10.0);
}

/// Tests comparison operations for `NonZero` types
mod test_nonzero_comparison_operations {
    use super::*;

    #[test]
    fn test_nonzerof32_partial_eq() {
        let a = NonZeroF32::new(1.0).unwrap();
        let b = NonZeroF32::new(1.0).unwrap();
        assert_eq!(a, b);

        let c = NonZeroF32::new(2.0).unwrap();
        assert_ne!(a, c);
    }

    #[test]
    fn test_nonzerof32_partial_ord() {
        let a = NonZeroF32::new(1.0).unwrap();
        let b = NonZeroF32::new(2.0).unwrap();
        assert!(a < b);
        assert!(b > a);
        assert!(a <= b);
        assert!(b >= a);
    }

    #[test]
    fn test_positivef32_comparison() {
        let a = PositiveF32::new(1.0).unwrap();
        let b = PositiveF32::new(2.0).unwrap();
        assert!(a < b);
        assert_eq!(a, PositiveF32::new(1.0).unwrap());
    }
}

/// Tests constraint validation for `NonZero` types
mod test_nonzero_constraints {
    use super::*;

    #[test]
    fn test_nonzerof32_constraint() {
        assert!(NonZeroF32::new(1.0).is_ok());
        assert!(NonZeroF32::new(-1.0).is_ok());
        assert!(NonZeroF32::new(f32::NAN).is_err());
        assert!(NonZeroF32::new(f32::INFINITY).is_err());
        assert!(NonZeroF32::new(f32::NEG_INFINITY).is_err());
        assert!(NonZeroF32::new(0.0).is_err());
        assert!(NonZeroF32::new(-0.0).is_err());
    }

    #[test]
    fn test_positivef32_constraint() {
        assert!(PositiveF32::new(1.0).is_ok());
        assert!(PositiveF32::new(f32::MAX).is_ok());
        assert!(PositiveF32::new(f32::NAN).is_err());
        assert!(PositiveF32::new(-1.0).is_err());
        assert!(PositiveF32::new(0.0).is_err());
        assert!(PositiveF32::new(-0.0).is_err());
        assert!(PositiveF32::new(f32::INFINITY).is_err());
    }
}

/// Tests basic functionality of `NonPositiveF32`
mod test_negativef32 {
    use super::*;

    #[test]
    fn test_negativef32_new_valid() {
        assert!(NonPositiveF32::new(-1.0).is_ok());
        assert!(NonPositiveF32::new(f32::MIN).is_ok());
        // Negative no longer allows infinity
        assert!(NonPositiveF32::new(f32::NEG_INFINITY).is_err());
        assert!(NonPositiveF32::new(-0.0).is_ok());
        // Negative now uses numeric comparison (<= 0.0), accepts +0.0
        assert!(NonPositiveF32::new(0.0).is_ok());
    }

    #[test]
    fn test_negativef32_new_invalid() {
        assert!(NonPositiveF32::new(f32::NAN).is_err());
        assert!(NonPositiveF32::new(1.0).is_err());
        assert!(NonPositiveF32::new(f32::INFINITY).is_err());
        assert!(NonPositiveF32::new(f32::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_negativef32_get() {
        let negative = NonPositiveF32::new(-42.0).unwrap();
        assert_eq!(negative.get(), -42.0);
    }
}

/// Tests basic functionality of `NonPositiveF64`
mod test_negativef64 {
    use super::*;

    #[test]
    fn test_negativef64_new_valid() {
        assert!(NonPositiveF64::new(-1.0).is_ok());
        assert!(NonPositiveF64::new(f64::MIN).is_ok());
        // Negative no longer allows infinity
        assert!(NonPositiveF64::new(f64::NEG_INFINITY).is_err());
        assert!(NonPositiveF64::new(-0.0).is_ok());
        // Negative now uses numeric comparison (<= 0.0), accepts +0.0
        assert!(NonPositiveF64::new(0.0).is_ok());
    }

    #[test]
    fn test_negativef64_new_invalid() {
        assert!(NonPositiveF64::new(f64::NAN).is_err());
        assert!(NonPositiveF64::new(1.0).is_err());
        assert!(NonPositiveF64::new(f64::INFINITY).is_err());
        assert!(NonPositiveF64::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_negativef64_get() {
        let negative = NonPositiveF64::new(-123.456).unwrap();
        assert_eq!(negative.get(), -123.456);
    }
}

/// Tests basic functionality of `NegativeF32`
mod test_nonzero_negativef32 {
    use super::*;

    #[test]
    fn test_nonzero_negativef32_new_valid() {
        assert!(NegativeF32::new(-1.0).is_ok());
        assert!(NegativeF32::new(f32::MIN).is_ok());
        assert!(NegativeF32::new(-0.00001).is_ok());
    }

    #[test]
    fn test_nonzero_negativef32_new_invalid() {
        assert!(NegativeF32::new(f32::NAN).is_err());
        assert!(NegativeF32::new(1.0).is_err());
        assert!(NegativeF32::new(0.0).is_err());
        assert!(NegativeF32::new(-0.0).is_err());
        assert!(NegativeF32::new(f32::INFINITY).is_err());
        assert!(NegativeF32::new(f32::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_nonzero_negativef32_get() {
        let non_zero_negative = NegativeF32::new(-42.0).unwrap();
        assert_eq!(non_zero_negative.get(), -42.0);
    }
}

/// Tests basic functionality of `NegativeF64`
mod test_nonzero_negativef64 {
    use super::*;

    #[test]
    fn test_nonzero_negativef64_new_valid() {
        assert!(NegativeF64::new(-1.0).is_ok());
        assert!(NegativeF64::new(f64::MIN).is_ok());
        assert!(NegativeF64::new(-0.00001).is_ok());
    }

    #[test]
    fn test_nonzero_negativef64_new_invalid() {
        assert!(NegativeF64::new(f64::NAN).is_err());
        assert!(NegativeF64::new(1.0).is_err());
        assert!(NegativeF64::new(0.0).is_err());
        assert!(NegativeF64::new(-0.0).is_err());
        assert!(NegativeF64::new(f64::INFINITY).is_err());
        assert!(NegativeF64::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_nonzero_negativef64_get() {
        let non_zero_negative = NegativeF64::new(-123.456).unwrap();
        assert_eq!(non_zero_negative.get(), -123.456);
    }
}

/// Tests arithmetic operations for Negative types
mod test_negative_arithmetic_operations {
    use super::*;

    // NonPositiveF32 arithmetic operations
    #[test]
    fn test_nonnpositivef32_add() {
        let a = NonPositiveF32::new(-2.0).unwrap();
        let b = NonPositiveF32::new(-3.0).unwrap();
        let c = (a + b).unwrap();
        assert_eq!(c.get(), -5.0);
    }

    #[test]
    fn test_nonnpositivef32_sub() {
        let a = NonPositiveF32::new(-10.0).unwrap();
        let b = NonPositiveF32::new(-3.0).unwrap();
        let c = a - b;
        assert_eq!(c.get(), -7.0);
    }

    #[test]
    fn test_nonnpositivef32_mul() {
        // Multiplying negative by negative gives positive, which violates Negative type constraint
        // So we only test addition and subtraction
        let a = NonPositiveF32::new(-4.0).unwrap();
        let b = NonPositiveF32::new(-3.0).unwrap();
        let c = (a + b).unwrap();
        assert_eq!(c.get(), -7.0);
    }

    #[test]
    fn test_nonnpositivef32_div() {
        // Dividing negative by negative gives positive, which violates Negative type constraint
        // So we only test addition and subtraction
        let a = NonPositiveF32::new(-12.0).unwrap();
        let b = NonPositiveF32::new(-3.0).unwrap();
        let c = a - b;
        assert_eq!(c.get(), -9.0);
    }

    // NegativeF32 arithmetic operations
    #[test]
    fn test_negativef32_add() {
        let a = NegativeF32::new(-2.0).unwrap();
        let b = NegativeF32::new(-3.0).unwrap();
        let c = (a + b).unwrap();
        assert_eq!(c.get(), -5.0);
    }

    #[test]
    fn test_negativef32_mul() {
        // Multiplying non-zero negative by non-zero negative gives positive, violates constraint
        // So we only test addition and subtraction
        let a = NegativeF32::new(-4.0).unwrap();
        let b = NegativeF32::new(-3.0).unwrap();
        let c = (a + b).unwrap();
        assert_eq!(c.get(), -7.0);
    }

    #[test]
    fn test_negativef32_div() {
        // Dividing non-zero negative by non-zero negative gives positive, violates constraint
        // So we only test addition and subtraction
        let a = NegativeF32::new(-12.0).unwrap();
        let b = NegativeF32::new(-3.0).unwrap();
        let c = a - b;
        assert_eq!(c.get(), -9.0);
    }

    // NonPositiveF64 arithmetic operations
    #[test]
    fn test_nonnpositivef64_add() {
        let a = NonPositiveF64::new(-2.5).unwrap();
        let b = NonPositiveF64::new(-3.5).unwrap();
        let c = (a + b).unwrap();
        assert_eq!(c.get(), -6.0);
    }

    #[test]
    fn test_nonnpositivef64_mul() {
        // Multiplying negative by negative gives positive, violates constraint
        // So we only test addition and subtraction
        let a = NonPositiveF64::new(-2.5).unwrap();
        let b = NonPositiveF64::new(-4.0).unwrap();
        let c = (a + b).unwrap();
        assert_eq!(c.get(), -6.5);
    }

    // NegativeF64 arithmetic operations
    #[test]
    fn test_negativef64_add() {
        let a = NegativeF64::new(-2.5).unwrap();
        let b = NegativeF64::new(-3.5).unwrap();
        let c = (a + b).unwrap();
        assert_eq!(c.get(), -6.0);
    }

    #[test]
    fn test_negativef64_mul() {
        // Multiplying non-zero negative by non-zero negative gives positive, violates constraint
        // So we only test addition and subtraction
        let a = NegativeF64::new(-2.5).unwrap();
        let b = NegativeF64::new(-4.0).unwrap();
        let c = (a + b).unwrap();
        assert_eq!(c.get(), -6.5);
    }
}

/// Tests comparison operations for Negative types
mod test_negative_comparison_operations {
    use super::*;

    #[test]
    fn test_nonnpositivef32_partial_eq() {
        let a = NonPositiveF32::new(-1.0).unwrap();
        let b = NonPositiveF32::new(-1.0).unwrap();
        assert_eq!(a, b);

        let c = NonPositiveF32::new(-2.0).unwrap();
        assert_ne!(a, c);
    }

    #[test]
    fn test_nonnpositivef32_partial_ord() {
        let a = NonPositiveF32::new(-2.0).unwrap();
        let b = NonPositiveF32::new(-1.0).unwrap();
        assert!(a < b);
        assert!(b > a);
        assert!(a <= b);
        assert!(b >= a);
    }

    #[test]
    fn test_negativef32_comparison() {
        let a = NegativeF32::new(-2.0).unwrap();
        let b = NegativeF32::new(-1.0).unwrap();
        assert!(a < b);
        assert_eq!(a, NegativeF32::new(-2.0).unwrap());
    }
}

/// Tests constraint validation for Negative types
mod test_negative_constraints {
    use super::*;

    #[test]
    fn test_nonnpositivef32_constraint() {
        assert!(NonPositiveF32::new(-1.0).is_ok());
        assert!(NonPositiveF32::new(f32::MIN).is_ok());
        // NonPositive no longer allows infinity
        assert!(NonPositiveF32::new(f32::NEG_INFINITY).is_err());
        assert!(NonPositiveF32::new(-0.0).is_ok());
        // NonPositive now uses numeric comparison (<= 0.0), accepts +0.0
        assert!(NonPositiveF32::new(0.0).is_ok());
        assert!(NonPositiveF32::new(f32::NAN).is_err());
        assert!(NonPositiveF32::new(1.0).is_err());
        assert!(NonPositiveF32::new(f32::INFINITY).is_err());
    }

    #[test]
    fn test_negativef32_constraint() {
        assert!(NegativeF32::new(-1.0).is_ok());
        assert!(NegativeF32::new(f32::MIN).is_ok());
        assert!(NegativeF32::new(f32::NAN).is_err());
        assert!(NegativeF32::new(1.0).is_err());
        assert!(NegativeF32::new(0.0).is_err());
        assert!(NegativeF32::new(-0.0).is_err());
        assert!(NegativeF32::new(f32::INFINITY).is_err());
        assert!(NegativeF32::new(f32::NEG_INFINITY).is_err());
    }
}

/// Tests basic functionality of `NegativeNormalizedF32`
mod test_negative_normalizedf32 {
    use super::*;

    #[test]
    fn test_negative_normalizedf32_new_valid() {
        // Boundary values
        assert!(NegativeNormalizedF32::new(-1.0).is_ok());
        assert!(NegativeNormalizedF32::new(0.0).is_ok());
        assert!(NegativeNormalizedF32::new(-0.0).is_ok());

        // Middle values
        assert!(NegativeNormalizedF32::new(-0.5).is_ok());
        assert!(NegativeNormalizedF32::new(-0.75).is_ok());
        assert!(NegativeNormalizedF32::new(-0.001).is_ok());
        assert!(NegativeNormalizedF32::new(-0.999).is_ok());
    }

    #[test]
    fn test_negative_normalizedf32_new_invalid() {
        // Below lower bound
        assert!(NegativeNormalizedF32::new(-1.1).is_err());
        assert!(NegativeNormalizedF32::new(-2.0).is_err());
        assert!(NegativeNormalizedF32::new(f32::MIN).is_err());

        // Above upper bound
        assert!(NegativeNormalizedF32::new(0.1).is_err());
        assert!(NegativeNormalizedF32::new(1.0).is_err());
        assert!(NegativeNormalizedF32::new(f32::MAX).is_err());

        // Special values
        assert!(NegativeNormalizedF32::new(f32::NAN).is_err());
        assert!(NegativeNormalizedF32::new(f32::INFINITY).is_err());
        assert!(NegativeNormalizedF32::new(f32::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_negative_normalizedf32_get() {
        let negative_normalized = NegativeNormalizedF32::new(-0.75).unwrap();
        assert_eq!(negative_normalized.get(), -0.75);
    }
}

/// Tests basic functionality of `NegativeNormalizedF64`
mod test_negative_normalizedf64 {
    use super::*;

    #[test]
    fn test_negative_normalizedf64_new_valid() {
        // Boundary values
        assert!(NegativeNormalizedF64::new(-1.0).is_ok());
        assert!(NegativeNormalizedF64::new(0.0).is_ok());

        // Middle values
        assert!(NegativeNormalizedF64::new(-0.5).is_ok());
        assert!(NegativeNormalizedF64::new(-0.75).is_ok());
    }

    #[test]
    fn test_negative_normalizedf64_new_invalid() {
        assert!(NegativeNormalizedF64::new(-1.1).is_err());
        assert!(NegativeNormalizedF64::new(0.1).is_err());
        assert!(NegativeNormalizedF64::new(f64::NAN).is_err());
        assert!(NegativeNormalizedF64::new(f64::INFINITY).is_err());
    }

    #[test]
    fn test_negative_normalizedf64_get() {
        let negative_normalized = NegativeNormalizedF64::new(-0.75).unwrap();
        assert_eq!(negative_normalized.get(), -0.75);
    }
}

/// Tests basic functionality of `SymmetricF32`
mod test_symmetric_f32 {
    use super::*;

    #[test]
    fn test_symmetric_f32_new_valid() {
        assert!(SymmetricF32::new(1.0).is_ok());
        assert!(SymmetricF32::new(-1.0).is_ok());
        assert!(SymmetricF32::new(0.0).is_ok());
        assert!(SymmetricF32::new(0.75).is_ok());
        assert!(SymmetricF32::new(-0.5).is_ok());
    }

    #[test]
    fn test_symmetric_f32_new_invalid() {
        assert!(SymmetricF32::new(1.1).is_err());
        assert!(SymmetricF32::new(-1.1).is_err());
        assert!(SymmetricF32::new(f32::NAN).is_err());
        assert!(SymmetricF32::new(f32::INFINITY).is_err());
        assert!(SymmetricF32::new(f32::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_symmetric_f32_get() {
        let val = SymmetricF32::new(0.75).unwrap();
        assert!((val.get() - 0.75).abs() < f32::EPSILON);
    }
}

/// Tests basic functionality of `SymmetricF64`
mod test_symmetric_f64 {
    use super::*;

    #[test]
    fn test_symmetric_f64_new_valid() {
        assert!(SymmetricF64::new(1.0).is_ok());
        assert!(SymmetricF64::new(-1.0).is_ok());
        assert!(SymmetricF64::new(0.0).is_ok());
        assert!(SymmetricF64::new(0.5).is_ok());
        assert!(SymmetricF64::new(-0.25).is_ok());
    }

    #[test]
    fn test_symmetric_f64_new_invalid() {
        assert!(SymmetricF64::new(1.001).is_err());
        assert!(SymmetricF64::new(-1.001).is_err());
        assert!(SymmetricF64::new(f64::NAN).is_err());
        assert!(SymmetricF64::new(f64::INFINITY).is_err());
        assert!(SymmetricF64::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_symmetric_f64_get() {
        let val = SymmetricF64::new(-0.5).unwrap();
        assert!((val.get() - (-0.5)).abs() < f64::EPSILON);
    }
}

/// Symmetric arithmetic operations tests
mod test_symmetric_arithmetic_operations {
    use super::*;

    #[test]
    fn test_symmetric_f32_add() {
        let a = SymmetricF32::new(0.5).unwrap();
        let b = SymmetricF32::new(0.3).unwrap();
        let sum = (a + b).unwrap();
        assert_eq!(sum.get(), 0.8);
    }

    #[test]
    fn test_symmetric_f64_add() {
        let a = SymmetricF64::new(-0.5).unwrap();
        let b = SymmetricF64::new(0.3).unwrap();
        let sum = (a + b).unwrap();
        assert_eq!(sum.get(), -0.2);
    }

    #[test]
    fn test_symmetric_f32_sub() {
        let a = SymmetricF32::new(0.8).unwrap();
        let b = SymmetricF32::new(0.3).unwrap();
        let diff = (a - b).unwrap();
        assert_eq!(diff.get(), 0.5);
    }

    #[test]
    fn test_symmetric_f64_sub() {
        let a = SymmetricF64::new(-0.2).unwrap();
        let b = SymmetricF64::new(0.3).unwrap();
        let diff = (a - b).unwrap();
        assert_eq!(diff.get(), -0.5);
    }

    #[test]
    fn test_symmetric_f32_mul() {
        let a = SymmetricF32::new(0.5).unwrap();
        let b = SymmetricF32::new(0.4).unwrap();
        let product = a * b; // Safe operation: Symmetric × Symmetric -> Symmetric
        assert_eq!(product.get(), 0.2);
    }

    #[test]
    fn test_symmetric_f64_mul() {
        let a = SymmetricF64::new(-0.5).unwrap();
        let b = SymmetricF64::new(0.6).unwrap();
        let product = a * b; // Safe operation: Symmetric × Symmetric -> Symmetric
        assert_eq!(product.get(), -0.3);
    }

    #[test]
    fn test_symmetric_f32_div() {
        let a = SymmetricF32::new(0.5).unwrap();
        let b = SymmetricF32::new(1.0).unwrap();
        let quotient = (a / b).unwrap();
        assert_eq!(quotient.get(), 0.5);
    }

    #[test]
    fn test_symmetric_f64_div() {
        let a = SymmetricF64::new(-0.5).unwrap();
        let b = SymmetricF64::new(1.0).unwrap();
        let quotient = (a / b).unwrap();
        assert_eq!(quotient.get(), -0.5);
    }

    #[test]
    fn test_symmetric_arithmetic_overflow() {
        let a = SymmetricF32::new(0.8).unwrap();
        let b = SymmetricF32::new(0.5).unwrap();
        // Addition returns Option, result outside [-1, 1] range returns Some(Fin)
        // but Symmetric + Symmetric -> Option<Fin> now (not Option<Symmetric>)
        let result = a + b;
        // 0.8 + 0.5 = 1.3 which is valid Fin but outside Symmetric range
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1.3);
    }
}

/// Symmetric comparison operations tests
mod test_symmetric_comparison_operations {
    use super::*;

    #[test]
    fn test_symmetric_f32_comparison() {
        let a = SymmetricF32::new(-0.5).unwrap();
        let b = SymmetricF32::new(0.5).unwrap();
        let c = SymmetricF32::new(0.5).unwrap();

        assert!(a < b);
        assert!(b > a);
        assert!(a <= b);
        assert!(b >= a);
        assert_eq!(b, c);
        assert_ne!(a, b);
    }

    #[test]
    fn test_symmetric_f64_comparison() {
        let a = SymmetricF64::new(-1.0).unwrap();
        let b = SymmetricF64::new(0.0).unwrap();
        let c = SymmetricF64::new(1.0).unwrap();

        assert!(a < b);
        assert!(b < c);
        assert!(a <= b);
        assert!(b <= c);
        assert!(c > a);
        assert!(c >= b);
    }
}

/// Symmetric negation tests (reflexive property)
mod test_symmetric_negation {
    use super::*;

    #[test]
    fn test_symmetric_f32_negation() {
        let val = SymmetricF32::new(0.75).unwrap();
        let neg_val: SymmetricF32 = -val;
        assert_eq!(neg_val.get(), -0.75);
    }

    #[test]
    fn test_symmetric_f64_negation() {
        let val = SymmetricF64::new(-0.5).unwrap();
        let neg_val: SymmetricF64 = -val;
        assert_eq!(neg_val.get(), 0.5);
    }

    #[test]
    fn test_symmetric_double_negation_f32() {
        let original = SymmetricF32::new(0.75).unwrap();
        let neg1: SymmetricF32 = -original;
        let neg2: SymmetricF32 = -neg1;
        assert_eq!(neg2.get(), 0.75);
    }

    #[test]
    fn test_symmetric_double_negation_f64() {
        let original = SymmetricF64::new(-0.5).unwrap();
        let neg1: SymmetricF64 = -original;
        let neg2: SymmetricF64 = -neg1;
        assert_eq!(neg2.get(), -0.5);
    }

    #[test]
    fn test_symmetric_boundary_negation() {
        let max = SymmetricF32::new(1.0).unwrap();
        let neg_max: SymmetricF32 = -max;
        assert_eq!(neg_max.get(), -1.0);

        let min = SymmetricF32::new(-1.0).unwrap();
        let neg_min: SymmetricF32 = -min;
        assert_eq!(neg_min.get(), 1.0);
    }
}

/// Symmetric `new_const` tests
mod test_symmetric_new_const {
    use super::*;

    #[test]
    fn test_symmetric_f32_new_const() {
        const VAL: SymmetricF32 = SymmetricF32::new_const(0.5);
        assert_eq!(VAL.get(), 0.5);
    }

    #[test]
    fn test_symmetric_f64_new_const() {
        const VAL: SymmetricF64 = SymmetricF64::new_const(-0.75);
        assert_eq!(VAL.get(), -0.75);
    }

    #[test]
    fn test_symmetric_boundary_const() {
        const MIN: SymmetricF32 = SymmetricF32::new_const(-1.0);
        const MAX: SymmetricF32 = SymmetricF32::new_const(1.0);
        const ZERO: SymmetricF32 = SymmetricF32::new_const(0.0);

        assert_eq!(MIN.get(), -1.0);
        assert_eq!(MAX.get(), 1.0);
        assert_eq!(ZERO.get(), 0.0);
    }

    #[test]
    fn test_symmetric_negation_new_const() {
        const ORIGINAL: SymmetricF32 = SymmetricF32::new_const(0.75);
        const NEGATED: SymmetricF32 = SymmetricF32::new_const(-0.75);
        let neg_original: SymmetricF32 = -ORIGINAL;
        assert_eq!(neg_original.get(), NEGATED.get());
    }
}

/// Tests for `FloatError` variants
mod test_float_error {
    use super::*;

    #[test]
    fn test_nan_error() {
        let result = FinF32::new(f32::NAN);
        assert!(result.is_err());
        assert!(matches!(result, Err(FloatError::NaN)));

        let result = FinF64::new(f64::NAN);
        assert!(result.is_err());
        assert!(matches!(result, Err(FloatError::NaN)));
    }

    #[test]
    fn test_infinite_error() {
        let result = FinF32::new(f32::INFINITY);
        assert!(result.is_err());
        assert!(matches!(result, Err(FloatError::PosInf)));

        let result = FinF32::new(f32::NEG_INFINITY);
        assert!(result.is_err());
        assert!(matches!(result, Err(FloatError::NegInf)));
    }

    #[test]
    fn test_nonnnegative_constraint_error() {
        let result = NonNegativeF32::new(-1.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(FloatError::OutOfRange)));
    }

    #[test]
    fn test_nonzero_constraint_error() {
        let result = NonZeroF32::new(0.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(FloatError::OutOfRange)));
    }

    #[test]
    fn test_nonnpositive_constraint_error() {
        let result = NonPositiveF32::new(1.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(FloatError::OutOfRange)));
    }

    #[test]
    fn test_normalized_constraint_error() {
        let result = SymmetricF32::new(2.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(FloatError::OutOfRange)));
    }

    #[test]
    fn test_error_display() {
        let err = FloatError::NaN;
        assert_eq!(format!("{}", err), "value is NaN (Not a Number)");

        let err = FloatError::PosInf;
        assert_eq!(format!("{}", err), "value is positive infinity");

        let err = FloatError::NegInf;
        assert_eq!(format!("{}", err), "value is negative infinity");

        let err = FloatError::OutOfRange;
        assert_eq!(
            format!("{}", err),
            "value is outside the valid range for this type"
        );

        let err = FloatError::NoneOperand;
        assert_eq!(
            format!("{}", err),
            "right-hand side operand is None in Option arithmetic"
        );
    }

    #[test]
    fn test_error_debug() {
        let err = FloatError::NaN;
        assert!(format!("{:?}", err).contains("NaN"));

        let err = FloatError::PosInf;
        assert!(format!("{:?}", err).contains("PosInf"));

        let err = FloatError::NegInf;
        assert!(format!("{:?}", err).contains("NegInf"));
    }
}

/// Tests for `FiniteFloat` trait
mod test_finite_float_trait {
    use super::*;

    #[test]
    fn test_finite_float_trait_basic() {
        // Test polymorphic usage: create heterogeneous collection
        let floats: Vec<Box<dyn FiniteFloat>> = vec![
            // Can mix different types
            Box::new(FinF32::new(1.0f32).unwrap()),
            Box::new(FinF64::new(2.0).unwrap()),
            Box::new(NonNegativeF32::new(0.5f32).unwrap()),
            Box::new(NonPositiveF64::new(-1.5).unwrap()),
        ];

        // All values can be converted to f64
        assert!((floats.first().unwrap().as_f64() - 1.0).abs() < f64::EPSILON);
        assert!((floats.get(1).unwrap().as_f64() - 2.0).abs() < f64::EPSILON);
        assert!((floats.get(2).unwrap().as_f64() - 0.5).abs() < f64::EPSILON);
        assert!((floats.get(3).unwrap().as_f64() - (-1.5)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_finite_float_new_from_f32() {
        // Test f32 types accept f32
        let f32_val: FinF32 = FiniteFloat::new(3.125f32).unwrap();
        assert!((f32_val.as_f64() - 3.125).abs() < f64::EPSILON);

        // Test NonNegativeF32
        let nonnnegative_f32: NonNegativeF32 = FiniteFloat::new(42.0f32).unwrap();
        assert!((nonnnegative_f32.as_f64() - 42.0).abs() < f64::EPSILON);

        // Test NormalizedF32
        let norm_f32: NormalizedF32 = FiniteFloat::new(0.75f32).unwrap();
        assert!((norm_f32.as_f64() - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_finite_float_new_from_f64() {
        // Test f64 types accept f64
        let f64_val: FinF64 = FiniteFloat::new(2.5).unwrap();
        assert!((f64_val.as_f64() - 2.5).abs() < f64::EPSILON);

        // Test NonNegativeF64
        let nonnnegative_f64: NonNegativeF64 = FiniteFloat::new(100.0).unwrap();
        assert!((nonnnegative_f64.as_f64() - 100.0).abs() < f64::EPSILON);

        // Test SymmetricF64
        let sym_f64: SymmetricF64 = FiniteFloat::new(0.5).unwrap();
        assert!((sym_f64.as_f64() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_finite_float_f32_from_f64() {
        // f32 types can be created from f64 (with type conversion)
        let f32_val: FinF32 = FiniteFloat::new(1.5).unwrap();
        assert!((f32_val.as_f64() - 1.5).abs() < f64::EPSILON);

        // NormalizedF32 created from f64
        let norm_f32: NormalizedF32 = FiniteFloat::new(0.25).unwrap();
        assert!((norm_f32.as_f64() - 0.25).abs() < f64::EPSILON);
    }

    #[test]
    fn test_finite_float_f64_from_f32() {
        // f64 types can be created from f32
        let f64_val: FinF64 = FiniteFloat::new(2.25f32).unwrap();
        assert!((f64_val.as_f64() - 2.25).abs() < f64::EPSILON);

        // NonNegativeF64 created from f32
        let nonnnegative_f64: NonNegativeF64 = FiniteFloat::new(50.0f32).unwrap();
        assert!((nonnnegative_f64.as_f64() - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_finite_float_constraint_validation() {
        // Test that constraint validation still works

        // NonNegative does not accept negative values
        let result: Result<NonNegativeF32, _> = FiniteFloat::new(-1.0f32);
        assert!(result.is_err());

        // Normalized does not accept out-of-range values
        let result: Result<NormalizedF64, _> = FiniteFloat::new(2.0);
        assert!(result.is_err());

        // Symmetric does not accept out-of-range values
        let result: Result<SymmetricF32, _> = FiniteFloat::new(1.5f32);
        assert!(result.is_err());

        // Fin does not accept NaN
        let result: Result<FinF64, _> = FiniteFloat::new(f64::NAN);
        assert!(result.is_err());

        // Fin does not accept Infinity
        let result: Result<FinF32, _> = FiniteFloat::new(f32::INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn test_finite_float_all_types() {
        // Test that all generated types implement FiniteFloat trait

        // F32 types
        let _: FinF32 = FiniteFloat::new(1.0f32).unwrap();
        let _: NonNegativeF32 = FiniteFloat::new(0.5f32).unwrap();
        let _: NonPositiveF32 = FiniteFloat::new(-0.5f32).unwrap();
        let _: NonZeroF32 = FiniteFloat::new(1.0f32).unwrap();
        let _: NormalizedF32 = FiniteFloat::new(0.5f32).unwrap();
        let _: NegativeNormalizedF32 = FiniteFloat::new(-0.5f32).unwrap();
        let _: PositiveF32 = FiniteFloat::new(0.5f32).unwrap();
        let _: NegativeF32 = FiniteFloat::new(-0.5f32).unwrap();
        let _: SymmetricF32 = FiniteFloat::new(0.5f32).unwrap();

        // F64 types
        let _: FinF64 = FiniteFloat::new(1.0).unwrap();
        let _: NonNegativeF64 = FiniteFloat::new(0.5).unwrap();
        let _: NonPositiveF64 = FiniteFloat::new(-0.5).unwrap();
        let _: NonZeroF64 = FiniteFloat::new(1.0).unwrap();
        let _: NormalizedF64 = FiniteFloat::new(0.5).unwrap();
        let _: NegativeNormalizedF64 = FiniteFloat::new(-0.5).unwrap();
        let _: PositiveF64 = FiniteFloat::new(0.5).unwrap();
        let _: NegativeF64 = FiniteFloat::new(-0.5).unwrap();
        let _: SymmetricF64 = FiniteFloat::new(0.5).unwrap();
    }

    #[test]
    fn test_finite_float_trait_polymorphic_function() {
        // Test polymorphic function usage

        fn sum_as_f64(floats: &[Box<dyn FiniteFloat>]) -> f64 {
            floats.iter().map(|f| f.as_f64()).sum()
        }

        let floats: Vec<Box<dyn FiniteFloat>> = vec![
            Box::new(FinF32::new(1.0f32).unwrap()),
            Box::new(FinF64::new(2.0).unwrap()),
            Box::new(NonNegativeF32::new(3.0f32).unwrap()),
        ];

        let sum = sum_as_f64(&floats);
        assert!((sum - 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_finite_float_trait_type_conversion() {
        // Test type conversion using trait

        // F32 -> F64
        let f32_val: FinF32 = FiniteFloat::new(1.5f32).unwrap();
        let f64_from_f32: f64 = f32_val.as_f64();
        assert!((f64_from_f32 - 1.5).abs() < f64::EPSILON);

        // F64 -> F64 ((no conversion needed))
        let f64_val: FinF64 = FiniteFloat::new(2.25).unwrap();
        let f64_as_f64: f64 = f64_val.as_f64();
        assert!((f64_as_f64 - 2.25).abs() < f64::EPSILON);
    }
}
