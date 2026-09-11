use std::{fs, hint::black_box};

use criterion::{Criterion, criterion_group, criterion_main};

#[cfg(not(feature = "codspeed"))]
use generic::escape_generic;
use json_escape_simd::escape;

#[cfg(not(feature = "codspeed"))]
mod generic;

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

fn run_benchmarks(c: &mut Criterion, sources: &[String], prefix: &str) {
    let first = &sources[0];
    assert_eq!(escape(first), sonic_rs::to_string(first).unwrap());
    assert_eq!(escape(first), serde_json::to_string(first).unwrap());

    c.bench_function(&format!("{} escape simd", prefix), |b| {
        b.iter(|| {
            for source in sources {
                black_box(escape(source));
            }
        })
    });
    #[cfg(not(feature = "codspeed"))]
    c.bench_function(&format!("{} escape sonic", prefix), |b| {
        b.iter(|| {
            for source in sources {
                black_box(sonic_rs::to_string(source).unwrap());
            }
        })
    });
    #[cfg(not(feature = "codspeed"))]
    c.bench_function(&format!("{} escape v_jsonescape", prefix), |b| {
        b.iter(|| {
            for source in sources {
                black_box(v_jsonescape::escape_fmt(source).to_string());
            }
        })
    });
    #[cfg(not(feature = "codspeed"))]
    c.bench_function(&format!("{} json-escape", prefix), |b| {
        b.iter(|| {
            for source in sources {
                black_box(json_escape::escape_str(source).collect::<String>());
            }
        })
    });
    #[cfg(not(feature = "codspeed"))]
    c.bench_function(&format!("{} escape generic", prefix), |b| {
        b.iter(|| {
            for source in sources {
                black_box(escape_generic(source));
            }
        })
    });
    #[cfg(not(feature = "codspeed"))]
    c.bench_function(&format!("{} serde_json", prefix), |b| {
        b.iter(|| {
            for source in sources {
                black_box(serde_json::to_string(source).unwrap());
            }
        })
    });
}

fn short_string_benchmark(c: &mut Criterion) {
    let sources = vec![
        "Hello, world!".to_string(),
        r#"abcdefghijklmnopqrstuvwxyz .*? hello world escape json string"#.to_string(),
        "normal string 🥹".to_string(),
        "中文 English 🚀 \n❓ 𝄞".to_string(),
    ];
    run_benchmarks(c, &sources, "short string");
}

fn rxjs_benchmark(c: &mut Criterion) {
    let sources = get_rxjs_sources();
    if !sources.is_empty() {
        run_benchmarks(c, &sources, "rxjs");
    }
}

fn affine_sources_benchmark(c: &mut Criterion) {
    let sources = get_affine_sources();
    if !sources.is_empty() {
        run_benchmarks(c, &sources, "fixtures");
    }
}

criterion_group!(
    benches,
    short_string_benchmark,
    rxjs_benchmark,
    affine_sources_benchmark
);
criterion_main!(benches);
