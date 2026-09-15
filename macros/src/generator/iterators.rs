//! Arithmetic operation iterators
//!
//! Contains iterator functions for generating arithmetic operations across type combinations.

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

use super::type_utils::make_type_alias;
use crate::config::{ArithmeticOp, ArithmeticResult, TypeConfig};

/// Generates arithmetic operation implementations for all constraint type combinations
///
/// This function encapsulates the common logic of iterating through lhs × rhs × `float_types`
/// combinations and looking up result types from the precomputed arithmetic results table.
///
/// # Arguments
///
/// * `config` - Type configuration
/// * `ops` - Operator definition array, format: (operator, trait name, method name, operator symbol)
/// * `impl_generator` - User-provided implementation generator function
///
/// # Returns
///
/// `TokenStream` of all generated arithmetic operation implementations
///
/// # Examples
///
/// ```ignore
/// let ops = [
///     (ArithmeticOp::Add, "Add", "add", code! { + }),
///     (ArithmeticOp::Sub, "Sub", "sub", code! { - }),
/// ];
///
/// generate_arithmetic_for_all_types(config, &ops, |lhs, rhs, output, trait_ident, method_ident, op_symbol, result, op| {
///     // Generate specific trait implementation
///     code! {
///         impl #trait_ident for #lhs {
///             // ...
///         }
///     }
/// })
/// ```
pub fn generate_arithmetic_for_all_types<F>(
    config: &TypeConfig,
    ops: &[(ArithmeticOp, &str, &str, TokenStream2)],
    mut impl_generator: F,
) -> TokenStream2
where
    F: FnMut(
        Ident,
        Ident,
        Ident,
        Ident,
        Ident,
        TokenStream2,
        &ArithmeticResult,
        ArithmeticOp,
        bool,
    ) -> TokenStream2,
{
    let mut impls = Vec::new();

    for lhs_type in &config.constraint_types {
        for rhs_type in &config.constraint_types {
            for (op, trait_name, method_name, op_symbol) in ops {
                let trait_ident = Ident::new(trait_name, Span::call_site());
                let method_ident = Ident::new(method_name, Span::call_site());

                // Get arithmetic result from precomputed table
                let key = (
                    *op,
                    lhs_type.type_name.to_string(),
                    rhs_type.type_name.to_string(),
                );
                #[expect(clippy::expect_used)]
                let result = config
                    .arithmetic_results
                    .get(&key)
                    .expect("Arithmetic result not found");

                for float_type in &lhs_type.float_types {
                    let lhs_alias = make_type_alias(&lhs_type.type_name, float_type);
                    let rhs_alias = make_type_alias(&rhs_type.type_name, float_type);
                    let output_alias = make_type_alias(&result.output_type, float_type);

                    let impl_code = impl_generator(
                        lhs_alias,
                        rhs_alias,
                        output_alias,
                        trait_ident.clone(),
                        method_ident.clone(),
                        op_symbol.clone(),
                        result,
                        *op,
                        false, // not reversed for constraint-constraint operations
                    );

                    impls.push(impl_code);
                }
            }
        }
    }

    code! {
        #(#impls)*
    }
}

/// Generates arithmetic operation implementations for constraint types with primitive types (f32, f64).
///
/// This function generates implementations for:
/// 1. Constraint type op primitive type (e.g., `FinF64` + f64)
/// 2. Primitive type op constraint type (e.g., f64 + `FinF64`)
///
/// The primitive type is treated as a Fin constraint, and the result type is determined
/// by the existing arithmetic results table.
///
/// # Arguments
///
/// * `config` - Type configuration
/// * `ops` - Operator definition array
/// * `impl_generator` - User-provided implementation generator function
///
/// # Returns
///
/// `TokenStream` of all generated arithmetic operation implementations
pub fn generate_arithmetic_for_primitive_types<F>(
    config: &TypeConfig,
    ops: &[(ArithmeticOp, &str, &str, TokenStream2)],
    mut impl_generator: F,
) -> TokenStream2
where
    F: FnMut(
        Ident,
        Ident,
        Ident,
        Ident,
        Ident,
        TokenStream2,
        &ArithmeticResult,
        ArithmeticOp,
        bool,
    ) -> TokenStream2,
{
    let mut impls = Vec::new();

    // Define primitive type mappings - use "Fin" as the constraint name
    let primitive_mappings = vec![("f32", "Fin"), ("f64", "Fin")];

    // 1. Constraint type op primitive type (e.g., FinF64 + f64)
    for lhs_type in &config.constraint_types {
        for (primitive_name, fin_constraint) in &primitive_mappings {
            for (op, trait_name, method_name, op_symbol) in ops {
                let trait_ident = Ident::new(trait_name, Span::call_site());
                let method_ident = Ident::new(method_name, Span::call_site());

                // Look up the arithmetic result (constraint type op Fin constraint)
                let key = (
                    *op,
                    lhs_type.type_name.to_string(),
                    fin_constraint.to_string(),
                );
                if let Some(result) = config.arithmetic_results.get(&key) {
                    for float_type in &lhs_type.float_types {
                        // Only generate operations where the primitive type matches the float type
                        if float_type.to_string().as_str() != *primitive_name {
                            continue;
                        }
                        let lhs_alias = make_type_alias(&lhs_type.type_name, float_type);
                        let primitive_ident = Ident::new(primitive_name, Span::call_site());
                        let output_alias = make_type_alias(&result.output_type, float_type);

                        let impl_code = impl_generator(
                            lhs_alias,
                            primitive_ident,
                            output_alias,
                            trait_ident.clone(),
                            method_ident.clone(),
                            op_symbol.clone(),
                            result,
                            *op,
                            false, // not reversed
                        );

                        impls.push(impl_code);
                    }
                }
            }
        }
    }

    // 2. Primitive type op constraint type (e.g., f64 + FinF64)
    for (primitive_name, fin_constraint) in &primitive_mappings {
        for rhs_type in &config.constraint_types {
            for (op, trait_name, method_name, op_symbol) in ops {
                let trait_ident = Ident::new(trait_name, Span::call_site());
                let method_ident = Ident::new(method_name, Span::call_site());

                // Look up the arithmetic result (Fin constraint op constraint type)
                let key = (
                    *op,
                    fin_constraint.to_string(),
                    rhs_type.type_name.to_string(),
                );
                if let Some(result) = config.arithmetic_results.get(&key) {
                    for float_type in &rhs_type.float_types {
                        // Only generate operations where the primitive type matches the float type
                        if float_type.to_string().as_str() != *primitive_name {
                            continue;
                        }
                        let primitive_ident = Ident::new(primitive_name, Span::call_site());
                        let rhs_alias = make_type_alias(&rhs_type.type_name, float_type);
                        let output_alias = make_type_alias(&result.output_type, float_type);

                        let impl_code = impl_generator(
                            primitive_ident,
                            rhs_alias,
                            output_alias,
                            trait_ident.clone(),
                            method_ident.clone(),
                            op_symbol.clone(),
                            result,
                            *op,
                            true, // reversed: primitive is on the left
                        );

                        impls.push(impl_code);
                    }
                }
            }
        }
    }

    code! {
        #(#impls)*
    }
}
