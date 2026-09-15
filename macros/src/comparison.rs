//! Comparison and formatting trait implementations module

use crate::config::TypeConfig;
use crate::generator::{for_all_constraint_float_types, make_type_alias};

/// Generates comparison and formatting trait implementations for concrete types.
pub fn generate_comparison_traits() -> proc_macro2::TokenStream {
    // This is now a placeholder that returns empty code
    // Comparison traits are generated for each concrete type in arithmetic.rs and other modules
    code! {}
}

/// Generates comparison and formatting traits for all concrete types
pub fn generate_concrete_comparison_traits(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        let struct_name = make_type_alias(type_name, float_type);

        code! {
            impl PartialEq for #struct_name {
                fn eq(&self, other: &Self) -> bool {
                    self.value == other.value
                }
            }

            impl Eq for #struct_name {}

            impl Ord for #struct_name {
                fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                    self.value
                        .partial_cmp(&other.value)
                        .expect("values should always be comparable")
                }
            }

            impl PartialOrd for #struct_name {
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }

            impl core::fmt::Display for #struct_name {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{}", self.value)
                }
            }

            impl core::fmt::Debug for #struct_name {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "FiniteFloat({:?})", self.value)
                }
            }
        }
    });

    code! {
        #(#impls)*
    }
}
