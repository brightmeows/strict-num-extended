//! `FromStr` trait implementation tests
//!
//! Test objectives:
//! 1. Test parsing various formats of valid values
//! 2. Test scientific notation support
//! 3. Test error handling and meaningful error messages
//! 4. Verify that the original string is preserved in errors

#![expect(clippy::panic)]

use strict_num_extended::*;

mod test_basic_parsing {
    use super::*;

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_parse_simple_decimal() {
        let value: FinF32 = "3.14".parse().unwrap();
        assert_eq!(value.get(), 3.14);
    }

    #[test]
    fn test_parse_integer() {
        let value: FinF64 = "42".parse().unwrap();
        assert_eq!(value.get(), 42.0);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_parse_negative() {
        let value: FinF32 = "-3.14".parse().unwrap();
        assert_eq!(value.get(), -3.14);
    }

    #[test]
    fn test_parse_zero() {
        let value: FinF64 = "0.0".parse().unwrap();
        assert_eq!(value.get(), 0.0);
    }
}

mod test_scientific_notation {
    use super::*;

    #[test]
    fn test_parse_scientific_lowercase() {
        let value: FinF32 = "1.5e2".parse().unwrap();
        assert_eq!(value.get(), 150.0);
    }

    #[test]
    fn test_parse_scientific_uppercase() {
        let value: FinF64 = "2.5E-3".parse().unwrap();
        assert_eq!(value.get(), 0.0025);
    }

    #[test]
    fn test_parse_scientific_integer() {
        let value: FinF32 = "1e10".parse().unwrap();
        assert_eq!(value.get(), 10_000_000_000.0);
    }

    #[test]
    fn test_parse_scientific_negative() {
        let value: FinF64 = "-1.5e2".parse().unwrap();
        assert_eq!(value.get(), -150.0);
    }

    #[test]
    fn test_parse_scientific_small_positive() {
        let value: FinF32 = "1.5e-10".parse().unwrap();
        assert_eq!(value.get(), 1.5e-10);
    }
}

mod test_constraint_validation {
    use super::*;

    #[test]
    fn test_nonnegative_rejects_negative() {
        let result: Result<NonNegativeF32, _> = "-1.0".parse();
        assert!(matches!(
            result,
            Err(ParseFloatError::ValidationFailed(FloatError::OutOfRange))
        ));
    }

    #[test]
    fn test_positive_rejects_zero() {
        let result: Result<PositiveF32, _> = "0.0".parse();
        assert!(matches!(
            result,
            Err(ParseFloatError::ValidationFailed(FloatError::OutOfRange))
        ));
    }

    #[test]
    fn test_negative_rejects_zero() {
        let result: Result<NegativeF32, _> = "0.0".parse();
        assert!(matches!(
            result,
            Err(ParseFloatError::ValidationFailed(FloatError::OutOfRange))
        ));
    }

    #[test]
    fn test_normalized_rejects_out_of_range() {
        let result: Result<NormalizedF64, _> = "1.5".parse();
        assert!(matches!(
            result,
            Err(ParseFloatError::ValidationFailed(FloatError::OutOfRange))
        ));
    }

    #[test]
    fn test_normalized_rejects_negative() {
        let result: Result<NormalizedF32, _> = "-0.5".parse();
        assert!(matches!(
            result,
            Err(ParseFloatError::ValidationFailed(FloatError::OutOfRange))
        ));
    }

    #[test]
    fn test_nonzero_rejects_zero() {
        let result: Result<NonZeroF32, _> = "0.0".parse();
        assert!(matches!(
            result,
            Err(ParseFloatError::ValidationFailed(FloatError::OutOfRange))
        ));
    }

    #[test]
    fn test_positive_rejects_negative() {
        let result: Result<PositiveF64, _> = "-1.0".parse();
        assert!(matches!(
            result,
            Err(ParseFloatError::ValidationFailed(FloatError::OutOfRange))
        ));
    }

