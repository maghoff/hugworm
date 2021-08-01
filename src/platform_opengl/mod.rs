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
    let mut pargs = pico_args::Arguments::from_env();

    simple_logger::init_with_env()?;

    // nice_mode only renders a frame whenever the simulation has stepped forward. However,
    // for unknown reasons, this creates jitter. Even if a frame is always rendered per
    // simulation step.
    let nice_mode = pargs.contains("--nice");

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
            let mut ticks_ticked = 0;
            let mut need_render = false;
            let now = Instant::now();
            while now >= next_tick {
                next_tick += tick_length;
                scene.update();
                need_render = true;
                ticks_ticked += 1;
            }

            if nice_mode {
                if ticks_ticked > 0 {
                    log::trace!("--------------- {}", ticks_ticked);
                }

                if need_render {
                    let before = Instant::now();
                    renderer.render_scene(&scene);
                    let duration = Instant::now() - before;
                    log::trace!("{:5.5}", duration.as_micros());
                }

                *control_flow = ControlFlow::WaitUntil(next_tick);
            } else {
                renderer.render_scene(&scene);
            }
        }
    });
}
