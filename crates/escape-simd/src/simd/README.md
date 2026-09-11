# sonic_simd

Borrowed from https://github.com/cloudwego/sonic-rs.
With the runtime SIMD features detection rather than compile-time detection.

A portable SIMD library that provides low-level APIs for x86, ARM. Other platforms will use the fallback scalar implementation.

TODO:

1. support RISC-V.
2. support wasm.