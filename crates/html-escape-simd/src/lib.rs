//! Optimized SIMD routines for escaping HTML strings.
//!
//! Thin facade over [`escape_simd::html`]. Escapes `"`, `&`, `'`, `/`, `<`,
//! `>` byte-identically to `v_htmlescape` / `@napi-rs/escape`. Unlike JSON
//! escaping, no surrounding quotes are added.

#[doc(inline)]
pub use escape_simd::html::{InputTooLarge, escape_html, escape_html_into};
