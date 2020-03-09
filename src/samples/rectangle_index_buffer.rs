use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use crate::gl::resize;
use crate::samples::utility::SampleProgram;

const FACES: [f32; 12] = [
    -0.5, -0.5, 1., // index 0
    0.5, -0.5, 1., // index 1
    0.5, 0.5, 1., // index 2
    -0.5, 0.5, 1., // index 3
];
const INDICES: [u16; 6] = [0, 1, 2, 3, 0, 2];
const COLOR_GREEN: [f32; 4] = [0.3, 8., 0.25, 1.];

pub fn draw(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    resize(&canvas)?;
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    gl.clear_color(0.1, 0.1, 0.1, 1.);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    let sample_program = SampleProgram::new(&gl, FACES.to_vec(), Some(INDICES.to_vec()))?;
    sample_program.render(COLOR_GREEN)?;

    Ok(())
}
