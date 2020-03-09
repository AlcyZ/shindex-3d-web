use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use crate::gl::resize;
use crate::samples::utility::SampleProgram;

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

    resize(&canvas)?;
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    gl.clear_color(0.1, 0.1, 0.1, 1.);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    let programs = vec![
        TriangleProgram {
            sample_program: SampleProgram::new(&gl, FACE_ZERO.to_vec(), None)?,
            color: [1., 1., 1., 1.],
        },
        TriangleProgram {
            sample_program: SampleProgram::new(&gl, FACE_ONE.to_vec(), None)?,
            color: [1.0, 0.0, 0.0, 1.0],
        },
        TriangleProgram {
            sample_program: SampleProgram::new(&gl, FACE_TWO.to_vec(), None)?,
            color: [0.0, 1.0, 0.0, 1.0],
        },
        TriangleProgram {
            sample_program: SampleProgram::new(&gl, FACE_THREE.to_vec(), None)?,
            color: [0.0, 0.0, 1.0, 1.0],
        },
    ];

    for program in programs {
        program.sample_program.render(program.color)?;
    }

    Ok(())
}

type Result<T> = std::result::Result<T, JsValue>;

struct TriangleProgram<'a> {
    sample_program: SampleProgram<'a>,
    color: [f32; 4],
}
