#![cfg(target_arch = "wasm32")]

mod player;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use player::Player;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn load_player() {
    assert_eq!(Player::from(""), Player {
        name: "teste".to_string(),
    });
}