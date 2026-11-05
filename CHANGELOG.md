# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/) and the project adheres to
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Fixed

- Document the public API and enable `#![deny(missing_docs)]` to catch future documentation gaps.

## [0.1.0] — 2026-11-02

### Added

- `ScopeGuard` — RAII-style closure run at scope exit, with `dismiss()`.
- `LazyInit` — one-time lazy initialization on first access
  (`get`, `is_initialized`, `into_inner`).
- `Maybe<T>` — explicit value-or-empty wrapper with combinators
  (`map`, `and_then`, `unwrap_or`, `unwrap_or_default`, `unwrap_or_else`,
  `as_ref`, `as_mut`, `contains`) and `Option` conversions.
- `ResultExt` — `tap_ok` / `tap_err` side-effect hooks and `map_ok`.
- `OptionExt` — `tap_some` / `tap_none` side-effect hooks and `into_maybe`.
- `tinyboost::prelude` — curated re-exports.
- Integration test suite per module, smoke tests, CI (fmt, clippy, test on
  stable + MSRV 1.70).
