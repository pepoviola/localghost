#![cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn beacon_can_send() {
    let beacon = coast::net::Beacon::new("https://example.com".to_string());
    beacon.send(&mut b"hello world".to_owned());
}
