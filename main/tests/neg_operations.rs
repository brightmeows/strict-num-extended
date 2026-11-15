//! # Negation Operation Tests
//!
//! Comprehensive test of negation operation type inference and constraint validation

// Strict floating-point comparisons and unwrap usage in test code are justified
#![expect(clippy::float_cmp_const, clippy::items_after_statements)]

use strict_num_extended::*;

/// Macro for testing basic negation operations with exact comparison
macro_rules! test_neg {
    ($test_name:ident, $InputType:ty, $input_value:expr, $OutputType:ty, $expected_value:expr) => {
        #[test]
        fn $test_name() {
            const INPUT: $InputType = <$InputType>::new_const($input_value);
            let output: $OutputType = -INPUT;
            assert_eq!(output.get(), $expected_value);
        }
    };
}

/// Macro for testing negation operations with floating-point tolerance
macro_rules! test_neg_approx {
    ($test_name:ident, $InputType:ty, $input_value:expr, $OutputType:ty, $expected_value:expr, $eps:ty) => {
        #[test]
        fn $test_name() {
            const INPUT: $InputType = <$InputType>::new_const($input_value);
            let output: $OutputType = -INPUT;
            assert!((output.get() - $expected_value).abs() < <$eps>::EPSILON);
        }
    };
}

/// Macro for testing double negation operations
macro_rules! test_double_neg {
    ($test_name:ident, $InputType:ty, $intermediate_type:ty, $value:expr) => {
        #[test]
        fn $test_name() {
            const ORIGINAL: $InputType = <$InputType>::new_const($value);
            let neg1: $intermediate_type = -ORIGINAL;
            let back: $InputType = -neg1;
            assert_eq!(back.get(), $value);
        }
    };
}

/// Macro for testing double negation with floating-point tolerance
macro_rules! test_double_neg_approx {
    ($test_name:ident, $InputType:ty, $value:expr, $eps:ty) => {
        #[test]
        fn $test_name() {
            const ORIGINAL: $InputType = <$InputType>::new_const($value);
            let neg1: $InputType = -ORIGINAL;
            let back: $InputType = -neg1;
            assert!((back.get() - $value).abs() < <$eps>::EPSILON);
        }
    };
}

/// Tests for `NonNegative` ↔ `NonPositive` conversion
mod test_nonnegative_nonpositive {
    use super::*;

    test_neg!(
        test_nonnegative_to_nonpositive_f64,
        NonNegativeF64,
        5.0,
        NonPositiveF64,
        -5.0
    );
    test_neg_approx!(
        test_nonpositive_to_nonnegative_f32,
        NonPositiveF32,
        -2.5,
        NonNegativeF32,
        2.5,
        f32
    );

    #[test]
    fn test_zero_negation() {
        const POS_ZERO: NonNegativeF32 = NonNegativeF32::new_const(0.0);
        let neg_zero: NonPositiveF32 = -POS_ZERO;
        assert_eq!(neg_zero.get(), -0.0);

        // Negate again to return to zero
        let back: NonNegativeF32 = -neg_zero;
        assert_eq!(back.get(), 0.0);
    }
}

/// Tests for `Positive` ↔ `Negative` conversion
mod test_positive_negative {
    use super::*;

    test_neg!(
        test_positive_to_negative,
        PositiveF64,
        10.0,
        NegativeF64,
        -10.0
    );
    test_neg!(
        test_negative_to_positive,
        NegativeF32,
        -2.5,
        PositiveF32,
        2.5
    );
    test_double_neg!(test_double_negation, PositiveF32, NegativeF32, 10.0);
}

/// Tests for Normalized ↔ `NegativeNormalized` conversion
mod test_normalized {
    use super::*;

    test_neg!(
        test_normalized_to_negative_normalized,
        NormalizedF64,
        0.75,
        NegativeNormalizedF64,
        -0.75
    );
    test_neg!(
        test_negative_normalized_to_normalized,
        NegativeNormalizedF32,
        -0.5,
        NormalizedF32,
        0.5
    );

    #[test]
    fn test_boundary_values() {
        // Test boundary values 0.0 and 1.0
        const ZERO: NormalizedF32 = NormalizedF32::new_const(0.0);
        let neg_zero: NegativeNormalizedF32 = -ZERO;
        assert_eq!(neg_zero.get(), -0.0);

        const ONE: NormalizedF32 = NormalizedF32::new_const(1.0);
        let neg_one: NegativeNormalizedF32 = -ONE;
        assert_eq!(neg_one.get(), -1.0);

        const NEG_MINUS_ONE: NegativeNormalizedF32 = NegativeNormalizedF32::new_const(-1.0);
        let back_to_one: NormalizedF32 = -NEG_MINUS_ONE;
        assert_eq!(back_to_one.get(), 1.0);
    }
}

/// Tests for reflexive constraints (Fin, `NonZero`)
mod test_reflexive {
    use super::*;

    test_neg!(test_fin_negation_f64, FinF64, 2.5, FinF64, -2.5);
    test_neg!(test_fin_negation_f32, FinF32, -1.5, FinF32, 1.5);
    test_neg!(test_nonzero_negation, NonZeroF32, 5.0, NonZeroF32, -5.0);
    test_double_neg_approx!(test_double_fin_negation, FinF64, 1.414, f64);
}

/// Tests for edge cases
mod test_edge_cases {
    use super::*;

    test_neg!(
        test_large_values,
        NonNegativeF64,
        1e100,
        NonPositiveF64,
        -1e100
    );
    test_neg!(
        test_small_values,
        NonNegativeF32,
        1e-30,
        NonPositiveF32,
        -1e-30
    );
    test_neg!(
        test_normalized_midpoint,
        NormalizedF64,
        0.5,
        NegativeNormalizedF64,
        -0.5
    );
}
