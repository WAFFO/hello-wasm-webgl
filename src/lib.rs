// External stuff
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// My stuff
pub mod engine_mod;
pub mod render_mod;
pub mod timer_mod;

use engine_mod::Engine;

// JS STUFF
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}


// MAIN
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let mut engine = Engine::new()?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {

        engine.tick();

        // Schedule ourselves for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}