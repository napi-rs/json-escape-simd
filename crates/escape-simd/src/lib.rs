//! Shared SIMD kernels for JSON and HTML string escaping.
//!
//! Enable `json` and/or `html`. Prefer the `json-escape-simd` and
//! `html-escape-simd` facades unless you need both in one crate.
//!
//! JSON kernel borrowed from <https://github.com/cloudwego/sonic-rs/blob/v0.5.5/src/util/string.rs>.

#![allow(clippy::incompatible_msrv)]

#[cfg(all(
    any(feature = "json", feature = "html"),
    any(target_arch = "x86_64", target_arch = "x86")
))]
use std::arch::is_x86_feature_detected;

#[cfg(any(feature = "json", feature = "html"))]
mod simd;

#[cfg(feature = "json")]
pub(crate) const QUOTE_TAB: [(u8, [u8; 8]); 256] = [
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

#[cfg(feature = "json")]
pub(crate) const NEED_ESCAPED: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[cfg(feature = "html")]
/// HTML escape table, byte-identical to `@napi-rs/escape@1.0.2` /
/// `v_htmlescape`: exactly `"` `&` `'` `/` `<` `>` are escaped, every other
/// byte (including UTF-8 bytes >= 0x80) passes through verbatim. Same
/// `(len, 8-byte padded replacement)` layout as [`QUOTE_TAB`].
pub(crate) const HTML_ESCAPE_TAB: [(u8, [u8; 8]); 256] = {
    let mut tab = [(0, [0; 8]); 256];
    tab[b'"' as usize] = (6, *b"&quot;\0\0");
    tab[b'&' as usize] = (5, *b"&amp;\0\0\0");
    tab[b'\'' as usize] = (6, *b"&#x27;\0\0");
    tab[b'/' as usize] = (6, *b"&#x2f;\0\0");
    tab[b'<' as usize] = (4, *b"&lt;\0\0\0\0");
    tab[b'>' as usize] = (4, *b"&gt;\0\0\0\0");
    tab
};

#[cfg(feature = "html")]
pub(crate) const HTML_NEED_ESCAPED: [u8; 256] = {
    let mut tab = [0; 256];
    tab[b'"' as usize] = 1;
    tab[b'&' as usize] = 1;
    tab[b'\'' as usize] = 1;
    tab[b'/' as usize] = 1;
    tab[b'<' as usize] = 1;
    tab[b'>' as usize] = 1;
    tab
};

#[cfg(feature = "json")]
#[inline(always)]
pub(crate) fn format_json_string(value: &str, dst: &mut [u8]) -> usize {
    #[cfg(target_arch = "aarch64")]
    {
        let has_neon = cfg!(target_os = "macos") || std::arch::is_aarch64_feature_detected!("neon");
        if has_neon {
            unsafe { simd::neon::format_string::<false>(value, dst) }
        } else {
            simd::v128::format_string::<false>(value, dst)
        }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        #[cfg(feature = "avx512")]
        {
            // The avx512 kernel uses AVX-512BW byte compares (vpcmpub) and
            // masked loads/stores (VL), so require both at runtime, not just F.
            if is_x86_feature_detected!("avx512bw") && is_x86_feature_detected!("avx512vl") {
                return unsafe { simd::avx512::format_string::<false>(value, dst) };
            }
        }
        if is_x86_feature_detected!("avx2") {
            unsafe { simd::avx2::format_string::<false>(value, dst) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { simd::sse2::format_string::<false>(value, dst) }
        } else {
            simd::v128::format_string::<false>(value, dst)
        }
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")))]
    {
        simd::v128::format_string::<false>(value, dst)
    }
}

/// Same dispatch as [`format_json_string`], but runs the HTML escape mode of the
/// kernels: no surrounding quotes, only the 6 HTML characters are escaped.
#[cfg(feature = "html")]
#[inline(always)]
pub(crate) fn format_html_string(value: &str, dst: &mut [u8]) -> usize {
    #[cfg(target_arch = "aarch64")]
    {
        let has_neon = cfg!(target_os = "macos") || std::arch::is_aarch64_feature_detected!("neon");
        if has_neon {
            unsafe { simd::neon::format_string::<true>(value, dst) }
        } else {
            simd::v128::format_string::<true>(value, dst)
        }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        #[cfg(feature = "avx512")]
        {
            // The avx512 kernel uses AVX-512BW byte compares (vpcmpub) and
            // masked loads/stores (VL), so require both at runtime, not just F.
            if is_x86_feature_detected!("avx512bw") && is_x86_feature_detected!("avx512vl") {
                return unsafe { simd::avx512::format_string::<true>(value, dst) };
            }
        }
        if is_x86_feature_detected!("avx2") {
            unsafe { simd::avx2::format_string::<true>(value, dst) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { simd::sse2::format_string::<true>(value, dst) }
        } else {
            simd::v128::format_string::<true>(value, dst)
        }
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")))]
    {
        simd::v128::format_string::<true>(value, dst)
    }
}

#[cfg(feature = "html")]
pub mod html;
#[cfg(feature = "json")]
pub mod json;
