#![allow(unsafe_op_in_unsafe_fn)]

use std::arch::x86_64::{
    __m128i, __m256i, __m512i, _MM_HINT_T0, _mm_add_epi8, _mm_cmpeq_epi8, _mm_cmpgt_epi8,
    _mm_load_si128, _mm_loadu_si128, _mm_movemask_epi8, _mm_or_si128, _mm_prefetch, _mm_set1_epi8,
    _mm256_add_epi8, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8, _mm256_load_si256, _mm256_loadu_si256,
    _mm256_movemask_epi8, _mm256_or_si256, _mm256_set1_epi8, _mm512_cmpeq_epi8_mask,
    _mm512_cmplt_epu8_mask, _mm512_load_si512, _mm512_loadu_si512, _mm512_set1_epi8,
};

use crate::generic::{ESCAPE, ESCAPE_TABLE, HEX_BYTES, UU};

// Constants for control character detection using signed comparison trick
const TRANSLATION_A: i8 = i8::MAX - 31i8;
const BELOW_A: i8 = i8::MAX - (31i8 - 0i8) - 1;
const B: i8 = 34i8; // '"'
const C: i8 = 92i8; // '\\'

const M512_VECTOR_SIZE: usize = std::mem::size_of::<__m512i>();
const M256_VECTOR_SIZE: usize = std::mem::size_of::<__m256i>();
const M128_VECTOR_SIZE: usize = std::mem::size_of::<__m128i>();
pub(crate) const LOOP_SIZE_AVX2: usize = 4 * M256_VECTOR_SIZE; // Process 128 bytes at a time
pub(crate) const LOOP_SIZE_AVX512: usize = 4 * M512_VECTOR_SIZE; // Process 256 bytes at a time
const PREFETCH_DISTANCE_AVX2: usize = 256; // Prefetch 256 bytes ahead for AVX2
const PREFETCH_DISTANCE_AVX512: usize = 512; // Prefetch 512 bytes ahead for AVX512

#[inline(always)]
fn sub(a: *const u8, b: *const u8) -> usize {
    debug_assert!(b <= a);
    (a as usize) - (b as usize)
}

#[inline(always)]
fn check_cross_page(ptr: *const u8, step: usize) -> bool {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        // Check if reading 'step' bytes from 'ptr' would cross a page boundary
        // Page size is typically 4096 bytes on x86_64 Linux and macOS
        const PAGE_SIZE: usize = 4096;
        ((ptr as usize & (PAGE_SIZE - 1)) + step) > PAGE_SIZE
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        // On other platforms, always use the safe path with temporary buffer
        // to avoid potential page faults
        true
    }
}

