use crate::gl::{compile_shader, ShaderType, link_program};
use wasm_bindgen::JsValue;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use crate::resize;

mod triangles;

pub fn draw_triangles(canvas: &HtmlCanvasElement)  -> Result<(), JsValue> {



    triangles::draw(&canvas)?;

    Ok(())
}

pub fn foo(canvas: &HtmlCanvasElement, context: &WebGlRenderingContext) -> Result<(), JsValue> {
    let vertex_shader = compile_shader(
        &context,
        ShaderType::Vertex,
        super::shaders::vertex::simple::SHADER,
    )?;
    let fragment_shader = compile_shader(
        &context,
        ShaderType::Fragment,
        super::shaders::fragment::simple::SHADER,
    )?;
    let program = link_program(&context, &vertex_shader, &fragment_shader)?;
    context.use_program(Some(&program));

    let buffer = context.create_buffer().ok_or("could not create webgl buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    let vertices = [
        -0.7, -0.7, 0.6, 0.7, -0.7, 0.6, 0.0, 0.7, 0.6,
        -0.5, -0.5, 0.8, 0.5, -0.5, 0.8, 0.0, 0.5, 0.8,
        -0.3, -0.3, 1.0, 0.3, -0.3, 1.0, 0.0, 0.3, 1.0
    ];

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

