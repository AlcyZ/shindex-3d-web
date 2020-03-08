use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext};

use crate::gl::resize;
use crate::shaders::*;

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

struct Rectangle {
    faces: [f32; 18],
    color: [f32; 4],
}

struct Triangle {
    face: [f32; 9],
    color: [f32; 4],
}

pub fn draw(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    resize(&canvas)?;
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    let program = sample::setup_program(&gl)?;
    gl.use_program(Some(&program));

    simple_rectangle(&gl, &program)?;
    two_different_colored_rectangles(&gl, &program)?;

    Ok(())
}

fn simple_rectangle(gl: &WebGlRenderingContext, program: &WebGlProgram) -> Result<(), JsValue> {
    let model = Rectangle {
        faces: FACES,
        color: _COLOR_RED,
    };

    let position_loc = gl.get_attrib_location(&program, "position") as u32;
    let color_loc = gl.get_uniform_location(&program, "color").expect("could not find 'color' uniform location");

    let pos_buffer = gl.create_buffer().ok_or("could not create webgl buffer")?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&pos_buffer));
    unsafe {
        let vert_array = js_sys::Float32Array::view(&model.faces);
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
    gl.vertex_attrib_pointer_with_i32(position_loc, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(position_loc);

    gl.uniform4fv_with_f32_array(Some(&color_loc), &model.color);

    gl.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (FACES.len() / 3) as i32,
    );

    Ok(())
}

fn two_different_colored_rectangles(
    gl: &WebGlRenderingContext,
    program: &WebGlProgram,
) -> Result<(), JsValue> {
    let model = vec![
        Triangle {
            face: _FACE_ONE,
            color: _COLOR_GREEN,
        },
        Triangle {
            face: _FACE_TWO,
            color: _COLOR_BLUE,
        },
    ];

    let position_loc = gl.get_attrib_location(&program, "position") as u32;
    let color_loc = gl.get_uniform_location(&program, "color").expect("could not find 'color' uniform location");

    for triangle in model {
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

        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (triangle.face.len() / 3) as i32,
        );
    }

    Ok(())
}
