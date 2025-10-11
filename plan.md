# Performance Analysis: sonic-rs vs json-escape-simd

## Benchmark Results
- **json-escape-simd**: 333.21 - 348.88 µs (median: ~341 µs)
- **sonic-rs**: 205.62 - 210.19 µs (median: ~208 µs)
- **Performance Gap**: sonic-rs is ~40% faster

## Key Differences in Implementation

### 1. Copy-First Strategy with Deferred Escaping

**sonic-rs Approach:**
```rust
// Always copy the SIMD chunk first
v.write_to_slice_unaligned_unchecked(dst_slice);
let mask = escaped_mask(v);
if mask.all_zero() {
    // Fast path: no escapes, just advance pointers
    advance_pointers();
} else {
    // Found escape, backtrack to handle it
    let escape_pos = mask.first_offset();
    adjust_and_escape();
}
```

**json-escape-simd Approach:**
```rust
// Check for escapes first, copy only if clean
if any_escape == 0 {
    // Copy whole chunk
    result.extend_from_slice(chunk);
} else {
    // Process each escape individually
    process_mask_avx(...);
}
```

**Why Copy-First is Faster:**
- Reduces branches in the common case (most chunks have no escapes)
- Better CPU pipeline utilization
- Simpler control flow
- Memory writes are buffered and can be overlapped with mask checking

### 2. Pre-allocated Output Buffer with MaybeUninit

**sonic-rs:**
```rust
// Pre-reserves worst-case buffer size upfront
let buf = writer.reserve_with(value.len() * 6 + 32 + 3)?;
// Works with MaybeUninit to avoid initialization overhead
pub fn format_string(value: &str, dst: &mut [MaybeUninit<u8>], ...) -> usize
```

**json-escape-simd:**
```rust
// Uses Vec with potential dynamic growth
let mut result = Vec::with_capacity(estimated_capacity);
// Multiple extend_from_slice calls may trigger reallocation
result.extend_from_slice(&bytes[start..i]);
```

**Benefits:**
- No reallocation overhead during processing
- No initialization cost for unused buffer space
- Direct pointer arithmetic instead of Vec method calls

### 3. Compact Escape Handling with Lookup Table

**sonic-rs:**
```rust
// Pre-formatted escape sequences in 8-byte blocks
pub const QUOTE_TAB: [(u8, [u8; 8]); 256] = [
    (6, *b"\\u0000\0\0"),  // Length and padded sequence
    (2, *b"\\t\0\0\0\0\0\0"),
    // ...
];
// Single memcpy for any escape type
std::ptr::copy_nonoverlapping(QUOTE_TAB[ch].1.as_ptr(), dst, 8);
dst += QUOTE_TAB[ch].0;
```

**json-escape-simd:**
```rust
// Conditional logic for each escape type
fn write_escape(result: &mut Vec<u8>, escape_byte: u8, c: u8) {
    result.push(b'\\');
    if escape_byte == UU {
        result.extend_from_slice(b"u00");
        result.push(hex_digits.0);
        result.push(hex_digits.1);
    } else {
        result.push(escape_byte);
    }
}
```

**Advantages:**
- Single memory operation vs multiple pushes
- No conditional branches in escape writing
- Better memory locality (8-byte aligned writes)

### 4. Simpler Mask Processing

**sonic-rs:**
- Uses `first_offset()` to find only the first escape
- Handles escapes sequentially from that point
- Minimal bit manipulation

**json-escape-simd:**
- Processes every set bit in the mask using `trailing_zeros()`
- Complex bit manipulation loop (`mask &= mask - 1`)
- More branches and iterations

### 5. Cross-Page Boundary Optimization

**sonic-rs includes page boundary checks:**
```rust
if check_cross_page(sptr, LANES) {
    // Use temporary buffer to avoid potential page faults
    std::ptr::copy_nonoverlapping(sptr, temp.as_mut_ptr(), remaining);
    load(temp.as_ptr())
}
```

This prevents potential page faults when reading past the end of allocated memory.

## Optimization Recommendations

### Priority 1: Adopt Copy-First Strategy
- Modify the SIMD loops to always write chunks first
- Only check for escapes after copying
- Backtrack when escapes are found

### Priority 2: Use Pre-allocated MaybeUninit Buffer
```rust
pub fn escape_into_uninit(input: &str, output: &mut [MaybeUninit<u8>]) -> usize {
    // Work directly with MaybeUninit buffer
    // Return actual bytes written
}
```

### Priority 3: Implement Compact Escape Table
```rust
const ESCAPE_TABLE: [(u8, [u8; 8]); 256] = [
    // Pre-format all escape sequences
    // Use single memcpy for writing
];
```

### Priority 4: Simplify Mask Processing
- Process only first escape per chunk
- Continue sequentially from escape point
- Reduce bit manipulation overhead

### Priority 5: Add Page Boundary Handling
- Implement cross-page detection for tail processing
- Use temporary buffer when crossing boundaries

## Expected Performance Improvements

Based on the analysis, implementing these optimizations should:
1. **Copy-First Strategy**: 15-20% improvement
2. **MaybeUninit Buffer**: 5-10% improvement
3. **Compact Escape Table**: 10-15% improvement
4. **Simplified Mask Processing**: 5-10% improvement
5. **Page Boundary Handling**: 2-3% improvement (safety/stability)

Combined, these changes could potentially close most of the 40% performance gap with sonic-rs.

## Implementation Strategy

1. **Phase 1**: Implement copy-first strategy (biggest impact)
2. **Phase 2**: Add compact escape table
3. **Phase 3**: Switch to MaybeUninit buffer
4. **Phase 4**: Optimize mask processing
5. **Phase 5**: Add page boundary handling ✅ **COMPLETED**

Each phase should be benchmarked independently to measure impact.

## Completed Optimizations

### Page Boundary Handling (Phase 5) - COMPLETED

Added page boundary checking to prevent potential page faults when reading past the end of input:

- Added `check_cross_page` function with conditional compilation for Linux/macOS
- Updated AVX512, AVX2, SSE2 tail handling to use temporary buffers when crossing page boundaries
- Updated aarch64 NEON implementation with page boundary checks
- On Linux/macOS: checks if reading would cross 4096-byte page boundary
- On other platforms: always uses safe path with temporary buffer

This optimization improves safety and stability without significant performance impact.