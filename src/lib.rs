use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::AudioContext;

mod app;
mod collision;
mod config;
mod input;
mod kinematics;
mod loading;
mod meteor;
mod player;
mod projectile;
mod ship;
mod sounds;
mod splash;
mod viewport;

#[wasm_bindgen]
pub fn run_app() {
    app::run();
}
