//! # From/TryFrom trait implementation generation module
//!
//! Automatically generates standard library trait implementations for all type conversions

use crate::config::TypeConfig;
use crate::generator::{
    filter_constraint_types_by_float, find_constraint_def, for_all_constraint_float_types,
};
use proc_macro2::Ident;
use quote::format_ident;

/// Generate all From/TryFrom implementations
pub fn generate_conversion_traits(config: &TypeConfig) -> proc_macro2::TokenStream {
    let mut all_code = vec![];

    // 1. Constraint type → Primitive (From)
    all_code.push(generate_constraint_to_primitive_from(config));

    // 2. Primitive → Constraint type (TryFrom)
    all_code.push(generate_primitive_to_constraint_tryfrom(config));

    // 3. Constraint type → Constraint type (From/TryFrom)
    all_code.push(generate_constraint_to_constraint_traits(config));

    // 4. F32 → F64 (From)
    all_code.push(generate_f32_to_f64_from(config));

    // 5. F64 → F32 (TryFrom)
    all_code.push(generate_f64_to_f32_tryfrom(config));

    // 6. f32 → F64 constraint type (TryFrom)
    all_code.push(generate_f32_to_f64_constraint_tryfrom(config));

    // 7. f64 → F32 constraint type (TryFrom)
    all_code.push(generate_f64_to_f32_constraint_tryfrom(config));

    code! { #(#all_code)* }
}

/// Check if source constraint is contained in target constraint (always safe)
fn is_subset_constraint(
    src_lower: Option<f64>,
    src_upper: Option<f64>,
    src_excludes_zero: bool,
    dst_lower: Option<f64>,
    dst_upper: Option<f64>,
    dst_excludes_zero: bool,
) -> bool {
    // 1. Target lower bound must be <= source lower bound
    let lower_contains = match (src_lower, dst_lower) {
        (Some(src), Some(dst)) => src >= dst,
        (None, Some(_)) => false,
        _ => true,
    };

    // 2. Target upper bound must be >= source upper bound
    let upper_contains = match (src_upper, dst_upper) {
        (Some(src), Some(dst)) => src <= dst,
        (None, Some(_)) => false,
        _ => true,
    };

    // 3. Zero exclusion requirements must be compatible
    let zero_compatible = !dst_excludes_zero || src_excludes_zero;

    lower_contains && upper_contains && zero_compatible
}

/// Generate: Constraint type → Primitive (From)
fn generate_constraint_to_primitive_from(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        let alias = format_ident!("{}{}", type_name, float_type.to_string().to_uppercase());

        code! {
            impl From<#alias> for #float_type {
                #[inline]
                fn from(value: #alias) -> Self {
                    value.get()
                }
            }
        }
    });

    code! { #(#impls)* }
}

/// Generate: Primitive → Constraint type (`TryFrom`)
fn generate_primitive_to_constraint_tryfrom(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        let alias = format_ident!("{}{}", type_name, float_type.to_string().to_uppercase());

        code! {
            impl TryFrom<#float_type> for #alias {
                type Error = FloatError;

                #[inline]
                fn try_from(value: #float_type) -> Result<Self, Self::Error> {
                    Self::new(value)
                }
            }
        }
    });

    code! { #(#impls)* }
}

/// Generate: Constraint type → Constraint type (From/TryFrom)
fn generate_constraint_to_constraint_traits(config: &TypeConfig) -> proc_macro2::TokenStream {
    let mut all_impls = vec![];

    // Generate conversions for each float type
    for float_type in &["f32", "f64"] {
        let float_ident = Ident::new(float_type, proc_macro2::Span::call_site());

        // Use helper function to filter constraint types that include this float type
        let types = filter_constraint_types_by_float(config, &float_ident);

        // Generate From or TryFrom for each pair of types
        for src_type in &types {
            for dst_type in &types {
                if src_type.type_name.eq(&dst_type.type_name) {
                    continue; // Skip same type
                }

                let src_alias = format_ident!(
                    "{}{}",
                    src_type.type_name,
                    float_type.to_string().to_uppercase()
                );
                let dst_alias = format_ident!(
                    "{}{}",
                    dst_type.type_name,
                    float_type.to_string().to_uppercase()
                );

                // Use helper function to find constraint definitions
                let src_constraint = find_constraint_def(config, &src_type.constraint_name);
                let dst_constraint = find_constraint_def(config, &dst_type.constraint_name);

                // Check if subset relationship
                let is_safe = is_subset_constraint(
                    src_constraint.bounds.lower,
                    src_constraint.bounds.upper,
                    src_constraint.excludes_zero,
                    dst_constraint.bounds.lower,
                    dst_constraint.bounds.upper,
                    dst_constraint.excludes_zero,
                );

                if is_safe {
                    // From implementation
                    all_impls.push(code! {
                        impl From<#src_alias> for #dst_alias {
                            #[inline]
                            fn from(value: #src_alias) -> Self {
                                unsafe { Self::new_unchecked(value.get()) }
                            }
                        }
                    });
                } else {
                    // TryFrom implementation
                    all_impls.push(code! {
                        impl TryFrom<#src_alias> for #dst_alias {
                            type Error = FloatError;

                            #[inline]
                            fn try_from(value: #src_alias) -> Result<Self, Self::Error> {
                                Self::new(value.get())
                            }
                        }
                    });
                }
            }
        }
    }

    code! { #(#all_impls)* }
}

/// Generate: F32 → F64 (From)
fn generate_f32_to_f64_from(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        if *float_type != "f32" {
            return code! {};
        }

        let f32_alias = format_ident!("{}F32", type_name);
        let f64_alias = format_ident!("{}F64", type_name);

        code! {
            impl From<#f32_alias> for #f64_alias {
                #[inline]
                fn from(value: #f32_alias) -> Self {
                    value.as_f64_type()
                }
            }
        }
    });

    code! { #(#impls)* }
}

/// Generate: F64 → F32 (`TryFrom`)
fn generate_f64_to_f32_tryfrom(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        if *float_type != "f64" {
            return code! {};
        }

        let f32_alias = format_ident!("{}F32", type_name);
        let f64_alias = format_ident!("{}F64", type_name);

        code! {
            impl TryFrom<#f64_alias> for #f32_alias {
                type Error = FloatError;

                #[inline]
                fn try_from(value: #f64_alias) -> Result<Self, Self::Error> {
                    value.try_into_f32_type()
                }
            }
        }
    });

    code! { #(#impls)* }
}

/// Generate: f32 → F64 constraint type (`TryFrom`)
fn generate_f32_to_f64_constraint_tryfrom(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        if *float_type != "f64" {
            return code! {};
        }

        let f64_alias = format_ident!("{}F64", type_name);

        code! {
            impl TryFrom<f32> for #f64_alias {
                type Error = FloatError;

                #[inline]
                fn try_from(value: f32) -> Result<Self, Self::Error> {
                    Self::new(value as f64)
                }
            }
        }
    });

    code! { #(#impls)* }
}

/// Generate: f64 → F32 constraint type (`TryFrom`)
fn generate_f64_to_f32_constraint_tryfrom(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        if *float_type != "f32" {
            return code! {};
        }

        let f32_alias = format_ident!("{}F32", type_name);

        code! {
            impl TryFrom<f64> for #f32_alias {
                type Error = FloatError;

                #[inline]
                fn try_from(value: f64) -> Result<Self, Self::Error> {
                    Self::new(value as f32)
                }
            }
        }
    });

    code! { #(#impls)* }
}
