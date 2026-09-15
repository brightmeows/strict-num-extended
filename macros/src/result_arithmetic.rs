//! Result type arithmetic operations module
//!
//! Generates arithmetic operations for `Result<T, FloatError>` types, supporting:
//! 1. `Lhs op Result<Rhs, FloatError>`
//! 2. `Result<Lhs, FloatError> op Rhs`
//!
//! # Orphan Rule Limitations
//!
//! Due to Rust's orphan rule, the following pattern is **not implementable**:
//! - `Neg for Result<T, E>` - Result type negation operation
//!
//! # Alternative for Result Negation
//!
//! Use the `.map()` method:
//!
//! ```text
//! let a: Result<PositiveF64, FloatError> = Ok(PositiveF64::new_const(5.0));
//! let neg: Result<NegativeF64, FloatError> = a.map(|x| -x);
//! assert!(neg.is_ok());
//! assert_eq!(neg.unwrap().get(), -5.0);
//!
//! // Error propagation
//! let err: Result<PositiveF64, FloatError> = Err(FloatError::NaN);
//! let neg_err: Result<NegativeF64, FloatError> = err.map(|x| -x);
//! assert!(neg_err.is_err());
//! ```

use proc_macro2::TokenStream as TokenStream2;

use crate::config::{ArithmeticOp, TypeConfig, get_standard_arithmetic_ops};
use crate::generator::generate_arithmetic_for_all_types;

/// Generates arithmetic operations for Result types.
///
/// Supports two patterns (pattern 3 violates orphan rule):
/// 1. `Lhs op Result<Rhs, FloatError>` -> Result<Output, `FloatError`>
/// 2. `Result<Lhs, FloatError> op Rhs` -> Result<Output, `FloatError`>
///
/// Error propagation strategy:
/// - Safe operations: wrap concrete result in Ok(...)
/// - Fallible operations: directly propagate Result from base operation
/// - Division: zero check is handled by base operation
pub fn generate_result_arithmetic_impls(config: &TypeConfig) -> TokenStream2 {
    let ops = get_standard_arithmetic_ops();

    // Generate implementations for all three patterns
    let pattern1_impls = generate_pattern_lhs_op_result_rhs(config, &ops);
    let pattern2_impls = generate_pattern_result_lhs_op_rhs(config, &ops);
    // let pattern3_impls = generate_pattern_result_lhs_op_result_rhs(config, &ops);

    code! {
        #pattern1_impls
        #pattern2_impls
        // #pattern3_impls
    }
}

/// Pattern 1: Lhs op Result<Rhs, `FloatError`>
fn generate_pattern_lhs_op_result_rhs(
    config: &TypeConfig,
    ops: &[(ArithmeticOp, &str, &str, TokenStream2)],
) -> TokenStream2 {
    generate_arithmetic_for_all_types(
        config,
        ops,
        |lhs_alias,
         rhs_alias,
         output_alias,
         trait_ident,
         method_ident,
         _op_symbol,
         result,
         _op,
         _| {
            if result.is_safe {
                // Safe operation: base returns concrete type, wrap in Ok
                code! {
                    impl #trait_ident<Result<#rhs_alias, FloatError>> for #lhs_alias {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: Result<#rhs_alias, FloatError>) -> Self::Output {
                            match rhs {
                                Ok(b) => Ok(self.#method_ident(b)),
                                Err(e) => Err(e),
                            }
                        }
                    }
                }
            } else {
                // Fallible operation: base returns Result, directly propagate
                code! {
                    impl #trait_ident<Result<#rhs_alias, FloatError>> for #lhs_alias {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: Result<#rhs_alias, FloatError>) -> Self::Output {
                            match rhs {
                                Ok(b) => self.#method_ident(b),
                                Err(e) => Err(e),
                            }
                        }
                    }
                }
            }
        },
    )
}

/// Pattern 2: Result<Lhs, `FloatError`> op Rhs
fn generate_pattern_result_lhs_op_rhs(
    config: &TypeConfig,
    ops: &[(ArithmeticOp, &str, &str, TokenStream2)],
) -> TokenStream2 {
    generate_arithmetic_for_all_types(
        config,
        ops,
        |lhs_alias,
         rhs_alias,
         output_alias,
         trait_ident,
         method_ident,
         _op_symbol,
         result,
         _op,
         _| {
            if result.is_safe {
                // Safe operation: base returns concrete type, wrap in Ok
                code! {
                    impl #trait_ident<#rhs_alias> for Result<#lhs_alias, FloatError> {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                            match self {
                                Ok(a) => Ok(a.#method_ident(rhs)),
                                Err(e) => Err(e),
                            }
                        }
                    }
                }
            } else {
                // Fallible operation: base returns Result, directly propagate
                code! {
                    impl #trait_ident<#rhs_alias> for Result<#lhs_alias, FloatError> {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                            match self {
                                Ok(a) => a.#method_ident(rhs),
                                Err(e) => Err(e),
                            }
                        }
                    }
                }
            }
        },
    )
}
