//! # `FromStr` trait implementation generation module
//!
//! Automatically generates `FromStr` implementations for all constraint types

use crate::config::TypeConfig;
use crate::generator::for_all_constraint_float_types;

/// Generate `ParseFloatError` type and its trait implementations
pub fn generate_parse_error_type() -> proc_macro2::TokenStream {
    code! {
        /// String parsing error
        ///
        /// Contains three possible error variants:
        /// 1. Empty string after trimming whitespace
        /// 2. String cannot be parsed as a floating-point number
        /// 3. Parsing succeeded but validation failed (wraps FloatError)
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum ParseFloatError {
            /// String is empty after trimming whitespace
            Empty,
            /// String cannot be parsed as a valid floating-point number
            Invalid,
            /// Parsing succeeded but value validation failed
            ValidationFailed(FloatError),
        }

        impl core::fmt::Display for ParseFloatError {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    ParseFloatError::Empty => {
                        write!(f, "failed to parse string: empty")
                    }
                    ParseFloatError::Invalid => {
                        write!(f, "failed to parse string as a floating-point number")
                    }
                    ParseFloatError::ValidationFailed(e) => write!(f, "{}", e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for ParseFloatError {}
    }
}

/// Generate `From` implementations for `ParseFloatError`
pub fn generate_parse_error_from_impls() -> proc_macro2::TokenStream {
    code! {
        // Implement From for ParseFloatError
        impl From<core::num::ParseFloatError> for ParseFloatError {
            fn from(_: core::num::ParseFloatError) -> Self {
                ParseFloatError::Invalid
            }
        }

        impl From<FloatError> for ParseFloatError {
            fn from(err: FloatError) -> Self {
                ParseFloatError::ValidationFailed(err)
            }
        }
    }
}

/// Generate all `FromStr` implementations
pub fn generate_fromstr_traits(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        let struct_name = crate::generator::make_type_alias(type_name, float_type);

        code! {
            impl core::str::FromStr for #struct_name {
                type Err = ParseFloatError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    // 1. Trim whitespace and check for empty string
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        return Err(ParseFloatError::Empty);
                    }

                    // 2. Parse using standard library (automatically supports scientific notation)
                    // Using ? operator which uses From trait for error conversion
                    let value: #float_type = trimmed.parse()?;

                    // 3. Validate constraints (reuses existing new() logic)
                    Self::new(value).map_err(ParseFloatError::ValidationFailed)
                }
            }
        }
    });

    code! { #(#impls)* }
}