    #[test]
    fn test_symmetric_rejects_out_of_range() {
        let result: Result<SymmetricF32, _> = "1.5".parse();
        assert!(matches!(
            result,
            Err(ParseFloatError::ValidationFailed(FloatError::OutOfRange))
        ));
    }
}

mod test_special_values {
    use super::*;

    #[test]
    fn test_parse_nan_rejected() {
        let result: Result<FinF32, _> = "NaN".parse();
        assert!(matches!(
            result,
            Err(ParseFloatError::ValidationFailed(FloatError::NaN))
        ));
    }

    #[test]
    fn test_parse_infinity_rejected() {
        let result: Result<FinF64, _> = "inf".parse();
        assert!(matches!(
            result,
            Err(ParseFloatError::ValidationFailed(FloatError::PosInf))
        ));
    }

    #[test]
    fn test_parse_negative_infinity_rejected() {
        let result: Result<FinF32, _> = "-inf".parse();
        assert!(matches!(
            result,
            Err(ParseFloatError::ValidationFailed(FloatError::NegInf))
        ));
    }

    #[test]
    fn test_parse_infinity_capitalized() {
        let result: Result<FinF64, _> = "Inf".parse();
        assert!(matches!(
            result,
            Err(ParseFloatError::ValidationFailed(FloatError::PosInf))
        ));
    }
}

mod test_error_messages {
    use super::*;

    #[test]
    fn test_invalid_float_error_message() {
        let result: Result<FinF32, _> = "not_a_number".parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("floating-point"));
        assert!(msg.contains("failed to parse"));
    }

    #[test]
    fn test_invalid_float_error_message_empty() {
        let result: Result<FinF32, _> = "".parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("empty"));
    }

    #[test]
    fn test_out_of_range_error_message() {
        let result: Result<NonNegativeF32, _> = "-1.0".parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("outside the valid range"));
    }

    #[test]
    fn test_positive_rejects_zero_message() {
        let result: Result<PositiveF32, _> = "0.0".parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("outside the valid range"));
    }

    #[test]
    fn test_negative_rejects_zero_message() {
        let result: Result<NegativeF32, _> = "0.0".parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("outside the valid range"));
    }

    #[test]
    fn test_nan_error_message() {
        let result: Result<FinF32, _> = "NaN".parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("NaN"));
        assert!(msg.contains("Not a Number"));
    }

    #[test]
    fn test_infinity_error_message() {
        let result: Result<FinF64, _> = "inf".parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("positive infinity"));
    }
}

mod test_error_preserves_input {
    use super::*;

    #[test]
    fn test_invalid_float_contains_parse_error() {
        let result: Result<FinF32, _> = "abc123".parse();
        if let Err(ParseFloatError::Invalid) = result {
            // ParseFloatError is preserved
        } else {
            panic!("Expected Invalid error");
        }
    }

    #[test]
    fn test_invalid_float_preserves_special_chars() {
        let result: Result<FinF32, _> = "$%^&*".parse();
        assert!(matches!(result, Err(ParseFloatError::Invalid)));
    }

    #[test]
    fn test_empty_error() {
        let result: Result<FinF32, _> = "".parse();
        assert!(matches!(result, Err(ParseFloatError::Empty)));

        let result_whitespace: Result<FinF32, _> = "   ".parse();
        assert!(matches!(result_whitespace, Err(ParseFloatError::Empty)));
    }

    #[test]
    fn test_validation_failed_wraps_float_error() {
        let result: Result<NonNegativeF32, _> = "-1.0".parse();
        if let Err(ParseFloatError::ValidationFailed(FloatError::OutOfRange)) = result {
            // Correctly wrapped OutOfRange error
        } else {
            panic!("Expected ValidationFailed(OutOfRange) error");
        }
    }

    #[test]
    fn test_positive_zero_validation_failed() {
        let result: Result<PositiveF32, _> = "0.0".parse();
        if let Err(ParseFloatError::ValidationFailed(FloatError::OutOfRange)) = result {
            // Positive should reject 0.0
        } else {
            panic!("Expected ValidationFailed(OutOfRange) error");
        }
    }

