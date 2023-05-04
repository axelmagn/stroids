use wasm_bindgen::prelude::wasm_bindgen;

mod app;
mod collision;
mod config;
mod input;
mod loading;
mod meteor;
mod player;
mod ship;
mod viewport;

#[wasm_bindgen]
pub fn run_app() {
    app::run();
}
