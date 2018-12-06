// Eternal stuff
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

// My stuff
pub mod render_sys;

use render_sys::Renderer;

// Engine
#[wasm_bindgen]
pub struct Engine {
    renderer: Renderer,
}

#[wasm_bindgen]
impl Engine {

    #[wasm_bindgen]
    pub fn new() -> Result<(Engine), JsValue> {
        let renderer = Renderer::new()?;

        Ok (Engine {
            renderer,
        })
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) -> Result<(), JsValue> {
        // do stuff here


        // the last thing we do
        self.renderer.draw()
    }
}