//! # Proc Macro Implementation
//!
//! Provides complete procedural macro code generation for strict-num-extended
#![cfg_attr(docsrs, feature(doc_cfg))]

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[macro_use]
mod helpers;
mod arithmetic;
mod comparison;
mod config;
mod constants;
mod conversion;
mod doc_generator;
mod finite_float;
mod finite_float_trait;
mod float_conversion;
mod fromstr_impl;
mod generator;
mod option_arithmetic;
mod result_arithmetic;
mod type_aliases;
mod unary_ops;

use arithmetic::{generate_arithmetic_impls, generate_neg_impls};
use comparison::{generate_comparison_traits, generate_concrete_comparison_traits};
use config::TypeConfig;
use constants::generate_constants;
use conversion::generate_conversion_traits;
use finite_float::{
    generate_concrete_impls, generate_concrete_serde_impls, generate_concrete_structs,
};
use finite_float_trait::{generate_finite_float_impls, generate_finite_float_trait};
use float_conversion::{
    generate_as_f32_primitive_methods, generate_as_f32_type_methods,
    generate_as_f64_primitive_methods, generate_as_f64_type_methods,
    generate_try_into_f32_type_methods,
};
use fromstr_impl::{
    generate_fromstr_traits, generate_parse_error_from_impls, generate_parse_error_type,
};
use option_arithmetic::generate_option_arithmetic_impls;
use result_arithmetic::generate_result_arithmetic_impls;
use type_aliases::generate_type_aliases;
use unary_ops::{
    generate_abs_impls, generate_cos_impls, generate_signum_impls, generate_sin_impls,
    generate_tan_impls,
};

/// Generates common definitions (constants)
fn generate_common_definitions() -> proc_macro2::TokenStream {
    code! {
        use core::marker::PhantomData;
        use core::ops::{Add, Sub, Mul, Div, Neg};

        // ========== f64 boundary bit representation constants ==========
        const F64_MIN_BITS: i64 = f64::MIN.to_bits() as i64;
        const F64_MAX_BITS: i64 = f64::MAX.to_bits() as i64;
        const ZERO_BITS: i64 = 0.0f64.to_bits() as i64;
        // Use minimum positive normal number instead of EPSILON (to avoid excluding very small positive numbers)
        const F64_MIN_POSITIVE_BITS: i64 = f64::MIN_POSITIVE.to_bits() as i64;
        const F64_NEG_MIN_POSITIVE_BITS: i64 = (-f64::MIN_POSITIVE).to_bits() as i64;
        const ONE_BITS: i64 = 1.0f64.to_bits() as i64;
        const NEG_ONE_BITS: i64 = (-1.0f64).to_bits() as i64;

        // ========== f32 boundary bit representation constants (stored as f64) ==========
        const F32_MIN_BITS: i64 = (f32::MIN as f64).to_bits() as i64;
        const F32_MAX_BITS: i64 = (f32::MAX as f64).to_bits() as i64;
        // Use minimum positive normal number instead of EPSILON
        const F32_MIN_POSITIVE_BITS: i64 = (f32::MIN_POSITIVE as f64).to_bits() as i64;
        const F32_NEG_MIN_POSITIVE_BITS: i64 = ((-f32::MIN_POSITIVE) as f64).to_bits() as i64;
    }
}

/// Generates the `FloatError` type and its trait implementations
fn generate_error_type() -> proc_macro2::TokenStream {
    code! {
        /// Errors that can occur when creating or operating on finite floats
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum FloatError {
            /// Value is NaN (Not a Number)
            NaN,
            /// Value is positive infinity
            PosInf,
            /// Value is negative infinity
            NegInf,
            /// Value is outside the valid range for this type
            OutOfRange,
            /// Right-hand side operand is None in Option arithmetic
            NoneOperand,
        }

        impl core::fmt::Display for FloatError {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    FloatError::NaN => write!(f, "value is NaN (Not a Number)"),
                    FloatError::PosInf => write!(f, "value is positive infinity"),
                    FloatError::NegInf => write!(f, "value is negative infinity"),
                    FloatError::OutOfRange => write!(f, "value is outside the valid range for this type"),
                    FloatError::NoneOperand => write!(f, "right-hand side operand is None in Option arithmetic"),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for FloatError {}
    }
}

/// Generates zero-sized constraint marker types dynamically from config
fn generate_constraint_markers(config: &TypeConfig) -> proc_macro2::TokenStream {
    let markers = config.constraints.iter().map(|constraint| {
        let name = &constraint.name;
        code! {
            #[doc(hidden)]
            #[derive(Debug, Clone, Copy)]
            pub(crate) struct #name;
        }
    });

    code! {
        #(#markers)*
    }
}

/// Main macro: generates finite floating-point types with automatic `is_finite()` checking.
#[proc_macro]
pub fn generate_finite_float_types(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as TypeConfig);

    // Collect all code to be generated
    let mut all_code = vec![
        generate_common_definitions(),
        generate_error_type(),
        generate_parse_error_type(),
        generate_parse_error_from_impls(),
        generate_constraint_markers(&config),
        generate_concrete_structs(&config),
        generate_comparison_traits(),
    ];

    // Generate concrete struct implementations (includes new, get, new_unchecked, new_const)
    all_code.push(generate_concrete_impls(&config));
    all_code.push(generate_concrete_serde_impls(&config));
    all_code.push(generate_concrete_comparison_traits(&config));

    // Generate type-safe arithmetic operations
    all_code.push(generate_arithmetic_impls(&config));

    // Generate arithmetic operations for Option types
    all_code.push(generate_option_arithmetic_impls(&config));

    // Generate arithmetic operations for Result types
    all_code.push(generate_result_arithmetic_impls(&config));

    // Generate negation operations
    all_code.push(generate_neg_impls(&config));

    // Generate unary operations (abs, signum)
    all_code.push(generate_abs_impls(&config));
    all_code.push(generate_signum_impls(&config));

    // Generate trigonometric operations (sin, cos, tan)
    all_code.push(generate_sin_impls(&config));
    all_code.push(generate_cos_impls(&config));
    all_code.push(generate_tan_impls(&config));

    // Generate negation operations for Result types
    // Note: Cannot implement Neg for Result<T, E> due to orphan rules
    // Users should use .map() instead: result.map(|x| -x)
    // all_code.push(generate_result_neg_impls(&config));

    // Generate F32/F64 conversion methods
    all_code.push(generate_as_f32_primitive_methods(&config));
    all_code.push(generate_as_f64_primitive_methods(&config));
    all_code.push(generate_as_f32_type_methods(&config));
    all_code.push(generate_as_f64_type_methods(&config));
    all_code.push(generate_try_into_f32_type_methods(&config));

    // Generate From/TryFrom traits
    all_code.push(generate_conversion_traits(&config));

    // Generate FromStr trait implementations
    all_code.push(generate_fromstr_traits(&config));

    // Generate FiniteFloat trait and implementations
    all_code.push(generate_finite_float_trait(&config));
    all_code.push(generate_finite_float_impls(&config));

    // Generate constants
    all_code.push(generate_constants(&config));

    // Generate type aliases
    all_code.push(generate_type_aliases(&config));

    // Combine all code
    let expanded = code! {
        #(#all_code)*
    };

    TokenStream::from(expanded)
}
