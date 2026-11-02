# Release & maintenance workflow

How changes move from idea to a published crate, and what must be true at
each gate.

## The pipeline at a glance

```
idea → TODOS.md entry → design check → implementation + tests → docs
     → fmt / clippy / test  → CHANGELOG.md → commit → push → CI green → tag
```

## Gate details

### 1. Design check (docs/design-principles.md)

Before writing code, answer the standard questions:

- Will everyday codebases genuinely re-implement this?
- Can it be done with zero third-party dependencies?
- Does the name shadow any inherent method on the target type?
- Is there a test the moment the API lands?

If any answer is "no" or "unsure", the idea goes to `TODOS.md` and waits.

### 2. Implementation + tests

- One type/trait per module; one `tests/<item>.rs` per public item.
- Tests must cover not just the happy path but also:
  - the *opposite variant* (hook does **not** fire),
  - panic/unwind semantics where relevant,
  - multi-threaded use where the type claims `Sync`.
- Follow the naming style recorded in `docs/testing.md`.

### 3. Docs

- Every public item gets a doc comment, ideally with a runnable example
  (a doc test). Use `cargo test --doc` to verify.
- Docs must include at least one *why*, not only the *how* — the crate's
  identity is explicitness.

### 4. Local gates

```sh
cargo fmt --all && cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-features
```

All three must be clean. CI runs the same commands plus the MSRV toolchain
(1.70).

### 5. Changelog

Add an entry under `CHANGELOG.md` — an `Unreleased` section gathers changes
until release time, then is folded into a dated version heading (Keep a
Changelog convention).

Categories: **Added** / **Changed** / **Deprecated** / **Removed** / **Fixed**
/ **Security**.

### 6. Versioning

Pre-1.0: any breaking change bumps the minor version and is called out in the
changelog under **Changed**. Post-1.0: strict SemVer, deprecation cycle
before removals.

### 7. Tag & publish

```sh
git tag v0.2.0
git push origin v0.2.0
cargo publish --dry-run    # then, when confident: cargo publish
```

Before publishing to crates.io:

- `cargo publish --dry-run` passes (packaging check)
- the docs at docs.rs build with `cargo doc --no-deps` (no broken intra-doc
  links; `cargo doc` reports them)
- README renders correctly and links land
- the version in `Cargo.toml` matches the tag and the changelog

## Maintenance checklist (periodic)

- [ ] `cargo audit` (when dependencies appear — currently none)
- [ ] Review `TODOS.md` pruning: remove stale ideas, promote validated ones
- [ ] Confirm the MSRV claim still holds on the oldest supported toolchain
- [ ] Ensure doc examples still compile and add value (`cargo test --doc`)
- [ ] Check test count and per-file coverage did not silently shrink
