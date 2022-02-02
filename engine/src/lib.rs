#[macro_use] extern crate juniper;

mod utils;
mod graphql;
pub mod types;
pub mod generated;
pub mod calculations;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, engine!");
}

#[wasm_bindgen]
pub fn execute(req: String) -> String {
  graphql::handle_request(&req)
}