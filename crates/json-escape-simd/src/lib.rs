//! Optimized SIMD routines for escaping JSON strings.
//!
//! Thin facade over [`escape_simd::json`]. The implementation is from
//! [sonic-rs](https://github.com/cloudwego/sonic-rs); we only take the string
//! escaping part to avoid the abstraction overhead.

#[doc(inline)]
pub use escape_simd::json::{escape, escape_into};
