#![feature(prelude_import)]
//! Borrowed from <https://github.com/cloudwego/sonic-rs/blob/v0.5.5/src/util/string.rs>
//!
//! Only takes the string escaping part to avoid the abstraction overhead.
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::slice::from_raw_parts;
use simd::{BitMask, Mask, Simd};
mod simd {
    #![allow(non_camel_case_types)]
    pub mod bits {
        use super::traits::BitMask;
        impl BitMask for u16 {
            const LEN: usize = std::mem::size_of::<u16>() * 8;
            #[inline]
            fn before(&self, rhs: &Self) -> bool {
                (self.as_little_endian() & rhs.as_little_endian().wrapping_sub(1)) != 0
            }
            #[inline]
            fn first_offset(&self) -> usize {
                self.as_little_endian().trailing_zeros() as usize
            }
            #[inline]
            fn as_little_endian(&self) -> Self {
                { self.clone() }
            }
            #[inline]
            fn all_zero(&self) -> bool {
                *self == 0
            }
            #[inline]
            fn clear_high_bits(&self, n: usize) -> Self {
                if true {
                    if !(n <= Self::LEN) {
                        ::core::panicking::panic("assertion failed: n <= Self::LEN")
                    }
                }
                *self & ((u64::MAX as u16) >> n)
            }
        }
        impl BitMask for u32 {
            const LEN: usize = std::mem::size_of::<u32>() * 8;
            #[inline]
            fn before(&self, rhs: &Self) -> bool {
                (self.as_little_endian() & rhs.as_little_endian().wrapping_sub(1)) != 0
            }
            #[inline]
            fn first_offset(&self) -> usize {
                self.as_little_endian().trailing_zeros() as usize
            }
            #[inline]
            fn as_little_endian(&self) -> Self {
                { self.clone() }
            }
            #[inline]
            fn all_zero(&self) -> bool {
                *self == 0
            }
            #[inline]
            fn clear_high_bits(&self, n: usize) -> Self {
                if true {
                    if !(n <= Self::LEN) {
                        ::core::panicking::panic("assertion failed: n <= Self::LEN")
                    }
                }
                *self & ((u64::MAX as u32) >> n)
            }
        }
        impl BitMask for u64 {
            const LEN: usize = std::mem::size_of::<u64>() * 8;
            #[inline]
            fn before(&self, rhs: &Self) -> bool {
                (self.as_little_endian() & rhs.as_little_endian().wrapping_sub(1)) != 0
            }
            #[inline]
            fn first_offset(&self) -> usize {
                self.as_little_endian().trailing_zeros() as usize
            }
            #[inline]
            fn as_little_endian(&self) -> Self {
                { self.clone() }
            }
            #[inline]
            fn all_zero(&self) -> bool {
                *self == 0
            }
            #[inline]
            fn clear_high_bits(&self, n: usize) -> Self {
                if true {
                    if !(n <= Self::LEN) {
                        ::core::panicking::panic("assertion failed: n <= Self::LEN")
                    }
                }
                *self & ((u64::MAX as u64) >> n)
            }
        }
        /// Use u64 representation the bitmask of Neon vector.
        ///         (low)
        /// Vector: 00-ff-ff-ff-ff-00-00-00
        /// Mask  : 0000-1111-1111-1111-1111-0000-0000-0000
        ///
        /// first_offset() = 1
        /// clear_high_bits(4) = Mask(0000-1111-1111-1111-[0000]-0000-0000-0000)
        ///
        /// reference: https://community.arm.com/arm-community-blogs/b/infrastructure-solutions-blog/posts/porting-x86-vector-bitmask-optimizations-to-arm-neon
        pub struct NeonBits(u64);
        impl NeonBits {
            #[inline]
            pub fn new(u: u64) -> Self {
                Self(u)
            }
        }
        impl BitMask for NeonBits {
            const LEN: usize = 16;
            #[inline]
            fn first_offset(&self) -> usize {
                (self.as_little_endian().0.trailing_zeros() as usize) >> 2
            }
            #[inline]
            fn before(&self, rhs: &Self) -> bool {
                (self.as_little_endian().0 & rhs.as_little_endian().0.wrapping_sub(1))
                    != 0
            }
            #[inline]
            fn as_little_endian(&self) -> Self {
                { Self::new(self.0) }
            }
            #[inline]
            fn all_zero(&self) -> bool {
                self.0 == 0
            }
            #[inline]
            fn clear_high_bits(&self, n: usize) -> Self {
                if true {
                    if !(n <= Self::LEN) {
                        ::core::panicking::panic("assertion failed: n <= Self::LEN")
                    }
                }
                Self(self.0 & u64::MAX >> (n * 4))
            }
        }
    }
    mod traits {
        use std::ops::{BitAnd, BitOr, BitOrAssign};
        /// Portbal SIMD traits
        pub trait Simd: Sized {
            const LANES: usize;
            type Element;
            type Mask: Mask;
            /// # Safety
            unsafe fn from_slice_unaligned_unchecked(slice: &[u8]) -> Self {
                if true {
                    if !(slice.len() >= Self::LANES) {
                        ::core::panicking::panic(
                            "assertion failed: slice.len() >= Self::LANES",
                        )
                    }
                }
                unsafe { Self::loadu(slice.as_ptr()) }
            }
            /// # Safety
            unsafe fn write_to_slice_unaligned_unchecked(&self, slice: &mut [u8]) {
                if true {
                    if !(slice.len() >= Self::LANES) {
                        ::core::panicking::panic(
                            "assertion failed: slice.len() >= Self::LANES",
                        )
                    }
                }
                unsafe { self.storeu(slice.as_mut_ptr()) }
            }
            /// # Safety
            unsafe fn loadu(ptr: *const u8) -> Self;
            /// # Safety
            unsafe fn storeu(&self, ptr: *mut u8);
            fn eq(&self, rhs: &Self) -> Self::Mask;
            fn splat(elem: Self::Element) -> Self;
            #[allow(unused)]
            /// greater than
            fn gt(&self, rhs: &Self) -> Self::Mask;
            /// less or equal
            fn le(&self, rhs: &Self) -> Self::Mask;
        }
        /// Portbal SIMD mask traits
        pub trait Mask: Sized + BitOr<Self> + BitOrAssign + BitAnd<Self> {
            type Element;
            type BitMask: BitMask;
            fn bitmask(self) -> Self::BitMask;
            fn splat(b: bool) -> Self;
        }
        /// Trait for the bitmask of a vector Mask.
        pub trait BitMask {
            /// Total bits in the bitmask.
            const LEN: usize;
            /// get the offset of the first `1` bit.
            fn first_offset(&self) -> usize;
            /// check if this bitmask is before the other bitmask.
            fn before(&self, rhs: &Self) -> bool;
            /// convert bitmask as little endian
            fn as_little_endian(&self) -> Self;
            /// whether all bits are zero.
            fn all_zero(&self) -> bool;
            /// clear high n bits.
            fn clear_high_bits(&self, n: usize) -> Self;
        }
    }
    pub(crate) mod neon {
        use std::arch::aarch64::*;
        use super::{Mask, Simd, bits::NeonBits};
        #[repr(transparent)]
        pub struct Simd128u(uint8x16_t);
        #[automatically_derived]
        impl ::core::fmt::Debug for Simd128u {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Simd128u",
                    &&self.0,
                )
            }
        }
        #[repr(transparent)]
        pub struct Simd128i(int8x16_t);
        #[automatically_derived]
        impl ::core::fmt::Debug for Simd128i {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Simd128i",
                    &&self.0,
                )
            }
        }
        impl Simd for Simd128u {
            const LANES: usize = 16;
            type Mask = Mask128;
            type Element = u8;
            #[inline(always)]
            unsafe fn loadu(ptr: *const u8) -> Self {
                unsafe { Self(vld1q_u8(ptr)) }
            }
            #[inline(always)]
            unsafe fn storeu(&self, ptr: *mut u8) {
                unsafe { vst1q_u8(ptr, self.0) };
            }
            #[inline(always)]
            fn eq(&self, lhs: &Self) -> Self::Mask {
                unsafe { Mask128(vceqq_u8(self.0, lhs.0)) }
            }
            #[inline(always)]
            fn splat(ch: u8) -> Self {
                unsafe { Self(vdupq_n_u8(ch)) }
            }
            #[inline(always)]
            fn le(&self, lhs: &Self) -> Self::Mask {
                unsafe { Mask128(vcleq_u8(self.0, lhs.0)) }
            }
            #[inline(always)]
            fn gt(&self, lhs: &Self) -> Self::Mask {
                unsafe { Mask128(vcgtq_u8(self.0, lhs.0)) }
            }
        }
        impl Simd for Simd128i {
            const LANES: usize = 16;
            type Mask = Mask128;
            type Element = i8;
            #[inline(always)]
            unsafe fn loadu(ptr: *const u8) -> Self {
                Self(unsafe { vld1q_s8(ptr as *const i8) })
            }
            #[inline(always)]
            unsafe fn storeu(&self, ptr: *mut u8) {
                unsafe { vst1q_s8(ptr as *mut i8, self.0) };
            }
            #[inline(always)]
            fn eq(&self, lhs: &Self) -> Self::Mask {
                unsafe { Mask128(vceqq_s8(self.0, lhs.0)) }
            }
            #[inline(always)]
            fn splat(elem: i8) -> Self {
                unsafe { Self(vdupq_n_s8(elem)) }
            }
            #[inline(always)]
            fn le(&self, lhs: &Self) -> Self::Mask {
                unsafe { Mask128(vcleq_s8(self.0, lhs.0)) }
            }
            #[inline(always)]
            fn gt(&self, lhs: &Self) -> Self::Mask {
                unsafe { Mask128(vcgtq_s8(self.0, lhs.0)) }
            }
        }
        pub(crate) const BIT_MASK_TAB: [u8; 16] = [
            0x01u8,
            0x02,
            0x4,
            0x8,
            0x10,
            0x20,
            0x40,
            0x80,
            0x01,
            0x02,
            0x4,
            0x8,
            0x10,
            0x20,
            0x40,
            0x80,
        ];
        #[repr(transparent)]
        pub struct Mask128(pub(crate) uint8x16_t);
        #[automatically_derived]
        impl ::core::fmt::Debug for Mask128 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Mask128", &&self.0)
            }
        }
        impl Mask for Mask128 {
            type BitMask = NeonBits;
            type Element = u8;
            /// Convert Mask Vector 0x00-ff-ff to Bits 0b0000-1111-1111
            /// Reference: https://community.arm.com/arm-community-blogs/b/infrastructure-solutions-blog/posts/porting-x86-vector-bitmask-optimizations-to-arm-neon
            #[inline(always)]
            fn bitmask(self) -> Self::BitMask {
                unsafe {
                    let v16 = vreinterpretq_u16_u8(self.0);
                    let sr4 = vshrn_n_u16(v16, 4);
                    let v64 = vreinterpret_u64_u8(sr4);
                    NeonBits::new(vget_lane_u64(v64, 0))
                }
            }
            #[inline(always)]
            fn splat(b: bool) -> Self {
                let v: i8 = if b { -1 } else { 0 };
                unsafe { Self(vdupq_n_u8(v as u8)) }
            }
        }
        impl std::ops::BitAnd<Mask128> for Mask128 {
            type Output = Self;
            #[inline(always)]
            fn bitand(self, rhs: Mask128) -> Self::Output {
                unsafe { Self(vandq_u8(self.0, rhs.0)) }
            }
        }
        impl std::ops::BitOr<Mask128> for Mask128 {
            type Output = Self;
            #[inline(always)]
            fn bitor(self, rhs: Mask128) -> Self::Output {
                unsafe { Self(vorrq_u8(self.0, rhs.0)) }
            }
        }
        impl std::ops::BitOrAssign<Mask128> for Mask128 {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Mask128) {
                unsafe {
                    self.0 = vorrq_u8(self.0, rhs.0);
                }
            }
        }
        #[inline(always)]
        pub unsafe fn to_bitmask64(
            v0: uint8x16_t,
            v1: uint8x16_t,
            v2: uint8x16_t,
            v3: uint8x16_t,
        ) -> u64 {
            let bit_mask = unsafe {
                std::mem::transmute::<[u8; 16], uint8x16_t>(BIT_MASK_TAB)
            };
            let t0 = unsafe { vandq_u8(v0, bit_mask) };
            let t1 = unsafe { vandq_u8(v1, bit_mask) };
            let t2 = unsafe { vandq_u8(v2, bit_mask) };
            let t3 = unsafe { vandq_u8(v3, bit_mask) };
            let pair0 = unsafe { vpaddq_u8(t0, t1) };
            let pair1 = unsafe { vpaddq_u8(t2, t3) };
            let quad = unsafe { vpaddq_u8(pair0, pair1) };
            let octa = unsafe { vpaddq_u8(quad, quad) };
            unsafe { vgetq_lane_u64(vreinterpretq_u64_u8(octa), 0) }
        }
        #[inline(always)]
        pub(crate) unsafe fn to_bitmask32(v0: uint8x16_t, v1: uint8x16_t) -> u32 {
            let bit_mask = unsafe {
                std::mem::transmute::<[u8; 16], uint8x16_t>(BIT_MASK_TAB)
            };
            let t0 = vandq_u8(v0, bit_mask);
            let t1 = vandq_u8(v1, bit_mask);
            let pair = vpaddq_u8(t0, t1);
            let quad = vpaddq_u8(pair, pair);
            let octa = vpaddq_u8(quad, quad);
            vgetq_lane_u32(vreinterpretq_u32_u8(octa), 0)
        }
    }
    pub(crate) mod v128 {
        use std::ops::{BitAnd, BitOr, BitOrAssign};
        use super::{Mask, Simd};
        pub struct Simd128i([i8; 16]);
        #[automatically_derived]
        impl ::core::fmt::Debug for Simd128i {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Simd128i",
                    &&self.0,
                )
            }
        }
        pub struct Simd128u([u8; 16]);
        #[automatically_derived]
        impl ::core::fmt::Debug for Simd128u {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Simd128u",
                    &&self.0,
                )
            }
        }
        pub struct Mask128(pub(crate) [u8; 16]);
        #[automatically_derived]
        impl ::core::fmt::Debug for Mask128 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Mask128", &&self.0)
            }
        }
        impl Simd for Simd128i {
            type Element = i8;
            const LANES: usize = 16;
            type Mask = Mask128;
            unsafe fn loadu(ptr: *const u8) -> Self {
                let v = unsafe { std::slice::from_raw_parts(ptr, Self::LANES) };
                let mut res = [0i8; 16];
                res.copy_from_slice(unsafe { std::mem::transmute::<&[u8], &[i8]>(v) });
                Self(res)
            }
            unsafe fn storeu(&self, ptr: *mut u8) {
                let data = unsafe { std::mem::transmute::<&[i8], &[u8]>(&self.0) };
                unsafe {
                    std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, Self::LANES)
                };
            }
            fn eq(&self, rhs: &Self) -> Self::Mask {
                let mut mask = [0u8; 16];
                for i in 0..Self::LANES {
                    mask[i] = if self.0[i] == rhs.0[i] { 1 } else { 0 };
                }
                Mask128(mask)
            }
            fn splat(value: i8) -> Self {
                Self([value as i8; Self::LANES])
            }
            fn le(&self, rhs: &Self) -> Self::Mask {
                let mut mask = [0u8; 16];
                for i in 0..Self::LANES {
                    mask[i] = if self.0[i] <= rhs.0[i] { 1 } else { 0 };
                }
                Mask128(mask)
            }
            fn gt(&self, rhs: &Self) -> Self::Mask {
                let mut mask = [0u8; 16];
                for i in 0..Self::LANES {
                    mask[i] = if self.0[i] > rhs.0[i] { 1 } else { 0 };
                }
                Mask128(mask)
            }
        }
        impl Simd for Simd128u {
            type Element = u8;
            const LANES: usize = 16;
            type Mask = Mask128;
            unsafe fn loadu(ptr: *const u8) -> Self {
                let v = unsafe { std::slice::from_raw_parts(ptr, Self::LANES) };
                let mut res = [0u8; 16];
                res.copy_from_slice(v);
                Self(res)
            }
            unsafe fn storeu(&self, ptr: *mut u8) {
                let data = &self.0;
                unsafe {
                    std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, Self::LANES)
                };
            }
            fn eq(&self, rhs: &Self) -> Self::Mask {
                let mut mask = [0u8; 16];
                for i in 0..Self::LANES {
                    mask[i] = if self.0[i] == rhs.0[i] { 1 } else { 0 };
                }
                Mask128(mask)
            }
            fn splat(value: u8) -> Self {
                Self([value; Self::LANES])
            }
            fn le(&self, rhs: &Self) -> Self::Mask {
                let mut mask = [0u8; 16];
                for i in 0..Self::LANES {
                    mask[i] = if self.0[i] <= rhs.0[i] { 1 } else { 0 };
                }
                Mask128(mask)
            }
            fn gt(&self, rhs: &Self) -> Self::Mask {
                let mut mask = [0u8; 16];
                for i in 0..Self::LANES {
                    mask[i] = if self.0[i] > rhs.0[i] { 1 } else { 0 };
                }
                Mask128(mask)
            }
        }
        impl Mask for Mask128 {
            type BitMask = u16;
            type Element = u8;
            fn bitmask(self) -> Self::BitMask {
                {
                    self.0
                        .iter()
                        .enumerate()
                        .fold(0, |acc, (i, &b)| acc | ((b as u16) << i))
                }
            }
            fn splat(b: bool) -> Self {
                Mask128([b as u8; 16])
            }
        }
        impl BitAnd for Mask128 {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output {
                let mut result = [0u8; 16];
                for i in 0..16 {
                    result[i] = self.0[i] & rhs.0[i];
                }
                Mask128(result)
            }
        }
        impl BitOr for Mask128 {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                let mut result = [0u8; 16];
                for i in 0..16 {
                    result[i] = self.0[i] | rhs.0[i];
                }
                Mask128(result)
            }
        }
        impl BitOrAssign for Mask128 {
            fn bitor_assign(&mut self, rhs: Self) {
                for i in 0..16 {
                    self.0[i] |= rhs.0[i];
                }
            }
        }
    }
    pub(crate) mod v256 {
        use std::ops::{BitAnd, BitOr, BitOrAssign};
        use super::{Mask, Simd, v128::{Mask128, Simd128i, Simd128u}};
        #[repr(transparent)]
        pub struct Simd256u((Simd128u, Simd128u));
        #[automatically_derived]
        impl ::core::fmt::Debug for Simd256u {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Simd256u",
                    &&self.0,
                )
            }
        }
        #[repr(transparent)]
        pub struct Simd256i((Simd128i, Simd128i));
        #[automatically_derived]
        impl ::core::fmt::Debug for Simd256i {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Simd256i",
                    &&self.0,
                )
            }
        }
        #[repr(transparent)]
        pub struct Mask256(pub(crate) (Mask128, Mask128));
        #[automatically_derived]
        impl ::core::fmt::Debug for Mask256 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Mask256", &&self.0)
            }
        }
        impl Mask for Mask256 {
            type BitMask = u32;
            type Element = u8;
            #[inline(always)]
            fn bitmask(self) -> Self::BitMask {
                fn combine_u16(lo: u16, hi: u16) -> u32 {
                    { (lo as u32) | ((hi as u32) << 16) }
                }
                combine_u16(self.0.0.bitmask(), self.0.1.bitmask())
            }
            #[inline(always)]
            fn splat(b: bool) -> Self {
                Mask256((Mask128::splat(b), Mask128::splat(b)))
            }
        }
        impl BitOr for Mask256 {
            type Output = Self;
            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self::Output {
                let lo = self.0.0 | rhs.0.0;
                let hi = self.0.1 | rhs.0.1;
                Mask256((lo, hi))
            }
        }
        impl BitOrAssign for Mask256 {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0.0 |= rhs.0.0;
                self.0.1 |= rhs.0.1;
            }
        }
        impl BitAnd<Mask256> for Mask256 {
            type Output = Self;
            #[inline(always)]
            fn bitand(self, rhs: Mask256) -> Self::Output {
                let lo = self.0.0 & rhs.0.0;
                let hi = self.0.1 & rhs.0.1;
                Mask256((lo, hi))
            }
        }
        impl Simd for Simd256u {
            const LANES: usize = 32;
            type Mask = Mask256;
            type Element = u8;
            #[inline(always)]
            unsafe fn loadu(ptr: *const u8) -> Self {
                let lo = unsafe { Simd128u::loadu(ptr) };
                let hi = unsafe { Simd128u::loadu(ptr.add(Simd128u::LANES)) };
                Simd256u((lo, hi))
            }
            #[inline(always)]
            unsafe fn storeu(&self, ptr: *mut u8) {
                unsafe { Simd128u::storeu(&self.0.0, ptr) };
                unsafe { Simd128u::storeu(&self.0.1, ptr.add(Simd128u::LANES)) };
            }
            #[inline(always)]
            fn eq(&self, rhs: &Self) -> Self::Mask {
                let lo = self.0.0.eq(&rhs.0.0);
                let hi = self.0.1.eq(&rhs.0.1);
                Mask256((lo, hi))
            }
            #[inline(always)]
            fn splat(elem: u8) -> Self {
                Simd256u((Simd128u::splat(elem), Simd128u::splat(elem)))
            }
            #[inline(always)]
            fn le(&self, rhs: &Self) -> Self::Mask {
                let lo = self.0.0.le(&rhs.0.0);
                let hi = self.0.1.le(&rhs.0.1);
                Mask256((lo, hi))
            }
            #[inline(always)]
            fn gt(&self, rhs: &Self) -> Self::Mask {
                let lo = self.0.0.gt(&rhs.0.0);
                let hi = self.0.1.gt(&rhs.0.1);
                Mask256((lo, hi))
            }
        }
        impl Simd for Simd256i {
            const LANES: usize = 32;
            type Mask = Mask256;
            type Element = i8;
            #[inline(always)]
            unsafe fn loadu(ptr: *const u8) -> Self {
                let lo = unsafe { Simd128i::loadu(ptr) };
                let hi = unsafe { Simd128i::loadu(ptr.add(Simd128i::LANES)) };
                Simd256i((lo, hi))
            }
            #[inline(always)]
            unsafe fn storeu(&self, ptr: *mut u8) {
                unsafe { Simd128i::storeu(&self.0.0, ptr) };
                unsafe { Simd128i::storeu(&self.0.1, ptr.add(Simd128i::LANES)) };
            }
            #[inline(always)]
            fn eq(&self, rhs: &Self) -> Self::Mask {
                let lo = self.0.0.eq(&rhs.0.0);
                let hi = self.0.1.eq(&rhs.0.1);
                Mask256((lo, hi))
            }
            #[inline(always)]
            fn splat(elem: i8) -> Self {
                Simd256i((Simd128i::splat(elem), Simd128i::splat(elem)))
            }
            #[inline(always)]
            fn le(&self, rhs: &Self) -> Self::Mask {
                let lo = self.0.0.le(&rhs.0.0);
                let hi = self.0.1.le(&rhs.0.1);
                Mask256((lo, hi))
            }
            #[inline(always)]
            fn gt(&self, rhs: &Self) -> Self::Mask {
                let lo = self.0.0.gt(&rhs.0.0);
                let hi = self.0.1.gt(&rhs.0.1);
                Mask256((lo, hi))
            }
        }
    }
    pub(crate) mod v512 {
        use std::ops::{BitAnd, BitOr, BitOrAssign};
        use super::{Mask, Simd, v256::{Mask256, Simd256i, Simd256u}};
        #[repr(transparent)]
        pub struct Simd512u((Simd256u, Simd256u));
        #[automatically_derived]
        impl ::core::fmt::Debug for Simd512u {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Simd512u",
                    &&self.0,
                )
            }
        }
        #[repr(transparent)]
        pub struct Simd512i((Simd256i, Simd256i));
        #[automatically_derived]
        impl ::core::fmt::Debug for Simd512i {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Simd512i",
                    &&self.0,
                )
            }
        }
        #[repr(transparent)]
        pub struct Mask512((Mask256, Mask256));
        #[automatically_derived]
        impl ::core::fmt::Debug for Mask512 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Mask512", &&self.0)
            }
        }
        impl Mask for Mask512 {
            type BitMask = u64;
            type Element = u8;
            #[inline(always)]
            fn bitmask(self) -> Self::BitMask {
                fn combine_u32(lo: u32, hi: u32) -> u64 {
                    { (lo as u64) | ((hi as u64) << 32) }
                }
                combine_u32(self.0.0.bitmask(), self.0.1.bitmask())
            }
            #[inline(always)]
            fn splat(b: bool) -> Self {
                Mask512((Mask256::splat(b), Mask256::splat(b)))
            }
        }
        impl BitOr for Mask512 {
            type Output = Self;
            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self::Output {
                let lo = self.0.0 | rhs.0.0;
                let hi = self.0.1 | rhs.0.1;
                Mask512((lo, hi))
            }
        }
        impl BitOrAssign for Mask512 {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0.0 |= rhs.0.0;
                self.0.1 |= rhs.0.1;
            }
        }
        impl BitAnd<Mask512> for Mask512 {
            type Output = Self;
            #[inline(always)]
            fn bitand(self, rhs: Mask512) -> Self::Output {
                let lo = self.0.0 & rhs.0.0;
                let hi = self.0.1 & rhs.0.1;
                Mask512((lo, hi))
            }
        }
        impl Simd for Simd512u {
            const LANES: usize = 64;
            type Element = u8;
            type Mask = Mask512;
            #[inline(always)]
            unsafe fn loadu(ptr: *const u8) -> Self {
                let lo = unsafe { Simd256u::loadu(ptr) };
                let hi = unsafe { Simd256u::loadu(ptr.add(Simd256u::LANES)) };
                Simd512u((lo, hi))
            }
            #[inline(always)]
            unsafe fn storeu(&self, ptr: *mut u8) {
                unsafe { Simd256u::storeu(&self.0.0, ptr) };
                unsafe { Simd256u::storeu(&self.0.1, ptr.add(Simd256u::LANES)) };
            }
            #[inline(always)]
            fn eq(&self, rhs: &Self) -> Self::Mask {
                let lo = self.0.0.eq(&rhs.0.0);
                let hi = self.0.1.eq(&rhs.0.1);
                Mask512((lo, hi))
            }
            #[inline(always)]
            fn splat(ch: u8) -> Self {
                Simd512u((Simd256u::splat(ch), Simd256u::splat(ch)))
            }
            #[inline(always)]
            fn le(&self, rhs: &Self) -> Self::Mask {
                let lo = self.0.0.le(&rhs.0.0);
                let hi = self.0.1.le(&rhs.0.1);
                Mask512((lo, hi))
            }
            #[inline(always)]
            fn gt(&self, rhs: &Self) -> Self::Mask {
                let lo = self.0.0.gt(&rhs.0.0);
                let hi = self.0.1.gt(&rhs.0.1);
                Mask512((lo, hi))
            }
        }
        impl Simd for Simd512i {
            const LANES: usize = 64;
            type Element = i8;
            type Mask = Mask512;
            #[inline(always)]
            unsafe fn loadu(ptr: *const u8) -> Self {
                let lo = unsafe { Simd256i::loadu(ptr) };
                let hi = unsafe { Simd256i::loadu(ptr.add(Simd256i::LANES)) };
                Simd512i((lo, hi))
            }
            #[inline(always)]
            unsafe fn storeu(&self, ptr: *mut u8) {
                unsafe { Simd256i::storeu(&self.0.0, ptr) };
                unsafe { Simd256i::storeu(&self.0.1, ptr.add(Simd256i::LANES)) };
            }
            #[inline(always)]
            fn eq(&self, rhs: &Self) -> Self::Mask {
                let lo = self.0.0.eq(&rhs.0.0);
                let hi = self.0.1.eq(&rhs.0.1);
                Mask512((lo, hi))
            }
            #[inline(always)]
            fn splat(elem: i8) -> Self {
                Simd512i((Simd256i::splat(elem), Simd256i::splat(elem)))
            }
            #[inline(always)]
            fn le(&self, rhs: &Self) -> Self::Mask {
                let lo = self.0.0.le(&rhs.0.0);
                let hi = self.0.1.le(&rhs.0.1);
                Mask512((lo, hi))
            }
            #[inline(always)]
            fn gt(&self, rhs: &Self) -> Self::Mask {
                let lo = self.0.0.gt(&rhs.0.0);
                let hi = self.0.1.gt(&rhs.0.1);
                Mask512((lo, hi))
            }
        }
    }
    pub use self::traits::{BitMask, Mask, Simd};
}
#[inline(always)]
unsafe fn load<V: Simd>(ptr: *const u8) -> V {
    let chunk = unsafe { from_raw_parts(ptr, V::LANES) };
    unsafe { V::from_slice_unaligned_unchecked(chunk) }
}
const QUOTE_TAB: [(u8, [u8; 8]); 256] = [
    (6, *b"\\u0000\0\0"),
    (6, *b"\\u0001\0\0"),
    (6, *b"\\u0002\0\0"),
    (6, *b"\\u0003\0\0"),
    (6, *b"\\u0004\0\0"),
    (6, *b"\\u0005\0\0"),
    (6, *b"\\u0006\0\0"),
    (6, *b"\\u0007\0\0"),
    (2, *b"\\b\0\0\0\0\0\0"),
    (2, *b"\\t\0\0\0\0\0\0"),
    (2, *b"\\n\0\0\0\0\0\0"),
    (6, *b"\\u000b\0\0"),
    (2, *b"\\f\0\0\0\0\0\0"),
    (2, *b"\\r\0\0\0\0\0\0"),
    (6, *b"\\u000e\0\0"),
    (6, *b"\\u000f\0\0"),
    (6, *b"\\u0010\0\0"),
    (6, *b"\\u0011\0\0"),
    (6, *b"\\u0012\0\0"),
    (6, *b"\\u0013\0\0"),
    (6, *b"\\u0014\0\0"),
    (6, *b"\\u0015\0\0"),
    (6, *b"\\u0016\0\0"),
    (6, *b"\\u0017\0\0"),
    (6, *b"\\u0018\0\0"),
    (6, *b"\\u0019\0\0"),
    (6, *b"\\u001a\0\0"),
    (6, *b"\\u001b\0\0"),
    (6, *b"\\u001c\0\0"),
    (6, *b"\\u001d\0\0"),
    (6, *b"\\u001e\0\0"),
    (6, *b"\\u001f\0\0"),
    (0, [0; 8]),
    (0, [0; 8]),
    (2, *b"\\\"\0\0\0\0\0\0"),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (2, *b"\\\\\0\0\0\0\0\0"),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
    (0, [0; 8]),
];
const NEED_ESCAPED: [u8; 256] = [
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    0,
    0,
    1,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[inline(always)]
unsafe fn escape_unchecked(src: &mut *const u8, nb: &mut usize, dst: &mut *mut u8) {
    if true {
        if !(*nb >= 1) {
            ::core::panicking::panic("assertion failed: *nb >= 1")
        }
    }
    loop {
        let ch = unsafe { *(*src) };
        let cnt = QUOTE_TAB[ch as usize].0 as usize;
        if true {
            if !(cnt != 0) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "char is {0}, cnt is {1},  NEED_ESCAPED is {2}",
                            ch as char,
                            cnt,
                            NEED_ESCAPED[ch as usize],
                        ),
                    );
                }
            }
        }
        unsafe {
            std::ptr::copy_nonoverlapping(QUOTE_TAB[ch as usize].1.as_ptr(), *dst, 8)
        };
        unsafe { (*dst) = (*dst).add(cnt) };
        unsafe { (*src) = (*src).add(1) };
        (*nb) -= 1;
        if (*nb) == 0 || unsafe { NEED_ESCAPED[*(*src) as usize] == 0 } {
            return;
        }
    }
}
#[inline(always)]
fn check_cross_page(ptr: *const u8, step: usize) -> bool {
    let page_size = 4096;
    ((ptr as usize & (page_size - 1)) + step) > page_size
}
const LANES: usize = 16;
#[inline]
fn escaped_mask_generic(v: simd::v128::Simd128u) -> u16 {
    use simd::v128::Simd128u as u8x16;
    let x1f = u8x16::splat(0x1f);
    let blash = u8x16::splat(b'\\');
    let quote = u8x16::splat(b'"');
    let v = v.le(&x1f) | v.eq(&blash) | v.eq(&quote);
    v.bitmask()
}
#[inline]
fn escaped_mask_neon(v: simd::neon::Simd128u) -> simd::bits::NeonBits {
    use simd::neon::Simd128u as u8x16;
    let x1f = u8x16::splat(0x1f);
    let blash = u8x16::splat(b'\\');
    let quote = u8x16::splat(b'"');
    let v = v.le(&x1f) | v.eq(&blash) | v.eq(&quote);
    v.bitmask()
}
#[inline(always)]
fn format_string(value: &str, dst: &mut [u8]) -> usize {
    let mut v_neon: simd::neon::Simd128u;
    let mut v_generic: simd::v128::Simd128u;
    unsafe {
        let slice = value.as_bytes();
        let mut sptr = slice.as_ptr();
        let mut dptr = dst.as_mut_ptr();
        let dstart = dptr;
        let mut nb: usize = slice.len();
        *dptr = b'"';
        dptr = dptr.add(1);
        while nb >= LANES {
            {
                if true || (true || ::std_detect::detect::__is_feature_detected::asimd())
                {
                    v_neon = load(sptr);
                    v_neon
                        .write_to_slice_unaligned_unchecked(
                            std::slice::from_raw_parts_mut(dptr, LANES),
                        );
                    let mask = escaped_mask_neon(v_neon);
                    if mask.all_zero() {
                        nb -= LANES;
                        dptr = dptr.add(LANES);
                        sptr = sptr.add(LANES);
                    } else {
                        let cn = mask.first_offset();
                        nb -= cn;
                        dptr = dptr.add(cn);
                        sptr = sptr.add(cn);
                        escape_unchecked(&mut sptr, &mut nb, &mut dptr);
                    };
                } else {
                    v_generic = load(sptr);
                    v_generic
                        .write_to_slice_unaligned_unchecked(
                            std::slice::from_raw_parts_mut(dptr, LANES),
                        );
                    let mask = escaped_mask_generic(v_generic);
                    if mask.all_zero() {
                        nb -= LANES;
                        dptr = dptr.add(LANES);
                        sptr = sptr.add(LANES);
                    } else {
                        let cn = mask.first_offset();
                        nb -= cn;
                        dptr = dptr.add(cn);
                        sptr = sptr.add(cn);
                        escape_unchecked(&mut sptr, &mut nb, &mut dptr);
                    };
                }
            }
        }
        #[allow(invalid_value, clippy::uninit_assumed_init)]
        let mut placeholder: [u8; LANES] = core::mem::MaybeUninit::uninit()
            .assume_init();
        while nb > 0 {
            {
                if true || (true || ::std_detect::detect::__is_feature_detected::asimd())
                {
                    v_neon = {
                        {
                            if check_cross_page(sptr, LANES) {
                                std::ptr::copy_nonoverlapping(
                                    sptr,
                                    placeholder[..].as_mut_ptr(),
                                    nb,
                                );
                                load(placeholder[..].as_ptr())
                            } else {
                                {
                                    std::ptr::copy_nonoverlapping(
                                        sptr,
                                        placeholder[..].as_mut_ptr(),
                                        nb,
                                    );
                                    load(placeholder[..].as_ptr())
                                }
                            }
                        }
                    };
                    v_neon
                        .write_to_slice_unaligned_unchecked(
                            std::slice::from_raw_parts_mut(dptr, LANES),
                        );
                    let mask = escaped_mask_neon(v_neon);
                    if mask.all_zero() {
                        dptr = dptr.add(nb);
                        break;
                    } else {
                        let cn = mask.first_offset();
                        nb -= cn;
                        dptr = dptr.add(cn);
                        sptr = sptr.add(cn);
                        escape_unchecked(&mut sptr, &mut nb, &mut dptr);
                    }
                } else {
                    v_generic = {
                        {
                            if check_cross_page(sptr, LANES) {
                                std::ptr::copy_nonoverlapping(
                                    sptr,
                                    placeholder[..].as_mut_ptr(),
                                    nb,
                                );
                                load(placeholder[..].as_ptr())
                            } else {
                                {
                                    std::ptr::copy_nonoverlapping(
                                        sptr,
                                        placeholder[..].as_mut_ptr(),
                                        nb,
                                    );
                                    load(placeholder[..].as_ptr())
                                }
                            }
                        }
                    };
                    v_generic
                        .write_to_slice_unaligned_unchecked(
                            std::slice::from_raw_parts_mut(dptr, LANES),
                        );
                    let mask = escaped_mask_generic(v_generic);
                    if mask.all_zero() {
                        dptr = dptr.add(nb);
                        break;
                    } else {
                        let cn = mask.first_offset();
                        nb -= cn;
                        dptr = dptr.add(cn);
                        sptr = sptr.add(cn);
                        escape_unchecked(&mut sptr, &mut nb, &mut dptr);
                    }
                }
            }
        }
        *dptr = b'"';
        dptr = dptr.add(1);
        dptr as usize - dstart as usize
    }
}
pub fn escape(value: &str) -> String {
    let capacity = value.len() * 6 + 32 + 3;
    let mut buf = Vec::with_capacity(capacity);
    unsafe { buf.set_len(capacity) };
    let cnt = format_string(value, &mut buf);
    unsafe { buf.set_len(cnt) };
    unsafe { String::from_utf8_unchecked(buf) }
}
pub fn escape_into<S: AsRef<str>>(value: S, dst: &mut Vec<u8>) -> usize {
    let value = value.as_ref();
    let needed_capacity = value.len() * 6 + 32 + 3;
    dst.reserve(needed_capacity);
    let old_len = dst.len();
    unsafe {
        let spare = std::slice::from_raw_parts_mut(
            dst.as_mut_ptr().add(old_len),
            dst.capacity() - old_len,
        );
        let cnt = format_string(value, spare);
        dst.set_len(old_len + cnt);
        cnt
    }
}
