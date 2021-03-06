use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use engine_mod::triangle_mod::Triangle;

pub struct Renderer {
    context: web_sys::WebGlRenderingContext,
}

impl Renderer {
    pub fn new() -> Result<(Renderer), JsValue> {
        // Gather our canvas from the DOM
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = element.dyn_into::<web_sys::HtmlCanvasElement>()?;

        // Cast our canvas into a WebGl context
        let context = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;

        // Compile our shaders
        let vert_shader = Renderer::compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            include_str!("shaders/basic_vertex.glsl"),
        )?;
        let frag_shader = Renderer::compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            include_str!("shaders/basic_fragment.glsl"),
        )?;

        // A WebGLProgram is the object that holds the two compiled shaders
        let program = Renderer::link_program(&context, [vert_shader, frag_shader].iter())?;
        context.use_program(Some(&program));

        // Attributes in shaders come from buffers, first get the buffer
        let buffer = context.create_buffer().ok_or("failed to create a buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        // Bind buffer to generic vertex attribute of the current vertex buffer object
        context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        // Attributes need to be enabled before use (could just use 0 since we know it's first)
        context.enable_vertex_attrib_array(context.get_attrib_location(&program, "position") as u32);

        // Return our WebGL object
        Ok(Renderer {
            context,
        })
    }

    pub fn draw(&mut self, triangle: &Triangle) -> Result<(), JsValue> {

        let vertices = triangle.get_vertices();

        // Get the buffer out of WebAssembly memory
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        // Figure out where the vertices are in the memory_buffer (convert pointer to index)
        let vertices_location = vertices.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + vertices.len() as u32);

        // Buffer_data will copy the data to the GPU memory
        self.context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );

        // Draw over the entire canvas and clear buffer to ur present one
        self.context.clear_color(0.9, 0.9, 0.9, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        // Draw our shape (Triangles, offset, count) Our vertex shader will run <count> times.
        self.context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (vertices.len() / 3) as i32,
        );

        Ok(())
    }
    
    // non pub //
    
    fn compile_shader(context: &WebGlRenderingContext, shader_type: u32, source: &str
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

    fn link_program<'a, T>(context: &WebGlRenderingContext, shaders: T
    ) -> Result<WebGlProgram, String>
        where T: IntoIterator<Item=&'a WebGlShader> {
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
}