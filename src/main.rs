extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: String) -> String {
    format!("Hi {} san!", name)
}

fn main() {
    println!("{}", greet(String::from("foobar")));
}
