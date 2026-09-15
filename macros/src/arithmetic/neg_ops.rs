//! Unary negation operation module

use proc_macro2::TokenStream as TokenStream2;

use crate::config::TypeConfig;
use crate::generator::{find_constraint_def, make_type_alias};

/// Generates unary negation operation implementations.
pub fn generate_neg_impls(config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    for type_def in &config.constraint_types {
        let type_name = &type_def.type_name;

        // Use helper function to find constraint definition
        let constraint_def = find_constraint_def(config, &type_def.constraint_name);

        // Skip if no corresponding negation type
        let Some(neg_constraint_name) = &constraint_def.neg_constraint_name else {
            continue;
        };

        for float_type in &type_def.float_types {
            let type_alias = make_type_alias(type_name, float_type);
            let neg_type_alias = make_type_alias(neg_constraint_name, float_type);

            impls.push(code! {
                impl Neg for #type_alias {
                    type Output = #neg_type_alias;

                    fn neg(self) -> Self::Output {
                        let result = -self.get();
                        // SAFETY: The negation constraint was computed at compile time by
                        // negating the source constraint's conditions and finding a matching
                        // constraint. Since neg_constraint_name was found through condition
                        // matching, the result is mathematically guaranteed to satisfy the
                        // target constraint. The runtime validation would be redundant.
                        unsafe { #neg_type_alias::new_unchecked(result) }
                    }
                }
            });
        }
    }

    code! {
        #(#impls)*
    }
}