#[target_feature(enable = "avx512f", enable = "avx512bw")]
#[inline]
pub unsafe fn escape_avx512(bytes: &[u8], result: &mut Vec<u8>) {
    let len = bytes.len();

    let start_ptr = bytes.as_ptr();
    let end_ptr = bytes[len..].as_ptr();
    let mut ptr = start_ptr;
    let mut start = 0;

    let v_b = _mm512_set1_epi8(B);
    let v_c = _mm512_set1_epi8(C);
    let v_ctrl_limit = _mm512_set1_epi8(0x20);

    // Handle alignment - skip if already aligned
    const M512_VECTOR_ALIGN: usize = M512_VECTOR_SIZE - 1;
    let misalignment = start_ptr as usize & M512_VECTOR_ALIGN;
    if misalignment != 0 {
        let align = M512_VECTOR_SIZE - misalignment;
        let a = _mm512_loadu_si512(ptr as *const __m512i);

        // Check for quotes, backslash, and control characters
        let quote_mask = _mm512_cmpeq_epi8_mask(a, v_b);
        let slash_mask = _mm512_cmpeq_epi8_mask(a, v_c);
        let ctrl_mask = _mm512_cmplt_epu8_mask(a, v_ctrl_limit);

        let mut mask = (quote_mask | slash_mask | ctrl_mask) as u64;
        if align < 64 {
            mask &= (1u64 << align) - 1;
        }

        if mask != 0 {
            let at = sub(ptr, start_ptr);
            while mask != 0 {
                let cur = mask.trailing_zeros() as usize;
                let c = *ptr.add(cur);
                let escape_byte = ESCAPE[c as usize];
                debug_assert!(escape_byte != 0);
                let i = at + cur;
                if start < i {
                    result.extend_from_slice(&bytes[start..i]);
                }
                write_escape(result, escape_byte, c);
                start = i + 1;
                mask &= mask - 1;
            }
        }
        ptr = ptr.add(align);
    }

    // Main loop processing 256 bytes at a time
    while ptr <= end_ptr.sub(LOOP_SIZE_AVX512) {
        debug_assert_eq!(0, (ptr as usize) % M512_VECTOR_SIZE);

        // Prefetch next iteration's data
        if ptr.add(LOOP_SIZE_AVX512 + PREFETCH_DISTANCE_AVX512) < end_ptr {
            _mm_prefetch(
                ptr.add(LOOP_SIZE_AVX512 + PREFETCH_DISTANCE_AVX512) as *const i8,
                _MM_HINT_T0,
            );
        }

        // Load all 4 vectors at once for better pipelining
        let a0 = _mm512_load_si512(ptr as *const __m512i);
        let a1 = _mm512_load_si512(ptr.add(M512_VECTOR_SIZE) as *const __m512i);
        let a2 = _mm512_load_si512(ptr.add(M512_VECTOR_SIZE * 2) as *const __m512i);
        let a3 = _mm512_load_si512(ptr.add(M512_VECTOR_SIZE * 3) as *const __m512i);

        // Check for quotes (") in all vectors
        let quote_0 = _mm512_cmpeq_epi8_mask(a0, v_b);
        let quote_1 = _mm512_cmpeq_epi8_mask(a1, v_b);
        let quote_2 = _mm512_cmpeq_epi8_mask(a2, v_b);
        let quote_3 = _mm512_cmpeq_epi8_mask(a3, v_b);

        // Check for backslash (\) in all vectors
        let slash_0 = _mm512_cmpeq_epi8_mask(a0, v_c);
        let slash_1 = _mm512_cmpeq_epi8_mask(a1, v_c);
        let slash_2 = _mm512_cmpeq_epi8_mask(a2, v_c);
        let slash_3 = _mm512_cmpeq_epi8_mask(a3, v_c);

        // Check for control characters (< 0x20) in all vectors
        let ctrl_0 = _mm512_cmplt_epu8_mask(a0, v_ctrl_limit);
        let ctrl_1 = _mm512_cmplt_epu8_mask(a1, v_ctrl_limit);
        let ctrl_2 = _mm512_cmplt_epu8_mask(a2, v_ctrl_limit);
        let ctrl_3 = _mm512_cmplt_epu8_mask(a3, v_ctrl_limit);

        // Combine all masks
        let mask_a = quote_0 | slash_0 | ctrl_0;
        let mask_b = quote_1 | slash_1 | ctrl_1;
        let mask_c = quote_2 | slash_2 | ctrl_2;
        let mask_d = quote_3 | slash_3 | ctrl_3;

        // Fast path: check if any escaping needed
        let any_escape = mask_a | mask_b | mask_c | mask_d;

        if any_escape == 0 {
            // No escapes needed, copy whole chunk
            let at = sub(ptr, start_ptr);
            if start < at {
                result.extend_from_slice(&bytes[start..at]);
            }
            result.extend_from_slice(std::slice::from_raw_parts(ptr, LOOP_SIZE_AVX512));
            start = at + LOOP_SIZE_AVX512;
        } else {
            // Process each 64-byte chunk that has escapes
            if mask_a != 0 {
                process_mask_avx512(ptr, start_ptr, result, &mut start, bytes, mask_a, 0);
            }
            if mask_b != 0 {
                process_mask_avx512(
                    ptr,
                    start_ptr,
                    result,
                    &mut start,
                    bytes,
                    mask_b,
                    M512_VECTOR_SIZE,
                );
            }
            if mask_c != 0 {
                process_mask_avx512(
                    ptr,
                    start_ptr,
                    result,
                    &mut start,
                    bytes,
                    mask_c,
                    M512_VECTOR_SIZE * 2,
                );
            }
            if mask_d != 0 {
                process_mask_avx512(
                    ptr,
                    start_ptr,
                    result,
                    &mut start,
                    bytes,
                    mask_d,
                    M512_VECTOR_SIZE * 3,
                );
            }
        }

        ptr = ptr.add(LOOP_SIZE_AVX512);
    }

    // Process remaining aligned chunks
    while ptr <= end_ptr.sub(M512_VECTOR_SIZE) {
        debug_assert_eq!(0, (ptr as usize) % M512_VECTOR_SIZE);
        let a = _mm512_load_si512(ptr as *const __m512i);

        let quote_mask = _mm512_cmpeq_epi8_mask(a, v_b);
        let slash_mask = _mm512_cmpeq_epi8_mask(a, v_c);
        let ctrl_mask = _mm512_cmplt_epu8_mask(a, v_ctrl_limit);

        let mut mask = (quote_mask | slash_mask | ctrl_mask) as u64;

        if mask != 0 {
            let at = sub(ptr, start_ptr);
            while mask != 0 {
                let cur = mask.trailing_zeros() as usize;
                let c = *ptr.add(cur);
                let escape_byte = ESCAPE[c as usize];
                debug_assert!(escape_byte != 0);
                let i = at + cur;
                if start < i {
                    result.extend_from_slice(&bytes[start..i]);
                }
                write_escape(result, escape_byte, c);
                start = i + 1;
                mask &= mask - 1;
            }
        }
        ptr = ptr.add(M512_VECTOR_SIZE);
    }

    // Handle tail
    if ptr < end_ptr {
        let remaining = sub(end_ptr, ptr);
        let d = M512_VECTOR_SIZE - remaining;

        // Use temporary buffer if reading would cross page boundary
        let a = if check_cross_page(ptr.sub(d), M512_VECTOR_SIZE) {
            let mut temp = [0u8; M512_VECTOR_SIZE];
            // Copy remaining bytes to the beginning of temp buffer
            std::ptr::copy_nonoverlapping(ptr, temp.as_mut_ptr(), remaining);
            _mm512_loadu_si512(temp.as_ptr() as *const __m512i)
        } else {
            _mm512_loadu_si512(ptr.sub(d) as *const __m512i)
        };

        let quote_mask = _mm512_cmpeq_epi8_mask(a, v_b);
        let slash_mask = _mm512_cmpeq_epi8_mask(a, v_c);
        let ctrl_mask = _mm512_cmplt_epu8_mask(a, v_ctrl_limit);

        let mut mask = if check_cross_page(ptr.sub(d), M512_VECTOR_SIZE) {
            // When using temp buffer, only check the valid bytes
            (quote_mask | slash_mask | ctrl_mask) as u64 & ((1u64 << remaining) - 1)
        } else {
            ((quote_mask | slash_mask | ctrl_mask) as u64).wrapping_shr(d as u32)
        };

        if mask != 0 {
            let at = sub(ptr, start_ptr);
            while mask != 0 {
                let cur = mask.trailing_zeros() as usize;
                let c = *ptr.add(cur);
                let escape_byte = ESCAPE[c as usize];
                debug_assert!(escape_byte != 0);
                let i = at + cur;
                if start < i {
                    result.extend_from_slice(&bytes[start..i]);
                }
                write_escape(result, escape_byte, c);
                start = i + 1;
                mask &= mask - 1;
            }
        }
    }

    // Copy any remaining bytes
    if start < len {
        result.extend_from_slice(&bytes[start..]);
    }
}

