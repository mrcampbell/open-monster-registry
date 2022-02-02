
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
extern crate engine;

wasm_bindgen_test_configure!(run_in_browser);


#[wasm_bindgen_test]
pub fn test_simple_query() {
  let res = engine::execute("query { speciesByID(id: 1) { name id elements} }".to_string());

  println!("RESULT: {}", res);
  assert_eq!(res, "{\"data\":{\"speciesByID\":{\"name\":\"Bulbasaur\",\"id\":1,\"elements\":[\"Grass\",\"Poison\"]}}}");
}