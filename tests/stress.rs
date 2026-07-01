//! Differential + buffer-safety stress test.
//!
//! Regression guard for the SIMD kernels' tail handling. Escapes strings of
//! every length across many escape densities and compares against `serde_json`.
//! This exercises every `len % LANES` tail remainder, the 64/256-byte chunk
//! boundaries, and worst-case 6x expansion (`\u00xx`) — exactly the paths that
//! must never write past the destination buffer.

use json_escape_simd::escape;

#[track_caller]
fn check(s: &str) {
    assert_eq!(
        escape(s),
        serde_json::to_string(s).unwrap(),
        "mismatch for input of len {}",
        s.len()
    );
}

#[test]
fn stress_all_lengths_and_densities() {
    // Fills spanning 6x expansion (control chars), 2x (\\ " \n) and clean.
    let fills: [char; 6] = ['\u{0}', '"', '\\', '\n', 'a', '\u{1f}'];
    for len in 0..=600usize {
        for &f in &fills {
            let s: String = std::iter::repeat_n(f, len).collect();
            check(&s);
        }
        if len > 0 {
            // single escape at the very end (tail-boundary trigger)
            let mut s = "a".repeat(len - 1);
            s.push('"');
            check(&s);
            // single escape at the very start
            let mut s2 = String::from("\"");
            s2.extend(std::iter::repeat_n('a', len - 1));
            check(&s2);
        }
    }
}

#[test]
fn escape_into_never_overflows_exact_capacity() {
    use json_escape_simd::escape_into;
    // `escape_into` must not write out of bounds even when the caller sizes the
    // destination to exactly the final output length (or less). Cover a range of
    // lengths and densities; grows are fine, overruns are UB.
    let fills: [char; 4] = ['\u{0}', '"', '\\', 'a'];
    for len in 0..=300usize {
        for &f in &fills {
            let input: String = std::iter::repeat_n(f, len).collect();
            let expected = serde_json::to_string(&input).unwrap();
            let mut dst = Vec::with_capacity(expected.len()); // exact, no slack
            escape_into(&input, &mut dst);
            assert_eq!(dst, expected.as_bytes(), "len {len} fill {f:?}");
        }
    }
}

#[test]
fn stress_multibyte_utf8_boundaries() {
    // Bytes >= 0x80 (UTF-8 lead/continuation) must pass through unescaped,
    // including right at chunk/lane boundaries.
    for len in 0..=200usize {
        let mut s = "a".repeat(len);
        s.push('中'); // 3-byte UTF-8
        s.push('"'); // followed by an escape
        s.push('𝄞'); // 4-byte UTF-8
        check(&s);
    }
}
