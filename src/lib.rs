use wasm_bindgen::prelude::*;

mod renderer;
mod segment;
mod sequence;
mod webgl;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let context = webgl::get_context("canvas")?;
    let renderer = renderer::Renderer::new(context)?;

    let callback = Closure::wrap(Box::new(move || {
        renderer.render_scene().unwrap();
    }) as Box<dyn FnMut()>);

    let window = web_sys::window().unwrap();
    window.request_animation_frame(callback.as_ref().unchecked_ref())?;

    callback.forget();

    Ok(())
}
