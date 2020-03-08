use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use crate::gl::resize;
use crate::shaders::*;

const FACE_ZERO: [f32; 9] = [-0.9, -0.9, 0.4, 0.9, -0.9, 0.4, 0.0, 0.9, 0.4];
const FACE_ONE: [f32; 9] = [-0.7, -0.7, 0.6, 0.7, -0.7, 0.6, 0.0, 0.7, 0.6];
const FACE_TWO: [f32; 9] = [-0.5, -0.5, 0.8, 0.5, -0.5, 0.8, 0.0, 0.5, 0.8];
const FACE_THREE: [f32; 9] = [-0.3, -0.3, 1.0, 0.3, -0.3, 1.0, 0.0, 0.3, 1.0];

#[derive(Debug)]
struct Triangle {
    face: [f32; 9],
    color: [f32; 4],
}

pub fn draw(canvas: &HtmlCanvasElement) -> Result<()> {
    let gl: web_sys::WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<web_sys::WebGlRenderingContext>()?;

    let triangles = vec![
        Triangle {
            face: FACE_ZERO,
            color: [1., 1., 1., 1.],
        },
        Triangle {
            face: FACE_ONE,
            color: [1.0, 0.0, 0.0, 1.0],
        },
        Triangle {
            face: FACE_TWO,
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Triangle {
            face: FACE_THREE,
            color: [0.0, 0.0, 1.0, 1.0],
        },
    ];

    // 1. init
    resize(&canvas)?;
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    gl.clear_color(0., 0., 0., 1.);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // setup shaders and program
    let program = sample::setup_program(&gl)?;
    gl.use_program(Some(&program));

    // lookup attributes
    let position_loc = gl.get_attrib_location(&program, "position") as u32;
    let color_loc = gl.get_uniform_location(&program, "color").expect("could not find 'color' uniform location");

    // 2. draw (foreach triangle)
    for triangle in triangles {
        // pos: enable vertex attrib array, bind buffer and vertex attrib pointer
        let pos_buffer = gl.create_buffer().ok_or("could not create webgl buffer")?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&pos_buffer));
        unsafe {
            let vert_array = js_sys::Float32Array::view(&triangle.face);
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }
        gl.vertex_attrib_pointer_with_i32(
            position_loc,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        gl.enable_vertex_attrib_array(position_loc);

        gl.uniform4fv_with_f32_array(Some(&color_loc), &triangle.color);

        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);
    }

    Ok(())
}

type Result<T> = std::result::Result<T, JsValue>;
