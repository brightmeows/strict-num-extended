//! `FiniteFloat` trait generation module
//!
//! Generates the `FiniteFloat` trait and its implementations for all generated types.

use proc_macro2::TokenStream;

use crate::config::TypeConfig;
use crate::generator::{for_all_constraint_float_types, make_type_alias};

/// Generates `IntoF64` trait and its implementations for f32 and f64
fn generate_into_f64_trait() -> TokenStream {
    code! {
        /// Type marker trait: types that can be converted to f64
        pub trait IntoF64 {
            /// Converts the value to f64
            fn into_f64(self) -> f64;
        }

        impl IntoF64 for f32 {
            #[inline]
            fn into_f64(self) -> f64 {
                self as f64
            }
        }

        impl IntoF64 for f64 {
            #[inline]
            fn into_f64(self) -> f64 {
                self
            }
        }
    }
}

/// Generates the `FiniteFloat` trait definition
///
/// This trait provides a unified interface for all finite floating-point types,
/// allowing polymorphic usage through `Box<dyn FiniteFloat>`.
pub fn generate_finite_float_trait(_config: &TypeConfig) -> TokenStream {
    let into_f64_trait = generate_into_f64_trait();

    code! {
        #into_f64_trait

        /// Common trait for all finite floating-point types
        ///
        /// This trait provides a unified interface for creating and working with
        /// finite floating-point numbers, supporting polymorphic usage.
        ///
        /// # Example
        ///
        /// ```
        /// use strict_num_extended::{FiniteFloat, FinF32, FinF64, PositiveF32};
        ///
        /// // Create a heterogeneous collection
        /// let mut floats: Vec<Box<dyn FiniteFloat>> = Vec::new();
        /// floats.push(Box::new(FinF32::new(1.0f32).unwrap()));
        /// floats.push(Box::new(FinF64::new(2.0).unwrap()));
        /// floats.push(Box::new(PositiveF32::new(0.5f32).unwrap()));
        ///
        /// // All values can be converted to f64
        /// assert_eq!(floats[0].as_f64(), 1.0);
        /// ```
        pub trait FiniteFloat {
            /// Creates a new instance from a value that can be converted to f64
            ///
            /// This method accepts both f32 and f64 values through the `IntoF64` trait.
            ///
            /// # Type Parameters
            ///
            /// * `T` - A type implementing `IntoF64` (f32 or f64)
            ///
            /// # Examples
            ///
            /// ```
            /// use strict_num_extended::{FiniteFloat, FinF32, FinF64};
            ///
            /// // Create from f32
            /// let f32_val: FinF32 = FiniteFloat::new(3.14f32).unwrap();
            ///
            /// // Create from f64
            /// let f64_val: FinF64 = FiniteFloat::new(2.71).unwrap();
            /// ```
            fn new<T: IntoF64>(value: T) -> Result<Self, FloatError>
            where
                Self: Sized;

            /// Converts the value to f64
            ///
            /// # Examples
            ///
            /// ```
            /// use strict_num_extended::{FiniteFloat, FinF32};
            ///
            /// let val = FinF32::new(2.5f32).unwrap();
            /// assert_eq!(val.as_f64(), 2.5);
            /// ```
            fn as_f64(&self) -> f64;
        }
    }
}

/// Generates `FiniteFloat` trait implementations for all constraint types
///
/// This function iterates through all generated types (`FinF32`, `FinF64`, `PositiveF32`, etc.)
/// and generates the corresponding `FiniteFloat` trait implementations.
pub fn generate_finite_float_impls(config: &TypeConfig) -> TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        let struct_name = make_type_alias(type_name, float_type);

        code! {
            impl FiniteFloat for #struct_name {
                fn new<T: IntoF64>(value: T) -> Result<Self, FloatError> {
                    let f64_val = value.into_f64();
                    #struct_name::new(f64_val as #float_type)
                }

                fn as_f64(&self) -> f64 {
                    self.get() as f64
                }
            }
        }
    });

    code! {
        #(#impls)*
    }
}
