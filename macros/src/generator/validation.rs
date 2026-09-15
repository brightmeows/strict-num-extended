//! Validation expression building
//!
//! Contains functions for building runtime validation expressions based on constraint definitions.

use proc_macro2::Ident;

use crate::config::ConstraintDef;

/// Dynamically builds validation expression based on constraint definition
pub fn build_validation_expr(
    constraint_def: &ConstraintDef,
    float_type: &Ident,
) -> proc_macro2::TokenStream {
    let mut checks = Vec::new();

    // 1. Base check: is_finite()
    checks.push(code! { value.is_finite() });

    // 2. Boundary checks
    if let Some(lower) = constraint_def.bounds.lower {
        let lower_check = build_bound_check(lower, true, constraint_def.excludes_zero, float_type);
        checks.push(lower_check);
    }

    if let Some(upper) = constraint_def.bounds.upper {
        let upper_check = build_bound_check(upper, false, constraint_def.excludes_zero, float_type);
        checks.push(upper_check);
    }

    // 3. Zero exclusion check (if not covered by bounds)
    if constraint_def.excludes_zero && needs_explicit_zero_check(constraint_def) {
        checks.push(code! { value != 0.0 });
    }

    // Combine all checks with &&
    code! {
        #(#checks)&&*
    }
}

/// Builds a single boundary check expression
fn build_bound_check(
    bound: f64,
    is_lower: bool,
    excludes_zero: bool,
    float_type: &Ident,
) -> proc_macro2::TokenStream {
    let is_f32 = *float_type == "f32";

    // For strict inequalities with excludes_zero and bound at zero, use strict comparison
    // Otherwise use non-strict comparison
    let use_strict = excludes_zero && bound == 0.0;

    // Determine the bound value to use (no substitution for strict inequalities)
    let bound_value = if is_f32 {
        code! { (#bound as f64) }
    } else {
        code! { #bound }
    };

    // f32 needs to convert value to f64 for comparison
    let value_expr = if is_f32 {
        code! { (value as f64) }
    } else {
        code! { value }
    };

    // Generate the appropriate comparison expression
    if is_lower {
        if use_strict {
            // For > x with excludes_zero and x == 0, use > to exclude 0.0 and -0.0
            code! { #value_expr > #bound_value }
        } else {
            code! { #value_expr >= #bound_value }
        }
    } else if use_strict {
        // For < x with excludes_zero and x == 0, use < to exclude 0.0 and -0.0
        code! { #value_expr < #bound_value }
    } else {
        code! { #value_expr <= #bound_value }
    }
}

/// Checks if an explicit zero check is needed
fn needs_explicit_zero_check(constraint_def: &ConstraintDef) -> bool {
    // If bounds already exclude zero through strict comparison (> or <), no need for explicit check
    let lower_excludes_zero =
        constraint_def.bounds.lower == Some(0.0) && constraint_def.excludes_zero;
    let upper_excludes_zero =
        constraint_def.bounds.upper == Some(0.0) && constraint_def.excludes_zero;

    // Need explicit check if bounds don't cover zero exclusion
    !lower_excludes_zero && !upper_excludes_zero
}
