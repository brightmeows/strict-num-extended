//! `FiniteFloat` struct and basic methods module

use crate::config::TypeConfig;
use crate::doc_generator;
use crate::generator::{build_validation_expr, for_all_constraint_float_types, make_type_alias};

/// Generates concrete struct definitions for each constraint × float type combination
pub fn generate_concrete_structs(config: &TypeConfig) -> proc_macro2::TokenStream {
    let structs =
        for_all_constraint_float_types(config, |type_name, float_type, constraint_def| {
            let struct_name = make_type_alias(type_name, float_type); // e.g., FinF32
            let constraint_name = type_name; // e.g., Fin
            let struct_doc =
                doc_generator::generate_struct_doc(type_name, float_type, constraint_def);

            code! {
                #[doc = #struct_doc]
                #[repr(transparent)]
                #[derive(Clone, Copy)]
                #[expect(clippy::approx_constant)]
                pub struct #struct_name {
                    value: #float_type,
                    _constraint: core::marker::PhantomData<#constraint_name>,
                }
            }
        });

    code! {
        // Concrete struct definitions
        #(#structs)*
    }
}

/// Generates implementations for each concrete struct
pub fn generate_concrete_impls(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, constraint_def| {
        let struct_name = make_type_alias(type_name, float_type);
        let validate_expr = build_validation_expr(constraint_def, float_type);
        let new_method_doc =
            doc_generator::generate_new_method_doc(&struct_name, float_type, constraint_def);

        code! {
            impl #struct_name {
                #[doc = #new_method_doc]
                #[must_use]
                pub fn new(value: #float_type) -> Result<Self, FloatError> {
                    let val_f64: f64 = value.into();

                    // Check for NaN
                    if val_f64.is_nan() {
                        return Err(FloatError::NaN);
                    }

                    // Check for positive infinity
                    if val_f64.is_infinite() && val_f64 > 0.0 {
                        return Err(FloatError::PosInf);
                    }

                    // Check for negative infinity
                    if val_f64.is_infinite() && val_f64 < 0.0 {
                        return Err(FloatError::NegInf);
                    }

                    // Validate bounds and zero exclusion
                    if #validate_expr {
                        Ok(Self {
                            value,
                            _constraint: core::marker::PhantomData,
                        })
                    } else {
                        Err(FloatError::OutOfRange)
                    }
                }

                /// Unsafely creates a finite floating-point number (no validation)
                ///
                /// # Safety
                ///
                /// Caller must ensure the value satisfies the constraint.
                /// Violating the constraint leads to undefined behavior.
                #[inline]
                pub const unsafe fn new_unchecked(value: #float_type) -> Self {
                    Self {
                        value,
                        _constraint: core::marker::PhantomData,
                    }
                }

                /// Gets the inner value
                ///
                /// # Example
                ///
                /// ```
                /// use strict_num_extended::FinF32;
                ///
                /// let finite = FinF32::new(2.5);
                /// assert_eq!(finite.unwrap().get(), 2.5);
                /// ```
                #[must_use]
                pub const fn get(&self) -> #float_type {
                    self.value
                }

                /// Gets the inner value (alias for `get()`)
                ///
                /// # Example
                ///
                /// ```
                /// use strict_num_extended::FinF32;
                ///
                /// let finite = FinF32::new(2.5);
                /// assert_eq!(finite.unwrap().value(), 2.5);
                /// ```
                #[must_use]
                pub const fn value(&self) -> #float_type {
                    self.value
                }

                /// Creates a value at compile time
                ///
                /// # Panics
                ///
                /// Will [`panic`] at compile time or runtime if the value does not satisfy the constraint.
                #[inline]
                #[must_use]
                pub const fn new_const(value: #float_type) -> Self {
                    if #validate_expr {
                        unsafe { Self::new_unchecked(value) }
                    } else {
                        panic!("Value does not satisfy the constraint");
                    }
                }
            }
        }
    });

    code! {
        #(#impls)*
    }
}

/// Generates serde support for concrete types
pub fn generate_concrete_serde_impls(config: &TypeConfig) -> proc_macro2::TokenStream {
    let serialize_impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        let struct_name = make_type_alias(type_name, float_type);

        code! {
            #[cfg(feature = "serde")]
            impl serde::Serialize for #struct_name
            where
                #float_type: serde::Serialize,
            {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    self.value.serialize(serializer)
                }
            }
        }
    });

    let deserialize_impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        let struct_name = make_type_alias(type_name, float_type);

        code! {
            #[cfg(feature = "serde")]
            impl<'de> serde::Deserialize<'de> for #struct_name
            where
                #float_type: serde::Deserialize<'de>,
            {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    // First deserialize the raw value
                    let value = #float_type::deserialize(deserializer)?;

                    // Then validate using the new() method
                    Self::new(value).map_err(|e| {
                        use serde::de::Error;
                        match e {
                            FloatError::NaN => D::Error::custom("value is NaN"),
                            FloatError::PosInf => D::Error::custom("value is positive infinity"),
                            FloatError::NegInf => D::Error::custom("value is negative infinity"),
                            FloatError::OutOfRange => D::Error::custom("value is out of range"),
                            FloatError::NoneOperand => D::Error::custom("none operand"),
                        }
                    })
                }
            }
        }
    });

    code! {
        #(#serialize_impls)*
        #(#deserialize_impls)*
    }
}
