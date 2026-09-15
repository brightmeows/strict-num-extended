//! Unary operations module
//!
//! Generates type-safe unary operations for all constraint types, including:
//! - `abs()`: Absolute value operation with automatic output type inference
//! - `signum()`: Sign function that always returns Symmetric type

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

use crate::config::{Bounds, ConstraintDef, Sign, TypeConfig};
use crate::generator::{find_constraint_def, make_type_alias};

/// Infers the output type for `abs()` operation based on constraint properties
fn infer_abs_output_type(constraint_def: &ConstraintDef, config: &TypeConfig) -> Ident {
    let bounds = &constraint_def.bounds;

    // Special bounded cases → Normalized [0, 1]
    if bounds.is_symmetric() || bounds.is_normalized() || bounds.is_negative_normalized() {
        let normalized_bounds = Bounds {
            lower: Some(0.0),
            upper: Some(1.0),
        };
        if let Some(ty) = config.find_type_by_constraints(Sign::Positive, &normalized_bounds, false)
        {
            return ty;
        }
    }

    // General case: absolute value is always non-negative
    let abs_bounds = Bounds {
        lower: Some(0.0),
        upper: None,
    };
    let excludes_zero = constraint_def.excludes_zero;

    if let Some(ty) = config.find_type_by_constraints(Sign::Positive, &abs_bounds, excludes_zero) {
        return ty;
    }

    // Fallback: construct type name based on properties
    if excludes_zero {
        Ident::new("Positive", Span::call_site())
    } else {
        Ident::new("NonNegative", Span::call_site())
    }
}

/// Infers the output type for `signum()` operation based on constraint properties
///
/// Rules:
/// - Positive types (>= 0) → signum in {0, 1} → Normalized
/// - Negative types (<= 0) → signum in {-1, 0} → `NegativeNormalized`
/// - Positive + excludes zero → signum = 1 → Normalized
/// - Negative + excludes zero → signum = -1 → `NegativeNormalized`
/// - Any + excludes zero (`NonZero`) → signum in {-1, 1} → Symmetric
/// - Any + includes zero (Fin, Symmetric) → signum in {-1, 0, 1} → Symmetric
fn infer_signum_output_type(constraint_def: &ConstraintDef, config: &TypeConfig) -> Ident {
    match (constraint_def.sign, constraint_def.excludes_zero) {
        // Positive types: signum ∈ {0, 1} or {1} → Normalized
        (Sign::Positive, _) => {
            let norm_bounds = Bounds {
                lower: Some(0.0),
                upper: Some(1.0),
            };
            config
                .find_type_by_constraints(Sign::Positive, &norm_bounds, false)
                .unwrap_or_else(|| Ident::new("Normalized", Span::call_site()))
        }

        // Negative types: signum ∈ {-1, 0} or {-1} → NegativeNormalized
        (Sign::Negative, _) => {
            let neg_norm_bounds = Bounds {
                lower: Some(-1.0),
                upper: Some(0.0),
            };
            config
                .find_type_by_constraints(Sign::Negative, &neg_norm_bounds, false)
                .unwrap_or_else(|| Ident::new("NegativeNormalized", Span::call_site()))
        }

        // Any sign types → Symmetric [-1, 1]
        (Sign::Any, _) => {
            let sym_bounds = Bounds {
                lower: Some(-1.0),
                upper: Some(1.0),
            };
            config
                .find_type_by_constraints(Sign::Any, &sym_bounds, false)
                .unwrap_or_else(|| Ident::new("Symmetric", Span::call_site()))
        }
    }
}

