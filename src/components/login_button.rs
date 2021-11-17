use crate::app::AppRoute;
use material_yew::button::MatButton;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct LoginButton;
impl Component for LoginButton {
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
			<RouterAnchor<AppRoute> route=AppRoute::Login>
				<MatButton label="login" raised=true/>
			</RouterAnchor<AppRoute>>
		</> }
	}
}
