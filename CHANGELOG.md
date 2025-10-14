# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.0.1](https://github.com/napi-rs/json-escape-simd/compare/json-escape-simd-v3.0.0...json-escape-simd-v3.0.1) - 2025-10-14

### Other

- hide avx512 behind feature flag ([#39](https://github.com/napi-rs/json-escape-simd/pull/39))

## [3.0.0](https://github.com/napi-rs/json-escape-simd/compare/json-escape-simd-v2.0.0...json-escape-simd-v3.0.0) - 2025-10-13

### Other

- [**breaking**] do not reserve memory in escape_into ([#36](https://github.com/napi-rs/json-escape-simd/pull/36))

## [2.0.0](https://github.com/napi-rs/json-escape-simd/compare/json-escape-simd-v1.1.0...json-escape-simd-v2.0.0) - 2025-10-13

### Fixed

- tests on avx512 host ([#35](https://github.com/napi-rs/json-escape-simd/pull/35))

### Other

- run miri on more platform ([#34](https://github.com/napi-rs/json-escape-simd/pull/34))
- cleanup codes ([#33](https://github.com/napi-rs/json-escape-simd/pull/33))
- split impls into arch ([#32](https://github.com/napi-rs/json-escape-simd/pull/32))
- runtime detect simd features ([#29](https://github.com/napi-rs/json-escape-simd/pull/29))
- remove useless deps ([#28](https://github.com/napi-rs/json-escape-simd/pull/28))
- update benchmark result
- borrow the sonic-rs string escape implementation ([#27](https://github.com/napi-rs/json-escape-simd/pull/27))
- *(deps)* update rust crate json-escape to 0.3.0 ([#24](https://github.com/napi-rs/json-escape-simd/pull/24))
- *(deps)* update rust crate json-escape to 0.2.0 ([#23](https://github.com/napi-rs/json-escape-simd/pull/23))
- omit other crates in codspeed ([#22](https://github.com/napi-rs/json-escape-simd/pull/22))
- add benchmark ([#20](https://github.com/napi-rs/json-escape-simd/pull/20))

## [1.1.0](https://github.com/napi-rs/json-escape-simd/compare/json-escape-simd-v1.0.4...json-escape-simd-v1.1.0) - 2025-09-23

### Added

- add escape_into ([#18](https://github.com/napi-rs/json-escape-simd/pull/18))

## [1.0.4](https://github.com/napi-rs/json-escape-simd/compare/json-escape-simd-v1.0.3...json-escape-simd-v1.0.4) - 2025-09-23

### Other

- reduce alignment overhead for small inputs ([#16](https://github.com/napi-rs/json-escape-simd/pull/16))

## [1.0.3](https://github.com/napi-rs/json-escape-simd/compare/json-escape-simd-v1.0.2...json-escape-simd-v1.0.3) - 2025-09-23

### Other

- reduce allocation on x86 ([#13](https://github.com/napi-rs/json-escape-simd/pull/13))

## [1.0.2](https://github.com/napi-rs/json-escape-simd/compare/json-escape-simd-v1.0.1...json-escape-simd-v1.0.2) - 2025-09-23

### Other

- code cleanup
- cleanup codes

## [1.0.1](https://github.com/napi-rs/json-escape-simd/compare/json-escape-simd-v1.0.0...json-escape-simd-v1.0.1) - 2025-09-23

### Other

- use oxc_sourcemap version of escape_generic ([#9](https://github.com/napi-rs/json-escape-simd/pull/9))
