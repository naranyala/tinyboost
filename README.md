# tinyboost

[![CI](https://github.com/naranyala/tinyboost/actions/workflows/ci.yml/badge.svg)](https://github.com/naranyala/tinyboost/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.70-blue)](#minimum-supported-rust-version)

**tinyboost** is a small, explicit Rust utility crate inspired by the practical
breadth of C++ Boost — without trying to become a giant collection of
everything.

It provides the kinds of reusable building blocks many Rust codebases keep
re-implementing: cleanup guards, lazy initialization, explicit value-or-empty
wrappers, and ergonomic result/option patterns. Zero third-party dependencies.
Small, clear APIs. Rust idioms, not translated C++ patterns.

## Why this crate exists

The Rust standard library is excellent but intentionally narrow. It focuses on
core semantics and the most common primitives. tinyboost fills the narrow gap
between `std` and everyday application code with a curated layer of small,
opinionated utilities.

This crate is designed around the belief that:

- small utilities beat broad frameworks
- a narrow, stable API is safer than a huge "kitchen sink" crate
- Rust code should read clearly, never hiding behavior behind clever abstraction

## Quick start

Add to your `Cargo.toml`:

```toml
[dependencies]
tinyboost = "0.1"
```

Or explore everything through the prelude:

```rust
use tinyboost::prelude::*;

// RAII cleanup, even on panic unwinding.
// Run cargo test or cargo doc --open to see every utility in action.
```

### `ScopeGuard` — cleanup at scope exit

```rust
use tinyboost::ScopeGuard;

let mut fd = open_something();

// Runs when the guard drops — including on panic unwinding.
// Call `guard.dismiss()` to cancel the cleanup early.
let _cleanup = ScopeGuard::new(|| {
    fd.close();
});
```

### `LazyInit` — one-time initialization

```rust
use tinyboost::LazyInit;

let config = String::from("dev");

// Any `Fn() -> T` works: function pointers, closures, captured state.
let client = LazyInit::new(|| format!("client://{config}"));

assert!(!client.is_initialized());
assert_eq!(client.get(), "client://dev");
assert!(client.is_initialized());
assert_eq!(client.into_inner(), Some(String::from("client://dev")));
```

### `Maybe<T>` — explicit value-or-empty

```rust
use tinyboost::Maybe;

let response: Maybe<String> = fetch().into(); // Option -> Maybe
let fallback = response
    .and_then(|text| text.is_empty().then(|| text)) // None stays None
    .unwrap_or_else(|| String::from("default"));
```

### `ResultExt` / `OptionExt` — side-effect hooks

```rust
use tinyboost::prelude::*;

let config = load_config()
    .tap_ok(|c| log::debug!("loaded {c:?}"))
    .tap_err(|e| log::warn!("falling back: {e}"))
    .map_ok(|c| c.to_string());

let first = [1, 2, 3]
    .into_iter()
    .next()
    .tap_some(|v| assert_eq!(*v, 1))
    .tap_none(|| panic!("never empty here"));
```

## The full API at a glance

| Type / Trait | Purpose | Key methods |
|---|---|---|
| `ScopeGuard<F>` | RAII-style closure run at scope exit (drop or panic unwind) | `new`, `dismiss` |
| `LazyInit<T, F>` | One-time lazy init; `Sync` when `T: Send + Sync` | `new`, `get`, `is_initialized`, `into_inner` |
| `Maybe<T>` | Explicit value-or-empty wrapper, `Option`-round-trip | `is_some`, `is_none`, `map`, `and_then`, `unwrap_or`, `unwrap_or_default`, `unwrap_or_else`, `as_ref`, `as_mut`, `contains` |
| `ResultExt` | Side-effect hooks + success mapping on `Result` | `tap_ok`, `tap_err`, `map_ok` |
| `OptionExt` | Side-effect hooks + `Maybe` conversion on `Option` | `tap_some`, `tap_none`, `into_maybe` |

Everything is re-exported from the crate root and from
[`tinyboost::prelude`](src/prelude.rs).

## Design philosophy

tinyboost is intentionally **not**:

- a replacement for `std`
- a complete Boost clone
- a framework for application architecture
- a giant library of unrelated helpers

Instead it delivers a few foundational utility categories: cleanup and scope
lifecycle helpers, lazy one-time initialization, explicit optional-value
wrappers, and result/option augmentation for common control-flow ergonomics.
See [`docs/design-principles.md`](docs/design-principles.md) for the reasoning
behind every constraint, and [`docs/architecture.md`](docs/architecture.md)
for how the crate is organized.

## Minimum supported Rust version

**MSRV: 1.70** (required by `std::sync::OnceLock`). The MSRV is enforced in CI
and tracked in `Cargo.toml` as `rust-version`.

## Testing

The test suite covers every public item, including panic-unwinding paths and
multi-threaded lazy initialization:

```sh
cargo test                # full suite
cargo test --doc          # doc tests only
cargo clippy --all-targets -- -D warnings   # lint gate (also run in CI)
cargo fmt --check         # formatting gate (also run in CI)
```

See [`docs/testing.md`](docs/testing.md) for the suite layout and testing
conventions.

## Project roadmap

Development is organized into milestones in [`TODOS.md`](TODOS.md), and
reasoned changes are tracked in [`CHANGELOG.md`](CHANGELOG.md). Current
status:

- ✅ Milestone 0 — foundation: core types, docs, full test suite
- ✅ Milestone 1 — API alignment: closures, `is_initialized`, `OptionExt`,
  `Maybe` ergonomics, `map_ok`
- ✅ Milestone 2 — publishability: MSRV, CI, changelog, doc tests
- 🔒 Milestone 3 — utility expansion only when a genuine use case appears

## Scope boundaries

To keep the crate valuable, tinyboost deliberately avoids: large data
structures already served by other crates, `std` reimplementations,
framework-style abstractions, magical or overly clever APIs, and broad feature
expansion without a clear use case.

## Contributing

Contributions are welcome when they follow the library's principles — see
[`docs/design-principles.md`](docs/design-principles.md) and
[`TODOS.md`](TODOS.md) for what's worth building. Every change needs:

- small and explicit APIs (no cleverness)
- strong docs and examples
- no unnecessary dependency growth
- test coverage

## License

MIT — see [`LICENSE`](LICENSE).
