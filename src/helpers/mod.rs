//! Crate-wide helpers shared across the `v1_6`, `v2_0_1` and `wip_v2_1` protocol
//! implementations.

/// A `no_std` (alloc-only) compatible replacement for
/// `rust_decimal::serde::arbitrary_precision` / `arbitrary_precision_option`.
///
/// Used automatically instead of the upstream helper when the `std` feature is
/// disabled - see [`decimal_arbitrary_precision`].
pub mod decimal_arbitrary_precision;