#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn escape_avx2(bytes: &[u8], result: &mut Vec<u8>) {
    let len = bytes.len();

    let start_ptr = bytes.as_ptr();
    let end_ptr = bytes[len..].as_ptr();
    let mut ptr = start_ptr;
    let mut start = 0;

    let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
    let v_below_a = _mm256_set1_epi8(BELOW_A);
    let v_b = _mm256_set1_epi8(B);
    let v_c = _mm256_set1_epi8(C);

    // Handle alignment - skip if already aligned
    const M256_VECTOR_ALIGN: usize = M256_VECTOR_SIZE - 1;
    let misalignment = start_ptr as usize & M256_VECTOR_ALIGN;
    if misalignment != 0 {
        let align = M256_VECTOR_SIZE - misalignment;
        let mut mask = {
            let a = _mm256_loadu_si256(ptr as *const __m256i);
            _mm256_movemask_epi8(_mm256_or_si256(
                _mm256_or_si256(_mm256_cmpeq_epi8(a, v_b), _mm256_cmpeq_epi8(a, v_c)),
                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_a), v_below_a),
            ))
        };

        if mask != 0 {
            let at = sub(ptr, start_ptr);
            let mut cur = mask.trailing_zeros() as usize;
            while cur < align {
                let c = *ptr.add(cur);
                let escape_byte = ESCAPE[c as usize];
                if escape_byte != 0 {
                    let i = at + cur;
                    if start < i {
                        result.extend_from_slice(&bytes[start..i]);
                    }
                    write_escape(result, escape_byte, c);
                    start = i + 1;
                }
                mask ^= 1 << cur;
                if mask == 0 {
                    break;
                }
                cur = mask.trailing_zeros() as usize;
            }
        }
        ptr = ptr.add(align);
    }

    // Main loop processing 128 bytes at a time
    while ptr <= end_ptr.sub(LOOP_SIZE_AVX2) {
        debug_assert_eq!(0, (ptr as usize) % M256_VECTOR_SIZE);

        // Prefetch next iteration's data
        if ptr.add(LOOP_SIZE_AVX2 + PREFETCH_DISTANCE_AVX2) < end_ptr {
            _mm_prefetch(
                ptr.add(LOOP_SIZE_AVX2 + PREFETCH_DISTANCE_AVX2) as *const i8,
                _MM_HINT_T0,
            );
        }

        // Load all 4 vectors at once for better pipelining
        let a0 = _mm256_load_si256(ptr as *const __m256i);
        let a1 = _mm256_load_si256(ptr.add(M256_VECTOR_SIZE) as *const __m256i);
        let a2 = _mm256_load_si256(ptr.add(M256_VECTOR_SIZE * 2) as *const __m256i);
        let a3 = _mm256_load_si256(ptr.add(M256_VECTOR_SIZE * 3) as *const __m256i);

        // Combined mask computation - all escape conditions in one operation
        // This reduces instruction count and improves pipelining
        let cmp_a = _mm256_or_si256(
            _mm256_or_si256(_mm256_cmpeq_epi8(a0, v_b), _mm256_cmpeq_epi8(a0, v_c)),
            _mm256_cmpgt_epi8(_mm256_add_epi8(a0, v_translation_a), v_below_a),
        );
        let cmp_b = _mm256_or_si256(
            _mm256_or_si256(_mm256_cmpeq_epi8(a1, v_b), _mm256_cmpeq_epi8(a1, v_c)),
            _mm256_cmpgt_epi8(_mm256_add_epi8(a1, v_translation_a), v_below_a),
        );
        let cmp_c = _mm256_or_si256(
            _mm256_or_si256(_mm256_cmpeq_epi8(a2, v_b), _mm256_cmpeq_epi8(a2, v_c)),
            _mm256_cmpgt_epi8(_mm256_add_epi8(a2, v_translation_a), v_below_a),
        );
        let cmp_d = _mm256_or_si256(
            _mm256_or_si256(_mm256_cmpeq_epi8(a3, v_b), _mm256_cmpeq_epi8(a3, v_c)),
            _mm256_cmpgt_epi8(_mm256_add_epi8(a3, v_translation_a), v_below_a),
        );

        // Fast path: check if any escaping needed
        let any_escape =
            _mm256_or_si256(_mm256_or_si256(cmp_a, cmp_b), _mm256_or_si256(cmp_c, cmp_d));

        if _mm256_movemask_epi8(any_escape) == 0 {
            // No escapes needed, copy whole chunk
            let at = sub(ptr, start_ptr);
            if start < at {
                result.extend_from_slice(&bytes[start..at]);
            }
            result.extend_from_slice(std::slice::from_raw_parts(ptr, LOOP_SIZE_AVX2));
            start = at + LOOP_SIZE_AVX2;
        } else {
            // Get individual masks only when needed
            let mask_a = _mm256_movemask_epi8(cmp_a);
            let mask_b = _mm256_movemask_epi8(cmp_b);
            let mask_c = _mm256_movemask_epi8(cmp_c);
            let mask_d = _mm256_movemask_epi8(cmp_d);

            // Process each 32-byte chunk that has escapes
            if mask_a != 0 {
                process_mask_avx(ptr, start_ptr, result, &mut start, bytes, mask_a, 0);
            }
            if mask_b != 0 {
                process_mask_avx(
                    ptr,
                    start_ptr,
                    result,
                    &mut start,
                    bytes,
                    mask_b,
                    M256_VECTOR_SIZE,
                );
            }
            if mask_c != 0 {
                process_mask_avx(
                    ptr,
                    start_ptr,
                    result,
                    &mut start,
                    bytes,
                    mask_c,
                    M256_VECTOR_SIZE * 2,
                );
            }
            if mask_d != 0 {
                process_mask_avx(
                    ptr,
                    start_ptr,
                    result,
                    &mut start,
                    bytes,
                    mask_d,
                    M256_VECTOR_SIZE * 3,
                );
            }
        }

        ptr = ptr.add(LOOP_SIZE_AVX2);
    }

    // Process remaining aligned chunks
    while ptr <= end_ptr.sub(M256_VECTOR_SIZE) {
        debug_assert_eq!(0, (ptr as usize) % M256_VECTOR_SIZE);
        let mut mask = {
            let a = _mm256_load_si256(ptr as *const __m256i);
            _mm256_movemask_epi8(_mm256_or_si256(
                _mm256_or_si256(_mm256_cmpeq_epi8(a, v_b), _mm256_cmpeq_epi8(a, v_c)),
                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_a), v_below_a),
            ))
        };

        if mask != 0 {
            let at = sub(ptr, start_ptr);
            let mut cur = mask.trailing_zeros() as usize;
            loop {
                let c = *ptr.add(cur);
                let escape_byte = ESCAPE[c as usize];
                if escape_byte != 0 {
                    let i = at + cur;
                    if start < i {
                        result.extend_from_slice(&bytes[start..i]);
                    }
                    write_escape(result, escape_byte, c);
                    start = i + 1;
                }
                mask ^= 1 << cur;
                if mask == 0 {
                    break;
                }
                cur = mask.trailing_zeros() as usize;
            }
        }
        ptr = ptr.add(M256_VECTOR_SIZE);
    }

    // Handle tail
    if ptr < end_ptr {
        let remaining = sub(end_ptr, ptr);
        let d = M256_VECTOR_SIZE - remaining;

        // Use temporary buffer if reading would cross page boundary
        let a = if check_cross_page(ptr.sub(d), M256_VECTOR_SIZE) {
            let mut temp = [0u8; M256_VECTOR_SIZE];
            // Copy remaining bytes to the beginning of temp buffer
            std::ptr::copy_nonoverlapping(ptr, temp.as_mut_ptr(), remaining);
            _mm256_loadu_si256(temp.as_ptr() as *const __m256i)
        } else {
            _mm256_loadu_si256(ptr.sub(d) as *const __m256i)
        };

        let mut mask = if check_cross_page(ptr.sub(d), M256_VECTOR_SIZE) {
            // When using temp buffer, only check the valid bytes
            (_mm256_movemask_epi8(_mm256_or_si256(
                _mm256_or_si256(_mm256_cmpeq_epi8(a, v_b), _mm256_cmpeq_epi8(a, v_c)),
                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_a), v_below_a),
            )) as u32) & ((1u32 << remaining) - 1)
        } else {
            (_mm256_movemask_epi8(_mm256_or_si256(
                _mm256_or_si256(_mm256_cmpeq_epi8(a, v_b), _mm256_cmpeq_epi8(a, v_c)),
                _mm256_cmpgt_epi8(_mm256_add_epi8(a, v_translation_a), v_below_a),
            )) as u32).wrapping_shr(d as u32)
        };

        if mask != 0 {
            let at = sub(ptr, start_ptr);
            let mut cur = mask.trailing_zeros() as usize;
            loop {
                let c = *ptr.add(cur);
                let escape_byte = ESCAPE[c as usize];
                if escape_byte != 0 {
                    let i = at + cur;
                    if start < i {
                        result.extend_from_slice(&bytes[start..i]);
                    }
                    write_escape(result, escape_byte, c);
                    start = i + 1;
                }
                mask ^= 1 << cur;
                if mask == 0 {
                    break;
                }
                cur = mask.trailing_zeros() as usize;
            }
        }
    }

    // Copy any remaining bytes
    if start < len {
        result.extend_from_slice(&bytes[start..]);
    }
}

