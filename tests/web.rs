#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use expy::add;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}

#[wasm_bindgen_test]
fn test_add_zero() {
    assert_eq!(add(0, 0), 0);
}
