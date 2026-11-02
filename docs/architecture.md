# Architecture

How the crate is laid out and why each piece lives where it does.

```
src/
├── lib.rs            crate root: re-exports the entire public API
├── prelude.rs        curated `use tinyboost::prelude::*` surface
├── core/
│   ├── mod.rs        module list for core types
│   ├── guard.rs      ScopeGuard        — RAII cleanup
│   ├── lazy.rs       LazyInit          — one-time initialization
│   └── maybe.rs      Maybe<T>          — explicit value-or-empty
└── traits/
    ├── mod.rs        module list for extension traits
    ├── result_ext.rs ResultExt         — tap_ok / tap_err / map_ok
    └── option_ext.rs OptionExt         — tap_some / tap_none / into_maybe
```

## Layer rules

1. **`core` never depends on `traits`.** The core types stand on their own;
   extension traits may reference core types (e.g. `OptionExt::into_maybe`
   returns a `Maybe`), but never the other way around. This keeps the two
   layers independently prunable.

2. **`lib.rs` is the single aggregation point.** Every public item is
   reachable as `tinyboost::Item` *and* as `tinyboost::prelude::Item`. The
   test suite asserts this from both paths so the re-exports cannot silently
   drift.

3. **One type (or trait) per file.** Grep-ability beats cleverness; the
   file name is the item name.

## Type notes

### `ScopeGuard<F>`

Built on `Drop` with `closure: Option<F>` so `dismiss()` can cancel the
cleanup without needing interior mutability. `F: FnOnce()` is the broadest
useful bound: closures may capture state, panic (which propagates from the
drop site — the documented behavior), and clean up in reverse creation order
through Rust's normal reverse-drop rules.

### `LazyInit<T, F = fn() -> T>`

A thin wrapper over `std::sync::OnceLock`. The initializer type is generic
(`F: Fn() -> T`) so callers can capture configuration by shared reference,
but falls back to the zero-cost `fn() -> T` pointer via the default type
parameter when no capture is needed.

`Fn` (not `FnOnce`) is deliberate: `get(&self)` receives `&self`, and
`OnceLock::get_or_init` may or may not run the closure depending on race
outcomes. A stored `Fn` keeps the semantics safe without `unsafe`.

### `Maybe<T>`

A two-variant enum mirroring `Option`. It currently exists as a stable anchor
point for "value-or-empty" semantics that the crate documents and tests
independently of `Option`; several combinators (`unwrap_or_else`, `contains`,
`as_ref`/`as_mut`) mirror `Option` so migration is mechanical. Its long-term
position is tracked as an open design question in `TODOS.md` Milestone 1 —
the crate is honest that this is a judgment call, not settled.

### `ResultExt` / `OptionExt`

Blanket-impl extension traits for `Result` and `Option`. Methods are
side-effect hooks (`tap_*`) that never mutate the value, plus a small number
of combinators that std does not offer under a non-shadowed name (`map_ok`).
The rule for adding a combinator here lives in
[design-principles.md](design-principles.md) #5.

## Testing architecture

Integration tests are split one file per public item (`scope_guard.rs`,
`lazy_init.rs`, `maybe.rs`, `result_ext.rs`, `option_ext.rs`), with
`basic.rs` as an all-API smoke suite. This mirrors the source layout so a
new type implies a new test file. Details in [testing.md](testing.md).

## Future evolution constraints

- `no_std` support requires replacing `OnceLock`; the abstraction cost must
  be weighed against ecosystem needs — tracked in `TODOS.md` Milestone 2.
- Any new category (iterators, coalescing, etc.) starts as a `traits/` or
  dedicated submodule *only after* the use case survives the review questions
  in [design-principles.md](design-principles.md).
