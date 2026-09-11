# html-escape-simd

Optimized SIMD routines for escaping HTML strings. Thin facade over
[`escape-simd`](https://crates.io/crates/escape-simd) with the `html` feature.

Escapes `"`, `&`, `'`, `/`, `<`, `>` byte-identically to `v_htmlescape` /
`@napi-rs/escape`. Unlike JSON escaping, no surrounding quotes are added.

```toml
html-escape-simd = "0.1"
```

```rust
use html_escape_simd::escape_html;

assert_eq!(escape_html("<div>").unwrap(), "&lt;div&gt;");
```
