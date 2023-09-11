pub(crate) mod error;

use console_error_panic_hook;
use minifb::{Window, WindowOptions};
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::simulation::State;
use crate::window_size::*;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut state = State::new();
    let mut window = Window::new(
        "Julia Explorer",
        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    // A reference counted pointer to the closure that will update and render the game
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    // create the closure for updating and rendering the game.
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let (width, height) = window.get_size();
        let pixels = state.update(&window, width, height);
        window.update_with_buffer(pixels, width, height).unwrap();
        // schedule this closure for running again at next frame
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() + 'static>));

    // start the animation loop
    request_animation_frame(g.borrow().as_ref().unwrap());
}
