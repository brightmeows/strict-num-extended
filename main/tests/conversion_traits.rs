//! From/TryFrom trait tests

use std::convert::TryFrom;

// 1. Constraint type → Primitive
#[test]
fn test_constraint_to_primitive() {
    let fin = strict_num_extended::FinF32::new_const(2.5);
    let f32_val: f32 = fin.into();
    assert_eq!(f32_val, 2.5);
}

// 2. Primitive → Constraint type (success)
#[test]
fn test_primitive_to_constraint_success() {
    let fin: strict_num_extended::FinF32 = strict_num_extended::FinF32::try_from(2.5).unwrap();
    assert_eq!(fin.get(), 2.5);
}

// 3. Primitive → Constraint type (failure)
#[test]
fn test_primitive_to_constraint_failure() {
    let result: Result<strict_num_extended::FinF32, _> =
        strict_num_extended::FinF32::try_from(f32::NAN);
    assert!(result.is_err());
}

// 4. Subset → Superset (From)
#[test]
fn test_subset_to_superset() {
    let normalized = strict_num_extended::NormalizedF32::new_const(0.5);
    let fin: strict_num_extended::FinF32 = normalized.into();
    assert_eq!(fin.get(), 0.5);
}

// 5. Superset → Subset (TryFrom success)
#[test]
fn test_superset_to_subset_success() {
    let fin = strict_num_extended::FinF32::new_const(0.5);
    let normalized: Result<strict_num_extended::NormalizedF32, _> = fin.try_into();
    assert!(normalized.is_ok());
}

// 6. Superset → Subset (TryFrom failure)
#[test]
fn test_superset_to_subset_failure() {
    let fin = strict_num_extended::FinF32::new_const(2.0);
    let normalized: Result<strict_num_extended::NormalizedF32, _> = fin.try_into();
    assert!(normalized.is_err());
}

// 7. F32 → F64 (From)
#[test]
fn test_f32_to_f64_from() {
    let fin32 = strict_num_extended::FinF32::new_const(3.0);
    let fin64: strict_num_extended::FinF64 = fin32.into();
    assert_eq!(fin64.get(), 3.0);
}

// 8. F64 → F32 (TryFrom)
#[test]
fn test_f64_to_f32_tryfrom() {
    let fin64 = strict_num_extended::FinF64::new_const(3.0);
    let fin32: Result<strict_num_extended::FinF32, _> = fin64.try_into();
    assert!(fin32.is_ok());
}
