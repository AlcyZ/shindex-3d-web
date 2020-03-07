use web_sys::{WebGlShader, WebGlProgram, WebGlRenderingContext};

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: ShaderType,
    shader_src: &str,
) -> Result<WebGlShader, String> {
    let shader_type = match shader_type {
        ShaderType::Vertex => WebGlRenderingContext::VERTEX_SHADER,
        ShaderType::Fragment => WebGlRenderingContext::FRAGMENT_SHADER
    };
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Could not create shader"))?;
    context.shader_source(&shader, shader_src);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Could not create program"))?;
    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
