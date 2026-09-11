use std::fs::read_dir;
use std::path::{Path, PathBuf};

use html_escape_simd::{escape_html, escape_html_into};
use rand::seq::SliceRandom;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn html_oracle(s: &str) -> String {
    let mut out = String::new();
    v_htmlescape::escape_string(s, &mut out);
    out
}

fn check_html(s: &str) {
    assert_eq!(escape_html(s).unwrap(), html_oracle(s), "input: {s:?}");
    let mut output = String::with_capacity(s.len() * 6 + 32);
    escape_html_into(s, unsafe { output.as_mut_vec() }).unwrap();
    assert_eq!(output, html_oracle(s), "into input: {s:?}");
}

#[test]
fn test_html_escape_ascii() {
    let fixture = r#"abcdefghijklmnopqrstuvwxyz .*? hello world escape html string"#;
    check_html(fixture);
}

#[test]
fn test_html_empty_string() {
    assert_eq!(escape_html("").unwrap(), "");
}

#[test]
fn test_html_very_small_strings() {
    for s in [
        "a", "ab", "hello", "<", "&", "a&b", "<a>", "'q'", "/", "a/b<c>d", "tail<", ">head",
    ] {
        check_html(s);
    }
}

#[test]
fn test_html_small_strings_16_bytes() {
    let s16 = "0123456789abcdef";
    assert_eq!(s16.len(), 16);
    check_html(s16);

    let s16_esc = "01234567<9abcde";
    assert_eq!(s16_esc.len(), 15);
    check_html(s16_esc);
    check_html("<123456789abcde");
    check_html("0123456789abcd&");
}

#[test]
fn test_html_medium_strings_32_bytes() {
    let s32 = "0123456789abcdef0123456789abcdef";
    assert_eq!(s32.len(), 32);
    check_html(s32);

    check_html("0123456789abcde<0123456789abcde");
    check_html("&123456789abcdef0123456789abcde");
    check_html("0123456789abcdef0123456789abcd'");
}

#[test]
fn test_html_large_strings_128_bytes() {
    let s128 = "0123456789abcdef".repeat(8);
    assert_eq!(s128.len(), 128);
    check_html(&s128);

    let mut s128_esc = String::new();
    for i in 0..8 {
        if i % 2 == 0 {
            s128_esc.push_str("0123456789abcd<");
        } else {
            s128_esc.push_str("0123456789abcd&");
        }
    }
    check_html(&s128_esc);
}

#[test]
fn test_html_unaligned_data() {
    for offset in 0..32 {
        let padding = " ".repeat(offset);
        let test_str = format!("{}{}", padding, "test<string>&'with'\"escapes\"/here");
        check_html(&test_str[offset..]);
    }
}

#[test]
fn test_html_sparse_escapes() {
    let mut s = String::new();
    s.push('<');
    s.push_str(&"a".repeat(500));
    s.push('>');
    check_html(&s);
}

#[test]
fn test_html_dense_escapes() {
    let s = "<&>/'\"".repeat(100);
    check_html(&s);
    let all_lt = "<".repeat(300);
    check_html(&all_lt);
}

#[test]
fn test_html_boundary_conditions() {
    for size in 250..260 {
        let s = "a".repeat(size);
        check_html(&s);

        let mut s_esc = "a".repeat(size - 1);
        s_esc.push('"');
        check_html(&s_esc);
    }
}

#[test]
fn test_html_all_six_chars() {
    for ch in ['"', '&', '\'', '/', '<', '>'] {
        check_html(ch.to_string().as_str());
    }
    let runs = "\"&'/<> ".repeat(20);
    check_html(&runs);
    check_html(&format!("<{}>", "a".repeat(38)));
    check_html(&format!("&{}'", "a".repeat(38)));
    for ch in ['"', '&', '\'', '/', '<', '>'] {
        for pos in 0..40 {
            let mut s = "a".repeat(40);
            s.insert(pos, ch);
            check_html(&s);
        }
    }
}

#[test]
fn test_html_exhaustive_ascii() {
    for b in 0u8..0x80 {
        let ch = b as char;
        for len in [1usize, 15, 16, 17, 63, 64, 65] {
            let s: String = std::iter::repeat_n(ch, len).collect();
            check_html(&s);
        }
        let s = format!("aaaaaaaa{}bbbbbbbb{}cccccccc", ch, ch);
        check_html(&s);
    }
}

#[test]
fn test_html_mixed_content() {
    let mixed = r#"Hello <b>"World"</b>!
    Tab:	Here & There
    Emoji: 😀 Chinese: 中文
    Math: ∑∫∂ Music: 𝄞
    Attr: <a href='/x?a=1&b=2'>link</a>"#;
    check_html(mixed);
}

#[test]
fn test_html_repeated_patterns() {
    check_html(&"abcd".repeat(100));
    check_html(&"a<b>".repeat(100));
    check_html(&"&amp;".repeat(100));
}

#[test]
fn test_html_rxjs() {
    let mut sources = Vec::new();
    read_dir_recursive(
        workspace_root().join("node_modules/rxjs/src"),
        &mut sources,
        |p| matches!(p.extension().and_then(|e| e.to_str()), Some("ts")),
    )
    .unwrap();
    assert!(!sources.is_empty());
    sources.shuffle(&mut rand::rng());
    for source in sources
        .iter()
        .take(if cfg!(miri) { 10 } else { sources.len() })
    {
        assert_eq!(escape_html(source).unwrap(), html_oracle(source));
        let mut output = String::with_capacity(source.len() * 6 + 32);
        escape_html_into(source, unsafe { output.as_mut_vec() }).unwrap();
        assert_eq!(output, html_oracle(source));
    }
}

#[test]
fn test_html_sources() {
    for source in load_affine_sources().unwrap() {
        assert_eq!(escape_html(&source).unwrap(), html_oracle(&source));
        let mut output = String::with_capacity(source.len() * 6 + 32);
        escape_html_into(&source, unsafe { output.as_mut_vec() }).unwrap();
        assert_eq!(output, html_oracle(&source));
    }
}

fn load_affine_sources() -> Result<impl Iterator<Item = String>, std::io::Error> {
    let mut sources = Vec::new();
    read_dir_recursive(workspace_root().join("fixtures"), &mut sources, |p| {
        matches!(
            p.extension().and_then(|e| e.to_str()),
            Some("ts") | Some("tsx") | Some("js") | Some("mjs") | Some("cjs")
        )
    })?;
    assert!(!sources.is_empty());
    let len = sources.len();
    sources.shuffle(&mut rand::rng());
    Ok(sources.into_iter().take(if cfg!(miri) { 10 } else { len }))
}

fn read_dir_recursive<P: AsRef<Path>, F: Fn(PathBuf) -> bool + Copy>(
    dir: P,
    sources: &mut Vec<String>,
    f: F,
) -> Result<(), std::io::Error> {
    let dir = read_dir(dir)?;
    for entry in dir {
        let p = entry?;
        let metadata = std::fs::metadata(p.path())?;
        if metadata.is_file() && f(p.path()) {
            sources.push(std::fs::read_to_string(p.path())?);
        }
        if metadata.is_dir() {
            read_dir_recursive(p.path(), sources, f)?;
        }
    }
    Ok(())
}
