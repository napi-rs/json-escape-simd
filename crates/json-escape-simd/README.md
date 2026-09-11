# json-escape-simd

![Crates.io Version](https://img.shields.io/crates/v/json-escape-simd)
![docs.rs](https://img.shields.io/docsrs/json-escape-simd)

Optimized SIMD routines for escaping JSON strings. Thin facade over
[`escape-simd`](https://crates.io/crates/escape-simd) with the `json` feature.

The implementation is from [sonic-rs](https://github.com/cloudwego/sonic-rs); we
only take the string escaping part to avoid the abstraction overhead.

```toml
json-escape-simd = "3"
```

```rust
use json_escape_simd::{escape, escape_into};

assert_eq!(escape("hello\n"), r#""hello\n""#);
```
