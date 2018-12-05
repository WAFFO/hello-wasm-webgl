extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

#[wasm_bindgen]
pub fn init() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn draw() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        include_str!("shaders/basic_vertex.glsl"),
    )?;
    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        include_str!("shaders/basic_fragment.glsl"),
    )?;
    let program = link_program(&context, [vert_shader, frag_shader].iter())?;
    context.use_program(Some(&program));

    // set the vertices of our shape
    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    // get the buffer out of WebAssembly memory
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()?
        .buffer();
    // figure out where the vertices are in the memory_buffer (convert pointer to index)
    let vertices_location = vertices.as_ptr() as u32 / 4;
    let vert_array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(vertices_location, vertices_location + vertices.len() as u32);

    // attributes in shaders come from buffers, first get the buffer
    let buffer = context.create_buffer().ok_or("failed to create a buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    // buffer_data will copy the data to the GPU memory
    context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vert_array,
        WebGlRenderingContext::STATIC_DRAW,
    );
    // bind buffer to generic vertex attribute of the current vertex buffer object
    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    // attributes need to be enabled before use (could just use 0 since we know it's first)
    context.enable_vertex_attrib_array(context.get_attrib_location(&program, "position") as u32);

    // draw over the entire canvas and clear buffer to ur present one
    context.clear_color(0.1, 0.1, 0.1, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // draw our shape (Triangles, offset, count) Our vertex shader will run <count> times.
    context.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );
    Ok(())
}

pub fn compile_shader( context: &WebGlRenderingContext, shader_type: u32, source: &str
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".into()))
    }
}

pub fn link_program<'a, T>( context: &WebGlRenderingContext, shaders: T
) -> Result<WebGlProgram, String>
where T: IntoIterator<Item = &'a WebGlShader> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    for shader in shaders {
        context.attach_shader(&program, shader)
    }
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
            .unwrap_or_else(|| "Unknown error creating program object".into()))
    }
}
