// External stuff
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

// My stuff
pub mod render_mod;
pub mod timer_mod;

use render_mod::Renderer;
use timer_mod::Timer;

// Engine
#[wasm_bindgen]
pub struct Engine {
    renderer: Renderer,
    timer: Timer,
}

#[wasm_bindgen]
impl Engine {

    #[wasm_bindgen]
    pub fn new() -> Result<(Engine), JsValue> {

        let timer = Timer::new();

        let renderer = Renderer::new()?;

        Ok (Engine {
            renderer,
            timer,
        })
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) -> Result<(), JsValue> {
        // first tick delta time
        let delta =self.timer.tick_delta();

        // do stuff here

        // TODO: Affect the triangle here

        // the last thing we do
        self.renderer.draw(delta)
    }
}
