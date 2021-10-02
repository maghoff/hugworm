mod renderer;

use std::time::{Duration, Instant};

use glium::{
    glutin,
    glutin::{
        event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
        event_loop::ControlFlow,
    },
};

use self::renderer::Renderer;
use crate::{scene::Scene, TICKS_PER_SECOND};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_env()?;

    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(500, 500));
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop)?;

    let mut scene = Scene::new();

    let tick_length = Duration::from_secs(1) / TICKS_PER_SECOND;
    log::debug!(
        "ticks_per_second={} tick_length={:?}",
        TICKS_PER_SECOND,
        tick_length
    );

    let mut next_tick = Instant::now() + tick_length;

    let renderer = Renderer::new(display)?;

    renderer.render_scene(&scene);

    event_loop.run(move |event, _target, control_flow| {
        if let Event::WindowEvent { event, .. } = &event {
            match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => match keycode {
                    VirtualKeyCode::Escape => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    _ => {}
                },
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => (),
            };
        }

        if let Event::MainEventsCleared = &event {
            let mut need_render = false;
            let now = Instant::now();
            while now >= next_tick {
                next_tick += tick_length;
                scene.update();
                need_render = true;
            }

            if need_render {
                renderer.render_scene(&scene);
            }

            *control_flow = ControlFlow::WaitUntil(next_tick);
        }
    });
}
