extern crate wasm_bindgen;
extern crate image;

use wasm_bindgen::prelude::*;
use image::GenericImageView;

#[wasm_bindgen]
pub fn greet(name: String) -> String {
    format!("Hi {} san!", name)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FilterParams {
    pub resize_scale: f32,
    pub rotate_angle: u32,
    pub huerotate_angle: i32,
    pub blur_sigma: f32,
    pub brighten_value: i32,
}

fn scale_pixel(pixels: u32, scale: f32) -> u32 {
    (pixels as f32 * scale) as u32
}

#[wasm_bindgen]
pub fn convimg(input_img: js_sys::Uint8Array,
               filters_checkbox: &JsValue,
               filter_params_js: &JsValue) -> js_sys::Uint8Array {
    let filters_vec: Vec<String> = filters_checkbox.into_serde().unwrap();
    let mut img = image::load_from_memory(input_img.to_vec().as_slice())
                      .expect("failed to load image");
    let filter_params: FilterParams = filter_params_js.into_serde().unwrap();
    let ( img_width, img_height ) = img.dimensions();

    for filter_name in filters_vec {
        img = match filter_name.as_str() {
            "resize" => img.resize(
                scale_pixel(img_width, filter_params.resize_scale),
                scale_pixel(img_height, filter_params.resize_scale),
                image::imageops::Triangle),
            "rotate" => match filter_params.rotate_angle {
                 90 => img.rotate90(),
                 180 => img.rotate180(),
                 270 => img.rotate270(),
                 _ => img,
            },
            "grayscale" => img.grayscale(),
            "huerotate" => img.huerotate(filter_params.huerotate_angle),
            "blur" => img.blur(filter_params.blur_sigma),
            "brighten" => img.brighten(filter_params.brighten_value),
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
