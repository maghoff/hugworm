use crate::scene::Scene;
use crate::webgl;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};

pub struct Renderer {
    context: WebGlRenderingContext,
    program: WebGlProgram,
}

fn build_shader_program(context: &WebGlRenderingContext) -> Result<WebGlProgram, JsValue> {
    let vert_shader = webgl::compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        include_str!("vertex.v.glsl"),
    )?;
    let frag_shader = webgl::compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        include_str!("frag.f.glsl"),
    )?;
    let program = webgl::link_program(&context, &vert_shader, &frag_shader)?;

    Ok(program)
}

impl Renderer {
    pub fn new(context: WebGlRenderingContext) -> Result<Renderer, JsValue> {
        let program = build_shader_program(&context)?;
        Ok(Renderer { context, program })
    }

    pub fn render_scene(&self, scene: &Scene) -> Result<(), JsValue> {
        self.context.use_program(Some(&self.program));

        let buffer = self
            .context
            .create_buffer()
            .ok_or("failed to create buffer")?;
        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        let mut vertices = vec![];
        scene.worm.generate_geometry(&mut vertices);

        unsafe {
            // Safe, because we're not allocating memory until view is out of scope
            let vert_array = js_sys::Float32Array::view(&vertices);

            self.context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        self.context.vertex_attrib_pointer_with_i32(
            0,
            4,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context.enable_vertex_attrib_array(0);

        self.context.clear_color(1.0, 1.0, 1.0, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.context.draw_arrays(
            WebGlRenderingContext::TRIANGLE_STRIP,
            0,
            (vertices.len() / 4) as i32,
        );

        Ok(())
    }
}
