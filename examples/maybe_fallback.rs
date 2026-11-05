//! Demonstrates using `Maybe` for an optional input and making the fallback
//! explicit.

use tinyboost::Maybe;

fn main() {
    let supplied_name: Option<String> = Some(String::from("Ada"));
    let name: Maybe<String> = supplied_name.into();

    let greeting_name = name
        .as_ref()
        .map(|value| value.trim())
        .and_then(|value| (!value.is_empty()).then_some(value))
        .unwrap_or_else(|| "guest");

    println!("Hello, {greeting_name}!");

    let missing_name: Maybe<String> = None;
    assert_eq!(missing_name.unwrap_or_else(|| String::from("guest")), "guest");
}
