use std::fs::read_dir;
use std::path::{Path, PathBuf};

use json_escape_simd::{escape, escape_into};
use rand::seq::SliceRandom;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn test_escape_ascii_json_string() {
    let fixture = r#"abcdefghijklmnopqrstuvwxyz .*? hello world escape json string"#;
    assert_eq!(escape(fixture), serde_json::to_string(fixture).unwrap());
}

#[test]
fn test_escape_json_string() {
    let mut fixture = String::new();
    for i in 0u8..=0x1F {
        fixture.push(i as char);
    }
    fixture.push('\t');
    fixture.push('\x08');
    fixture.push('\x09');
    fixture.push('\x0A');
    fixture.push('\x0C');
    fixture.push('\x0D');
    fixture.push('\x22');
    fixture.push('\x5C');
    fixture.push_str("normal string");
    fixture.push('😊');
    fixture.push_str("中文 English 🚀 \n❓ 𝄞");
    assert_eq!(
        escape(fixture.as_str()),
        serde_json::to_string(fixture.as_str()).unwrap(),
        "fixture: {:?}",
        fixture
    );
}

#[test]
fn test_empty_string() {
    assert_eq!(escape(""), r#""""#);
}

#[test]
fn test_very_small_strings() {
    assert_eq!(escape("a"), r#""a""#);
    assert_eq!(escape("ab"), r#""ab""#);
    assert_eq!(escape("hello"), r#""hello""#);
    assert_eq!(escape("hello\n"), r#""hello\n""#);
    assert_eq!(escape("\""), r#""\"""#);
    assert_eq!(escape("\\"), r#""\\""#);
    assert_eq!(escape("\t"), r#""\t""#);
    assert_eq!(escape("\r\n"), r#""\r\n""#);
}

#[test]
fn test_small_strings_16_bytes() {
    let s16 = "0123456789abcdef";
    assert_eq!(s16.len(), 16);
    assert_eq!(escape(s16), serde_json::to_string(s16).unwrap());

    let s16_esc = "01234567\t9abcde";
    assert_eq!(s16_esc.len(), 15);
    assert_eq!(escape(s16_esc), serde_json::to_string(s16_esc).unwrap());
}

#[test]
fn test_medium_strings_32_bytes() {
    let s32 = "0123456789abcdef0123456789abcdef";
    assert_eq!(s32.len(), 32);
    assert_eq!(escape(s32), serde_json::to_string(s32).unwrap());

    let s32_esc = "0123456789abcde\"0123456789abcde";
    assert_eq!(escape(s32_esc), serde_json::to_string(s32_esc).unwrap());
}

#[test]
fn test_large_strings_128_bytes() {
    let s128 = "0123456789abcdef".repeat(8);
    assert_eq!(s128.len(), 128);
    assert_eq!(escape(&s128), serde_json::to_string(&s128).unwrap());

    let mut s128_esc = String::new();
    for i in 0..8 {
        if i % 2 == 0 {
            s128_esc.push_str("0123456789abcd\n");
        } else {
            s128_esc.push_str("0123456789abcd\"");
        }
    }
    assert_eq!(escape(&s128_esc), serde_json::to_string(&s128_esc).unwrap());
}

#[test]
fn test_unaligned_data() {
    for offset in 0..32 {
        let padding = " ".repeat(offset);
        let test_str = format!("{}{}", padding, "test\nstring\"with\\escapes");
        let result = escape(&test_str[offset..]);
        let expected = serde_json::to_string(&test_str[offset..]).unwrap();
        assert_eq!(result, expected, "Failed at offset {}", offset);
    }
}

#[test]
fn test_sparse_escapes() {
    let mut s = String::new();
    s.push('"');
    s.push_str(&"a".repeat(500));
    s.push('\\');
    assert_eq!(escape(&s), serde_json::to_string(&s).unwrap());
}

#[test]
fn test_dense_escapes() {
    let s = "\"\\\"\\\"\\\"\\".repeat(50);
    assert_eq!(escape(&s), serde_json::to_string(&s).unwrap());

    let mut ctrl = String::new();
    for _ in 0..10 {
        for i in 0u8..32 {
            ctrl.push(i as char);
        }
    }
    assert_eq!(escape(&ctrl), serde_json::to_string(&ctrl).unwrap());
}

#[test]
fn test_boundary_conditions() {
    for size in 250..260 {
        let s = "a".repeat(size);
        assert_eq!(escape(&s), serde_json::to_string(&s).unwrap());

        let mut s_esc = "a".repeat(size - 1);
        s_esc.push('"');
        assert_eq!(escape(&s_esc), serde_json::to_string(&s_esc).unwrap());
    }
}

#[test]
fn test_all_escape_types() {
    assert_eq!(escape("\x00"), r#""\u0000""#);
    assert_eq!(escape("\x08"), r#""\b""#);
    assert_eq!(escape("\x09"), r#""\t""#);
    assert_eq!(escape("\x0A"), r#""\n""#);
    assert_eq!(escape("\x0C"), r#""\f""#);
    assert_eq!(escape("\x0D"), r#""\r""#);
    assert_eq!(escape("\x1F"), r#""\u001f""#);
    assert_eq!(escape("\""), r#""\"""#);
    assert_eq!(escape("\\"), r#""\\""#);

    for i in 0u8..32 {
        let s = String::from_utf8(vec![i]).unwrap();
        assert_eq!(
            escape(&s),
            serde_json::to_string(&s).unwrap(),
            "Failed for byte 0x{i:02x}"
        );
    }
}

#[test]
fn test_mixed_content() {
    let mixed = r#"Hello "World"!
    Tab:	Here
    Emoji: 😀 Chinese: 中文
    Math: ∑∫∂ Music: 𝄞
    Escape: \" \\ \n \r \t"#;
    assert_eq!(escape(mixed), serde_json::to_string(mixed).unwrap());
}

#[test]
fn test_repeated_patterns() {
    let pattern1 = "abcd".repeat(100);
    assert_eq!(escape(&pattern1), serde_json::to_string(&pattern1).unwrap());

    let pattern2 = "a\"b\"".repeat(100);
    assert_eq!(escape(&pattern2), serde_json::to_string(&pattern2).unwrap());

    let pattern3 = "\t\n".repeat(100);
    assert_eq!(escape(&pattern3), serde_json::to_string(&pattern3).unwrap());
}

#[test]
fn test_rxjs() {
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
        assert_eq!(escape(source), serde_json::to_string(&source).unwrap());
        let mut output = String::with_capacity(source.len() * 6 + 32 + 3);
        escape_into(source, unsafe { output.as_mut_vec() });
        assert_eq!(output, serde_json::to_string(&source).unwrap());
    }
}

#[test]
fn test_sources() {
    for source in load_affine_sources().unwrap() {
        assert_eq!(escape(&source), serde_json::to_string(&source).unwrap());
        let mut output = String::with_capacity(source.len() * 6 + 32 + 3);
        escape_into(&source, unsafe { output.as_mut_vec() });
        assert_eq!(output, serde_json::to_string(&source).unwrap());
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
