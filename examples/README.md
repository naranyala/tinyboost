# tinyboost examples

These runnable examples show the crate's public API in small, self-contained
programs. Run them from the repository root:

```sh
cargo run --example scope_guard
cargo run --example lazy_config
cargo run --example maybe_fallback
cargo run --example result_option_hooks
```

- `scope_guard` shows cleanup on normal scope exit and canceling cleanup with
  `dismiss()`.
- `lazy_config` shows a `LazyInit` closure capturing configuration and reusing
  its initialized value.
- `maybe_fallback` shows converting from `Option`, borrowing the value, and
  providing an explicit fallback.
- `result_option_hooks` shows logging/observation with `tap_ok`, `tap_err`,
  `tap_some`, and `tap_none`, while preserving the original values.

All examples use only tinyboost and the Rust standard library. Check all of them
in CI or locally with `cargo test --all-targets`.