/// Generates `abs()` method implementations for all constraint types
///
/// For each type, generates code similar to:
/// ```text
/// impl TypeAlias {
///     #[inline]
///     #[must_use]
///     pub fn abs(self) -> OutputAlias {
///         let result = self.get().abs();
///         // SAFETY: ...
///         unsafe { OutputAlias::new_unchecked(result) }
///     }
/// }
/// ```
pub fn generate_abs_impls(config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    for type_def in &config.constraint_types {
        let type_name = &type_def.type_name;
        let constraint_def = find_constraint_def(config, type_name);

        // Infer abs() output type
        let output_type = infer_abs_output_type(constraint_def, config);

        for float_type in &type_def.float_types {
            let type_alias = make_type_alias(type_name, float_type);
            let output_alias = make_type_alias(&output_type, float_type);

            impls.push(code! {
                impl #type_alias {
                    /// Computes the absolute value.
                    ///
                    /// The return type is automatically inferred based on the source constraint:
                    /// - `NonNegative`/`NonPositive` → `NonNegative`
                    /// - `NonZero` → `Positive`
                    /// - `Normalized` → `Normalized` (reflexive)
                    /// - `Symmetric` → `Normalized`
                    /// - `Fin` → `NonNegative`
                    ///
                    /// # Examples
                    /// ```
                    /// use strict_num_extended::*;
                    ///
                    /// let neg = NegativeF64::new(-5.0).unwrap();
                    /// let abs_val: PositiveF64 = neg.abs();
                    /// assert_eq!(abs_val.get(), 5.0);
                    /// ```
                    #[inline]
                    #[must_use]
                    pub fn abs(self) -> #output_alias {
                        let result = self.get().abs();
                        // SAFETY: The output type for abs() was determined at compile time
                        // through constraint analysis. For any input value, its absolute value
                        // is guaranteed to satisfy the output type's constraints. Runtime
                        // validation would be redundant, so we use unchecked for performance.
                        unsafe { #output_alias::new_unchecked(result) }
                    }
                }
            });
        }
    }

    code! {
        #(#impls)*
    }
}

/// Generates `signum()` method implementations for all constraint types
///
/// For each type, generates code similar to:
/// ```text
/// impl TypeAlias {
///     #[inline]
///     #[must_use]
///     pub fn signum(self) -> OutputAlias {
///         let result = self.get().signum();
///         // SAFETY: ...
///         unsafe { OutputAlias::new_unchecked(result) }
///     }
/// }
/// ```
pub fn generate_signum_impls(config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    for type_def in &config.constraint_types {
        let type_name = &type_def.type_name;
        let constraint_def = find_constraint_def(config, type_name);

        // Infer signum() output type based on constraint properties
        let output_type = infer_signum_output_type(constraint_def, config);

        for float_type in &type_def.float_types {
            let type_alias = make_type_alias(type_name, float_type);
            let output_alias = make_type_alias(&output_type, float_type);

            impls.push(code! {
                impl #type_alias {
                    /// Computes the sign function.
                    ///
                    /// Returns the sign of the number:
                    /// - `1.0` if the number is positive
                    /// - `0.0` if the number is zero
                    /// - `-1.0` if the number is negative
                    ///
                    /// The return type is automatically inferred based on the source constraint:
                    /// - `Positive` types → `Normalized` (signum in {0, 1})
                    /// - `Negative` types → `NegativeNormalized` (signum in {-1, 0})
                    /// - `NonZero` types → `Symmetric` (signum in {-1, 1})
                    /// - `Fin`/`Symmetric` → `Symmetric` (signum in {-1, 0, 1})
                    ///
                    /// # Examples
                    /// ```
                    /// use strict_num_extended::*;
                    ///
                    /// let pos = PositiveF64::new(5.0).unwrap();
                    /// let sign: NormalizedF64 = pos.signum();
                    /// assert_eq!(sign.get(), 1.0);
                    ///
                    /// let neg = NegativeF64::new(-5.0).unwrap();
                    /// let sign: NegativeNormalizedF64 = neg.signum();
                    /// assert_eq!(sign.get(), -1.0);
                    /// ```
                    #[inline]
                    #[must_use]
                    pub fn signum(self) -> #output_alias {
                        let result = self.get().signum();
                        // SAFETY: The output type for signum() was determined at compile time
                        // through constraint analysis. For any input value, its signum is
                        // guaranteed to satisfy the output type's constraints. Runtime
                        // validation would be redundant, so we use unchecked for performance.
                        unsafe { #output_alias::new_unchecked(result) }
                    }
                }
            });
        }
    }

    code! {
        #(#impls)*
    }
}

