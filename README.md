# tinyboost

A niche Rust utility library inspired by the broad, practical philosophy of C++ Boost: small building blocks for everyday systems programming and application architecture.

This project aims to fill the gap between the core Rust standard library and the higher-level utilities that many teams end up re-implementing over and over again.

## Goals

- Provide reusable, dependency-light building blocks.
- Favor clarity and explicitness over magic.
- Keep the API small, composable, and predictable.
- Cover guard patterns, optional/fallback utilities, and useful value transformation helpers.

## Current modules

- `core::ScopeGuard` for RAII cleanup
- `core::LazyInit` for one-time initialization without custom macros
- `core::Maybe<T>` for a lightweight option-like container with explicit ergonomics
- `traits::ResultExt` for `tap_ok` and `tap_err` style control flow

## Example

```rust
use tinyboost::prelude::*;

let x = LazyInit::new(|| 42);
assert_eq!(x.get(), &42);

let maybe = Maybe::from(Some("hello"));
assert_eq!(maybe.unwrap_or("fallback"), "hello");

let result: Result<i32, &'static str> = Ok(7)
    .tap_ok(|value| assert_eq!(*value, 7))
    .tap_err(|_| panic!("unexpected error"));

assert!(result.is_ok());
```

## Why this exists

The official Rust standard library is excellent, but it intentionally stays narrow. tinyboost is meant to be a small collection of opinionated utilities that make product code easier to read and safer to maintain without pulling in a large ecosystem of dependencies.
