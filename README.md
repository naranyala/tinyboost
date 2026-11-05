# tinyboost

[![CI](https://github.com/naranyala/tinyboost/actions/workflows/ci.yml/badge.svg)](https://github.com/naranyala/tinyboost/actions/workflows/ci.yml)
[![License: MIT](LICENSE)
[![MSRV: 1.70](https://img.shields.io/badge/MSRV-1.70-blue)](#rust-version)

**tinyboost is a small, dependency-free Rust crate of explicit everyday utilities.** It offers scope-exit cleanup, lazy one-time initialization, an optional-value wrapper, and `Option`/`Result` observation hooks. It aims to fill a narrow gap between `std` and application code—not to replace the standard library or become a kitchen sink.

## What the crate offers

| Utility | What it helps with | Main API |
| --- | --- | --- |
| [`ScopeGuard`](#scopeguard) | Run cleanup when a scope exits; dismiss it if cleanup is no longer needed. | `new`, `dismiss` |
| [`LazyInit`](#lazyinit) | Compute a value the first time it is accessed and reuse it thereafter. | `new`, `get`, `is_initialized`, `into_inner` |
| [`Maybe<T>`](#maybet) | Represent a value-or-empty state with a named type that converts to and from `Option<T>`. | `map`, `and_then`, `as_ref`, fallback methods |
| [`ResultExt`](#resultext-and-optionext) | Observe success or error paths, or map only the success value. | `tap_ok`, `tap_err`, `map_ok` |
| [`OptionExt`](#resultext-and-optionext) | Observe present or absent values, or convert to `Maybe<T>`. | `tap_some`, `tap_none`, `into_maybe` |

All items are available from the crate root. The commonly used types and traits are also re-exported from `tinyboost::prelude`.

## Add tinyboost to a project

For a local checkout, use a path dependency:

```toml
[dependencies]
tinyboost = { path = "../tinyboost" }
```

For a crates.io release, use its published version:

```toml
[dependencies]
tinyboost = "0.1"
```

## Examples

The repository includes runnable programs demonstrating the public API:

```sh
cargo run --example scope_guard
cargo run --example lazy_config
cargo run --example maybe_fallback
cargo run --example result_option_hooks
```

See [`examples/README.md`](examples/README.md) for what each one demonstrates. These are small, self-contained illustrations; use [`cargo test --all-targets`](#development) to compile the examples as part of the full test suite.

### `ScopeGuard`

A guard runs its `FnOnce` cleanup closure on drop, including during panic unwinding. Dismiss it when ownership or cleanup responsibility has moved elsewhere.

```rust
use std::cell::Cell;
use std::rc::Rc;
use tinyboost::ScopeGuard;

let cleaned = Rc::new(Cell::new(false));
let cleanup_flag = Rc::clone(&cleaned);
{
    let _guard = ScopeGuard::new(move || cleanup_flag.set(true));
    // Work with the resource here.
}
assert!(cleaned.get());

let guard = ScopeGuard::new(|| println!("this will not run"));
guard.dismiss();
```

### `LazyInit`

`LazyInit` stores an initializer and a `OnceLock`. The initializer may be a function pointer or a closure that captures shared configuration. `get()` initializes on first access; later calls return the same value.

```rust
use tinyboost::LazyInit;

let environment = String::from("development");
let service_url = LazyInit::new(|| format!("https://{environment}.example.test"));

assert!(!service_url.is_initialized());
assert_eq!(service_url.get(), "https://development.example.test");
assert!(service_url.is_initialized());
```

`LazyInit` can be shared across threads when its value and initializer satisfy the relevant `Send`/`Sync` bounds.

### `Maybe<T>`

`Maybe<T>` gives an optional value a crate-specific type and provides familiar mapping, borrowing, and fallback methods. It converts both ways with `Option<T>`. Its behavior currently overlaps with `Option<T>`, so use standard `Option` when a separate named type is not useful to your code.

```rust
use tinyboost::Maybe;

let supplied: Option<String> = Some(String::from("Ada"));
let name: Maybe<String> = supplied.into();
let greeting_name = name
    .as_ref()
    .map(|value| value.trim())
    .and_then(|value| Maybe::from((!value.is_empty()).then_some(value)))
    .unwrap_or_else(|| "guest");

assert_eq!(greeting_name, "Ada");
```

### `ResultExt` and `OptionExt`

The `tap_*` methods run a side effect only for the matching variant and return the original value unchanged. `map_ok` transforms a `Result`'s success value while preserving its error. Import the extension traits (or use the prelude) to make their methods available.

```rust
use tinyboost::prelude::*;

let port: Result<u16, &str> = Ok(8080);
let address = port
    .tap_ok(|port| println!("loaded port {port}"))
    .tap_err(|error| eprintln!("could not load port: {error}"))
    .map_ok(|port| format!("127.0.0.1:{port}"));
assert_eq!(address.as_deref(), Ok("127.0.0.1:8080"));

let host = Some("localhost")
    .tap_some(|host| println!("using {host}"))
    .tap_none(|| eprintln!("using default host"));
assert_eq!(host, Some("localhost"));
```

## Design and boundaries

- **Small and explicit:** a few composable tools, with no framework or hidden control flow.
- **Zero third-party dependencies:** the crate currently uses only the Rust standard library.
- **Rust-first:** these utilities complement standard types; they do not aim to reimplement `std` or translate C++ APIs directly.
- **Standard-library requirement:** this crate is currently `std`-based; `LazyInit` uses `std::sync::OnceLock`. It does not currently promise `no_std` support.
- **Curated scope:** proposed additions are considered only when there is a concrete use case, tests, and documentation.

## Rust version

The declared minimum supported Rust version is **1.70**, matching the use of `std::sync::OnceLock`. CI is intended to check the MSRV and stable Rust.

## Development

From the repository root, run the formatting, lint, test, and documentation checks:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets --all-features
cargo doc --no-deps
```

The integration tests are in `tests/`; runnable examples are in `examples/`. See [`docs/testing.md`](docs/testing.md) for test conventions and [`docs/architecture.md`](docs/architecture.md) for the module layout. Development decisions and deferred ideas are tracked in [`TODOS.md`](TODOS.md), and release notes are in [`CHANGELOG.md`](CHANGELOG.md).

## Contributing

Keep changes small and idiomatic. New public API should have focused tests and documentation, and should not add dependencies without a clear reason. See [`docs/design-principles.md`](docs/design-principles.md) before proposing a new utility.

## License

MIT. See [`LICENSE`](LICENSE).
