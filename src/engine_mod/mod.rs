extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub mod triangle_mod;

use self::triangle_mod::Triangle;
use render_mod::Renderer;
use timer_mod::Timer;


// Engine
#[wasm_bindgen]
pub struct Engine {
    triangle: Triangle,
    renderer: Renderer,
    timer: Timer,
}

#[wasm_bindgen]
impl Engine {

    #[wasm_bindgen]
    pub fn new() -> Result<(Engine), JsValue> {

        let timer = Timer::new();

        let renderer = Renderer::new()?;

        let triangle = Triangle::new();

        Ok (Engine {
            triangle,
            renderer,
            timer,
        })
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) -> Result<(), JsValue> {
        // first tick delta time
        let delta =self.timer.tick_delta();

        // do stuff here
        self.triangle.rotate(delta as f32);

        // the last thing we do
        self.renderer.draw(&self.triangle)
    }
}