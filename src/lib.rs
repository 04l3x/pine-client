mod app;
mod components;
mod models;
mod utils;

use wasm_bindgen::prelude::*;

pub(crate) const ENDPOINT: &str = "http://localhost:9000/api";

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
	yew::start_app::<app::App>();
	Ok(())
}
