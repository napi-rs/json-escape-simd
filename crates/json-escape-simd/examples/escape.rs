use std::fs;

use json_escape_simd::escape;

fn main() {
    for fixture in get_rxjs_sources() {
        let encoded = escape(&fixture);
        assert_eq!(encoded, sonic_rs::to_string(&fixture).unwrap());
        assert_eq!(encoded, serde_json::to_string(&fixture).unwrap());
    }
}

fn get_rxjs_sources() -> Vec<String> {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let pattern = root.join("node_modules/rxjs/src/**/*.ts");
    let dir = glob::glob(&pattern.to_string_lossy().replace('\\', "/")).unwrap();
    let mut sources = Vec::new();
    for entry in dir {
        sources.push(fs::read_to_string(entry.unwrap()).unwrap());
    }
    assert!(!sources.is_empty());
    sources
}
