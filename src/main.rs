use sycamore::prelude::*;
pub mod components;
pub mod js_bindings;
pub mod models;
pub mod routes;
pub mod services;
extern crate console_error_panic_hook;
use std::panic;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    sycamore::render(|cx| {
        view! { cx,
            routes::Routes()
        }
    });
}
