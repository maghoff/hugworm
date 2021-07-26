#[cfg(target_arch = "wasm32")]
mod platform_wasm;

#[cfg(not(target_arch = "wasm32"))]
extern crate glium;

#[cfg(not(target_arch = "wasm32"))]
pub mod platform_opengl;

mod scene;
mod segment;
mod sequence;
mod turn;
