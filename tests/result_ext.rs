//! Comprehensive tests for the `ResultExt` trait (`tap_ok` / `tap_err`).

use tinyboost::traits::ResultExt;

// --- tap_ok ----------------------------------------------------------------

#[test]
fn tap_ok_calls_the_closure_on_ok() {
    let seen = std::cell::Cell::new(0);

    let result: Result<i32, String> = Ok(10).tap_ok(|v| seen.set(*v));

    assert_eq!(seen.get(), 10);
    assert_eq!(result, Ok(10));
}

#[test]
fn tap_ok_does_not_call_the_closure_on_err() {
    let called = std::cell::Cell::new(false);

    let result: Result<i32, String> = Err("bad".to_string());
    let result = result.tap_ok(|_| called.set(true));

    assert!(!called.get(), "tap_ok must not run for Err");
    assert_eq!(result, Err(String::from("bad")));
}

// --- tap_err ---------------------------------------------------------------

#[test]
fn tap_err_calls_the_closure_on_err() {
    let seen = std::cell::Cell::new(String::new());

    let result: Result<i32, String> = Err("boom".to_string());
    let result = result.tap_err(|e| seen.set(e.clone()));

    assert_eq!(seen.take().as_str(), "boom");
    assert_eq!(result, Err(String::from("boom")));
}

#[test]
fn tap_err_does_not_call_the_closure_on_ok() {
    let called = std::cell::Cell::new(false);

    let result: Result<i32, String> = Ok(1);
    let result = result.tap_err(|_| called.set(true));

    assert!(!called.get(), "tap_err must not run for Ok");
    assert_eq!(result, Ok(1));
}

// --- value preservation ----------------------------------------------------

#[test]
fn taps_never_change_the_value_or_error() {
    let ok: Result<u32, String> = Ok(7).tap_ok(|_| {}).tap_err(|_| {}).tap_ok(|_| {});
    assert_eq!(ok, Ok(7));

    let err: Result<u32, String> = Err("e".to_string())
        .tap_err(|_| {})
        .tap_ok(|_| {})
        .tap_err(|_| {});
    assert_eq!(err, Err(String::from("e")));
}

#[test]
fn tap_ok_then_std_map_composes() {
    let result: Result<u32, String> = Ok(3).tap_ok(|v| assert_eq!(*v, 3)).map(|v| v * v);

    assert_eq!(result, Ok(9));
}

#[test]
fn tap_err_with_error_transformation_still_preserves_original_for_caller() {
    // The tap sees the original error; the returned value is unchanged, so the
    // caller can transform it afterwards.
    let result: Result<i32, String> = Err("oops".to_string())
        .tap_err(|e| assert!(e.contains("oops")))
        .map_err(|e| format!("{e}!"));

    assert_eq!(result, Err(String::from("oops!")));
}

// --- chaining & types ------------------------------------------------------

#[test]
fn taps_can_be_chained_and_interleaved() {
    let ok_steps = std::cell::Cell::new(0);
    let err_steps = std::cell::Cell::new(0);

    let ok: Result<i32, String> = Ok(1)
        .tap_ok(|_| ok_steps.set(ok_steps.get() + 1))
        .tap_err(|_| err_steps.set(err_steps.get() + 1))
        .tap_ok(|_| ok_steps.set(ok_steps.get() + 1));

    assert_eq!(ok, Ok(1));
    assert_eq!(ok_steps.get(), 2);
    assert_eq!(err_steps.get(), 0);

    let err: Result<i32, String> = Err("x".into())
        .tap_ok(|_| ok_steps.set(ok_steps.get() + 10))
        .tap_err(|_| err_steps.set(err_steps.get() + 1));

    assert_eq!(err, Err(String::from("x")));
    assert_eq!(ok_steps.get(), 2);
    assert_eq!(err_steps.get(), 1);
}

#[test]
fn works_with_different_error_types() {
    // &'static str errors
    let result: Result<i32, &'static str> = Err("static");
    let seen = std::cell::Cell::new("");
    let result = result.tap_err(|e| seen.set(e));
    assert_eq!(seen.get(), "static");
    assert_eq!(result, Err("static"));

    // custom error structs
    #[derive(Debug, PartialEq, Clone)]
    struct Custom(u8);

    let result: Result<(), Custom> = Err(Custom(3));
    let seen = std::cell::Cell::new(None);
    let result = result.tap_err(|e| seen.set(Some(e.clone())));
    assert_eq!(seen.take(), Some(Custom(3)));
    assert_eq!(result, Err(Custom(3)));
}

#[test]
fn works_with_reference_and_unit_payloads() {
    // Unit ok type
    let result: Result<(), String> = Ok(()).tap_ok(|_| {});
    assert_eq!(result, Ok(()));

    // Reference ok type
    let data = String::from("payload");
    let result: Result<&String, String> = Ok(&data).tap_ok(|v| assert_eq!(v.len(), 7));
    assert_eq!(result, Ok(&data));
}

// --- map_ok -----------------------------------------------------------------

#[test]
fn map_ok_transforms_the_success_value() {
    let result: Result<i32, String> = Ok(2);
    assert_eq!(result.map_ok(|v| v * v), Ok(4));
}

#[test]
fn map_ok_passes_the_error_through_unchanged() {
    let result: Result<i32, String> = Err("keep".to_string());
    let mapped: Result<String, String> = result.map_ok(|v| v.to_string());
    assert_eq!(mapped, Err(String::from("keep")));
}

#[test]
fn map_ok_can_change_the_success_type() {
    let result: Result<u16, u8> = Ok(300);
    let mapped: Result<String, u8> = result.map_ok(|v: u16| format!("n{v}"));
    assert_eq!(mapped, Ok(String::from("n300")));
}

#[test]
fn map_ok_does_not_call_the_closure_on_err() {
    let called = std::cell::Cell::new(false);

    let result: Result<i32, i32> = Err(1).map_ok(|_: i32| {
        called.set(true);
        0
    });

    assert_eq!(result, Err(1));
    assert!(!called.get());
}

#[test]
fn map_ok_composes_with_taps_in_a_chain() {
    let ok_steps = std::cell::Cell::new(0);

    let result: Result<i32, String> = Ok(2)
        .tap_ok(|_| ok_steps.set(ok_steps.get() + 1))
        .map_ok(|v| v + 3)
        .tap_ok(|_| ok_steps.set(ok_steps.get() + 1))
        .map_ok(|v| v * 2);

    assert_eq!(result, Ok(10));
    assert_eq!(ok_steps.get(), 2);
}

// --- trait reachability ------------------------------------------------------

#[test]
fn trait_is_reachable_through_all_public_paths() {
    // The same behavior must apply whether the trait is imported from
    // `tinyboost::traits::ResultExt` or `tinyboost::prelude::ResultExt`.
    use tinyboost::prelude::ResultExt as PreludeExt;

    let a: Result<u8, u8> = Ok(1).tap_ok(|_| {});
    let b: Result<u8, u8> = tinyboost::prelude::ResultExt::tap_ok(Ok(1), |_| {});
    let _c: Result<u8, u8> = PreludeExt::tap_ok(Ok(1), |_| {});

    assert_eq!(a, b);
}
