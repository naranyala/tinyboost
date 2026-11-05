//! Demonstrates lazy initialization from captured configuration.

use tinyboost::LazyInit;

fn main() {
    let environment = String::from("development");
    let service_url = LazyInit::new(|| format!("https://{environment}.example.test"));

    assert!(!service_url.is_initialized());
    println!("service URL: {}", service_url.get());
    assert!(service_url.is_initialized());

    // Subsequent access returns the initialized value rather than rebuilding it.
    assert_eq!(service_url.get(), "https://development.example.test");
}
