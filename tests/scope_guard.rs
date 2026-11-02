//! Comprehensive tests for `ScopeGuard`.

use tinyboost::core::ScopeGuard;

#[test]
fn guard_runs_closure_exactly_once_on_drop() {
    let count = std::rc::Rc::new(std::cell::Cell::new(0u32));

    {
        let count = count.clone();
        let _guard = ScopeGuard::new(move || {
            count.set(count.get() + 1);
        });
    }

    assert_eq!(count.get(), 1);
}

#[test]
fn disarm_through_dismiss_prevents_execution() {
    let ran = std::rc::Rc::new(std::cell::Cell::new(false));

    {
        let ran = ran.clone();
        let guard = ScopeGuard::new(move || {
            ran.set(true);
        });
        guard.dismiss();
    }

    assert!(!ran.get(), "dismissed guard must not run its closure");
}

#[test]
fn guard_returns_nothing_but_closure_side_effects_remain() {
    let mut log: Vec<&'static str> = Vec::new();
    {
        let _g = ScopeGuard::new(|| log.push("cleaned"));
    }
    assert_eq!(log, vec!["cleaned"]);
}

#[test]
fn guards_on_the_same_scope_run_in_reverse_creation_order() {
    // Drop order is reverse declaration order, and the guard must respect it.
    let order = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));

    {
        let o1 = order.clone();
        let _first = ScopeGuard::new(move || o1.borrow_mut().push("first"));

        let o2 = order.clone();
        let _second = ScopeGuard::new(move || o2.borrow_mut().push("second"));
    }

    assert_eq!(order.borrow().clone(), ["second", "first"]);
}

#[test]
fn guard_runs_during_panic_unwinding() {
    let ran = std::rc::Rc::new(std::cell::Cell::new(false));
    let ran_in_panic = ran.clone();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _guard = ScopeGuard::new(move || {
            ran_in_panic.set(true);
        });
        panic!("boom");
    }));

    assert!(result.is_err());
    assert!(ran.get(), "guard must run while the panic unwinds");
}

#[test]
fn guard_dismissed_before_panic_does_not_run() {
    let ran = std::rc::Rc::new(std::cell::Cell::new(false));
    let ran_in_panic = ran.clone();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let guard = ScopeGuard::new(move || {
            ran_in_panic.set(true);
        });
        guard.dismiss();
        panic!("boom");
    }));

    assert!(result.is_err());
    assert!(!ran.get(), "dismissed guard must stay inert even on unwind");
}

#[test]
fn guard_can_be_moved_across_functions() {
    struct Timer {
        guard: Option<ScopeGuard<Box<dyn FnOnce()>>>,
    }

    let ticks = std::rc::Rc::new(std::cell::Cell::new(0));

    {
        let t = ticks.clone();
        let mut timer = Timer {
            guard: Some(ScopeGuard::new(Box::new(move || {
                t.set(t.get() + 100);
            }) as Box<dyn FnOnce()>)),
        };

        // Move the guard out of the struct and let it drop in this scope.
        let moved = timer.guard.take().unwrap();
        drop(moved);
    }

    assert_eq!(ticks.get(), 100);
}

#[test]
fn guard_supports_non_copy_capture_types() {
    let buffer = std::sync::Arc::new(std::sync::Mutex::new(String::new()));

    {
        let buffer = buffer.clone();
        let _g = ScopeGuard::new(move || {
            buffer.lock().unwrap().push_str("released");
        });
    }

    assert_eq!(buffer.lock().unwrap().as_str(), "released");
}
