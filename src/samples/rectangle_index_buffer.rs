use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use crate::gl::{create_buffer, resize};
use crate::shaders::sample::setup_program;

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

    let program = setup_program(&gl)?;
    gl.use_program(Some(&program));

    let pos_loc = gl.get_attrib_location(&program, "position") as u32;
    let color_loc = gl.get_uniform_location(&program, "color").expect("could not find 'color' uniform location");

    let pos_buffer = create_buffer(&gl)?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&pos_buffer));
    unsafe {
        let vert_array = js_sys::Float32Array::view(&FACES);
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
    gl.vertex_attrib_pointer_with_i32(pos_loc, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(pos_loc);

    gl.uniform4fv_with_f32_array(Some(&color_loc), &COLOR_GREEN);

    // bind index buffer
    let index_buffer = create_buffer(&gl)?;
    gl.bind_buffer(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&index_buffer),
    );
    unsafe {
        let index_array = js_sys::Uint16Array::view(&INDICES[..]);
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &index_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    gl.draw_elements_with_i32(
        WebGlRenderingContext::TRIANGLES,
        INDICES.len() as i32,
        WebGlRenderingContext::UNSIGNED_SHORT,
        0,
    );

    Ok(())
}
