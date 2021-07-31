#![cfg(not(target_arch = "wasm32"))]

use hugworm::platform_opengl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_env()?;

    platform_opengl::main()
}
