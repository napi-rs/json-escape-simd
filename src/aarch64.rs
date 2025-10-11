use std::arch::aarch64::{
    vceqq_u8, vdupq_n_u8, vld1q_u8_x4, vmaxvq_u8, vorrq_u8, vqtbl4q_u8, vst1q_u8,
};

use crate::generic::{ESCAPE, ESCAPE_TABLE, HEX_BYTES, UU};
use crate::util::check_cross_page;

const CHUNK: usize = 64;
// 128 bytes ahead
const PREFETCH_DISTANCE: usize = CHUNK * 2;
const SLASH_SENTINEL: u8 = 0xFF;

#[inline]
pub fn escape_neon(bytes: &[u8], output: &mut Vec<u8>) {
    let n = bytes.len();

    unsafe {
        let tbl = vld1q_u8_x4(ESCAPE.as_ptr());
        let slash = vdupq_n_u8(b'\\');
        let mut i = 0usize;

        // Scratch buffer reused for mask materialisation; stay uninitialised.
        #[allow(invalid_value)]
        let mut placeholder: [u8; 16] = core::mem::MaybeUninit::uninit().assume_init();

        while i + CHUNK <= n {
            let ptr = bytes.as_ptr().add(i);

            // Only prefetch if we won't go past the end
            if i + CHUNK + PREFETCH_DISTANCE <= n {
                core::arch::asm!(
                    "prfm pldl1keep, [{0}]",
                    in(reg) ptr.add(PREFETCH_DISTANCE),
                );
            }

            // Use temporary buffer if reading would cross page boundary
            let quad = if i + CHUNK == n || !check_cross_page(ptr, CHUNK) {
                // Safe to read directly
                vld1q_u8_x4(ptr)
            } else {
                // Need to use temporary buffer
                let mut temp = [0u8; CHUNK];
                std::ptr::copy_nonoverlapping(ptr, temp.as_mut_ptr(), CHUNK);
                vld1q_u8_x4(temp.as_ptr())
            };

            let a = quad.0;
            let b = quad.1;
            let c = quad.2;
            let d = quad.3;

            let mask_1 = vorrq_u8(vqtbl4q_u8(tbl, a), vceqq_u8(slash, a));
            let mask_2 = vorrq_u8(vqtbl4q_u8(tbl, b), vceqq_u8(slash, b));
            let mask_3 = vorrq_u8(vqtbl4q_u8(tbl, c), vceqq_u8(slash, c));
            let mask_4 = vorrq_u8(vqtbl4q_u8(tbl, d), vceqq_u8(slash, d));

            let mask_r_1 = vmaxvq_u8(mask_1);
            let mask_r_2 = vmaxvq_u8(mask_2);
            let mask_r_3 = vmaxvq_u8(mask_3);
            let mask_r_4 = vmaxvq_u8(mask_4);

            if mask_r_1 | mask_r_2 | mask_r_3 | mask_r_4 == 0 {
                output.extend_from_slice(std::slice::from_raw_parts(ptr, CHUNK));
                i += CHUNK;
                continue;
            }

            // Process each 16-byte chunk that has escapes
            if mask_r_1 != 0 {
                vst1q_u8(placeholder.as_mut_ptr(), mask_1);
                handle_block(&bytes[i..i + 16], &placeholder, output);
            } else {
                output.extend_from_slice(std::slice::from_raw_parts(ptr, 16));
            }

            if mask_r_2 != 0 {
                vst1q_u8(placeholder.as_mut_ptr(), mask_2);
                handle_block(&bytes[i + 16..i + 32], &placeholder, output);
            } else {
                output.extend_from_slice(std::slice::from_raw_parts(ptr.add(16), 16));
            }

            if mask_r_3 != 0 {
                vst1q_u8(placeholder.as_mut_ptr(), mask_3);
                handle_block(&bytes[i + 32..i + 48], &placeholder, output);
            } else {
                output.extend_from_slice(std::slice::from_raw_parts(ptr.add(32), 16));
            }

            if mask_r_4 != 0 {
                vst1q_u8(placeholder.as_mut_ptr(), mask_4);
                handle_block(&bytes[i + 48..i + 64], &placeholder, output);
            } else {
                output.extend_from_slice(std::slice::from_raw_parts(ptr.add(48), 16));
            }

            i += CHUNK;
        }

        if i < n {
            handle_tail(&bytes[i..], output);
        }
    }
}

#[inline(always)]
fn handle_tail(src: &[u8], dst: &mut Vec<u8>) {
    unsafe {
        let mut dst_ptr = dst.as_mut_ptr().add(dst.len());
        let dst_start = dst_ptr;
        let mut i = 0;

        while i < src.len() {
            let c = src[i];
            let escape_byte = ESCAPE[c as usize];

            if escape_byte == 0 {
                // No escape needed
                *dst_ptr = c;
                dst_ptr = dst_ptr.add(1);
                i += 1;
            } else {
                // Handle continuous escapes
                let consumed = escape_continuous(src, &mut dst_ptr, i);
                i += consumed;
            }
        }

        let bytes_written = dst_ptr as usize - dst_start as usize;
        dst.set_len(dst.len() + bytes_written);
    }
}

/// Process continuous escaped characters efficiently
/// Returns the number of source bytes consumed
#[inline(always)]
unsafe fn escape_continuous(src: &[u8], dst: &mut *mut u8, start_idx: usize) -> usize {
    let mut i = start_idx;

    while i < src.len() {
        let c = src[i];
        let escape_byte = ESCAPE[c as usize];

        if escape_byte == 0 {
            break;
        }

        let (len, escape_bytes) = ESCAPE_TABLE[c as usize];

        unsafe {
            if len > 0 {
                // Copy 8 bytes at once (actual escape + padding)
                std::ptr::copy_nonoverlapping(escape_bytes.as_ptr(), *dst, 8);
                *dst = dst.add(len as usize);
            } else {
                // Rare fallback for characters not in table
                **dst = b'\\';
                *dst = dst.add(1);
                if escape_byte == UU {
                    std::ptr::copy_nonoverlapping(b"u00".as_ptr(), *dst, 3);
                    *dst = dst.add(3);
                    let hex = &HEX_BYTES[c as usize];
                    **dst = hex.0;
                    *dst = dst.add(1);
                    **dst = hex.1;
                    *dst = dst.add(1);
                } else {
                    **dst = escape_byte;
                    *dst = dst.add(1);
                }
            }
        }

        i += 1;

        // Check if next character also needs escaping
        if i < src.len() && ESCAPE[src[i] as usize] == 0 {
            break;
        }
    }

    i - start_idx
}

#[inline(always)]
fn handle_block(src: &[u8], mask: &[u8; 16], dst: &mut Vec<u8>) {
    unsafe {
        let mut dst_ptr = dst.as_mut_ptr().add(dst.len());
        let dst_start = dst_ptr;
        let mut j = 0;

        while j < 16 {
            let m = mask[j];
            let c = src[j];

            if m == 0 {
                // No escape needed, copy directly
                *dst_ptr = c;
                dst_ptr = dst_ptr.add(1);
                j += 1;
            } else if m == SLASH_SENTINEL {
                // Backslash escape
                std::ptr::copy_nonoverlapping(b"\\\\".as_ptr(), dst_ptr, 2);
                dst_ptr = dst_ptr.add(2);
                j += 1;
            } else {
                // Handle continuous escapes
                let consumed = escape_continuous(src, &mut dst_ptr, j);
                j += consumed;
            }
        }

        let bytes_written = dst_ptr as usize - dst_start as usize;
        dst.set_len(dst.len() + bytes_written);
    }
}