#[target_feature(enable = "sse2")]
#[inline]
pub unsafe fn escape_sse2(bytes: &[u8], result: &mut Vec<u8>) {
    let len = bytes.len();

    let start_ptr = bytes.as_ptr();
    let end_ptr = bytes[len..].as_ptr();
    let mut ptr = start_ptr;
    let mut start = 0;

    const M128_VECTOR_ALIGN: usize = M128_VECTOR_SIZE - 1;

    let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
    let v_below_a = _mm_set1_epi8(BELOW_A);
    let v_b = _mm_set1_epi8(B);
    let v_c = _mm_set1_epi8(C);

    // Handle alignment - skip if already aligned
    let misalignment = start_ptr as usize & M128_VECTOR_ALIGN;
    if misalignment != 0 {
        let align = M128_VECTOR_SIZE - misalignment;
        let mut mask = {
            let a = _mm_loadu_si128(ptr as *const __m128i);
            _mm_movemask_epi8(_mm_or_si128(
                _mm_or_si128(_mm_cmpeq_epi8(a, v_b), _mm_cmpeq_epi8(a, v_c)),
                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
            ))
        };

        if mask != 0 {
            let at = sub(ptr, start_ptr);
            let mut cur = mask.trailing_zeros() as usize;
            while cur < align {
                let c = *ptr.add(cur);
                let escape_byte = ESCAPE[c as usize];
                if escape_byte != 0 {
                    let i = at + cur;
                    if start < i {
                        result.extend_from_slice(&bytes[start..i]);
                    }
                    write_escape(result, escape_byte, c);
                    start = i + 1;
                }
                mask ^= 1 << cur;
                if mask == 0 {
                    break;
                }
                cur = mask.trailing_zeros() as usize;
            }
        }
        ptr = ptr.add(align);
    }

    // Main loop
    while ptr <= end_ptr.sub(M128_VECTOR_SIZE) {
        debug_assert_eq!(0, (ptr as usize) % M128_VECTOR_SIZE);
        let mut mask = {
            let a = _mm_load_si128(ptr as *const __m128i);
            _mm_movemask_epi8(_mm_or_si128(
                _mm_or_si128(_mm_cmpeq_epi8(a, v_b), _mm_cmpeq_epi8(a, v_c)),
                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
            ))
        };

        if mask != 0 {
            let at = sub(ptr, start_ptr);
            let mut cur = mask.trailing_zeros() as usize;
            loop {
                let c = *ptr.add(cur);
                let escape_byte = ESCAPE[c as usize];
                if escape_byte != 0 {
                    let i = at + cur;
                    if start < i {
                        result.extend_from_slice(&bytes[start..i]);
                    }
                    write_escape(result, escape_byte, c);
                    start = i + 1;
                }
                mask ^= 1 << cur;
                if mask == 0 {
                    break;
                }
                cur = mask.trailing_zeros() as usize;
            }
        }
        ptr = ptr.add(M128_VECTOR_SIZE);
    }

    // Handle tail
    if ptr < end_ptr {
        let remaining = sub(end_ptr, ptr);
        let d = M128_VECTOR_SIZE - remaining;

        // Use temporary buffer if reading would cross page boundary
        let a = if check_cross_page(ptr.sub(d), M128_VECTOR_SIZE) {
            let mut temp = [0u8; M128_VECTOR_SIZE];
            // Copy remaining bytes to the beginning of temp buffer
            std::ptr::copy_nonoverlapping(ptr, temp.as_mut_ptr(), remaining);
            _mm_loadu_si128(temp.as_ptr() as *const __m128i)
        } else {
            _mm_loadu_si128(ptr.sub(d) as *const __m128i)
        };

        let mut mask = if check_cross_page(ptr.sub(d), M128_VECTOR_SIZE) {
            // When using temp buffer, only check the valid bytes
            (_mm_movemask_epi8(_mm_or_si128(
                _mm_or_si128(_mm_cmpeq_epi8(a, v_b), _mm_cmpeq_epi8(a, v_c)),
                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
            )) as u16) & ((1u16 << remaining) - 1)
        } else {
            (_mm_movemask_epi8(_mm_or_si128(
                _mm_or_si128(_mm_cmpeq_epi8(a, v_b), _mm_cmpeq_epi8(a, v_c)),
                _mm_cmpgt_epi8(_mm_add_epi8(a, v_translation_a), v_below_a),
            )) as u16).wrapping_shr(d as u32)
        };

        if mask != 0 {
            let at = sub(ptr, start_ptr);
            let mut cur = mask.trailing_zeros() as usize;
            loop {
                let c = *ptr.add(cur);
                let escape_byte = ESCAPE[c as usize];
                if escape_byte != 0 {
                    let i = at + cur;
                    if start < i {
                        result.extend_from_slice(&bytes[start..i]);
                    }
                    write_escape(result, escape_byte, c);
                    start = i + 1;
                }
                mask ^= 1 << cur;
                if mask == 0 {
                    break;
                }
                cur = mask.trailing_zeros() as usize;
            }
        }
    }

    // Copy any remaining bytes
    if start < len {
        result.extend_from_slice(&bytes[start..]);
    }
}

