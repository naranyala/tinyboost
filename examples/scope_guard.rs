//! Demonstrates cleanup on scope exit and canceling a guard when cleanup is
//! no longer needed.

use std::cell::Cell;
use std::rc::Rc;
use tinyboost::ScopeGuard;

fn main() {
    let cleaned_up = Rc::new(Cell::new(false));
    let cleanup_flag = Rc::clone(&cleaned_up);

    {
        let _guard = ScopeGuard::new(move || {
            cleanup_flag.set(true);
            println!("resource cleaned up on scope exit");
        });
        println!("using resource");
    }

    assert!(cleaned_up.get());

    let dismissed_cleanup = Rc::new(Cell::new(false));
    let cleanup_flag = Rc::clone(&dismissed_cleanup);
    let guard = ScopeGuard::new(move || cleanup_flag.set(true));
    guard.dismiss();

    assert!(!dismissed_cleanup.get());
    println!("dismissed cleanup when ownership was transferred");
}
