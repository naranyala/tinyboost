//! Comprehensive tests for the `OptionExt` trait (`tap_some` / `tap_none` /
//! `into_maybe`).

use tinyboost::prelude::*;

// --- tap_some ---------------------------------------------------------------

#[test]
fn tap_some_calls_the_closure_on_some() {
    let seen = std::cell::Cell::new(0);

    let option: Option<i32> = Some(10).tap_some(|v| seen.set(*v));

    assert_eq!(seen.get(), 10);
    assert_eq!(option, Some(10));
}

#[test]
fn tap_some_does_not_call_the_closure_on_none() {
    let called = std::cell::Cell::new(false);

    let option: Option<i32> = None.tap_some(|_| called.set(true));

    assert!(!called.get(), "tap_some must not run for None");
    assert_eq!(option, None);
}

// --- tap_none ---------------------------------------------------------------

#[test]
fn tap_none_calls_the_closure_on_none() {
    let called = std::cell::Cell::new(false);

    let option: Option<i32> = None.tap_none(|| called.set(true));

    assert!(called.get());
    assert_eq!(option, None);
}

#[test]
fn tap_none_does_not_call_the_closure_on_some() {
    let called = std::cell::Cell::new(false);

    let option: Option<i32> = Some(1).tap_none(|| called.set(true));

    assert!(!called.get(), "tap_none must not run for Some");
    assert_eq!(option, Some(1));
}

// --- combined behavior ------------------------------------------------------

#[test]
fn taps_never_change_the_value() {
    let option = Some(7u8).tap_some(|_| {}).tap_none(|| {}).tap_some(|_| {});
    assert_eq!(option, Some(7));

    let option: Option<u8> = None.tap_some(|_| {}).tap_none(|| {}).tap_some(|_| {});
    assert_eq!(option, None);
}

#[test]
fn taps_can_be_chained_and_interleaved() {
    let some_steps = std::cell::Cell::new(0);
    let none_steps = std::cell::Cell::new(0);

    let some: Option<i32> = Some(1)
        .tap_some(|_| some_steps.set(some_steps.get() + 1))
        .tap_none(|| none_steps.set(none_steps.get() + 1))
        .tap_some(|_| some_steps.set(some_steps.get() + 1));

    assert_eq!(some, Some(1));
    assert_eq!(some_steps.get(), 2);
    assert_eq!(none_steps.get(), 0);

    let none: Option<i32> = None
        .tap_some(|_| some_steps.set(some_steps.get() + 10))
        .tap_none(|| none_steps.set(none_steps.get() + 1));

    assert_eq!(none, None);
    assert_eq!(some_steps.get(), 2);
    assert_eq!(none_steps.get(), 1);
}

#[test]
fn tap_some_composes_with_std_map() {
    let option: Option<i32> = Some(3).tap_some(|v| assert_eq!(*v, 3)).map(|v| v * v);

    assert_eq!(option, Some(9));
}

// --- into_maybe -------------------------------------------------------------

#[test]
fn into_maybe_converts_some_to_maybe_some() {
    let maybe: Maybe<i32> = Some(5).into_maybe();
    assert_eq!(maybe, Maybe::Some(5));
    assert!(maybe.is_some());
}

#[test]
fn into_maybe_converts_none_to_maybe_none() {
    let maybe: Maybe<i32> = None.into_maybe();
    assert_eq!(maybe, Maybe::None);
    assert!(maybe.is_none());
}

#[test]
fn into_maybe_round_trips_through_back_into_option() {
    let original: Option<i32> = Some(3);
    let maybe: Maybe<i32> = original.into_maybe();
    let back: Option<i32> = maybe.into();

    assert_eq!(back, Some(3));
}

// --- trait reachability ------------------------------------------------------

#[test]
fn trait_is_reachable_through_all_public_paths() {
    // The same behavior must apply whether the trait is imported from
    // `tinyboost::traits::OptionExt` or `tinyboost::prelude::OptionExt`.
    use tinyboost::traits::OptionExt as TraitsExt;

    let a = Some(2u8).tap_some(|_| {});
    let b = tinyboost::traits::OptionExt::tap_some(Some(2u8), |_| {});
    let c = TraitsExt::tap_some(Some(2u8), |_| {});

    assert_eq!(a, b);
    assert_eq!(b, c);
}

// --- different payload types -------------------------------------------------

#[test]
fn works_with_reference_payloads() {
    let data = String::from("payload");
    let option: Option<&String> = Some(&data)
        .tap_some(|v| assert_eq!(v.len(), 7))
        .tap_none(|| panic!("must not fire"));

    assert_eq!(option, Some(&data));
}

#[test]
fn works_with_unit_payloads() {
    let option: Option<()> = Some(())
        .tap_some(|_| {})
        .tap_none(|| panic!("must not fire"));
    assert_eq!(option, Some(()));

    let option: Option<()> = None.tap_none(|| {});
    assert_eq!(option, None);
}
