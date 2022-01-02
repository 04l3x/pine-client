use crate::components;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
	#[to = "/login"]
	Login,
	#[to = "/register"]
	Register,
	#[to = "/"]
	Home,
}

pub struct App;

pub enum Msg {}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		App {}
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
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