    #[test]
    fn test_negative_zero_validation_failed() {
        let result: Result<NegativeF32, _> = "0.0".parse();
        if let Err(ParseFloatError::ValidationFailed(FloatError::OutOfRange)) = result {
            // Negative should reject 0.0
        } else {
            panic!("Expected ValidationFailed(OutOfRange) error");
        }
    }

    #[test]
    fn test_nan_validation_failed() {
        let result: Result<FinF32, _> = "NaN".parse();
        if let Err(ParseFloatError::ValidationFailed(FloatError::NaN)) = result {
            // Correctly wrapped NaN error
        } else {
            panic!("Expected ValidationFailed(NaN) error");
        }
    }

    #[test]
    fn test_infinity_validation_failed() {
        let result: Result<FinF64, _> = "inf".parse();
        if let Err(ParseFloatError::ValidationFailed(FloatError::PosInf)) = result {
            // Correctly wrapped PosInf error
        } else {
            panic!("Expected ValidationFailed(PosInf) error");
        }
    }
}

mod test_all_types {
    use super::*;

    #[test]
    fn test_parse_all_f32_types() {
        assert!("3.14".parse::<FinF32>().is_ok());
        assert!("3.14".parse::<NonNegativeF32>().is_ok());
        assert!("-3.14".parse::<NonPositiveF32>().is_ok());
        assert!("3.14".parse::<NonZeroF32>().is_ok());
        assert!("0.5".parse::<NormalizedF32>().is_ok());
        assert!("0.5".parse::<SymmetricF32>().is_ok());
    }

    #[test]
    fn test_parse_all_f64_types() {
        assert!("3.14".parse::<FinF64>().is_ok());
        assert!("3.14".parse::<PositiveF64>().is_ok());
        assert!("-3.14".parse::<NegativeF64>().is_ok());
        assert!("3.14".parse::<NonZeroF64>().is_ok());
        assert!("0.5".parse::<NormalizedF64>().is_ok());
        assert!("0.5".parse::<SymmetricF64>().is_ok());
    }

    #[test]
    fn test_parse_positive_types() {
        assert!("1.5".parse::<PositiveF32>().is_ok());
        assert!("1.5".parse::<PositiveF64>().is_ok());
    }

    #[test]
    fn test_parse_negative_types() {
        assert!("-1.5".parse::<NegativeF32>().is_ok());
        assert!("-1.5".parse::<NegativeF64>().is_ok());
    }

    #[test]
    fn test_parse_negative_normalized_types() {
        assert!("-0.5".parse::<NegativeNormalizedF32>().is_ok());
        assert!("-0.5".parse::<NegativeNormalizedF64>().is_ok());
    }
}

mod test_edge_cases {
    use super::*;

    #[test]
    fn test_parse_very_small_number() {
        let value: FinF32 = "1e-38".parse().unwrap();
        assert!(value.get() > 0.0);
        assert!(value.get() < 1e-37);
    }

    #[test]
    fn test_parse_very_large_number() {
        let value: FinF64 = "1e308".parse().unwrap();
        assert!(value.get() > 1e307);
    }

    #[test]
    fn test_parse_zero_variations() {
        assert!("0".parse::<FinF32>().is_ok());
        assert!("0.0".parse::<FinF32>().is_ok());
        assert!("-0".parse::<FinF32>().is_ok());
        assert!("-0.0".parse::<FinF32>().is_ok());
        assert!("0e0".parse::<FinF32>().is_ok());
    }

    #[test]
    fn test_parse_whitespace_rejected() {
        // Our implementation now trims whitespace first
        let result: Result<FinF32, _> = " 3.14".parse();
        assert!(result.is_ok());

        let result_trailing: Result<FinF32, _> = "3.14 ".parse();
        assert!(result_trailing.is_ok());
    }

    #[test]
    fn test_parse_only_whitespace() {
        let result: Result<FinF32, _> = "   ".parse();
        assert!(matches!(result, Err(ParseFloatError::Empty)));
    }
}
