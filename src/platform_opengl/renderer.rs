use std::convert::TryInto;

use glium::{
    implement_vertex, program, program::ProgramChooserCreationError, uniform, Blend,
    BlendingFunction, DrawParameters, LinearBlendingFactor, Surface,
};

use crate::scene::Scene;

pub struct Renderer {
    display: glium::Display,
    program: glium::Program,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 4],
}

implement_vertex!(Vertex, position);

impl Renderer {
    pub fn new(display: glium::Display) -> Result<Renderer, ProgramChooserCreationError> {
        let program = program!(&display,
            140 => {
                vertex: include_str!("vertex.v.glsl"),
                fragment: include_str!("frag.f.glsl"),
            },
        )?;

        Ok(Renderer { display, program })
    }

    pub fn render_scene(&self, scene: &Scene) {
        let params = &DrawParameters {
            blend: Blend {
                color: BlendingFunction::Addition {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha,
                },
                ..Default::default()
            },
            ..Default::default()
        };

        let mut raw_vertices = vec![];
        scene.worm.generate_geometry(&mut raw_vertices);

        let vertices: Vec<_> = raw_vertices
            .chunks_exact(4)
            .map(|chunk| Vertex {
                position: chunk.try_into().unwrap(),
            })
            .collect();

        let buffer = glium::VertexBuffer::new(&self.display, &vertices).unwrap();

        let mut target = self.display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        target
            .draw(
                (&buffer,),
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                &self.program,
                &uniform! {},
                params,
            )
            .unwrap();

        target.finish().unwrap();
    }
}
