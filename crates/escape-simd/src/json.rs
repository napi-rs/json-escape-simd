use crate::format_json_string;

/// Escapes `value` as a JSON string, including the surrounding `"` quotes.
pub fn escape(value: &str) -> String {
    let capacity = value.len() * 6 + 32 + 3;
    let mut buf = Vec::with_capacity(capacity);
    #[allow(clippy::uninit_vec)]
    unsafe {
        buf.set_len(capacity)
    };
    let cnt = format_json_string(value, &mut buf);
    unsafe { buf.set_len(cnt) };
    unsafe { String::from_utf8_unchecked(buf) }
}

/// Escapes `value` (including the surrounding `"`) and appends the result to
/// `dst`, growing `dst` as needed.
pub fn escape_into<S: AsRef<str>>(value: S, dst: &mut Vec<u8>) {
    let value = value.as_ref();

    // The SIMD kernels perform full-register speculative stores and copy 8 bytes
    // per escape, so they need up to `len * 6 + 32 + 3` scratch bytes past the
    // current end regardless of the final output length. Reserve that up front
    // so the unchecked writes below can never exceed the allocation. `reserve`
    // is effectively free when the caller already sized `dst` large enough.
    dst.reserve(value.len() * 6 + 32 + 3);
    let old_len = dst.len();

    // SAFETY: the `reserve` above guarantees `dst.capacity() - old_len` is at
    // least `value.len() * 6 + 32 + 3`, which upper-bounds every store
    // `format_json_string` performs. It writes valid UTF-8 and returns the
    // number of bytes written, which we then commit as the new length.
    unsafe {
        let spare =
            std::slice::from_raw_parts_mut(dst.as_mut_ptr().add(old_len), dst.capacity() - old_len);
        let cnt = format_json_string(value, spare);
        dst.set_len(old_len + cnt);
    }
}
