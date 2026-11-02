use tinyboost::prelude::*;

#[test]
fn lazy_init_runs_once_and_stays_stable() {
    let calls = std::sync::atomic::AtomicUsize::new(0);

    let lazy = LazyInit::new(|| {
        calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        42
    });

    assert_eq!(lazy.get(), &42);
    assert_eq!(lazy.get(), &42);
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(lazy.is_initialized());
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

    assert_eq!(some.unwrap_or(0), 7);
    assert_eq!(none.unwrap_or(9), 9);
    assert!(some.is_some());
    assert!(none.is_none());
}

#[test]
fn result_ext_helpers_work_for_ok_and_err_paths() {
    let ok: Result<i32, &str> = Ok(5)
        .map_ok(|value| value + 2)
        .tap_ok(|value| assert_eq!(*value, 7));

    let err: Result<i32, &str> = Err("oops")
        .map_err(|value| format!("{value}!"))
        .tap_err(|value| assert_eq!(*value, "oops!"));

    assert_eq!(ok, Ok(7));
    assert_eq!(err, Err(String::from("oops!")));
}
