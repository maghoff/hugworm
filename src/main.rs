#![cfg(not(target_arch = "wasm32"))]

use hugworm::platform_opengl;

fn main() {
    platform_opengl::main()
}
