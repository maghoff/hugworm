use wasm_bindgen::prelude::*;

mod renderer;
mod scene;
mod segment;
mod sequence;
mod webgl;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    let window = web_sys::window().unwrap();

    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut scene = scene::Scene::new();

    let context = webgl::get_context("canvas")?;
    let renderer = renderer::Renderer::new(context)?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        scene.update();
        renderer.render_scene(&scene).unwrap();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
