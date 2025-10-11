#[inline]
pub fn escape_generic<S: AsRef<str>>(s: S) -> String {
    let s = s.as_ref();
    let bytes = s.as_bytes();
    // Estimate capacity - most strings don't need much escaping
    // Add some padding for potential escapes
    let estimated_capacity = bytes.len() + bytes.len() / 2 + 2;
    let mut result = Vec::with_capacity(estimated_capacity);
    result.push(b'"');
    escape_inner(bytes, &mut result);
    result.push(b'"');
    // SAFETY: We only pushed valid UTF-8 bytes (original string bytes and ASCII escape sequences)
    unsafe { String::from_utf8_unchecked(result) }
}

#[inline]
pub fn escape_into_generic<S: AsRef<str>>(s: S, output: &mut Vec<u8>) {
    let s = s.as_ref();
    let bytes = s.as_bytes();
    output.push(b'"');
    escape_inner(bytes, output);
    output.push(b'"');
}

#[inline]
// Slightly modified version of
// <https://github.com/serde-rs/json/blob/d12e943590208da738c092db92c34b39796a2538/src/ser.rs#L2079>
// Borrowed from:
// <https://github.com/oxc-project/oxc-sourcemap/blob/e533e6ca4d08c538d8d4df74eacd29437851591f/src/encode.rs#L331>
pub(crate) fn escape_inner(bytes: &[u8], result: &mut Vec<u8>) {
    let mut start = 0;
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        // Use lookup table to check if escaping is needed
        let escape_byte = ESCAPE[b as usize];

        if escape_byte == 0 {
            // No escape needed, continue scanning
            i += 1;
            continue;
        }

        // Copy any unescaped bytes before this position
        if start < i {
            result.extend_from_slice(&bytes[start..i]);
        }

        // Handle the escape
        result.push(b'\\');
        if escape_byte == b'u' {
            // Unicode escape for control characters
            result.extend_from_slice(b"u00");
            let hex_digits = &HEX_BYTES[b as usize];
            result.push(hex_digits.0);
            result.push(hex_digits.1);
        } else {
            // Simple escape
            result.push(escape_byte);
        }

        i += 1;
        start = i;
    }

    // Copy any remaining unescaped bytes
    if start < bytes.len() {
        result.extend_from_slice(&bytes[start..]);
    }
}

const BB: u8 = b'b'; // \x08
const TT: u8 = b't'; // \x09
const NN: u8 = b'n'; // \x0A
const FF: u8 = b'f'; // \x0C
const RR: u8 = b'r'; // \x0D
const QU: u8 = b'"'; // \x22
const BS: u8 = b'\\'; // \x5C
pub(crate) const UU: u8 = b'u'; // \x00...\x1F except the ones above
const __: u8 = 0;

// Lookup table of escape sequences. A value of b'x' at index i means that byte
// i is escaped as "\x" in JSON. A value of 0 means that byte i is not escaped.
pub(crate) static ESCAPE: [u8; 256] = [
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    UU, UU, UU, UU, UU, UU, UU, UU, BB, TT, NN, UU, FF, RR, UU, UU, // 0
    UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, // 1
    __, __, QU, __, __, __, __, __, __, __, __, __, __, __, __, __, // 2
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 3
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 4
    __, __, __, __, __, __, __, __, __, __, __, __, BS, __, __, __, // 5
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 6
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 7
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 8
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 9
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // A
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // B
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // C
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // D
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // E
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // F
];

// Pre-computed hex digit pairs for control characters
pub(crate) struct HexPair(pub(crate) u8, pub(crate) u8);

pub(crate) static HEX_BYTES: [HexPair; 32] = [
    HexPair(b'0', b'0'),
    HexPair(b'0', b'1'),
    HexPair(b'0', b'2'),
    HexPair(b'0', b'3'),
    HexPair(b'0', b'4'),
    HexPair(b'0', b'5'),
    HexPair(b'0', b'6'),
    HexPair(b'0', b'7'),
    HexPair(b'0', b'8'),
    HexPair(b'0', b'9'),
    HexPair(b'0', b'a'),
    HexPair(b'0', b'b'),
    HexPair(b'0', b'c'),
    HexPair(b'0', b'd'),
    HexPair(b'0', b'e'),
    HexPair(b'0', b'f'),
    HexPair(b'1', b'0'),
    HexPair(b'1', b'1'),
    HexPair(b'1', b'2'),
    HexPair(b'1', b'3'),
    HexPair(b'1', b'4'),
    HexPair(b'1', b'5'),
    HexPair(b'1', b'6'),
    HexPair(b'1', b'7'),
    HexPair(b'1', b'8'),
    HexPair(b'1', b'9'),
    HexPair(b'1', b'a'),
    HexPair(b'1', b'b'),
    HexPair(b'1', b'c'),
    HexPair(b'1', b'd'),
    HexPair(b'1', b'e'),
    HexPair(b'1', b'f'),
];

// Optimized escape table with 8-byte arrays for fast bulk writing
// First element is the length of escape sequence, followed by the escape bytes
pub(crate) static ESCAPE_TABLE: [(u8, [u8; 8]); 256] = {
    let mut table = [(0u8, [0u8; 8]); 256];

    // Control characters \u0000 - \u001f
    table[0x00] = (6, *b"\\u0000\0\0");
    table[0x01] = (6, *b"\\u0001\0\0");
    table[0x02] = (6, *b"\\u0002\0\0");
    table[0x03] = (6, *b"\\u0003\0\0");
    table[0x04] = (6, *b"\\u0004\0\0");
    table[0x05] = (6, *b"\\u0005\0\0");
    table[0x06] = (6, *b"\\u0006\0\0");
    table[0x07] = (6, *b"\\u0007\0\0");
    table[0x08] = (2, *b"\\b\0\0\0\0\0\0");
    table[0x09] = (2, *b"\\t\0\0\0\0\0\0");
    table[0x0A] = (2, *b"\\n\0\0\0\0\0\0");
    table[0x0B] = (6, *b"\\u000b\0\0");
    table[0x0C] = (2, *b"\\f\0\0\0\0\0\0");
    table[0x0D] = (2, *b"\\r\0\0\0\0\0\0");
    table[0x0E] = (6, *b"\\u000e\0\0");
    table[0x0F] = (6, *b"\\u000f\0\0");
    table[0x10] = (6, *b"\\u0010\0\0");
    table[0x11] = (6, *b"\\u0011\0\0");
    table[0x12] = (6, *b"\\u0012\0\0");
    table[0x13] = (6, *b"\\u0013\0\0");
    table[0x14] = (6, *b"\\u0014\0\0");
    table[0x15] = (6, *b"\\u0015\0\0");
    table[0x16] = (6, *b"\\u0016\0\0");
    table[0x17] = (6, *b"\\u0017\0\0");
    table[0x18] = (6, *b"\\u0018\0\0");
    table[0x19] = (6, *b"\\u0019\0\0");
    table[0x1A] = (6, *b"\\u001a\0\0");
    table[0x1B] = (6, *b"\\u001b\0\0");
    table[0x1C] = (6, *b"\\u001c\0\0");
    table[0x1D] = (6, *b"\\u001d\0\0");
    table[0x1E] = (6, *b"\\u001e\0\0");
    table[0x1F] = (6, *b"\\u001f\0\0");

    // Special characters
    table[0x22] = (2, *b"\\\"\0\0\0\0\0\0"); // "
    table[0x5C] = (2, *b"\\\\\0\0\0\0\0\0"); // \

    table
};
