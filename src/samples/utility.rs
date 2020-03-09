use wasm_bindgen::JsValue;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};

use crate::gl::create_buffer;
use crate::shaders::sample::setup_program;

pub struct SampleProgram<'a> {
    gl: &'a WebGlRenderingContext,
    program: WebGlProgram,
    locations: SampleProgramLocations,
    buffers: SampleProgramBuffers,

    vertices: Vec<f32>,
    indices: Option<Vec<u16>>,
}

impl SampleProgram<'_> {
    pub fn new(gl: &WebGlRenderingContext, vertices: Vec<f32>, indices: Option<Vec<u16>>) -> Result<SampleProgram, JsValue> {
        let shader_program = setup_program(&gl)?;
        let locations = SampleProgramLocations {
            position: gl.get_attrib_location(&shader_program, "position") as u32,
            color: gl.get_uniform_location(&shader_program, "color").expect("could not find 'color' uniform location"),
        };

        let position_buffer = create_buffer(&gl)?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let index_buffer = match &indices {
            Some(index) => {
                let index_buffer = create_buffer(&gl)?;
                gl.bind_buffer(
                    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                    Some(&index_buffer),
                );
                unsafe {
                    let index_array = js_sys::Uint16Array::view(&index);
                    gl.buffer_data_with_array_buffer_view(
                        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                        &index_array,
                        WebGlRenderingContext::STATIC_DRAW,
                    );
                }

                Some(index_buffer)
            }
            None => None
        };

        let buffers = SampleProgramBuffers {
            position: position_buffer,
            index: index_buffer,
        };

        let program = SampleProgram {
            gl: &gl,
            program: shader_program,
            locations,
            buffers,
            vertices,
            indices,
        };

        Ok(program)
    }


    pub fn render(&self, color: [f32; 4]) -> Result<(), JsValue> {
        self.gl.use_program(Some(&self.program));

        self.gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.buffers.position),
        );
        self.gl.vertex_attrib_pointer_with_i32(self.locations.position, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(self.locations.position);

        self.gl.uniform4fv_with_f32_array(Some(&self.locations.color), &color);

        if let Some(index_buffer) = &self.buffers.index {
            self.gl.bind_buffer(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(index_buffer),
            );

            self.gl.draw_elements_with_i32(
                WebGlRenderingContext::TRIANGLES,
                self.indices.as_ref().unwrap().len() as i32,
                WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        } else {
            self.gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (self.vertices.len() / 3) as i32);
        }

        Ok(())
    }
}

struct SampleProgramLocations {
    position: u32,
    color: WebGlUniformLocation,
}

struct SampleProgramBuffers {
    position: WebGlBuffer,
    index: Option<WebGlBuffer>,
}
