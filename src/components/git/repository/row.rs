use yew::prelude::*;
//use yew::virtual_dom::VNode;

pub struct View;

pub enum Msg {}

impl Component for View {
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
		html! {
			<>
				<div>
					{"repo list view"}
				</div>
			</>
		}
	}
}
