//use crate::utils::{new_style, parser};
use material_yew::button::MatButton;
use yew::prelude::*;

pub struct RegisterButton;
impl Component for RegisterButton {
	type Message = ();
	type Properties = ();

	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		Self {}
	}

	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! { <>
			<MatButton label="register" raised=true/>
		</> }
	}
}
