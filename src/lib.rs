use wasm_bindgen::prelude::wasm_bindgen;

mod app;
mod config;
mod input;
mod player;
mod ship;

#[wasm_bindgen]
pub fn run_app() {
    app::run();
}
