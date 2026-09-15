//! F32/F64 conversion method generator module

use quote::format_ident;

use crate::config::TypeConfig;
use crate::generator::{for_all_constraint_float_types, make_type_alias};

// ========== Primitive value accessors ==========

/// Generates `as_f32` methods for all XXXF32 types
pub fn generate_as_f32_primitive_methods(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _constraint_def| {
        let type_alias = make_type_alias(type_name, float_type);

        // Only F32 types need as_f32
        if *float_type != "f32" {
            return code! {};
        }

        code! {
            impl #type_alias {
                /// Returns the inner f32 value
                #[must_use]
                pub const fn as_f32(self) -> f32 {
                    self.value
                }
            }
        }
    });

    code! {
        #(#impls)*
    }
}

/// Generates `as_f64` methods for all XXXF64 types
pub fn generate_as_f64_primitive_methods(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _constraint_def| {
        let type_alias = make_type_alias(type_name, float_type);

        // Only F64 types need as_f64
        if *float_type != "f64" {
            return code! {};
        }

        code! {
            impl #type_alias {
                /// Returns the inner f64 value
                #[must_use]
                pub const fn as_f64(self) -> f64 {
                    self.value
                }
            }
        }
    });

    code! {
        #(#impls)*
    }
}

// ========== F32 type methods ==========

/// Generates `as_f32_type` methods for all XXXF32 types
pub fn generate_as_f32_type_methods(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _constraint_def| {
        let type_alias = make_type_alias(type_name, float_type);

        // Only F32 types need as_f32_type
        if *float_type != "f32" {
            return code! {};
        }

        code! {
            impl #type_alias {
                /// Creates a clone of the current F32 type instance
                ///
                /// This is equivalent to the Clone trait but provides a descriptive
                /// name for type conversion context.
                #[must_use]
                pub const fn as_f32_type(self) -> Self {
                    self
                }
            }
        }
    });

    code! {
        #(#impls)*
    }
}

/// Generates `as_f64_type` methods for all XXXF32 types
pub fn generate_as_f64_type_methods(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _constraint_def| {
        let type_alias = make_type_alias(type_name, float_type);

        // Only F32 types need as_f64_type
        if *float_type != "f32" {
            return code! {};
        }

        // Generate the corresponding F64 type name
        let f64_type_alias = make_type_alias(type_name, &format_ident!("f64"));

        // Generate conversion logic for F32 types
        code! {
            impl #type_alias {
                /// Converts to the corresponding F64 type
                ///
                /// Since F64 has a larger range than F32, this conversion
                /// is always safe in terms of range and precision.
                #[must_use]
                pub const fn as_f64_type(self) -> #f64_type_alias {
                    let value_f32 = self.value;
                    let value_f64 = value_f32 as f64;

                    // Use new_const to validate constraints
                    // Since F64 range is larger than F32, this should always succeed
                    #f64_type_alias::new_const(value_f64)
                }
            }
        }
    });

    code! {
        #(#impls)*
    }
}

// ========== F64 type methods ==========

/// Generates `try_into_f32_type` methods for all XXXF64 types
pub fn generate_try_into_f32_type_methods(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _constraint_def| {
        let type_alias = make_type_alias(type_name, float_type);

        // Only F64 types need try_into_f32_type
        if *float_type != "f64" {
            return code! {};
        }

        // Generate the corresponding F32 type name
        let f32_type_alias = make_type_alias(type_name, &format_ident!("f32"));

        // Generate conversion logic for F64 types
        code! {
            impl #type_alias {
                /// Attempts to convert to the corresponding F32 type
                ///
                /// # Errors
                ///
                /// Returns `Err(FloatError)` if the converted value does not satisfy
                /// the target constraint (e.g., NaN, infinity, or out of range).
                #[must_use = "Return value may contain an error and should not be ignored"]
                pub fn try_into_f32_type(self) -> Result<#f32_type_alias, FloatError> {
                    let value_f64 = self.value;
                    let value_f32 = value_f64 as f32;

                    // Use new() to validate all constraints (runtime check)
                    #f32_type_alias::new(value_f32)
                }
            }
        }
    });

    code! {
        #(#impls)*
    }
}
