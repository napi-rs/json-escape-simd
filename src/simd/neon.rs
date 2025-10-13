use std::arch::aarch64::*;

use super::{Mask, Simd, bits::NeonBits};

#[derive(Debug)]
#[repr(transparent)]
pub struct Simd128u(uint8x16_t);

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

    // less or equal
    #[inline(always)]
    fn le(&self, lhs: &Self) -> Self::Mask {
        unsafe { Mask128(vcleq_u8(self.0, lhs.0)) }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Mask128(pub(crate) uint8x16_t);

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
}

// Bitwise AND for Mask128
impl std::ops::BitAnd<Mask128> for Mask128 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Mask128) -> Self::Output {
        unsafe { Self(vandq_u8(self.0, rhs.0)) }
    }
}

// Bitwise OR for Mask128
impl std::ops::BitOr<Mask128> for Mask128 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Mask128) -> Self::Output {
        unsafe { Self(vorrq_u8(self.0, rhs.0)) }
    }
}

// Bitwise OR assignment for Mask128
impl std::ops::BitOrAssign<Mask128> for Mask128 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Mask128) {
        unsafe {
            self.0 = vorrq_u8(self.0, rhs.0);
        }
    }
}
