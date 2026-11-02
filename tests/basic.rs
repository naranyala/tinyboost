//! Smoke tests over the whole public API surface.
//!
//! Deep coverage lives in the per-module integration tests:
//! `scope_guard.rs`, `lazy_init.rs`, `maybe.rs`, `result_ext.rs`,
//! `option_ext.rs`.

use tinyboost::prelude::*;

#[test]
fn lazy_init_runs_once_and_stays_stable() {
    let calls = std::sync::atomic::AtomicUsize::new(0);

    let lazy = LazyInit::new(|| {
        calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        42
    });

    assert!(!lazy.is_initialized());
    assert_eq!(lazy.get(), &42);
    assert_eq!(lazy.get(), &42);
    assert!(lazy.is_initialized());
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 1);
}

#[test]
fn scope_guard_runs_on_drop() {
    let flag = std::rc::Rc::new(std::cell::Cell::new(false));
    let flag_clone = flag.clone();

    {
        let _guard = ScopeGuard::new(move || {
            flag_clone.set(true);
        });
    }

    assert!(flag.get());
}

#[test]
fn maybe_round_trip_and_fallback_behave() {
    let some: Maybe<i32> = Some(7).into();
    let none: Maybe<i32> = None.into();

    assert_eq!(some.clone().unwrap_or_default(), 7);
    assert_eq!(none.clone().unwrap_or(9), 9);
    assert!(some.is_some());
    assert!(none.is_none());
}

#[test]
fn result_ext_helpers_work_for_ok_and_err_paths() {
    let ok: Result<i32, String> = Ok(5)
        .tap_err(|_| panic!("tap_err must not fire on Ok"))
        .tap_ok(|value| assert_eq!(*value, 5))
        .map_ok(|value| value * 2);

    let err: Result<i8, i8> = Err(3)
        .tap_ok(|_| panic!("tap_ok must not fire on Err"))
        .map_ok(|value: i8| value + 1);

    assert_eq!(ok, Ok(10));
    assert_eq!(err, Err(3));
}
