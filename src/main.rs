extern crate wasm_bindgen;
extern crate image;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: String) -> String {
    format!("Hi {} san!", name)
}

#[wasm_bindgen]
pub fn convimg(input_img: js_sys::Uint8Array,
               filters_checkbox: &JsValue) -> js_sys::Uint8Array {
    let filters_vec: Vec<String> = filters_checkbox.into_serde().unwrap();
    let mut img = image::load_from_memory(input_img.to_vec().as_slice())
                      .expect("failed to load image");

    for filter_name in filters_vec {
        img = match filter_name.as_str() {
            "resize" => img.resize(100, 100, image::imageops::Triangle),
            "rotate" => img.rotate90(),
            "grayscale" => img.grayscale(),
            "huerotate" => img.huerotate(180),
            "blur" => img.blur(10.0),
            "brighten" => img.brighten(10),
            &_ => img
        };
    }

    let mut img_buf = Vec::new();
    img.write_to(&mut img_buf, image::ImageOutputFormat::Png)
        .expect("failed to write image to buffer");

    unsafe {
        js_sys::Uint8Array::view(img_buf.as_slice())
    }
}

fn main() {
    println!("{}", greet(String::from("foobar")));
}
