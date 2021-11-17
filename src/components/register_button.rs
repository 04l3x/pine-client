use crate::app::AppRoute;
use material_yew::button::MatButton;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

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
			<RouterAnchor<AppRoute> route=AppRoute::Register>
				<MatButton label="register" raised=true/>
			</RouterAnchor<AppRoute>>
		</> }
	}
}
