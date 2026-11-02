//! Comprehensive tests for `Maybe<T>`.

use std::collections::HashSet;
use tinyboost::core::Maybe;

// --- predicates -------------------------------------------------------------

#[test]
fn is_some_and_is_none_track_the_variant() {
    let some: Maybe<i32> = Maybe::Some(1);
    let none: Maybe<i32> = Maybe::None;

    assert!(some.is_some());
    assert!(!some.is_none());
    assert!(none.is_none());
    assert!(!none.is_some());
}

// --- combinators ------------------------------------------------------------

#[test]
fn map_transforms_some() {
    let some = Maybe::Some(2);
    assert_eq!(some.map(|v| v * 10), Maybe::Some(20));
}

#[test]
fn map_passes_none_through() {
    let none: Maybe<i32> = Maybe::None;
    let mapped: Maybe<String> = none.map(|v: i32| v.to_string());
    assert_eq!(mapped, Maybe::None);
}

#[test]
fn map_changes_the_inner_type() {
    let some = Maybe::Some(3);
    let stringified: Maybe<String> = some.map(|v| format!("n{v}"));
    assert_eq!(stringified, Maybe::Some(String::from("n3")));
}

#[test]
fn and_then_propagates_some() {
    let some = Maybe::Some(4);
    let chained = some.and_then(|v| Maybe::Some(v * 2));
    assert_eq!(chained, Maybe::Some(8));
}

#[test]
fn and_then_can_short_circuit_to_none() {
    let some = Maybe::Some(4);
    let chained = some.and_then(|_| Maybe::None::<i32>);
    assert_eq!(chained, Maybe::None);
}

#[test]
fn and_then_propagates_none_without_calling_the_closure() {
    let none: Maybe<i32> = Maybe::None;
    let called = std::cell::Cell::new(false);

    let result = none.and_then(|_| {
        called.set(true);
        Maybe::Some(99)
    });

    assert_eq!(result, Maybe::None);
    assert!(!called.get(), "closure must not run for None");
}

// --- fallbacks --------------------------------------------------------------

#[test]
fn unwrap_or_returns_inner_value_for_some() {
    let some = Maybe::Some(7);
    assert_eq!(some.unwrap_or(0), 7);
}

#[test]
fn unwrap_or_returns_default_for_none() {
    let none: Maybe<i32> = Maybe::None;
    assert_eq!(none.unwrap_or(9), 9);
}

#[test]
fn unwrap_or_default_uses_type_default_for_none() {
    let none: Maybe<i32> = Maybe::None;
    assert_eq!(none.unwrap_or_default(), i32::default());

    let none_str: Maybe<String> = Maybe::None;
    assert_eq!(none_str.unwrap_or_default(), String::new());
}

#[test]
fn unwrap_or_default_returns_inner_value_for_some() {
    let some: Maybe<i32> = Maybe::Some(5);
    assert_eq!(some.unwrap_or_default(), 5);
}

// --- conversions ------------------------------------------------------------

#[test]
fn from_option_some_maps_to_maybe_some() {
    let converted: Maybe<&str> = Some("x").into();
    assert_eq!(converted, Maybe::Some("x"));
}

#[test]
fn from_option_none_maps_to_maybe_none() {
    let converted: Maybe<&str> = None.into();
    assert_eq!(converted, Maybe::None);
}

#[test]
fn maybe_some_converts_to_option_some() {
    let option: Option<&str> = Maybe::Some("y").into();
    assert_eq!(option, Some("y"));
}

#[test]
fn maybe_none_converts_to_option_none() {
    let option: Option<&str> = Maybe::None.into();
    assert_eq!(option, None);
}

#[test]
fn option_maybe_round_trip_preserves_the_value() {
    let original: Option<Vec<u8>> = Some(vec![1, 2, 3]);
    let round: Option<Vec<u8>> = Maybe::from(original.clone()).into();
    assert_eq!(round, original);

    let original: Option<Vec<u8>> = None;
    let round: Option<Vec<u8>> = Maybe::from(original).into();
    assert_eq!(round, None);
}

// --- newer combinators -------------------------------------------------------

#[test]
fn unwrap_or_else_only_invokes_the_closure_for_none() {
    let called = std::cell::Cell::new(false);

    let some = Maybe::<i32>::Some(4).unwrap_or_else(|| {
        called.set(true);
        -1
    });
    assert_eq!(some, 4);
    assert!(!called.get());

    let none = Maybe::<i32>::None.unwrap_or_else(|| {
        called.set(true);
        -1
    });
    assert_eq!(none, -1);
    assert!(called.get());
}

#[test]
fn as_ref_maps_to_maybe_of_references() {
    let some = Maybe::Some(String::from("inner"));
    let borrowed: Maybe<&String> = some.as_ref();

    assert_eq!(borrowed, Maybe::Some(&String::from("inner")));
    // Original untouched — usable afterwards.
    assert_eq!(some.clone().unwrap_or_default(), "inner");
}

#[test]
fn as_ref_on_none_stays_none() {
    let none: Maybe<i32> = Maybe::None;
    assert_eq!(none.as_ref(), Maybe::None);
}

#[test]
fn as_mut_allows_in_place_mutation() {
    let mut some = Maybe::Some(10);
    if let Maybe::Some(value) = some.as_mut() {
        *value += 5;
    }
    assert_eq!(some, Maybe::Some(15));
}

#[test]
fn as_mut_on_none_cannot_mutate_but_source_stays_valid() {
    let mut none: Maybe<i32> = Maybe::None;
    let borrowed: Maybe<&mut i32> = none.as_mut();
    assert_eq!(borrowed, Maybe::None);
    assert_eq!(none, Maybe::None);
}

#[test]
fn contains_matches_only_equal_values() {
    let some = Maybe::Some(5);
    assert!(some.contains(&5));
    assert!(!some.contains(&6));

    let none: Maybe<i32> = Maybe::None;
    assert!(!none.contains(&5));
    assert!(!none.contains(&0));
}

// --- derived traits ---------------------------------------------------------

#[test]
fn clone_produces_an_independent_value() {
    let some = Maybe::Some(String::from("clonable"));
    let cloned = some.clone();
    assert_eq!(some, cloned);

    let none: Maybe<String> = Maybe::None;
    assert_eq!(none.clone(), Maybe::None);
}

#[test]
fn debug_rendering_is_useful_for_both_variants() {
    assert_eq!(format!("{:?}", Maybe::Some(1)), "Some(1)");
    assert_eq!(format!("{:?}", Maybe::None::<i32>), "None");
}

#[test]
fn equality_distinguishes_variants_and_values() {
    assert_eq!(Maybe::Some(1), Maybe::Some(1));
    assert_ne!(Maybe::Some(1), Maybe::Some(2));
    assert_ne!(Maybe::Some(1), Maybe::None);
    assert_eq!(Maybe::None::<i32>, Maybe::None);
}

#[test]
fn ordering_places_some_before_none() {
    // `Maybe` derives `Ord` with the variants declared as `Some(T), None`,
    // so `Some` orders before `None`.
    assert!(Maybe::Some(0) < Maybe::None::<i32>);
    assert!(Maybe::Some(1) < Maybe::Some(2));
    assert!(Maybe::Some(2) > Maybe::Some(1));
}

#[test]
fn hash_consistency_for_sets() {
    let mut set = HashSet::new();
    set.insert(Maybe::Some(1));
    set.insert(Maybe::Some(1)); // duplicate
    set.insert(Maybe::None::<i32>);

    assert_eq!(set.len(), 2);
    assert!(set.contains(&Maybe::Some(1)));
    assert!(set.contains(&Maybe::None));
}
