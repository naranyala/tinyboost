//! Demonstrates side-effect hooks that observe `Result` and `Option` values
//! without changing their values.

use tinyboost::prelude::*;

fn load_port() -> Result<u16, &'static str> {
    Ok(8080)
}

fn main() {
    let port = load_port()
        .tap_ok(|port| println!("loaded port {port}"))
        .tap_err(|error| eprintln!("could not load port: {error}"))
        .map_ok(|port| port.to_string());
    assert_eq!(port.as_deref(), Ok("8080"));

    let configured_host = Some("127.0.0.1")
        .tap_some(|host| println!("using host {host}"))
        .tap_none(|| eprintln!("no host configured"));
    assert_eq!(configured_host, Some("127.0.0.1"));

    let absent_host: Option<&str> = None;
    let _ = absent_host.tap_none(|| println!("using the default host"));
}
