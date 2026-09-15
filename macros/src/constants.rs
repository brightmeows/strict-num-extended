//! Constants generation module
//!
//! Generates mathematical constants for all constraint types.

use proc_macro2::{Ident, Span};

use crate::config::{ConstraintDef, TypeConfig};
use crate::generator::for_all_constraint_float_types;

/// Constant definition structure
struct ConstantDef {
    /// Constant name (e.g., "ZERO", "PI")
    name: &'static str,
    /// Documentation comment
    doc: &'static str,
    /// f32 value expression (None means use literal value)
    f32_expr: Option<&'static str>,
    /// f64 value expression (None means use literal value)
    f64_expr: Option<&'static str>,
    /// Literal value (for constraint checking)
    literal_value: f64,
}

/// All constant definitions
const ALL_CONSTANTS: &[ConstantDef] = &[
    // ========== Basic constants ==========
    ConstantDef {
        name: "ZERO",
        doc: "Zero (0.0)",
        f32_expr: None,
        f64_expr: None,
        literal_value: 0.0,
    },
    ConstantDef {
        name: "ONE",
        doc: "One (1.0)",
        f32_expr: None,
        f64_expr: None,
        literal_value: 1.0,
    },
    ConstantDef {
        name: "NEG_ONE",
        doc: "Negative one (-1.0)",
        f32_expr: None,
        f64_expr: None,
        literal_value: -1.0,
    },
    ConstantDef {
        name: "TWO",
        doc: "Two (2.0)",
        f32_expr: None,
        f64_expr: None,
        literal_value: 2.0,
    },
    ConstantDef {
        name: "NEG_TWO",
        doc: "Negative two (-2.0)",
        f32_expr: None,
        f64_expr: None,
        literal_value: -2.0,
    },
    ConstantDef {
        name: "HALF",
        doc: "Half (0.5)",
        f32_expr: None,
        f64_expr: None,
        literal_value: 0.5,
    },
    ConstantDef {
        name: "NEG_HALF",
        doc: "Negative half (-0.5)",
        f32_expr: None,
        f64_expr: None,
        literal_value: -0.5,
    },
    // ========== Mathematical constants ==========
    ConstantDef {
        name: "PI",
        doc: "Pi, the ratio of a circle's circumference to its diameter",
        f32_expr: Some("core::f32::consts::PI"),
        f64_expr: Some("core::f64::consts::PI"),
        literal_value: core::f64::consts::PI,
    },
    ConstantDef {
        name: "NEG_PI",
        doc: "Negative pi",
        f32_expr: None,
        f64_expr: None,
        literal_value: -core::f64::consts::PI,
    },
    ConstantDef {
        name: "E",
        doc: "Euler's number, the base of natural logarithms",
        f32_expr: Some("core::f32::consts::E"),
        f64_expr: Some("core::f64::consts::E"),
        literal_value: core::f64::consts::E,
    },
    ConstantDef {
        name: "NEG_E",
        doc: "Negative Euler's number",
        f32_expr: None,
        f64_expr: None,
        literal_value: -core::f64::consts::E,
    },
    // ========== Pi fraction constants ==========
    ConstantDef {
        name: "FRAC_1_PI",
        doc: "1/pi",
        f32_expr: Some("core::f32::consts::FRAC_1_PI"),
        f64_expr: Some("core::f64::consts::FRAC_1_PI"),
        literal_value: core::f64::consts::FRAC_1_PI,
    },
    ConstantDef {
        name: "FRAC_2_PI",
        doc: "2/pi",
        f32_expr: Some("core::f32::consts::FRAC_2_PI"),
        f64_expr: Some("core::f64::consts::FRAC_2_PI"),
        literal_value: core::f64::consts::FRAC_2_PI,
    },
    ConstantDef {
        name: "FRAC_PI_2",
        doc: "pi/2",
        f32_expr: Some("core::f32::consts::FRAC_PI_2"),
        f64_expr: Some("core::f64::consts::FRAC_PI_2"),
        literal_value: core::f64::consts::FRAC_PI_2,
    },
    ConstantDef {
        name: "FRAC_PI_3",
        doc: "pi/3",
        f32_expr: Some("core::f32::consts::FRAC_PI_3"),
        f64_expr: Some("core::f64::consts::FRAC_PI_3"),
        literal_value: core::f64::consts::FRAC_PI_3,
    },
    ConstantDef {
        name: "FRAC_PI_4",
        doc: "pi/4",
        f32_expr: Some("core::f32::consts::FRAC_PI_4"),
        f64_expr: Some("core::f64::consts::FRAC_PI_4"),
        literal_value: core::f64::consts::FRAC_PI_4,
    },
    ConstantDef {
        name: "FRAC_PI_6",
        doc: "pi/6",
        f32_expr: Some("core::f32::consts::FRAC_PI_6"),
        f64_expr: Some("core::f64::consts::FRAC_PI_6"),
        literal_value: core::f64::consts::FRAC_PI_6,
    },
    ConstantDef {
        name: "FRAC_PI_8",
        doc: "pi/8",
        f32_expr: Some("core::f32::consts::FRAC_PI_8"),
        f64_expr: Some("core::f64::consts::FRAC_PI_8"),
        literal_value: core::f64::consts::FRAC_PI_8,
    },
];

/// Checks if a constant value matches the type constraint
fn constant_matches_constraint(value: f64, constraint_def: &ConstraintDef) -> bool {
    // 1. Check if finite (all constants are finite, this check always passes)
    if !value.is_finite() {
        return false;
    }

    // 2. Check bounds
    if let Some(lower) = constraint_def.bounds.lower {
        if value < lower {
            return false;
        }
    }
    if let Some(upper) = constraint_def.bounds.upper {
        if value > upper {
            return false;
        }
    }

    // 3. Check zero exclusion
    if constraint_def.excludes_zero && value == 0.0 {
        return false;
    }

    true
}

/// Generates constants for all types
pub fn generate_constants(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, constraint_def| {
        let struct_name = crate::generator::make_type_alias(type_name, float_type);

        // Filter constants applicable to this type
        let applicable_constants: Vec<_> = ALL_CONSTANTS
            .iter()
            .filter(|c| constant_matches_constraint(c.literal_value, constraint_def))
            .collect();

        if applicable_constants.is_empty() {
            return code! {};
        }

        // Generate code for each constant
        let constant_defs = applicable_constants.iter().map(|const_def| {
            let name = Ident::new(const_def.name, Span::call_site());
            let doc = const_def.doc;

            // Get value expression
            #[expect(clippy::cast_possible_truncation)]
            let value_expr = if *float_type == "f32" {
                const_def.f32_expr.map_or_else(
                    || {
                        let v = const_def.literal_value as f32;
                        code! { #v }
                    },
                    |expr| {
                        #[expect(clippy::expect_used)]
                        // Parse expression path - known valid at compile time
                        expr.parse().expect("Invalid f32 expression")
                    },
                )
            } else {
                const_def.f64_expr.map_or_else(
                    || {
                        let v = const_def.literal_value;
                        code! { #v }
                    },
                    |expr| {
                        #[expect(clippy::expect_used)]
                        // Parse expression path - known valid at compile time
                        expr.parse().expect("Invalid f64 expression")
                    },
                )
            };

            code! {
                #[doc = #doc]
                #[must_use]
                pub const #name: Self = unsafe { Self::new_unchecked(#value_expr) };
            }
        });

        code! {
            #[expect(clippy::approx_constant)]
            impl #struct_name {
                #(#constant_defs)*
            }
        }
    });

    code! {
        #(#impls)*
    }
}
