use wasm_bindgen::prelude::*;

mod renderer;
mod webgl;

use crate::{scene::Scene, TICKS_PER_SECOND};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

fn request_animation_frame(f: &Closure<dyn FnMut(f32)>) {
    let window = web_sys::window().unwrap();

    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn init_keyboard(scene: Rc<RefCell<Scene>>) {
    let keyup = {
        let scene = scene.clone();
        Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let handled = scene.borrow_mut().key_event(event.key_code(), false);
            if handled {
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>)
    };

    let keydown = {
        let scene = scene.clone();
        Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let handled = scene.borrow_mut().key_event(event.key_code(), true);
            if handled {
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>)
    };

    let window = web_sys::window().unwrap();
    window
        .add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref())
        .unwrap();
    window
        .add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())
        .unwrap();

    keyup.forget();
    keydown.forget();
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    log::set_logger(&wasm_bindgen_console_logger::DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let scene = Rc::new(RefCell::new(Scene::new()));

    let tick_length_ms = 1000. / (TICKS_PER_SECOND as f32);
    let mut prev_tick_opt = None;

    init_keyboard(scene.clone());

    let context = webgl::get_context("canvas")?;
    let renderer = renderer::Renderer::new(context)?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp_ms: f32| {
        let prev_tick = prev_tick_opt.unwrap_or(timestamp_ms);
        let ticks = ((timestamp_ms - prev_tick) / tick_length_ms) as u32;
        prev_tick_opt = Some(prev_tick + ticks as f32 * tick_length_ms);

        log::trace!("Simulating {} tick(s)", ticks);
        let mut scene = scene.borrow_mut();
        for _ in 0..ticks {
            scene.update();
        }

        renderer.render_scene(&*scene).unwrap();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(_)>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
