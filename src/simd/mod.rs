#![allow(non_camel_case_types)]

pub mod bits;
mod traits;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub(crate) mod avx2;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub(crate) mod avx512;
#[cfg(target_arch = "aarch64")]
pub(crate) mod neon;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub(crate) mod sse2;
pub(crate) mod v128;

pub use self::traits::{BitMask, Mask, Simd};
