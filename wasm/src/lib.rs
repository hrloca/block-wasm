extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Call from, {}!", name));
}

#[wasm_bindgen(start)]
pub fn main() {
    greet("Rust");
}
