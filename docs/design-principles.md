# Design principles

Every design decision in tinyboost follows a small set of hard constraints.
This document explains the reasoning, so future changes can be judged against
the same standard.

## 1. Small beats broad

A utility crate earns its place in a project by doing a few things so reliably
that developers stop re-implementing them. Every proposed addition must answer
one question: *will everyday Rust codebases genuinely re-implement this?* If
the answer is "maybe" or "it would be nice", the item stays out.

**Consequence:** the crate will probably never have more than a few dozen
public items. That is a feature, not an accident.

## 2. Explicit beats magical

Helpers should make the calling code *more* readable, not less. That rules
out:

- macros that hide control flow (`if let` sugar, async automata)
- syntax tricks that rely on snippets of serde/proc-macro magic
- clever operator overloading or Deref abuse to fake language features

The only accepted macro shape (if ever added) is `defer!`-style scope sugar,
which mirrors what the underlying type visibly does.

## 3. Zero third-party dependencies

`[dependencies]` is empty and stays empty. Anything requiring an external
crate either does not belong in tinyboost or must live behind an
optional feature the user explicitly opts into.

**Reasoning:** utility crates are pulled into *every* transitive closure down
the tree. The cost of one extra dependency is multiplied across an entire
ecosystem; the benefit is rarely worth it.

## 4. Rust idioms, not translated C++ patterns

Boost is inspiration for *which problems matter*, not for how types look.
tinyboost types adopt Rust conventions:

- construction via `new`, conversion via `From`/`Into`
- interior mutability via real primitives (`OnceLock`), never hand-rolled
- iteration and mutation via standard traits where applicable
- drop semantics via `Drop`, not lifetime-parameterized destructors

## 5. Naming that does not shadow `std`

Trait methods must never collide with beings that take priority: inherent
methods on the inner type win trait resolution in Rust. That is why
`ResultExt` offers `map_ok` (std has `map` / `map_err` as *inherent*
methods — a trait method named `map_err` would be unreachable dead code).
When picking names, test them against the inner type's inherent surface.

## 6. Panic-aware, allocation-conscious

Utilities participate in real code paths: guards must run during unwinding,
and lazy initialization must be correct for multi-threaded access. Where the
std primitive does the work, borrow it (`OnceLock`); hand-rolled atomics are
strongly discouraged here.

The crate aims to be allocation-free wherever practical. It is not yet
`no_std`; see `TODOS.md` Milestone 2 for the open evaluation.

## 7. Stability expectations

- Until **1.0**: additions are cheap, mutations of existing APIs are avoided
  where possible, removals require a strong reason and a changelog entry.
- After **1.0**: strict SemVer, deprecation cycle before removal, and MSRV
  bumps only with a documented justification.

## Anti-patterns to reject in code review

- "It's just one small dependency…"
- "This trait method has a great name — the fact std also has it as inherent
  is fine" (it is not; users will hit unreachable code)
- "Some users might want…" — build for the many codebases with the need, not
  the hypothetical few
- "Tests can come later" — no public change is merged without a test
