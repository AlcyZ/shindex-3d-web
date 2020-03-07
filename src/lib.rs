extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{WebGlRenderingContext, HtmlCanvasElement};
use crate::gl::{compile_shader, link_program};

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

    // let context: web_sys::WebGlRenderingContext = canvas
    //     .get_context("webgl")?
    //     .unwrap()
    //     .dyn_into::<web_sys::WebGlRenderingContext>()?;
    // samples::foo(&canvas, &context);

    samples::draw_triangles(&canvas)?;
    // _run_bak();

    Ok(())
}

#[wasm_bindgen]
pub fn _run_bak() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let canvas = document().get_element_by_id("surface").expect("could not find element by id 'surface'");
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context: web_sys::WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<web_sys::WebGlRenderingContext>()?;

    let vertex_shader = compile_shader(
        &context,
        gl::ShaderType::Vertex,
        shaders::vertex::simple::SHADER,
    )?;
    let fragment_shader = compile_shader(
        &context,
        gl::ShaderType::Fragment,
        shaders::fragment::simple::SHADER,
    )?;

    let program = link_program(&context, &vertex_shader, &fragment_shader)?;
    context.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    let buffer = context.create_buffer().ok_or("could not create webgl buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    let vertex_position = context.get_attrib_location(&program, "position") as u32;
    context.vertex_attrib_pointer_with_i32(
        vertex_position,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(vertex_position);

    resize(&canvas);
    context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    context.clear_color(0., 0., 0., 1.);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );

    Ok(())
}

fn _game_loop() -> Result<(), JsValue> {
    let cb_ref = Rc::new(RefCell::new(None));
    let cb_clone = cb_ref.clone();

    *cb_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        console_log!("loop");
        request_animation_frame(cb_ref.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut()>));

    request_animation_frame(cb_clone.borrow().as_ref().unwrap());

    Ok(())
}

fn resize(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let display_width = canvas.client_width();
    let display_height = canvas.client_height();

    if canvas.width() != display_width as u32 || canvas.height() != display_height as u32 {
        canvas.set_width(display_width as u32);
        canvas.set_height(display_height as u32);
    }

    Ok(())
}

fn request_animation_frame(callback: &Closure<dyn FnMut()>) {
    window().request_animation_frame(callback.as_ref().unchecked_ref()).expect("failed to register request animation frame");
}

fn window() -> web_sys::Window {
    web_sys::window().expect("JS window object is not available")
}

fn document() -> web_sys::Document {
    window().document().expect("JS document object is not available")
}
