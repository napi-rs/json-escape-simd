use std::{fs, hint::black_box};

use criterion::{Criterion, criterion_group, criterion_main};
use html_escape_simd::escape_html;

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn glob_utf8(pattern: &str) -> glob::Paths {
    let pat = workspace_root().join(pattern);
    glob::glob(&pat.to_string_lossy().replace('\\', "/")).unwrap()
}

fn get_rxjs_sources() -> Vec<String> {
    let rxjs_paths = glob_utf8("node_modules/rxjs/src/**/*.ts");
    let mut sources = Vec::new();
    for entry in rxjs_paths {
        let p = entry.unwrap();
        if fs::metadata(&p).unwrap().is_file() {
            sources.push(fs::read_to_string(&p).unwrap());
        }
    }
    sources
}

fn get_affine_sources() -> Vec<String> {
    let ts_paths = glob_utf8("fixtures/**/*.ts");
    let tsx_paths = glob_utf8("fixtures/**/*.tsx");
    let js_paths = glob_utf8("fixtures/**/*.js");
    let mjs_paths = glob_utf8("fixtures/**/*.mjs");
    let cjs_paths = glob_utf8("fixtures/**/*.cjs");
    let mut sources = Vec::new();
    for entry in ts_paths
        .chain(tsx_paths)
        .chain(js_paths)
        .chain(mjs_paths)
        .chain(cjs_paths)
    {
        let p = entry.unwrap();
        if fs::metadata(&p).unwrap().is_file() {
            sources.push(fs::read_to_string(&p).unwrap());
        }
    }
    sources
}

fn get_html_sources() -> Vec<String> {
    let html_paths = glob_utf8("fixtures/**/*.html");
    let mut sources = Vec::new();
    for entry in html_paths {
        let p = entry.unwrap();
        if fs::metadata(&p).unwrap().is_file() {
            sources.push(fs::read_to_string(&p).unwrap());
        }
    }
    sources
}

fn run_html_benchmarks(c: &mut Criterion, sources: &[String], prefix: &str) {
    let first = &sources[0];
    let mut expected = String::new();
    v_htmlescape::escape_string(first, &mut expected);
    assert_eq!(escape_html(first).unwrap(), expected);

    c.bench_function(&format!("{} escape_html simsimd", prefix), |b| {
        b.iter(|| {
            for source in sources {
                black_box(escape_html(source).unwrap());
            }
        })
    });
    c.bench_function(&format!("{} v_htmlescape escape_string", prefix), |b| {
        b.iter(|| {
            for source in sources {
                let mut s = String::with_capacity(source.len() * 6 + 32);
                v_htmlescape::escape_string(source, &mut s);
                black_box(s);
            }
        })
    });
    c.bench_function(&format!("{} v_htmlescape escape_fmt", prefix), |b| {
        b.iter(|| {
            for source in sources {
                black_box(v_htmlescape::escape_fmt(source).to_string());
            }
        })
    });
}

fn html_benchmark(c: &mut Criterion) {
    let short = vec![
        "Hello, world!".to_string(),
        r#"abcdefghijklmnopqrstuvwxyz .*? hello world escape json string"#.to_string(),
        "normal string 🥹".to_string(),
        "中文 English 🚀 \n❓ 𝄞".to_string(),
        r#"<a href="https://x.com?a=1&b=2">it's "quoted" & <tagged></a>"#.to_string(),
    ];
    run_html_benchmarks(c, &short, "html short string");

    let rxjs = get_rxjs_sources();
    if !rxjs.is_empty() {
        run_html_benchmarks(c, &rxjs, "html rxjs");
    }

    let affine = get_affine_sources();
    if !affine.is_empty() {
        run_html_benchmarks(c, &affine, "html fixtures");
    }

    let html = get_html_sources();
    if !html.is_empty() {
        run_html_benchmarks(c, &html, "html files");
    }
}

criterion_group!(benches, html_benchmark);
criterion_main!(benches);
