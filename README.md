# strict-num-extended

[![Crates.io](https://img.shields.io/crates/v/strict-num-extended)](https://crates.io/crates/strict-num-extended)
[![Documentation](https://docs.rs/strict-num-extended/badge.svg)](https://docs.rs/strict-num-extended)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![CI](https://github.com/brightmeows/strict-num-extended/actions/workflows/rust.yml/badge.svg)](https://github.com/brightmeows/strict-num-extended/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/brightmeows/strict-num-extended/graph/badge.svg)](https://codecov.io/gh/brightmeows/strict-num-extended)
[![Cargo Deny](https://github.com/brightmeows/strict-num-extended/actions/workflows/cargo-deny.yml/badge.svg)](https://github.com/brightmeows/strict-num-extended/actions/workflows/cargo-deny.yml)
[![Downloads](https://img.shields.io/crates/d/strict-num-extended)](https://crates.io/crates/strict-num-extended)
[![Rust Edition: 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

Type-safe finite floating-point numbers for Rust.

A Rust library providing zero-cost finite floating-point types through the type system, guaranteeing safety by eliminating NaN and infinity values at compile time or runtime.

## Features

- **Type Safety** - Catch floating-point errors at compile time or runtime
- **Zero-Cost Abstractions** - No performance overhead compared to built-in types except the additional value checking
- **Comprehensive Constraints** - Finite, Positive, Negative, NonZero, Normalized, NegativeNormalized, Symmetric, ...
- **Full Trait Implementation** - PartialEq, Eq, PartialOrd, Ord, Display, Debug, arithmetic operators
- **Const Support** - Create compile-time constants with validation
- **Option/Result Arithmetic** - Optional and fallible operations with automatic error propagation
- **Type Conversions** - Safe F32↔F64 conversions with precision detection

## Available Types

| Type | Valid Range | Example |
|------|-------------|---------|
| `FinF32` / `FinF64` | All real numbers | `-∞ < x < ∞` |
| `NonNegativeF32` / `NonNegativeF64` | `x ≥ 0` | `0.0, 1.5, 100.0` |
| `NonPositiveF32` / `NonPositiveF64` | `x ≤ 0` | `0.0, -1.5, -100.0` |
| `NonZeroF32` / `NonZeroF64` | `x ≠ 0` | `1.0, -1.0, 0.001` |
| `PositiveF32` / `PositiveF64` | `x > 0` | `0.001, 1.0, 100.0` |
| `NegativeF32` / `NegativeF64` | `x < 0` | `-0.001, -1.0, -100.0` |
| `NormalizedF32` / `NormalizedF64` | `0.0 ≤ x ≤ 1.0` | `0.0, 0.5, 1.0` |
| `NegativeNormalizedF32` / `NegativeNormalizedF64` | `-1.0 ≤ x ≤ 0.0` | `-0.5, -0.75, -1.0` |
| `SymmetricF32` / `SymmetricF64` | `-1.0 ≤ x ≤ 1.0` | `-1.0, 0.0, 0.5, 1.0` |

## Error Handling

All fallible operations return `Result<T, FloatError>` with detailed error information:

**Error Types**:
- `NaN` - Value is Not a Number
- `PosInf` - Value is positive infinity
- `NegInf` - Value is negative infinity
- `OutOfRange` - Value is outside the valid range for the target type
- `DivisionByZero` - Division by zero occurred
- `NoneOperand` - Right-hand side operand is None in Option arithmetic

The `FloatError` enum provides comprehensive error information for proper error handling and debugging, allowing precise error matching and recovery strategies.

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
