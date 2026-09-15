//! Type definitions for configuration structures
//!
//! Contains all type definitions used in configuration parsing and arithmetic inference.

use proc_macro2::Ident;
use std::collections::HashMap;

// ============================================================================
// Sign and bound type definitions for arithmetic operations
// ============================================================================

/// Sign property of a constraint type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    /// Non-negative values (>= 0)
    Positive,
    /// Non-positive values (<= 0)
    Negative,
    /// Any sign (including zero)
    Any,
}

/// Bound information for a constraint type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    /// Lower bound (None means -∞)
    pub lower: Option<f64>,
    /// Upper bound (None means +∞)
    pub upper: Option<f64>,
}

impl Bounds {
    /// Epsilon for floating-point comparison
    const EPSILON: f64 = 1e-10;

    /// Check if this type is bounded (has both upper and lower bounds)
    pub const fn is_bounded(&self) -> bool {
        self.lower.is_some() && self.upper.is_some()
    }

    /// Check if this is the standard normalized range [0, 1]
    pub fn is_normalized(&self) -> bool {
        matches!(self, Bounds { lower: Some(l), upper: Some(u) }
            if (*l - 0.0).abs() < Self::EPSILON && (*u - 1.0).abs() < Self::EPSILON)
    }

    /// Check if this is the symmetric range [-1, 1]
    pub fn is_symmetric(&self) -> bool {
        matches!(self, Bounds { lower: Some(l), upper: Some(u) }
            if (*l - (-1.0)).abs() < Self::EPSILON && (*u - 1.0).abs() < Self::EPSILON)
    }

    /// Check if this is the negative normalized range [-1, 0]
    pub fn is_negative_normalized(&self) -> bool {
        matches!(self, Bounds { lower: Some(l), upper: Some(u) }
            if (*l - (-1.0)).abs() < Self::EPSILON && (*u - 0.0).abs() < Self::EPSILON)
    }
}

/// Arithmetic operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// Result of arithmetic operation type inference.
#[derive(Debug, Clone)]
pub struct ArithmeticResult {
    /// Output constraint type name
    pub output_type: Ident,
    /// Whether this operation is safe (no overflow/divide-by-zero possible)
    pub is_safe: bool,
}

// ============================================================================
// Configuration structure definitions
// ============================================================================

/// Main configuration structure.
pub struct TypeConfig {
    /// List of constraint definitions.
    pub constraints: Vec<ConstraintDef>,
    /// List of constraint type definitions.
    pub constraint_types: Vec<TypeDef>,
    /// Arithmetic operation results: (op, `lhs_name`, `rhs_name`) -> `ArithmeticResult`
    pub arithmetic_results: HashMap<(ArithmeticOp, String, String), ArithmeticResult>,
    /// Type aliases: [(`OriginalName`, `AliasName`), ...]
    pub type_aliases: Vec<TypeAliasDef>,
}

/// Single constraint definition.
pub struct ConstraintDef {
    /// Constraint name.
    pub name: Ident,
    /// Name of the constraint type after negation (e.g., Positive -> Negative).
    pub neg_constraint_name: Option<Ident>,
    /// Raw conditions before adding `is_finite()` check (used for negation calculation).
    pub raw_conditions: Vec<String>,
    /// Sign property of this constraint.
    pub sign: Sign,
    /// Bound information of this constraint.
    pub bounds: Bounds,
    /// Whether this constraint excludes zero.
    pub excludes_zero: bool,
}

/// Type definition (single constraint).
pub struct TypeDef {
    /// Type name.
    pub type_name: Ident,
    /// List of floating-point types.
    pub float_types: Vec<Ident>,
    /// Constraint name.
    pub constraint_name: Ident,
}

/// Gets standard arithmetic operator definition array
///
/// Contains four basic arithmetic operations (addition, subtraction, multiplication, division)
/// and their corresponding:
/// - Operator enum
/// - Trait name (e.g., "Add")
/// - Method name (e.g., "add")
/// - Operator symbol (e.g., code! { + })
///
/// # Returns
///
/// An array containing all standard arithmetic operation definitions
pub fn get_standard_arithmetic_ops() -> [(
    ArithmeticOp,
    &'static str,
    &'static str,
    proc_macro2::TokenStream,
); 4] {
    [
        (ArithmeticOp::Add, "Add", "add", code! { + }),
        (ArithmeticOp::Sub, "Sub", "sub", code! { - }),
        (ArithmeticOp::Mul, "Mul", "mul", code! { * }),
        (ArithmeticOp::Div, "Div", "div", code! { / }),
    ]
}

// ============================================================================
// Type alias definitions
// ============================================================================

/// Type alias definition
#[derive(Debug, Clone)]
pub struct TypeAliasDef {
    /// Original type name (e.g., Positive)
    pub original_name: Ident,
    /// Alias name (e.g., Pos)
    pub alias_name: Ident,
}
