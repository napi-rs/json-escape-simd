#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::ops::{BitAnd, BitOr, BitOrAssign};

use super::{Mask, Simd};

#[derive(Debug)]
#[repr(transparent)]
pub struct Simd128u(__m128i);

#[derive(Debug)]
#[repr(transparent)]
pub struct Mask128(__m128i);

impl Mask for Mask128 {
    type BitMask = u16;
    type Element = u8;

    #[inline(always)]
    fn bitmask(self) -> Self::BitMask {
        unsafe { _mm_movemask_epi8(self.0) as u16 }
    }

    #[inline(always)]
    fn splat(b: bool) -> Self {
        let v: i8 = if b { -1 } else { 0 };
        unsafe { Mask128(_mm_set1_epi8(v)) }
    }
}

impl BitAnd<Mask128> for Mask128 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Mask128) -> Self::Output {
        unsafe { Mask128(_mm_and_si128(self.0, rhs.0)) }
    }
}

impl BitOr<Mask128> for Mask128 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Mask128) -> Self::Output {
        unsafe { Mask128(_mm_or_si128(self.0, rhs.0)) }
    }
}

impl BitOrAssign<Mask128> for Mask128 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Mask128) {
        self.0 = unsafe { _mm_or_si128(self.0, rhs.0) };
    }
}

impl Simd for Simd128u {
    const LANES: usize = 16;
    type Mask = Mask128;
    type Element = u8;

    #[inline(always)]
    unsafe fn loadu(ptr: *const u8) -> Self {
        Simd128u(unsafe { _mm_loadu_si128(ptr as *const __m128i) })
    }

    #[inline(always)]
    unsafe fn storeu(&self, ptr: *mut u8) {
        unsafe { _mm_storeu_si128(ptr as *mut __m128i, self.0) }
    }

    #[inline(always)]
    fn eq(&self, rhs: &Self) -> Self::Mask {
        Mask128(unsafe { _mm_cmpeq_epi8(self.0, rhs.0) })
    }

    #[inline(always)]
    fn splat(ch: u8) -> Self {
        Simd128u(unsafe { _mm_set1_epi8(ch as i8) })
    }

    #[inline(always)]
    fn le(&self, rhs: &Self) -> Self::Mask {
        unsafe {
            let max = _mm_max_epu8(self.0, rhs.0);
            let eq = _mm_cmpeq_epi8(max, rhs.0);
            Mask128(eq)
        }
    }

    #[inline(always)]
    fn gt(&self, _rhs: &Self) -> Self::Mask {
        todo!()
    }
}
