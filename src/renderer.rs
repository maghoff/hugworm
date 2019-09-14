use crate::segment;
use crate::webgl;
use cgmath::{prelude::*, vec2};
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};

pub struct Renderer<'a> {
    context: &'a WebGlRenderingContext,
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

impl<'a> Renderer<'a> {
    pub fn new(context: &WebGlRenderingContext) -> Result<Renderer, JsValue> {
        Ok(Renderer {
            context,
            program: build_shader_program(context)?,
        })
    }

    pub fn render_scene(&self) -> Result<(), JsValue> {
        self.context.use_program(Some(&self.program));

        let mut vertices = vec![];

        let line = segment::Segment::Line {
            start: vec2(-0.7, 0.),
            dir: vec2(2., 1.).normalize(),
            len: 0.4,
            reach: 0.0,
        };
        line.generate_geometry(&mut vertices);

        let (start, dir, reach) = line.ending();
        let arc = segment::arc(start, dir, 0.3, 0.3, true, reach);
        arc.generate_geometry(&mut vertices);

        let (start, dir, reach) = arc.ending();
        let arc = segment::arc(start, dir, 0.3, 0.6, false, reach);
        arc.generate_geometry(&mut vertices);

        let (start, dir, reach) = arc.ending();
        let line = segment::Segment::Line {
            start: start,
            dir: dir,
            len: 0.2,
            reach: reach,
        };
        line.generate_geometry(&mut vertices);

        let buffer = self
            .context
            .create_buffer()
            .ok_or("failed to create buffer")?;
        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

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

        self.context.clear_color(0.0, 0.0, 0.0, 0.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.context.draw_arrays(
            WebGlRenderingContext::TRIANGLE_STRIP,
            0,
            (vertices.len() / 4) as i32,
        );

        Ok(())
    }
}
