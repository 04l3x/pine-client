use yew::prelude::*;

use crate::components::dashboard::Dashboard;
//use crate::components::login::Login;
//use crate::components::register::Register;

pub struct App {}

pub enum Msg {}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		App {}
	}

	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! { <>
			//<Login />
			//<Register />
			<Dashboard />
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
