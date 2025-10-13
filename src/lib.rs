//! Borrowed from <https://github.com/cloudwego/sonic-rs/blob/v0.5.5/src/util/string.rs>
//!
//! Only takes the string escaping part to avoid the abstraction overhead.

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use std::arch::is_x86_feature_detected;
use std::slice::from_raw_parts;

use simd::{BitMask, Mask, Simd};

mod simd;

#[inline(always)]
unsafe fn load<V: Simd>(ptr: *const u8) -> V {
    let chunk = unsafe { from_raw_parts(ptr, V::LANES) };
    unsafe { V::from_slice_unaligned_unchecked(chunk) }
}

const QUOTE_TAB: [(u8, [u8; 8]); 256] = [
    // 0x00 ~ 0x1f
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
    // 0x20 ~ 0x2f
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
    // 0x30 ~ 0x3f
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
    // 0x40 ~ 0x4f
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
    // 0x50 ~ 0x5f
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
    // 0x60 ~ 0xff
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
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(feature = "codspeed")
))]
static COMPUTE_LANES: std::sync::Once = std::sync::Once::new();
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(feature = "codspeed")
))]
static mut LANES: usize = simd::avx2::Simd256u::LANES;
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "codspeed"))]
const LANES: usize = simd::avx2::Simd256u::LANES;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const LANES: usize = 16;

