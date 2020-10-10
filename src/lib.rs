use wasm_bindgen::prelude::*;

mod renderer;
mod scene;
mod segment;
mod sequence;
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
    let callback = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let handled = match event.key_code() {
            37 => { scene.borrow_mut().turn_left(); true }
            38 => { scene.borrow_mut().turn_straight(); true }
            39 => { scene.borrow_mut().turn_right(); true }
            _ => false,
        };

        if handled {
            event.prevent_default();
        }
    }) as Box<dyn FnMut(_)>);

    let window = web_sys::window().unwrap();
    window.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref()).unwrap();

    callback.forget();
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
