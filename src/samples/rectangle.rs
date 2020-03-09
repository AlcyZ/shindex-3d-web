use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use crate::gl::resize;
use crate::samples::utility::SampleProgram;

const FACES: [f32; 18] = [
    // first face
    -0.75, -0.75, 1., 0.75, -0.75, 1., 0.75, 0.75, 1., // second face
    -0.75, 0.75, 1., -0.75, -0.75, 1., 0.75, 0.75, 1.,
];
const _FACE_ONE: [f32; 9] = [-0.5, -0.5, 1., 0.5, -0.5, 1., 0.5, 0.5, 1.];
const _FACE_TWO: [f32; 9] = [-0.5, 0.5, 1., -0.5, -0.5, 1., 0.5, 0.5, 1.];

const _COLOR_RED: [f32; 4] = [1., 0.0, 0.0, 1.];
const _COLOR_GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.];
const _COLOR_BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.];
const _COLOR_WHITE: [f32; 4] = [1., 1., 1., 1.];
const _COLOR_BLACK: [f32; 4] = [0., 0., 0., 1.];

pub fn draw(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    resize(&canvas)?;
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    gl.clear_color(0.1, 0.1, 0.1, 1.);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    simple_rectangle(&gl)?;
    two_different_colored_rectangles(&gl)?;

    Ok(())
}

fn simple_rectangle(gl: &WebGlRenderingContext) -> Result<(), JsValue> {
    let triangle_program = TriangleProgram {
        sample_program: SampleProgram::new(&gl, FACES.to_vec(), None)?,
        color: _COLOR_RED,
    };

    triangle_program.sample_program.render(triangle_program.color)?;

    Ok(())
}

fn two_different_colored_rectangles(
    gl: &WebGlRenderingContext,
) -> Result<(), JsValue> {
    let programs = vec![
        TriangleProgram {
            sample_program: SampleProgram::new(&gl, _FACE_ONE.to_vec(), None)?,
            color: _COLOR_GREEN,
        },
        TriangleProgram {
            sample_program: SampleProgram::new(&gl, _FACE_TWO.to_vec(), None)?,
            color: _COLOR_BLUE,
        }
    ];

    for program in programs {
        program.sample_program.render(program.color)?;
    }

    Ok(())
}

struct TriangleProgram<'a> {
    sample_program: SampleProgram<'a>,
    color: [f32; 4],
}
