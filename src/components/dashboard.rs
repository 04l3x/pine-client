use yew::prelude::*;

pub struct Dashboard {}

pub enum Msg {}

impl Component for Dashboard {
	type Message = Msg;
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
			<div>
				<p>{"welcome"}</p>
			</div>
		</> }
	}
}