// only check the src length.
#[inline(always)]
unsafe fn escape_unchecked(src: &mut *const u8, nb: &mut usize, dst: &mut *mut u8) {
    debug_assert!(*nb >= 1);
    loop {
        let ch = unsafe { *(*src) };
        let cnt = QUOTE_TAB[ch as usize].0 as usize;
        debug_assert!(
            cnt != 0,
            "char is {}, cnt is {},  NEED_ESCAPED is {}",
            ch as char,
            cnt,
            NEED_ESCAPED[ch as usize]
        );
        unsafe { std::ptr::copy_nonoverlapping(QUOTE_TAB[ch as usize].1.as_ptr(), *dst, 8) };
        unsafe { (*dst) = (*dst).add(cnt) };
        unsafe { (*src) = (*src).add(1) };
        (*nb) -= 1;
        if (*nb) == 0 || unsafe { NEED_ESCAPED[*(*src) as usize] == 0 } {
            return;
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[inline(always)]
fn check_cross_page(ptr: *const u8, step: usize) -> bool {
    let page_size = 4096;
    ((ptr as usize & (page_size - 1)) + step) > page_size
}

#[inline(always)]
fn escaped_mask_generic(v: simd::v128::Simd128u) -> u16 {
    use simd::v128::Simd128u as u8x16;

    let x1f = u8x16::splat(0x1f); // 0x00 ~ 0x20
    let blash = u8x16::splat(b'\\');
    let quote = u8x16::splat(b'"');
    let v = v.le(&x1f) | v.eq(&blash) | v.eq(&quote);
    v.bitmask()
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn escaped_mask_neon(v: simd::neon::Simd128u) -> simd::bits::NeonBits {
    use simd::neon::Simd128u as u8x16;

    let x1f = u8x16::splat(0x1f); // 0x00 ~ 0x20
    let blash = u8x16::splat(b'\\');
    let quote = u8x16::splat(b'"');
    let v = v.le(&x1f) | v.eq(&blash) | v.eq(&quote);
    v.bitmask()
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline(always)]
fn escaped_mask_sse2(v: simd::sse2::Simd128u) -> u16 {
    use simd::sse2::Simd128u as u8x16;

    let x1f = u8x16::splat(0x1f); // 0x00 ~ 0x20
    let blash = u8x16::splat(b'\\');
    let quote = u8x16::splat(b'"');
    let v = v.le(&x1f) | v.eq(&blash) | v.eq(&quote);
    v.bitmask()
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline(always)]
fn escaped_mask_avx2(v: simd::avx2::Simd256u) -> u32 {
    use simd::avx2::Simd256u as u8x32;

    let x1f = u8x32::splat(0x1f); // 0x00 ~ 0x20
    let blash = u8x32::splat(b'\\');
    let quote = u8x32::splat(b'"');
    let v = v.le(&x1f) | v.eq(&blash) | v.eq(&quote);
    v.bitmask()
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline(always)]
fn escaped_mask_avx512(v: simd::avx512::Simd512u) -> u64 {
    use simd::avx512::Simd512u as u8x64;

    let x1f = u8x64::splat(0x1f); // 0x00 ~ 0x20
    let blash = u8x64::splat(b'\\');
    let quote = u8x64::splat(b'"');
    let v = v.le(&x1f) | v.eq(&blash) | v.eq(&quote);
    v.bitmask()
}

macro_rules! escape {
    ($mask:expr, $nb:expr, $dptr:expr, $sptr:expr) => {
        if $mask.all_zero() {
            $nb -= LANES;
            $dptr = $dptr.add(LANES);
            $sptr = $sptr.add(LANES);
        } else {
            let cn = $mask.first_offset();
            $nb -= cn;
            $dptr = $dptr.add(cn);
            $sptr = $sptr.add(cn);
            escape_unchecked(&mut $sptr, &mut $nb, &mut $dptr);
        }
    };
}

macro_rules! load_v {
    ($placeholder:expr, $sptr:expr, $nb:expr) => {{
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            std::ptr::copy_nonoverlapping($sptr, $placeholder[..].as_mut_ptr(), $nb);
            load($placeholder[..].as_ptr())
        }
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            if check_cross_page($sptr, LANES) {
                std::ptr::copy_nonoverlapping($sptr, $placeholder[..].as_mut_ptr(), $nb);
                load($placeholder[..].as_ptr())
            } else {
                #[cfg(any(debug_assertions, miri))]
                {
                    std::ptr::copy_nonoverlapping($sptr, $placeholder[..].as_mut_ptr(), $nb);
                    load($placeholder[..].as_ptr())
                }
                #[cfg(not(any(debug_assertions, miri)))]
                {
                    load($sptr)
                }
            }
        }
    }};
}

#[inline(always)]
fn format_string(value: &str, dst: &mut [u8]) -> usize {
    #[cfg(target_arch = "aarch64")]
    let mut v_neon: simd::neon::Simd128u;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    let mut v_sse2: simd::sse2::Simd128u;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    let mut v_avx2: simd::avx2::Simd256u;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    let mut v_avx512: simd::avx512::Simd512u;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    let has_avx512 = is_x86_feature_detected!("avx512f");
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    let has_avx2 = is_x86_feature_detected!("avx2");
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    let has_sse2 = is_x86_feature_detected!("sse2");

    #[cfg(target_arch = "aarch64")]
    let has_neon = cfg!(target_os = "macos") || std::arch::is_aarch64_feature_detected!("neon");

    let mut v_generic: simd::v128::Simd128u;

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        not(feature = "codspeed")
    ))]
    COMPUTE_LANES.call_once(|| {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx512f") {
                unsafe {
                    LANES = simd::avx512::Simd512u::LANES;
                }
            } else if !is_x86_feature_detected!("avx2") {
                unsafe {
                    LANES = simd::sse2::Simd128u::LANES;
                }
            }
        }
    });

    unsafe {
        let slice = value.as_bytes();
        let mut sptr = slice.as_ptr();
        let mut dptr = dst.as_mut_ptr();
        let dstart = dptr;
        let mut nb: usize = slice.len();

        *dptr = b'"';
        dptr = dptr.add(1);
        while nb >= LANES {
            #[cfg(target_arch = "aarch64")]
            {
                if has_neon {
                    v_neon = load(sptr);
                    v_neon.write_to_slice_unaligned_unchecked(std::slice::from_raw_parts_mut(
                        dptr, LANES,
                    ));
                    let mask = escaped_mask_neon(v_neon);
                    escape!(mask, nb, dptr, sptr);
                } else {
                    v_generic = load(sptr);
                    v_generic.write_to_slice_unaligned_unchecked(std::slice::from_raw_parts_mut(
                        dptr, LANES,
                    ));
                    let mask = escaped_mask_generic(v_generic);
                    escape!(mask, nb, dptr, sptr);
                }
            }
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            {
                if has_avx512 {
                    v_avx512 = load(sptr);
                    v_avx512.write_to_slice_unaligned_unchecked(std::slice::from_raw_parts_mut(
                        dptr, LANES,
                    ));
                    let mask = escaped_mask_avx512(v_avx512);
                    escape!(mask, nb, dptr, sptr);
                } else if has_avx2 {
                    v_avx2 = load(sptr);
                    v_avx2.write_to_slice_unaligned_unchecked(std::slice::from_raw_parts_mut(
                        dptr, LANES,
                    ));
                    let mask = escaped_mask_avx2(v_avx2);
                    escape!(mask, nb, dptr, sptr);
                } else if has_sse2 {
                    v_sse2 = load(sptr);
                    v_sse2.write_to_slice_unaligned_unchecked(std::slice::from_raw_parts_mut(
                        dptr, LANES,
                    ));
                    let mask = escaped_mask_sse2(v_sse2);
                    escape!(mask, nb, dptr, sptr);
                } else {
                    v_generic = load(sptr);
                    v_generic.write_to_slice_unaligned_unchecked(std::slice::from_raw_parts_mut(
                        dptr, LANES,
                    ));
                    let mask = escaped_mask_generic(v_generic);
                    escape!(mask, nb, dptr, sptr);
                }
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            if has_neon {
                const LANES: usize = simd::neon::Simd128u::LANES;
                let mut placeholder: [u8; LANES] = [0; LANES];
                while nb > 0 {
                    v_neon = load_v!(placeholder, sptr, nb);
                    v_neon.write_to_slice_unaligned_unchecked(std::slice::from_raw_parts_mut(
                        dptr, LANES,
                    ));
                    let mask = escaped_mask_neon(v_neon).clear_high_bits(LANES - nb);
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
            } else {
                const LANES: usize = simd::v128::Simd128u::LANES;
                let mut placeholder: [u8; LANES] = [0; LANES];
                while nb > 0 {
                    v_generic = load_v!(placeholder, sptr, nb);
                    v_generic.write_to_slice_unaligned_unchecked(std::slice::from_raw_parts_mut(
                        dptr, LANES,
                    ));
                    let mask = escaped_mask_generic(v_generic).clear_high_bits(LANES - nb);
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
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            if has_avx512 {
                const LANES: usize = simd::avx512::Simd512u::LANES;
                let mut placeholder: [u8; LANES] = [0; LANES];
                while nb > 0 {
                    v_avx512 = load_v!(placeholder, sptr, nb);
                    v_avx512.write_to_slice_unaligned_unchecked(std::slice::from_raw_parts_mut(
                        dptr, LANES,
                    ));
                    let mask = escaped_mask_avx512(v_avx512).clear_high_bits(LANES - nb);
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
            } else if has_avx2 {
                const LANES: usize = simd::avx2::Simd256u::LANES;
                let mut placeholder: [u8; LANES] = [0; LANES];
                while nb > 0 {
                    v_avx2 = load_v!(placeholder, sptr, nb);
                    v_avx2.write_to_slice_unaligned_unchecked(std::slice::from_raw_parts_mut(
                        dptr, LANES,
                    ));
                    let mask = escaped_mask_avx2(v_avx2).clear_high_bits(LANES - nb);
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
            } else if has_sse2 {
                const LANES: usize = simd::sse2::Simd128u::LANES;
                let mut placeholder: [u8; LANES] = [0; LANES];
                while nb > 0 {
                    v_sse2 = load_v!(placeholder, sptr, nb);
                    v_sse2.write_to_slice_unaligned_unchecked(std::slice::from_raw_parts_mut(
                        dptr, LANES,
                    ));
                    let mask = escaped_mask_sse2(v_sse2).clear_high_bits(LANES - nb);
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
            } else {
                const LANES: usize = simd::v128::Simd128u::LANES;
                let mut placeholder: [u8; LANES] = [0; LANES];
                while nb > 0 {
                    v_generic = load_v!(placeholder, sptr, nb);
                    v_generic.write_to_slice_unaligned_unchecked(std::slice::from_raw_parts_mut(
                        dptr, LANES,
                    ));
                    let mask = escaped_mask_generic(v_generic).clear_high_bits(LANES - nb);
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
    #[allow(clippy::uninit_vec)]
    unsafe {
        buf.set_len(capacity)
    };
    let cnt = format_string(value, &mut buf);
    unsafe { buf.set_len(cnt) };
    unsafe { String::from_utf8_unchecked(buf) }
}

pub fn escape_into<S: AsRef<str>>(value: S, dst: &mut Vec<u8>) -> usize {
    let value = value.as_ref();
    let needed_capacity = value.len() * 6 + 32 + 3;

    // Ensure we have enough capacity
    dst.reserve(needed_capacity);

    let old_len = dst.len();

    // SAFETY: We've reserved enough capacity above, and format_string will
    // write valid UTF-8 bytes. We'll set the correct length after.
    unsafe {
        // Get a slice that includes the spare capacity
        let spare =
            std::slice::from_raw_parts_mut(dst.as_mut_ptr().add(old_len), dst.capacity() - old_len);
        let cnt = format_string(value, spare);
        dst.set_len(old_len + cnt);
        cnt
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_dir;
    use std::path::{Path, PathBuf};

    use rand::seq::SliceRandom;

    use super::*;

    #[test]
    fn test_escape_ascii_json_string() {
        let fixture = r#"abcdefghijklmnopqrstuvwxyz .*? hello world escape json string"#;
        assert_eq!(escape(fixture), serde_json::to_string(fixture).unwrap());
    }

    #[test]
    fn test_escape_json_string() {
        let mut fixture = String::new();
        for i in 0u8..=0x1F {
            fixture.push(i as char);
        }
        fixture.push('\t');
        fixture.push('\x08');
        fixture.push('\x09');
        fixture.push('\x0A');
        fixture.push('\x0C');
        fixture.push('\x0D');
        fixture.push('\x22');
        fixture.push('\x5C');
        fixture.push_str("normal string");
        fixture.push('😊');
        fixture.push_str("中文 English 🚀 \n❓ 𝄞");
        escape(fixture.as_str());
        assert_eq!(
            escape(fixture.as_str()),
            serde_json::to_string(fixture.as_str()).unwrap(),
            "fixture: {:?}",
            fixture
        );
    }

    // Test cases for various string sizes to cover different SIMD paths

    #[test]
    fn test_empty_string() {
        assert_eq!(escape(""), r#""""#);
    }

    #[test]
    fn test_very_small_strings() {
        // Less than 16 bytes (SSE register size)
        assert_eq!(escape("a"), r#""a""#);
        assert_eq!(escape("ab"), r#""ab""#);
        assert_eq!(escape("hello"), r#""hello""#);
        assert_eq!(escape("hello\n"), r#""hello\n""#);
        assert_eq!(escape("\""), r#""\"""#);
        assert_eq!(escape("\\"), r#""\\""#);
        assert_eq!(escape("\t"), r#""\t""#);
        assert_eq!(escape("\r\n"), r#""\r\n""#);
    }

    #[test]
    fn test_small_strings_16_bytes() {
        // Exactly 16 bytes - SSE register boundary
        let s16 = "0123456789abcdef";
        assert_eq!(s16.len(), 16);
        assert_eq!(escape(s16), serde_json::to_string(s16).unwrap());

        // 16 bytes with escapes
        let s16_esc = "01234567\t9abcde";
        assert_eq!(s16_esc.len(), 15); // \t is 1 byte
        assert_eq!(escape(s16_esc), serde_json::to_string(s16_esc).unwrap());
    }

    #[test]
    fn test_medium_strings_32_bytes() {
        // Exactly 32 bytes - AVX2 register boundary
        let s32 = "0123456789abcdef0123456789abcdef";
        assert_eq!(s32.len(), 32);
        assert_eq!(escape(s32), serde_json::to_string(s32).unwrap());

        // 32 bytes with escapes at different positions
        let s32_esc = "0123456789abcde\"0123456789abcde";
        assert_eq!(escape(s32_esc), serde_json::to_string(s32_esc).unwrap());
    }

    #[test]
    fn test_large_strings_128_bytes() {
        // Exactly 128 bytes - main loop size
        let s128 = "0123456789abcdef".repeat(8);
        assert_eq!(s128.len(), 128);
        assert_eq!(escape(&s128), serde_json::to_string(&s128).unwrap());

        // 128 bytes with escapes spread throughout
        let mut s128_esc = String::new();
        for i in 0..8 {
            if i % 2 == 0 {
                s128_esc.push_str("0123456789abcd\n");
            } else {
                s128_esc.push_str("0123456789abcd\"");
            }
        }
        assert_eq!(escape(&s128_esc), serde_json::to_string(&s128_esc).unwrap());
    }

    #[test]
    fn test_unaligned_data() {
        // Test strings that start at various alignments
        for offset in 0..32 {
            let padding = " ".repeat(offset);
            let test_str = format!("{}{}", padding, "test\nstring\"with\\escapes");
            let result = escape(&test_str[offset..]);
            let expected = serde_json::to_string(&test_str[offset..]).unwrap();
            assert_eq!(result, expected, "Failed at offset {}", offset);
        }
    }

    #[test]
    fn test_sparse_escapes() {
        // Large string with escapes only at the beginning and end
        let mut s = String::new();
        s.push('"');
        s.push_str(&"a".repeat(500));
        s.push('\\');
        assert_eq!(escape(&s), serde_json::to_string(&s).unwrap());
    }

    #[test]
    fn test_dense_escapes() {
        // String with many escapes
        let s = "\"\\\"\\\"\\\"\\".repeat(50);
        assert_eq!(escape(&s), serde_json::to_string(&s).unwrap());

        // All control characters
        let mut ctrl = String::new();
        for _ in 0..10 {
            for i in 0u8..32 {
                ctrl.push(i as char);
            }
        }
        assert_eq!(escape(&ctrl), serde_json::to_string(&ctrl).unwrap());
    }

    #[test]
    fn test_boundary_conditions() {
        // Test around 256 byte boundary (common cache line multiple)
        for size in 250..260 {
            let s = "a".repeat(size);
            assert_eq!(escape(&s), serde_json::to_string(&s).unwrap());

            // With escape at the end
            let mut s_esc = "a".repeat(size - 1);
            s_esc.push('"');
            assert_eq!(escape(&s_esc), serde_json::to_string(&s_esc).unwrap());
        }
    }

    #[test]
    fn test_all_escape_types() {
        // Test each escape type individually
        assert_eq!(escape("\x00"), r#""\u0000""#);
        assert_eq!(escape("\x08"), r#""\b""#);
        assert_eq!(escape("\x09"), r#""\t""#);
        assert_eq!(escape("\x0A"), r#""\n""#);
        assert_eq!(escape("\x0C"), r#""\f""#);
        assert_eq!(escape("\x0D"), r#""\r""#);
        assert_eq!(escape("\x1F"), r#""\u001f""#);
        assert_eq!(escape("\""), r#""\"""#);
        assert_eq!(escape("\\"), r#""\\""#);

        // Test all control characters
        for i in 0u8..32 {
            let s = String::from_utf8(vec![i]).unwrap();
            let result = escape(&s);
            let expected = serde_json::to_string(&s).unwrap();
            assert_eq!(result, expected, "Failed for byte 0x{:02x}", i);
        }
    }

    #[test]
    fn test_mixed_content() {
        // Mix of ASCII, escapes, and multi-byte UTF-8
        let mixed = r#"Hello "World"!
    Tab:	Here
    Emoji: 😀 Chinese: 中文
    Math: ∑∫∂ Music: 𝄞
    Escape: \" \\ \n \r \t"#;
        assert_eq!(escape(mixed), serde_json::to_string(mixed).unwrap());
    }

    #[test]
    fn test_repeated_patterns() {
        // Patterns that might benefit from or confuse SIMD operations
        let pattern1 = "abcd".repeat(100);
        assert_eq!(escape(&pattern1), serde_json::to_string(&pattern1).unwrap());

        let pattern2 = "a\"b\"".repeat(100);
        assert_eq!(escape(&pattern2), serde_json::to_string(&pattern2).unwrap());

        let pattern3 = "\t\n".repeat(100);
        assert_eq!(escape(&pattern3), serde_json::to_string(&pattern3).unwrap());
    }

    #[test]
    fn test_rxjs() {
        let mut sources = Vec::new();
        read_dir_recursive("node_modules/rxjs/src", &mut sources, |p| {
            matches!(p.extension().and_then(|e| e.to_str()), Some("ts"))
        })
        .unwrap();
        assert!(!sources.is_empty());
        sources.shuffle(&mut rand::rng());
        for source in sources
            .iter()
            .take(if cfg!(miri) { 10 } else { sources.len() })
        {
            assert_eq!(escape(&source), serde_json::to_string(&source).unwrap());
            let mut output = String::new();
            escape_into(&source, unsafe { output.as_mut_vec() });
            assert_eq!(output, serde_json::to_string(&source).unwrap());
        }
    }

    #[test]
    fn test_sources() {
        for source in load_affine_sources().unwrap() {
            assert_eq!(escape(&source), serde_json::to_string(&source).unwrap());
            let mut output = String::with_capacity(source.len() * 6 + 32 + 3);
            escape_into(&source, unsafe { output.as_mut_vec() });
            assert_eq!(output, serde_json::to_string(&source).unwrap());
        }
    }

    fn load_affine_sources() -> Result<impl Iterator<Item = String>, std::io::Error> {
        let mut sources = Vec::new();
        read_dir_recursive("fixtures", &mut sources, |p| {
            matches!(
                p.extension().and_then(|e| e.to_str()),
                Some("ts") | Some("tsx") | Some("js") | Some("mjs") | Some("cjs")
            )
        })?;
        assert!(!sources.is_empty());
        let len = sources.len();
        sources.shuffle(&mut rand::rng());
        Ok(sources.into_iter().take(if cfg!(miri) { 10 } else { len }))
    }

    fn read_dir_recursive<P: AsRef<Path>, F: Fn(PathBuf) -> bool + Copy>(
        dir: P,
        sources: &mut Vec<String>,
        f: F,
    ) -> Result<(), std::io::Error> {
        let dir = read_dir(dir)?;
        for entry in dir {
            let p = entry?;
            let metadata = std::fs::metadata(p.path())?;
            if metadata.is_file() {
                if f(p.path()) {
                    sources.push(std::fs::read_to_string(p.path())?);
                }
            }
            if metadata.is_dir() {
                read_dir_recursive(p.path(), sources, f)?;
            }
        }
        Ok(())
    }
}
