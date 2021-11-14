use yew::prelude::*;

pub struct RepositoryViewer;
impl Component for RepositoryViewer {
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
			{"A cool visor for each repo"}
		</> }
	}
}
