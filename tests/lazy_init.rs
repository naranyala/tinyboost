//! Comprehensive tests for `LazyInit`.
//!
//! `LazyInit` currently accepts `fn() -> T` (a plain function pointer), so
//! initializers cannot capture test state. Per-test call counters are declared
//! as local statics inside each test and incremented from nested functions.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tinyboost::core::LazyInit;

#[test]
fn init_runs_exactly_once_across_many_gets() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() -> i32 {
        CALLS.fetch_add(1, Ordering::SeqCst);
        42
    }

    let lazy = LazyInit::new(init);
    for _ in 0..10 {
        assert_eq!(lazy.get(), &42);
    }
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[test]
fn get_returns_a_stable_reference() {
    static UNUSED: AtomicUsize = AtomicUsize::new(0);
    fn init() -> i32 {
        UNUSED.fetch_add(1, Ordering::SeqCst);
        42
    }

    let lazy = LazyInit::new(init);
    let first: *const i32 = lazy.get();
    let second: *const i32 = lazy.get();
    assert_eq!(
        first, second,
        "successive gets must return the same address"
    );
    assert_eq!(*lazy.get(), 42);
}

#[test]
fn get_consumes_nothing_and_supports_repeated_borrows() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() -> i32 {
        CALLS.fetch_add(1, Ordering::SeqCst);
        42
    }

    let lazy = LazyInit::new(init);
    let a = lazy.get();
    let b = lazy.get();
    assert_eq!(a, b);
    assert_eq!(*a, 42);
    assert_eq!(*b, 42);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[test]
fn init_is_not_run_when_value_is_never_accessed() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() -> u8 {
        CALLS.fetch_add(1, Ordering::SeqCst);
        1
    }

    let lazy: LazyInit<u8> = LazyInit::new(init);
    drop(lazy);
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
}

#[test]
fn into_inner_before_any_access_returns_none_and_does_not_init() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() -> u8 {
        CALLS.fetch_add(1, Ordering::SeqCst);
        1
    }

    let lazy: LazyInit<u8> = LazyInit::new(init);
    assert!(lazy.into_inner().is_none());
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
}

#[test]
fn into_inner_after_access_returns_some_exactly_once() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() -> i32 {
        CALLS.fetch_add(1, Ordering::SeqCst);
        42
    }

    let lazy = LazyInit::new(init);
    assert_eq!(*lazy.get(), 42);
    assert_eq!(lazy.into_inner(), Some(42));
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[test]
fn works_with_non_copy_types() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() -> String {
        CALLS.fetch_add(1, Ordering::SeqCst);
        String::from("computed")
    }

    let lazy = LazyInit::new(init);
    assert_eq!(lazy.get(), "computed");
    assert_eq!(lazy.get().len(), 8);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(lazy.into_inner(), Some(String::from("computed")));
}

#[test]
fn supports_zero_sized_types() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() {
        CALLS.fetch_add(1, Ordering::SeqCst);
    }

    let lazy = LazyInit::new(init);
    let _: &() = lazy.get();
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[test]
fn two_instances_do_not_share_state_or_initializers() {
    static CALLS_A: AtomicUsize = AtomicUsize::new(0);
    fn init_a() -> &'static str {
        CALLS_A.fetch_add(1, Ordering::SeqCst);
        "a"
    }

    static CALLS_B: AtomicUsize = AtomicUsize::new(0);
    fn init_b() -> &'static str {
        CALLS_B.fetch_add(1, Ordering::SeqCst);
        "b"
    }

    let a = LazyInit::new(init_a);
    let b = LazyInit::new(init_b);
    assert_eq!(a.get(), &"a");
    assert_eq!(b.get(), &"b");
    assert_eq!(CALLS_A.load(Ordering::SeqCst), 1);
    assert_eq!(CALLS_B.load(Ordering::SeqCst), 1);
}

#[test]
fn init_runs_exactly_once_across_many_threads() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() -> u64 {
        CALLS.fetch_add(1, Ordering::SeqCst);
        0xABCD
    }

    let lazy = Arc::new(LazyInit::new(init));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let lazy = Arc::clone(&lazy);
            std::thread::spawn(move || *lazy.get())
        })
        .collect();

    let values: Vec<u64> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert!(values.iter().all(|v| *v == 0xABCD));
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

// --- is_initialized ----------------------------------------------------------

#[test]
fn is_initialized_is_false_before_access() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() -> u8 {
        CALLS.fetch_add(1, Ordering::SeqCst);
        1
    }

    let lazy: LazyInit<u8> = LazyInit::new(init);
    assert!(!lazy.is_initialized());
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
}

#[test]
fn is_initialized_is_true_after_access() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() -> i32 {
        CALLS.fetch_add(1, Ordering::SeqCst);
        42
    }

    let lazy = LazyInit::new(init);
    assert!(!lazy.is_initialized());
    let _ = lazy.get();
    assert!(lazy.is_initialized());
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[test]
fn is_initialized_never_triggers_initialization() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() -> u8 {
        CALLS.fetch_add(1, Ordering::SeqCst);
        1
    }

    let lazy: LazyInit<u8> = LazyInit::new(init);
    for _ in 0..5 {
        assert!(!lazy.is_initialized());
    }
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
    assert!(lazy.into_inner().is_none());
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
}

// --- closure-based initializers ----------------------------------------------

#[test]
fn init_accepts_non_capturing_closures_and_runs_once() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let lazy = LazyInit::new(|| {
        CALLS.fetch_add(1, Ordering::SeqCst);
        99
    });

    assert_eq!(lazy.get(), &99);
    assert_eq!(lazy.get(), &99);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[test]
fn init_accepts_closures_capturing_state_by_reference() {
    // The closure may capture local state by shared reference; `LazyInit`
    // keeps the capture alive and the value stays one-time.
    let mut config = 10;
    config += 5;

    let lazy = LazyInit::new(|| config * 2);
    assert_eq!(lazy.get(), &30);
    assert_eq!(lazy.get(), &30);
}

#[test]
fn closure_captured_counter_counts_exactly_one_init() {
    // State capture without statics: the AtomicUsize lives in the closure,
    // borrowed for the lifetime of the LazyInit.
    let calls = AtomicUsize::new(0);
    {
        let lazy = LazyInit::new(|| {
            calls.fetch_add(1, Ordering::SeqCst);
            calls.load(Ordering::SeqCst)
        });

        assert_eq!(lazy.get(), &1);
        assert_eq!(lazy.get(), &1);
    }
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn fn_pointer_initializers_still_work_in_the_generic_form() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() -> u8 {
        CALLS.fetch_add(1, Ordering::SeqCst);
        7
    }
    let explicit: LazyInit<u8, fn() -> u8> = LazyInit::new(init as fn() -> u8);
    assert_eq!(explicit.get(), &7);

    // Type annotation using the default type parameter (`fn() -> T`).
    let annotated: LazyInit<u8> = LazyInit::new(init);
    assert_eq!(annotated.get(), &7);
}

#[test]
fn is_initialized_visible_across_threads() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    fn init() -> u64 {
        CALLS.fetch_add(1, Ordering::SeqCst);
        5
    }

    let lazy = Arc::new(LazyInit::new(init));
    assert!(!lazy.is_initialized());
    assert_eq!(*lazy.get(), 5);

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let lazy = Arc::clone(&lazy);
            std::thread::spawn(move || {
                let _ = *lazy.get();
                lazy.is_initialized()
            })
        })
        .collect();

    for handle in handles {
        assert!(handle.join().unwrap());
    }
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}
