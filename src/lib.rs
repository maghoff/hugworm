use wasm_bindgen::prelude::*;

mod renderer;
mod segment;
mod sequence;
mod webgl;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let context = webgl::get_context("canvas")?;
    let renderer = renderer::Renderer::new(&context)?;

    renderer.render_scene()?;

    Ok(())
}
