use wasm_bindgen::JsValue;
use crate::gl::{compile_shader, ShaderType, link_program};
use web_sys::{WebGlProgram, WebGlRenderingContext};

const VERTEX_SHADER: &str = r#"
attribute vec4 position;
attribute vec4 color;

varying vec4 out_color;

void main() {
    out_color = color;

    gl_Position = position;
}
"#;
const FRAGMENT_SHADER: &str = r#"
precision mediump float;

varying vec4 out_color;

void main() {
    gl_FragColor = out_color;
}
"#;

pub fn setup_program(gl: &WebGlRenderingContext) -> Result<WebGlProgram, JsValue> {
    let vertex_shader = compile_shader(&gl, ShaderType::Vertex, VERTEX_SHADER)?;
    let fragment_shader = compile_shader(&gl, ShaderType::Fragment, FRAGMENT_SHADER)?;

    let program = link_program(&gl, &vertex_shader, &fragment_shader)?;

    Ok(program)
}