/// Generates `sin()` method implementations for all constraint types
///
/// # Mathematical Properties
///
/// For any finite real input x:
/// - sin(x) ∈ [-1, 1]
/// - sin(x) is always finite
///
/// Therefore, all types map to Symmetric [-1, 1]
pub fn generate_sin_impls(config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    // sin() always returns Symmetric [-1, 1]
    let sym_bounds = Bounds {
        lower: Some(-1.0),
        upper: Some(1.0),
    };
    let output_type = config
        .find_type_by_constraints(Sign::Any, &sym_bounds, false)
        .unwrap_or_else(|| Ident::new("Symmetric", Span::call_site()));

    for type_def in &config.constraint_types {
        let type_name = &type_def.type_name;

        for float_type in &type_def.float_types {
            let type_alias = make_type_alias(type_name, float_type);
            let output_alias = make_type_alias(&output_type, float_type);

            impls.push(code! {
                #[cfg(feature = "std")]
                impl #type_alias {
                    /// Computes the sine of the value.
                    ///
                    /// # Mathematical Properties
                    ///
                    /// For any finite real input x:
                    /// - **Range**: sin(x) ∈ [-1, 1]
                    /// - **Output Type**: Always returns `Symmetric` ([-1, 1])
                    /// - **Defined**: sin(x) is defined for all finite inputs
                    ///
                    /// # Examples
                    ///
                    /// ```
                    /// use strict_num_extended::*;
                    ///
                    /// let angle: FinF64 = FinF64::new(std::f64::consts::PI / 2.0).unwrap();
                    /// let sin_val: SymmetricF64 = angle.sin();
                    /// assert_eq!(sin_val.get(), 1.0);
                    ///
                    /// let zero: NonNegativeF64 = NonNegativeF64::new_const(0.0);
                    /// let sin_zero: SymmetricF64 = zero.sin();
                    /// assert_eq!(sin_zero.get(), 0.0);
                    /// ```
                    #[inline]
                    #[must_use]
                    pub fn sin(self) -> #output_alias {
                        let result = self.get().sin();
                        // SAFETY: sin(x) for any finite x always produces a value in [-1, 1],
                        // which satisfies the Symmetric constraint. The standard library
                        // guarantees that sin() returns finite values for finite inputs.
                        unsafe { #output_alias::new_unchecked(result) }
                    }
                }
            });
        }
    }

    code! {
        #(#impls)*
    }
}

