use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{HtmlCanvasElement, WebGlShader, WebGlProgram, WebGlRenderingContext};

use crate::shaders::*;
use crate::log;
use crate::gl::{compile_shader, link_program, ShaderType};

const FACE_ONE: [f32; 9] = [
    -0.7, -0.7, 0.6, 0.7, -0.7, 0.6, 0.0, 0.7, 0.6
];
const FACE_TWO: [f32; 9] = [
    -0.5, -0.5, 0.8, 0.5, -0.5, 0.8, 0.0, 0.5, 0.8
];
const FACE_THREE: [f32; 9] = [
    -0.3, -0.3, 1.0, 0.3, -0.3, 1.0, 0.0, 0.3, 1.0
];

struct Triangle {
    faces: [f64; 9],
    color: [i32; 4],
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn draw(canvas: &HtmlCanvasElement) -> Result<()> {
    let gl: web_sys::WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<web_sys::WebGlRenderingContext>()?;


    // 1. init
    // setup shaders and program
    let program = sample_program(&gl)?;
    // lookup attributes
    let position_loc = gl.get_attrib_location(&program, "position");
    let color_loc = gl.get_attrib_location(&program, "color");

    // initializes buffers foreach triangle
    let buffer_one = gl.create_buffer().ok_or("could not create webgl buffer")?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer_one));
    unsafe {
        let vert_one_array = js_sys::Float32Array::view(&FACE_ONE);
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_one_array,
            WebGlRenderingContext::STATIC_DRAW
        );
    }

    let buffer_two = gl.create_buffer().ok_or("could not create webgl buffer")?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer_two));
    unsafe {
        let vert_two_array = js_sys::Float32Array::view(&FACE_TWO);
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_two_array,
            WebGlRenderingContext::STATIC_DRAW
        );
    }

    let buffer_three = gl.create_buffer().ok_or("could not create webgl buffer")?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer_three));
    unsafe {
        let vert_three_array = js_sys::Float32Array::view(&FACE_TWO);
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_three_array,
            WebGlRenderingContext::STATIC_DRAW
        );
    }

    console_log!("{:#?}", buffer_one);

    // 2. draw (foreach triangle)
    // enable vertex attrib array
    gl.vertex_attrib_pointer_with_i32(
        position_loc,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );

    // bind buffer

    // vertex attrib pointer

    // draw array


    Ok(())
}

fn sample_program(gl: &WebGlRenderingContext) -> Result<WebGlProgram> {
    let v_shader = vertex_shader(&gl)?;
    let f_shader = fragment_shader(&gl)?;

    let program = link_program(
        &gl,
        &v_shader,
        &f_shader,
    )?;

    Ok(program)
}

fn vertex_shader(gl: &WebGlRenderingContext) -> Result<WebGlShader> {
    let shader = compile_shader(
        &gl,
        ShaderType::Vertex,
        sample::VERTEX_SHADER,
    )?;

    Ok(shader)
}

fn fragment_shader(gl: &WebGlRenderingContext) -> Result<WebGlShader> {
    let shader = compile_shader(
        &gl,
        ShaderType::Fragment,
        sample::FRAGMENT_SHADER,
    )?;

    Ok(shader)
}

type Result<T> = std::result::Result<T, JsValue>;
