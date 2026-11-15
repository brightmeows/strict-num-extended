//! Tests for `PiBounded` constraint types

#![expect(clippy::approx_constant)]

use strict_num_extended::*;

#[test]
fn test_pibounded_f64_creation() {
    // Valid values
    assert!(PiBoundedF64::new(0.0).is_ok());
    assert!(PiBoundedF64::new(3.14).is_ok());
    assert!(PiBoundedF64::new(-3.14).is_ok());
    assert!(PiBoundedF64::new(core::f64::consts::PI).is_ok());
    assert!(PiBoundedF64::new(-core::f64::consts::PI).is_ok());

    // Invalid values
    assert!(PiBoundedF64::new(4.0).is_err());
    assert!(PiBoundedF64::new(-4.0).is_err());
    assert!(PiBoundedF64::new(f64::NAN).is_err());
    assert!(PiBoundedF64::new(f64::INFINITY).is_err());
}

#[test]
fn test_pibounded_f32_creation() {
    // Valid values
    assert!(PiBoundedF32::new(0.0).is_ok());
    assert!(PiBoundedF32::new(3.14).is_ok());
    assert!(PiBoundedF32::new(-3.14).is_ok());

    // Note: f32::consts::PI is slightly larger than PI in f64 precision due to precision limits
    // So we test values close to PI
    assert!(PiBoundedF32::new(3.14159).is_ok());

    // Invalid values
    assert!(PiBoundedF32::new(4.0).is_err());
    assert!(PiBoundedF32::new(-4.0).is_err());
}

#[test]
fn test_pibounded_const_context() {
    const ZERO: PiBoundedF64 = PiBoundedF64::new_const(0.0);
    const PI: PiBoundedF64 = PiBoundedF64::new_const(core::f64::consts::PI);
    const NEG_PI: PiBoundedF64 = PiBoundedF64::new_const(-core::f64::consts::PI);

    assert_eq!(ZERO.get(), 0.0);
    assert_eq!(PI.get(), core::f64::consts::PI);
    assert_eq!(NEG_PI.get(), -core::f64::consts::PI);
}
