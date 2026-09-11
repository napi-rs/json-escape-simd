use crate::format_html_string;

/// The input is too large to HTML-escape: the required scratch size
/// (`len * 6 + 32`) overflows `usize`, which is only possible for gigantic
/// inputs on 32-bit targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputTooLarge;

impl std::fmt::Display for InputTooLarge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("input is too large to HTML-escape")
    }
}

impl std::error::Error for InputTooLarge {}

#[inline]
pub(crate) fn html_capacity(len: usize) -> Result<usize, InputTooLarge> {
    len.checked_mul(6)
        .and_then(|n| n.checked_add(32))
        .ok_or(InputTooLarge)
}

/// Escapes the 6 HTML-sensitive characters in `value` (`"` `&` `'` `/` `<`
/// `>`), byte-identical to `@napi-rs/escape` / `v_htmlescape`. Unlike JSON
/// escaping, no surrounding quotes are added.
///
/// # Errors
///
/// Returns [`InputTooLarge`] if the scratch size (`len * 6 + 32`) overflows
/// `usize`.
pub fn escape_html(value: &str) -> Result<String, InputTooLarge> {
    let capacity = html_capacity(value.len())?;
    let mut buf = Vec::with_capacity(capacity);
    #[allow(clippy::uninit_vec)]
    unsafe {
        buf.set_len(capacity)
    };
    let cnt = format_html_string(value, &mut buf);
    unsafe { buf.set_len(cnt) };
    unsafe { Ok(String::from_utf8_unchecked(buf)) }
}

/// Escapes `value` for HTML (no surrounding `"`) and appends the result to
/// `dst`, growing `dst` as needed.
///
/// # Errors
///
/// Returns [`InputTooLarge`] if the scratch size (`len * 6 + 32`) overflows
/// `usize`. `dst` is left untouched in that case.
pub fn escape_html_into<S: AsRef<str>>(value: S, dst: &mut Vec<u8>) -> Result<(), InputTooLarge> {
    let value = value.as_ref();

    // Same scratch-space argument as JSON `escape_into` minus the quotes: the
    // kernels perform full-register speculative stores and copy 8 bytes per
    // escape, so they need up to `len * 6 + 32` scratch bytes past the current
    // end regardless of the final output length.
    let spare = html_capacity(value.len())?;
    dst.reserve(spare);
    let old_len = dst.len();

    // SAFETY: the `reserve` above guarantees `dst.capacity() - old_len` is at
    // least `value.len() * 6 + 32`, which upper-bounds every store the HTML
    // kernel performs. It writes valid UTF-8 and returns the number of bytes
    // written, which we then commit as the new length.
    unsafe {
        let spare =
            std::slice::from_raw_parts_mut(dst.as_mut_ptr().add(old_len), dst.capacity() - old_len);
        let cnt = format_html_string(value, spare);
        dst.set_len(old_len + cnt);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HTML_ESCAPE_TAB, HTML_NEED_ESCAPED};

    #[test]
    fn test_html_capacity_overflow() {
        assert_eq!(html_capacity(usize::MAX), Err(InputTooLarge));
        assert_eq!(html_capacity(usize::MAX / 6), Err(InputTooLarge));
        assert_eq!(html_capacity(10), Ok(92));
        assert_eq!(html_capacity(0), Ok(32));
    }

    #[test]
    fn test_html_escape_tables() {
        let expected: [(u8, &str); 6] = [
            (b'"', "&quot;"),
            (b'&', "&amp;"),
            (b'\'', "&#x27;"),
            (b'/', "&#x2f;"),
            (b'<', "&lt;"),
            (b'>', "&gt;"),
        ];
        for i in 0..256usize {
            match expected.iter().find(|&&(ch, _)| ch as usize == i) {
                Some(&(_, esc)) => {
                    let (len, bytes) = &HTML_ESCAPE_TAB[i];
                    assert_eq!(*len as usize, esc.len(), "len for byte 0x{i:02x}");
                    assert_eq!(&bytes[..esc.len()], esc.as_bytes(), "bytes for 0x{i:02x}");
                    assert!(
                        bytes[esc.len()..].iter().all(|&b| b == 0),
                        "padding for byte 0x{i:02x}"
                    );
                    assert_eq!(HTML_NEED_ESCAPED[i], 1, "need_escaped for byte 0x{i:02x}");
                }
                None => {
                    assert_eq!(HTML_ESCAPE_TAB[i], (0, [0; 8]), "entry for byte 0x{i:02x}");
                    assert_eq!(HTML_NEED_ESCAPED[i], 0, "need_escaped for byte 0x{i:02x}");
                }
            }
        }
    }
}