#[inline(always)]
unsafe fn process_mask_avx(
    ptr: *const u8,
    start_ptr: *const u8,
    result: &mut Vec<u8>,
    start: &mut usize,
    bytes: &[u8],
    mask: i32,
    offset: usize,
) {
    let ptr = ptr.add(offset);
    let at = sub(ptr, start_ptr);

    // Process mask bits using bit manipulation
    let mut remaining = mask as u32;
    while remaining != 0 {
        let cur = remaining.trailing_zeros() as usize;
        let c = *ptr.add(cur);
        let escape_byte = ESCAPE[c as usize];
        debug_assert!(escape_byte != 0);

        let i = at + cur;
        // Copy unescaped portion if needed
        if *start < i {
            result.extend_from_slice(&bytes[*start..i]);
        }
        // Write escape sequence
        write_escape(result, escape_byte, c);
        *start = i + 1;

        // Clear the lowest set bit
        remaining &= remaining - 1;
    }
}

#[inline(always)]
unsafe fn process_mask_avx512(
    ptr: *const u8,
    start_ptr: *const u8,
    result: &mut Vec<u8>,
    start: &mut usize,
    bytes: &[u8],
    mask: u64,
    offset: usize,
) {
    let ptr = ptr.add(offset);
    let at = sub(ptr, start_ptr);

    // Process mask bits using bit manipulation
    let mut remaining = mask;
    while remaining != 0 {
        let cur = remaining.trailing_zeros() as usize;
        let c = *ptr.add(cur);
        let escape_byte = ESCAPE[c as usize];
        debug_assert!(escape_byte != 0);

        let i = at + cur;
        // Copy unescaped portion if needed
        if *start < i {
            result.extend_from_slice(&bytes[*start..i]);
        }
        // Write escape sequence
        write_escape(result, escape_byte, c);
        *start = i + 1;

        // Clear the lowest set bit
        remaining &= remaining - 1;
    }
}

#[inline(always)]
fn write_escape(result: &mut Vec<u8>, escape_byte: u8, c: u8) {
    // Use optimized escape table for bulk writing
    let (len, bytes) = ESCAPE_TABLE[c as usize];
    if len > 0 {
        // Ensure we have enough capacity for the escape sequence
        result.reserve(len as usize);
        let dst = result.as_mut_ptr().add(result.len());
        // Use copy_nonoverlapping for fast bulk copy
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), dst, 8);
        }
        // Update the length - only add the actual escape sequence length
        unsafe {
            result.set_len(result.len() + len as usize);
        }
    } else {
        // Fallback to old method for characters not in the table
        result.push(b'\\');
        if escape_byte == UU {
            // Unicode escape for control characters
            result.extend_from_slice(b"u00");
            let hex_digits = &HEX_BYTES[c as usize];
            result.push(hex_digits.0);
            result.push(hex_digits.1);
        } else {
            // Simple escape
            result.push(escape_byte);
        }
    }
}
