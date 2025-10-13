use std::ops::{BitAnd, BitOr, BitOrAssign};

use super::{Mask, Simd, util::escape_unchecked};

#[cfg(any(target_os = "linux", target_os = "macos"))]
use super::util::check_cross_page;

#[derive(Debug)]
pub struct Simd128u([u8; 16]);

#[derive(Debug)]
pub struct Mask128(pub(crate) [u8; 16]);

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
        unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, Self::LANES) };
    }

    fn eq(&self, rhs: &Self) -> Self::Mask {
        let mut mask = [0u8; 16];
        for (i, item) in mask.iter_mut().enumerate().take(Self::LANES) {
            *item = if self.0[i] == rhs.0[i] { 1 } else { 0 };
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
}

impl Mask for Mask128 {
    type BitMask = u16;
    type Element = u8;

    fn bitmask(self) -> Self::BitMask {
        #[cfg(target_endian = "little")]
        {
            self.0
                .iter()
                .enumerate()
                .fold(0, |acc, (i, &b)| acc | ((b as u16) << i))
        }
        #[cfg(target_endian = "big")]
        {
            self.0
                .iter()
                .enumerate()
                .fold(0, |acc, (i, &b)| acc | ((b as u16) << (15 - i)))
        }
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

#[inline(always)]
fn escaped_mask(v: Simd128u) -> u16 {
    let x1f = Simd128u::splat(0x1f); // 0x00 ~ 0x20
    let blash = Simd128u::splat(b'\\');
    let quote = Simd128u::splat(b'"');
    let v = v.le(&x1f) | v.eq(&blash) | v.eq(&quote);
    v.bitmask()
}

pub fn format_string(value: &str, dst: &mut [u8]) -> usize {
    unsafe {
        let slice = value.as_bytes();
        let mut sptr = slice.as_ptr();
        let mut dptr = dst.as_mut_ptr();
        let dstart = dptr;
        let mut nb: usize = slice.len();

        *dptr = b'"';
        dptr = dptr.add(1);

        // Main loop: process LANES bytes at a time
        while nb >= Simd128u::LANES {
            let v = Simd128u::loadu(sptr);
            v.storeu(dptr);
            let mask = escaped_mask(v);

            if mask == 0 {
                nb -= Simd128u::LANES;
                dptr = dptr.add(Simd128u::LANES);
                sptr = sptr.add(Simd128u::LANES);
            } else {
                let cn = mask.trailing_zeros() as usize;
                nb -= cn;
                dptr = dptr.add(cn);
                sptr = sptr.add(cn);
                escape_unchecked(&mut sptr, &mut nb, &mut dptr);
            }
        }

        // Handle remaining bytes
        let mut placeholder: [u8; 16] = [0; 16];
        while nb > 0 {
            #[cfg(not(any(target_os = "linux", target_os = "macos")))]
            let v = {
                std::ptr::copy_nonoverlapping(sptr, placeholder.as_mut_ptr(), nb);
                Simd128u::loadu(placeholder.as_ptr())
            };
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            let v = {
                if check_cross_page(sptr, Simd128u::LANES) {
                    std::ptr::copy_nonoverlapping(sptr, placeholder.as_mut_ptr(), nb);
                    Simd128u::loadu(placeholder.as_ptr())
                } else {
                    #[cfg(any(debug_assertions, miri))]
                    {
                        std::ptr::copy_nonoverlapping(sptr, placeholder.as_mut_ptr(), nb);
                        Simd128u::loadu(placeholder.as_ptr())
                    }
                    #[cfg(not(any(debug_assertions, miri)))]
                    {
                        Simd128u::loadu(sptr)
                    }
                }
            };

            v.storeu(dptr);
            let mut mask = escaped_mask(v);
            // Clear high bits for partial vector
            mask &= (1u16 << nb) - 1;

            if mask == 0 {
                dptr = dptr.add(nb);
                break;
            } else {
                let cn = mask.trailing_zeros() as usize;
                nb -= cn;
                dptr = dptr.add(cn);
                sptr = sptr.add(cn);
                escape_unchecked(&mut sptr, &mut nb, &mut dptr);
            }
        }

        *dptr = b'"';
        dptr = dptr.add(1);
        dptr as usize - dstart as usize
    }
}
