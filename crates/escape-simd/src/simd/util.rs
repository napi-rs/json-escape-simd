#[cfg(feature = "html")]
use crate::{HTML_ESCAPE_TAB, HTML_NEED_ESCAPED};
#[cfg(feature = "json")]
use crate::{NEED_ESCAPED, QUOTE_TAB};

/// Nibble tables for the simdjson-style two-shuffle HTML classifier. The 6
/// escaped bytes (`"` 0x22, `&` 0x26, `'` 0x27, `/` 0x2f, `<` 0x3c, `>` 0x3e)
/// all have distinct low nibbles (2, 6, 7, f, c, e) and high nibble 2 or 3, so
/// each owns one bit in the lo table while the hi table holds the per-high-
/// nibble union of those bits. A byte needs escaping iff
/// `HTML_LO_NIBBLE_TAB[b & 0x0f] & HTML_HI_NIBBLE_TAB[b >> 4] != 0` — exact, no
/// false positives.
#[cfg(all(
    feature = "html",
    any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86")
))]
pub(crate) const HTML_LO_NIBBLE_TAB: [u8; 16] = [0, 0, 1, 0, 0, 0, 2, 4, 0, 0, 0, 0, 16, 0, 32, 8];
#[cfg(all(
    feature = "html",
    any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86")
))]
pub(crate) const HTML_HI_NIBBLE_TAB: [u8; 16] = [0, 0, 15, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

#[inline(always)]
unsafe fn escape_with_tab(
    tab: &[(u8, [u8; 8]); 256],
    need_escaped: &[u8; 256],
    src: &mut *const u8,
    nb: &mut usize,
    dst: &mut *mut u8,
) {
    debug_assert!(*nb >= 1);
    loop {
        let ch = unsafe { *(*src) };
        let cnt = tab[ch as usize].0 as usize;
        debug_assert!(
            cnt != 0,
            "char is {}, cnt is {},  NEED_ESCAPED is {}",
            ch as char,
            cnt,
            need_escaped[ch as usize]
        );
        unsafe { std::ptr::copy_nonoverlapping(tab[ch as usize].1.as_ptr(), *dst, 8) };
        unsafe { (*dst) = (*dst).add(cnt) };
        unsafe { (*src) = (*src).add(1) };
        (*nb) -= 1;
        if (*nb) == 0 || unsafe { need_escaped[*(*src) as usize] == 0 } {
            return;
        }
    }
}

#[inline(always)]
pub(crate) unsafe fn escape_unchecked<const HTML: bool>(
    src: &mut *const u8,
    nb: &mut usize,
    dst: &mut *mut u8,
) {
    #[cfg(feature = "html")]
    if HTML {
        unsafe { escape_with_tab(&HTML_ESCAPE_TAB, &HTML_NEED_ESCAPED, src, nb, dst) };
        return;
    }
    #[cfg(feature = "json")]
    unsafe {
        escape_with_tab(&QUOTE_TAB, &NEED_ESCAPED, src, nb, dst)
    };
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[inline(always)]
pub(crate) fn check_cross_page(ptr: *const u8, step: usize) -> bool {
    let page_size = 4096;
    ((ptr as usize & (page_size - 1)) + step) > page_size
}
