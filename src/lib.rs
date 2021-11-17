mod app;
mod auth;
mod components;
mod graphql;
mod models;
mod utils;

use wasm_bindgen::prelude::*;

pub(crate) fn endpoint() -> &'static str {
	"http://localhost:9000/api"
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
	yew::start_app::<app::App>();
	Ok(())
}
