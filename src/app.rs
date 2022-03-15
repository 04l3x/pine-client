use crate::components;
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum AppRoute {
	#[at("/")]
	Home,
	#[at("/login")]
	Login,
	#[at("/register")]
	Register,
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub struct App;

impl Component for App {
	type Message = ();
	type Properties = ();

	fn create(_: &Context<Self>) -> Self {
		Self
	}

	fn view(&self, _: &Context<Self>) -> Html {
		html! { <>
			<components::home::Home />
		</> }
	}
}

#[cfg(test)]
mod tests {
	use wasm_bindgen_test::wasm_bindgen_test;

	#[wasm_bindgen_test]
	fn test() {
		assert!(true);
	}
}
