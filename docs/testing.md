# Testing

The test suite is the contract for tinyboost's public API. Every public item
is covered in depth, including panic paths and multi-threaded behavior.

## Running everything

```sh
cargo test                                  # all tests + doc tests
cargo test --test lazy_init                 # one module's suite
cargo test init_runs_exactly_once           # one test, by name
cargo test -- --nocapture                   # show println!/assert output
```

Gates (all also enforced in CI, see `.github/workflows/ci.yml`):

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-features
```

## Suite layout

Integration tests live in `tests/`, one file per public item, mirroring
`src/` layout:

| File | Covers | Count |
|---|---|---|
| `basic.rs` | Smoke tests over the whole public API through the prelude | 4 |
| `scope_guard.rs` | `ScopeGuard` | 8 |
| `lazy_init.rs` | `LazyInit` | 18 |
| `maybe.rs` | `Maybe<T>` | 27 |
| `result_ext.rs` | `ResultExt` | 13 |
| `option_ext.rs` | `OptionExt` | 16 |
| doc tests (`cargo test --doc`) | Compile-checked rustdoc examples | grows with docs |

**Rule of thumb for contributors:** a new public item ships together with a
new `tests/<item>.rs` file. Bug fixes should prefer a failing test before the
fix.

## What is covered, per type

### `ScopeGuard`
- runs exactly once on drop; `dismiss()` cancels it
- runs during panic unwinding; stays inert if dismissed before the panic
- reverse creation order in a single scope (Rust drop rules)
- can be moved across functions and structure boundaries
- works with non-`Copy`, heap-backed capture types

### `LazyInit`
- initializer runs exactly once across many gets and across many threads
- successful value initialization is stable (same address per get)
- `into_inner()` is `None` before init (without triggering it) and `Some`
  after
- `is_initialized()` never triggers initialization
- closures capturing state, ZSTs, non-`Copy` values, and `fn` pointers all
  work through the generic `F: Fn() -> T`

### `Maybe<T>`
- predicates for both variants; `map`/`and_then` including the
  closure-not-called rule for `None`
- all fallback paths (`unwrap_or`, `unwrap_or_default`, `unwrap_or_else`)
- `Option <-> Maybe` round-trips
- every derived trait (`Clone`, `Debug`, `PartialEq`, `Ord`, `Hash`),
  including the `Some`-before-`None` ordering forced by variant declaration
  order

### `ResultExt` / `OptionExt`
- `tap_*` fire on exactly the matching variant and never change the value
- `tap_*` / `map_ok` compose into longer chains with countable side effects
- closures are *not* invoked on the opposite variant
- traits are reachable from `tinyboost::traits::*` *and* `tinyboost::prelude`

## Testing conventions used in this suite

1. **Statics over captures for fn-derived state** where the test exercises
   plain-function initializers; local counters in closures where
   capability-under-test is capture itself.
2. **Per-test local `static` counters.** Tests run in parallel; shared
   module-level counters would race. Each test owns its counter.
3. **`std::panic::catch_unwind` + `AssertUnwindSafe`** to verify guard
   behavior during unwinding without failing the test run.
4. **Pointer-stability assertions** (`*const T` equality) where reference
   stability is part of the contract.
5. **`Cell`/`Rc`/`Arc` observation** to track how many times a hook fires
   without relying on side channels like `println!`.
6. **Explicit type annotations on `Result` chains** where inference through
   `map_ok` may be ambiguous in test expressions (`|_: i32|`).

## Adding a test

1. Pick the file matching the type; create one if the type is new.
2. Follow the naming convention `<behavior>_<expectation>` (e.g.
   `dismiss_through_dismiss_prevents_execution`,
   `init_runs_exactly_once_across_many_threads`).
3. Keep one assertion cluster per test — readable failure messages matter
   more than compact Tests.
4. Run the full gates (`fmt`, `clippy`, `test`) before pushing.