/// Generates `cos()` method implementations for all constraint types
///
/// # Mathematical Properties
///
/// For any finite real input x:
/// - cos(x) ∈ [-1, 1]
/// - cos(x) is always finite
///
/// Therefore, all types map to Symmetric [-1, 1]
pub fn generate_cos_impls(config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    // cos() always returns Symmetric [-1, 1]
    let sym_bounds = Bounds {
        lower: Some(-1.0),
        upper: Some(1.0),
    };
    let output_type = config
        .find_type_by_constraints(Sign::Any, &sym_bounds, false)
        .unwrap_or_else(|| Ident::new("Symmetric", Span::call_site()));

    for type_def in &config.constraint_types {
        let type_name = &type_def.type_name;

        for float_type in &type_def.float_types {
            let type_alias = make_type_alias(type_name, float_type);
            let output_alias = make_type_alias(&output_type, float_type);

            impls.push(code! {
                #[cfg(feature = "std")]
                impl #type_alias {
                    /// Computes the cosine of the value.
                    ///
                    /// # Mathematical Properties
                    ///
                    /// For any finite real input x:
                    /// - **Range**: cos(x) ∈ [-1, 1]
                    /// - **Output Type**: Always returns `Symmetric` ([-1, 1])
                    /// - **Defined**: cos(x) is defined for all finite inputs
                    ///
                    /// # Examples
                    ///
                    /// ```
                    /// use strict_num_extended::*;
                    ///
                    /// let angle: FinF64 = FinF64::new(0.0).unwrap();
                    /// let cos_val: SymmetricF64 = angle.cos();
                    /// assert_eq!(cos_val.get(), 1.0);
                    ///
                    /// let pi: FinF64 = FinF64::new(std::f64::consts::PI).unwrap();
                    /// let cos_pi: SymmetricF64 = pi.cos();
                    /// assert!((cos_pi.get() - (-1.0)).abs() < f64::EPSILON);
                    /// ```
                    #[inline]
                    #[must_use]
                    pub fn cos(self) -> #output_alias {
                        let result = self.get().cos();
                        // SAFETY: cos(x) for any finite x always produces a value in [-1, 1],
                        // which satisfies the Symmetric constraint. The standard library
                        // guarantees that cos() returns finite values for finite inputs.
                        unsafe { #output_alias::new_unchecked(result) }
                    }
                }
            });
        }
    }

    code! {
        #(#impls)*
    }
}

/// Generates `tan()` method implementations for all constraint types
///
/// # Mathematical Properties
///
/// For any finite real input x (excluding π/2 + kπ):
/// - tan(x) ∈ (-∞, +∞)
/// - tan(x) is finite except at singular points
///
/// Therefore, all types map to Fin (unbounded), returning Result to handle errors
pub fn generate_tan_impls(config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    for type_def in &config.constraint_types {
        let type_name = &type_def.type_name;

        // tan() always returns Fin (may be infinite at singular points)
        let output_type = Ident::new("Fin", Span::call_site());

        for float_type in &type_def.float_types {
            let type_alias = make_type_alias(type_name, float_type);
            let output_alias = make_type_alias(&output_type, float_type);

            impls.push(code! {
                #[cfg(feature = "std")]
                impl #type_alias {
                    /// Computes the tangent of the value.
                    ///
                    /// # Mathematical Properties
                    ///
                    /// For any finite real input x (excluding π/2 + kπ):
                    /// - **Range**: tan(x) ∈ (-∞, +∞)
                    /// - **Output Type**: Always returns `Fin` (unbounded)
                    /// - **Singular Points**: Undefined at π/2 + kπ (may return infinity)
                    ///
                    /// # Examples
                    ///
                    /// ```
                    /// use strict_num_extended::*;
                    ///
                    /// let angle: FinF64 = FinF64::new(0.0).unwrap();
                    /// let tan_val: Result<FinF64, FloatError> = angle.tan();
                    /// assert_eq!(tan_val.unwrap().get(), 0.0);
                    ///
                    /// let pi_over_4: FinF64 = FinF64::new(std::f64::consts::PI / 4.0).unwrap();
                    /// let tan_45: Result<FinF64, FloatError> = pi_over_4.tan();
                    /// assert!((tan_45.unwrap().get() - 1.0).abs() < f64::EPSILON);
                    /// ```
                    #[inline]
                    #[must_use]
                    pub fn tan(self) -> Result<#output_alias, FloatError> {
                        let result = self.get().tan();
                        // tan() may produce ±∞ at singular points (π/2 + kπ),
                        // which should return NaN error (not PosInf/NegInf)
                        if !result.is_finite() {
                            return Err(FloatError::NaN);
                        }
                        // SAFETY: When tan() produces a finite value, it satisfies the Fin constraint.
                        // The is_finite() check above ensures we only reach this point for valid values.
                        unsafe { Ok(#output_alias::new_unchecked(result)) }
                    }
                }
            });
        }
    }

    code! {
        #(#impls)*
    }
}
