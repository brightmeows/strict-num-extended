//! Binary arithmetic operations module

use proc_macro2::{Ident, TokenStream as TokenStream2};

use crate::config::{ArithmeticOp, ArithmeticResult, TypeConfig, get_standard_arithmetic_ops};
use crate::generator::{
    generate_arithmetic_for_all_types, generate_arithmetic_for_primitive_types,
};

/// Generates type-safe arithmetic operation implementations.
///
/// This generates cross-type arithmetic operations with automatic output type inference.
/// Safe operations return the result directly, while potentially failing operations return Option.
pub fn generate_arithmetic_impls(config: &TypeConfig) -> TokenStream2 {
    let ops = get_standard_arithmetic_ops();

    // Generate implementations for constraint type arithmetic (existing)
    let constraint_impls = generate_arithmetic_for_all_types(
        config,
        &ops,
        |lhs_alias,
         rhs_alias,
         output_alias,
         trait_ident,
         method_ident,
         op_symbol,
         result,
         op,
         _| {
            generate_arithmetic_impl(
                &lhs_alias,
                &rhs_alias,
                &output_alias,
                &trait_ident,
                &method_ident,
                &op_symbol,
                result,
                op,
                false,
            )
        },
    );

    // Generate implementations for primitive type arithmetic (new)
    let primitive_impls = generate_arithmetic_for_primitive_types(
        config,
        &ops,
        |lhs_alias,
         rhs_alias,
         output_alias,
         trait_ident,
         method_ident,
         op_symbol,
         result,
         op,
         is_reversed| {
            generate_arithmetic_impl(
                &lhs_alias,
                &rhs_alias,
                &output_alias,
                &trait_ident,
                &method_ident,
                &op_symbol,
                result,
                op,
                is_reversed,
            )
        },
    );

    code! {
        #constraint_impls
        #primitive_impls
    }
}

/// Generates a single arithmetic operation implementation
#[expect(clippy::too_many_lines)]
#[allow(clippy::too_many_arguments)]
fn generate_arithmetic_impl(
    lhs_alias: &Ident,
    rhs_alias: &Ident,
    output_alias: &Ident,
    trait_ident: &Ident,
    method_ident: &Ident,
    op_symbol: &TokenStream2,
    result: &ArithmeticResult,
    op: ArithmeticOp,
    is_reversed: bool,
) -> TokenStream2 {
    // Check if the operation involves a primitive type (f32 or f64)
    let rhs_is_primitive = rhs_alias == "f32" || rhs_alias == "f64";
    let lhs_is_primitive = lhs_alias == "f32" || lhs_alias == "f64";

    if result.is_safe && !rhs_is_primitive && !lhs_is_primitive {
        // Safe operation between constraint types: return result directly
        code! {
            impl #trait_ident<#rhs_alias> for #lhs_alias {
                type Output = #output_alias;

                fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                    let result = self.get() #op_symbol rhs.get();
                    // SAFETY: The arithmetic configuration has proven at compile time that
                    // for this specific combination of lhs_type and rhs_type, the result
                    // is guaranteed to satisfy output_type's constraints. The validation
                    // would be redundant since the type system already guarantees safety.
                    unsafe { #output_alias::new_unchecked(result) }
                }
            }
        }
    } else if rhs_is_primitive || lhs_is_primitive {
        // Operation with primitive type: must validate primitive value and return Result
        if is_reversed {
            // Primitive on left (e.g., f64 + FinF64)
            let fin_type = if lhs_alias == "f32" {
                code! { FinF32 }
            } else {
                code! { FinF64 }
            };
            // Division needs special handling to check for infinity
            if op == ArithmeticOp::Div {
                code! {
                    impl #trait_ident<#rhs_alias> for #lhs_alias {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                            // Validate that the primitive value is finite
                            let lhs_fin = #fin_type::new(self).map_err(|_| FloatError::NaN)?;
                            let result = lhs_fin.get() / rhs.get();
                            // Division may produce infinity, uniformly return NaN error
                            if !result.is_finite() {
                                return Err(FloatError::NaN);
                            }
                            #output_alias::new(result)
                        }
                    }
                }
            } else {
                code! {
                    impl #trait_ident<#rhs_alias> for #lhs_alias {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                            // Validate that the primitive value is finite
                            let lhs_fin = #fin_type::new(self).map_err(|_| FloatError::NaN)?;
                            let result = lhs_fin.get() #op_symbol rhs.get();
                            #output_alias::new(result)
                        }
                    }
                }
            }
        } else {
            // Primitive on right (e.g., FinF64 + f64)
            let fin_type = if rhs_alias == "f32" {
                code! { FinF32 }
            } else {
                code! { FinF64 }
            };
            // Division needs special handling to check for infinity
            if op == ArithmeticOp::Div {
                code! {
                    impl #trait_ident<#rhs_alias> for #lhs_alias {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                            // Validate that the primitive value is finite
                            let rhs_fin = #fin_type::new(rhs).map_err(|_| FloatError::NaN)?;
                            let result = self.get() / rhs_fin.get();
                            // Division may produce infinity, uniformly return NaN error
                            if !result.is_finite() {
                                return Err(FloatError::NaN);
                            }
                            #output_alias::new(result)
                        }
                    }
                }
            } else {
                code! {
                    impl #trait_ident<#rhs_alias> for #lhs_alias {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                            // Validate that the primitive value is finite
                            let rhs_fin = #fin_type::new(rhs).map_err(|_| FloatError::NaN)?;
                            let result = self.get() #op_symbol rhs_fin.get();
                            #output_alias::new(result)
                        }
                    }
                }
            }
        }
    } else if op == ArithmeticOp::Div {
        // Division between constraint types: result may be infinity, return NaN error
        code! {
            impl #trait_ident<#rhs_alias> for #lhs_alias {
                type Output = Result<#output_alias, FloatError>;

                fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                    let result = self.get() / rhs.get();
                    // Division may produce infinity, uniformly return NaN error
                    if !result.is_finite() {
                        return Err(FloatError::NaN);
                    }
                    #output_alias::new(result)
                }
            }
        }
    } else {
        // Potentially failing operation between constraint types: return Result
        code! {
            impl #trait_ident<#rhs_alias> for #lhs_alias {
                type Output = Result<#output_alias, FloatError>;

                fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                    let result = self.get() #op_symbol rhs.get();
                    #output_alias::new(result)
                }
            }
        }
    }
}
