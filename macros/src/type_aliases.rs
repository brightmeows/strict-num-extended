//! Type alias generation module
//!
//! Generates type aliases for constraint types.

use quote::format_ident;

use crate::config::TypeConfig;

/// Generates type aliases for all configured alias mappings
///
/// For each alias (`OriginalName`, `AliasName`), generates:
///
/// # Examples
///
/// If configured with `(Positive, Pos)`, generates:
/// ```ignore
/// type PosF32 = PositiveF32;
/// type PosF64 = PositiveF64;
/// ```
pub fn generate_type_aliases(config: &TypeConfig) -> proc_macro2::TokenStream {
    let aliases = config.type_aliases.iter().flat_map(|alias_def| {
        let original_name = &alias_def.original_name;
        let alias_name = &alias_def.alias_name;

        vec![
            generate_single_alias(original_name, alias_name, "f32"),
            generate_single_alias(original_name, alias_name, "f64"),
        ]
    });

    code! {
        // Type aliases
        #(#aliases)*
    }
}

/// Generates a single type alias for a specific float type
fn generate_single_alias(
    original_name: &proc_macro2::Ident,
    alias_name: &proc_macro2::Ident,
    float_type: &str,
) -> proc_macro2::TokenStream {
    let float_type_upper = float_type.to_uppercase();
    let original_struct_name = format_ident!("{}{}", original_name, float_type_upper);
    let alias_struct_name = format_ident!("{}{}", alias_name, float_type_upper);
    let doc_string = format!("Type alias for [`{original_struct_name}`]");

    code! {
        #[doc = #doc_string]
        pub type #alias_struct_name = #original_struct_name;
    }
}
