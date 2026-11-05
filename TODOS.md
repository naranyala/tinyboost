# tinyboost — Development TODO

A prioritized development plan. Items are grouped by milestone, roughly matching
the phases in `README.md`, but with concrete actionable steps.

Status markers: `- [ ]` pending, `- [x]` done, `- [~]` in progress.

---

## Milestone 0 — Stabilize the foundation (critical)

- [x] Fix `tests/basic.rs`, which currently does not compile:
  - `LazyInit::is_initialized()` does not exist — test rewritten to use the current API
  - `Result::map_ok` is not provided by `ResultExt` — test rewritten
  - `Maybe` test consumed values (`unwrap_or` takes `self`) before further assertions — reordered
- [x] Add a comprehensive integration test suite for the *existing* public API:
  - `tests/scope_guard.rs` — drop/dismiss/panic-unwind/ordering semantics
  - `tests/lazy_init.rs` — one-time init, stability, `into_inner`, thread safety
  - `tests/maybe.rs` — predicates, combinators, conversions, derived traits
  - `tests/result_ext.rs` — tap hooks on ok/err paths, value preservation, chaining
- [x] Ensure `cargo test`, `cargo fmt --check`, and `cargo clippy -- -D warnings` are clean

## Milestone 1 — API alignment (Phase 2 in README)

Small, non-breaking additions that make the API match its tested intent:

- [x] `LazyInit::is_initialized()` — cheap check via the inner `OnceLock`
- [x] `LazyInit` with closure-based init (`F: Fn() -> T` instead of `fn() -> T`) so initialization can capture state by shared reference. Struct is now `LazyInit<T, F = fn() -> T>`, keeping the old `fn`-pointer form usable via the default type parameter
- [x] `ResultExt::map_ok` (named to avoid shadowing by std's inherent `Result::map`/`map_err`, which always win trait-method resolution)
- [x] `OptionExt` trait with `tap_some` / `tap_none` and `into_maybe`
- [x] `Maybe` ergonomics pass: `as_ref()` / `as_mut()`, `contains`, `unwrap_or_else`

Deferred (needs a design decision — see Milestone 3):

- [ ] `LazyInit::try_get()` / fallible initialization (`fn() -> Result<T, E>`)
- [ ] Decide (and document) the distinct reason `Maybe<T>` exists vs `Option<T>` — currently it is a strict subset; either justify or fold it away

## Milestone 2 — Publishability (Phase 3 in README)

- [x] `rust-version = "1.70"` in `Cargo.toml` (MSRV — required by `OnceLock`)
- [x] GitHub Actions CI: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` on stable + MSRV
- [x] Documentation coverage and compile-checked usage examples in rustdoc
- [x] `CHANGELOG.md` following Keep a Changelog convention
- [x] `#![deny(missing_docs)]` and documentation for the complete public API
- [ ] Evaluate partial `no_std` support (all current utilities are alloc-free; only `OnceLock` is std-bound — document the constraint or abstract it)

## Milestone 3 — Utility expansion (only with a validated use case)

Ideas that fit the "small, explicit, zero-dependency" philosophy. Add only with a genuine use case; prune aggressively:

- [ ] `guard!` / `defer!` convenience macro for `ScopeGuard`
- [ ] `ScopeGuardOnUnwind` — cleanup only on panic, not on normal scope exit
- [ ] `take_mut` / `replace_with` helpers (mutate a `&mut T` with a fallible closure while keeping a default on panic)
- [ ] Fallback / coalescing helpers for `Result` (collect multiple errors into one)
- [ ] Iterator helpers: a tiny, curated set (e.g. `group_by_key`, `try_collect_if_ok`)
- [ ] More `Maybe<T>` accessors only as real needs appear

## Suggested next execution order

1. Resolve the current CI failure caused by the new missing-documentation lint. Add API docs for every public module, type, variant, and method; verify with `cargo doc --no-deps`, `cargo test --all-features`, and the MSRV CI job.
2. Decide whether `Maybe<T>` has a distinct semantic use from `Option<T>`. If not, deprecate it before 1.0 rather than expanding a duplicate API.
3. Evaluate `no_std` as a deliberate compatibility decision. `LazyInit` depends on `std::sync::OnceLock`; keep the crate std-only unless actual consumers need no_std and there is a sound design.
4. Add a new utility only after a concrete use case is documented, with tests and examples included in the same change.

## Explicit non-goals (do not do these)

- Large data structures already served by other crates
- Reimplementing `std`
- Framework-style abstractions or "clever" macros
- Dependency additions without a clear justification
- API surface growth without tests and docs
