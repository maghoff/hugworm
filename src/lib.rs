use wasm_bindgen::prelude::*;

mod renderer;
mod scene;
mod segment;
mod sequence;
mod turn;
mod webgl;
use crate::scene::Scene;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
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
    window.add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref()).unwrap();
    window.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref()).unwrap();

    keyup.forget();
    keydown.forget();
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let scene = Rc::new(RefCell::new(scene::Scene::new()));

    init_keyboard(scene.clone());

    let context = webgl::get_context("canvas")?;
    let renderer = renderer::Renderer::new(context)?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        scene.borrow_mut().update();
        renderer.render_scene(&scene.borrow()).unwrap();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
