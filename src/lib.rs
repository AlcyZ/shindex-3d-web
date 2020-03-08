extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;

mod gl;
mod samples;
mod shaders;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let canvas = document().get_element_by_id("surface").expect("could not find element by id 'surface'");
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    samples::draw_rectangle_with_index_buffer(&canvas)?;

    Ok(())
}

fn _game_loop() -> Result<(), JsValue> {
    let cb_ref = Rc::new(RefCell::new(None));
    let cb_clone = cb_ref.clone();

    *cb_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        console_log!("loop");
        _request_animation_frame(cb_ref.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut()>));

    _request_animation_frame(cb_clone.borrow().as_ref().unwrap());

    Ok(())
}


fn _request_animation_frame(callback: &Closure<dyn FnMut()>) {
    window().request_animation_frame(callback.as_ref().unchecked_ref()).expect("failed to register request animation frame");
}

fn window() -> web_sys::Window {
    web_sys::window().expect("JS window object is not available")
}

fn document() -> web_sys::Document {
    window().document().expect("JS document object is not available")
}
