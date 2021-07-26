use glium::{glutin, glutin::{event::VirtualKeyCode, event_loop::ControlFlow::{self, Exit}}};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new().with_inner_size(
        glium::glutin::dpi::LogicalSize::new(500, 500)
    );
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop)?;

    let program = program!(&display,
        140 => {
            vertex: include_str!("vertex.v.glsl"),
            fragment: include_str!("frag.f.glsl"),
        },
    )?;

    event_loop.run(move |event, _target, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::KeyboardInput { input, .. }
                    if input.state == glutin::event::ElementState::Pressed =>
                {
                    if let Some(keycode) = input.virtual_keycode {
                        match keycode {
                            VirtualKeyCode::Escape => *control_flow = Exit,
                            _ => {},
                        };
                    }
                }
                glutin::event::WindowEvent::CloseRequested => *control_flow = Exit,
                _ => (),
            },
            _ => (),
        };
    });
}
