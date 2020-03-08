use wasm_bindgen::JsValue;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::gl::{compile_shader, link_program, ShaderType};

const VERTEX_SHADER: &str = r#"
attribute vec4 position;

void main() {
    gl_Position = position;
}
"#;
const FRAGMENT_SHADER: &str = r#"
precision mediump float;

uniform vec4 color;

void main() {
    gl_FragColor = color;
}
"#;

pub fn setup_program(gl: &WebGlRenderingContext) -> Result<WebGlProgram, JsValue> {
    let vertex_shader = compile_shader(&gl, ShaderType::Vertex, VERTEX_SHADER)?;
    let fragment_shader = compile_shader(&gl, ShaderType::Fragment, FRAGMENT_SHADER)?;

    let program = link_program(&gl, &vertex_shader, &fragment_shader)?;

    Ok(program)
}
