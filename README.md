# tinyboost

A small, explicit Rust utility crate inspired by the practical breadth of C++ Boost without trying to become a giant collection of everything.

This project exists to provide the kinds of reusable building blocks that many Rust codebases keep re-implementing: cleanup guards, lazy initialization, explicit value-or-empty wrappers, and result/option ergonomics that remain small, clear, and dependency-light.

## Why this crate exists

The Rust standard library is excellent, but intentionally narrow. It focuses on core semantics and the most common primitives. tinyboost fills the gap between `std` and real-world application code by providing a curated layer of small, opinionated utilities.

This crate is designed around the belief that:

- many applications need predictable, explicit helpers
- small utilities are better than broad frameworks
- a narrow, stable API is safer than a huge “kitchen sink” crate
- Rust code should read clearly, not hide behavior behind clever abstraction

## Goals

- stay small and predictable
- favor explicitness over magic
- provide utility patterns that are genuinely useful in everyday Rust code
- keep the dependency footprint as close to zero as practical
- keep the design aligned with Rust idioms, not C++ patterns translated directly

## Design philosophy

tinyboost is intentionally not trying to be:

- a replacement for `std`
- a complete Boost clone
- a framework for application architecture
- a giant library of unrelated helpers

Instead, it focuses on a few foundational utility categories:

- cleanup and scope lifecycle helpers
- lazy, one-time initialization patterns
- explicit optional/value wrappers
- result and option augmentation for common control-flow ergonomics
- small, composable helpers that improve code clarity without large dependencies

## Current public API

The current crate includes a small, coherent set of utilities:

- `ScopeGuard` for RAII-style cleanup at scope exit
- `LazyInit` for one-time lazy initialization
- `Maybe<T>` for explicit value-or-empty semantics
- `ResultExt` for ergonomic result hooks and mapping helpers

## Installation

Add to your project:

```toml
[dependencies]
tinyboost = "0.1"
```

## Quick start

```rust
use tinyboost::prelude::*;

let value = LazyInit::new(|| 42);
assert_eq!(value.get(), &42);
assert_eq!(value.get(), &42);

let item: Maybe<i32> = Some(7).into();
assert_eq!(item.unwrap_or(0), 7);

let result: Result<i32, &str> = Ok(5)
    .map_ok(|n| n + 2)
    .tap_ok(|n| assert_eq!(*n, 7));

assert_eq!(result, Ok(7));
```

## Examples

### RAII cleanup with `ScopeGuard`

```rust
use tinyboost::ScopeGuard;

let flag = std::rc::Rc::new(std::cell::Cell::new(false));
let flag_clone = flag.clone();

{
    let _guard = ScopeGuard::new(move || {
        flag_clone.set(true);
    });
    // scope ends here; cleanup runs automatically
}

assert!(flag.get());
```

### Lazy initialization

```rust
use tinyboost::LazyInit;

let init_count = std::sync::atomic::AtomicUsize::new(0);
let value = LazyInit::new(|| {
    init_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    "hello"
});

assert_eq!(value.get(), &"hello");
assert_eq!(value.get(), &"hello");
assert_eq!(init_count.load(std::sync::atomic::Ordering::SeqCst), 1);
```

### Explicit optional semantics

```rust
use tinyboost::Maybe;

let present: Maybe<i32> = Some(9).into();
let absent: Maybe<i32> = None.into();

assert_eq!(present.unwrap_or(0), 9);
assert_eq!(absent.unwrap_or(42), 42);
```

### Result extension ergonomics

```rust
use tinyboost::ResultExt;

let result: Result<i32, &str> = Ok(5)
    .map_ok(|n| n + 1)
    .tap_ok(|n| assert_eq!(*n, 6));

assert_eq!(result, Ok(6));
```

## Crate roadmap

The crate is intentionally scoped to a few durable utility families:

### Phase 1: Foundation

- minimal utility crate skeleton
- explicit philosophy and design constraints
- core types (`ScopeGuard`, `LazyInit`, `Maybe`, `ResultExt`)
- documentation and public examples

### Phase 2: Core ergonomics

- refine `ResultExt` and `OptionExt`
- add missing convenience methods
- strengthen conversion semantics
- add tests for core edge cases

### Phase 3: Validation and polish

- CI workflow for formatting, linting, and tests
- docs and example coverage
- semver expectations and compatibility notes
- maintainability and contributor guidance

### Phase 4: Utility expansion

- iterator-level helper functions
- more small combinators and fallback semantics
- improved optional-value ergonomics
- continued narrowing of scope to avoid API bloat

## Planned future work

The library may eventually grow to include:

- `OptionExt` for parity with `ResultExt`
- more `Maybe<T>` convenience accessors
- iterator helper extensions
- fallback and coalescing utilities
- optional `no_std` / `alloc`-driven design refinements
- additional result/option combinators only if they genuinely improve clarity

## Scope boundaries

To keep the crate valuable, tinyboost deliberately avoids:

- large data structures that are already available elsewhere
- repeated `std` reimplementations
- framework-style abstractions
- magical or overly clever APIs
- broad feature expansion without a clear use case

## Contributing

Contributions are welcome if they follow the library’s principles:

- small and explicit APIs
- strong docs and examples
- no unnecessary dependency growth
- test coverage for all public changes
- consistent Rust idioms

## License

MIT
