use yew::prelude::*;

pub struct Login {}

pub enum Msg {}

impl Component for Login {
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
			<form>
				<div>
					<label for={"username"} >{"username"}</label>
					<input id={"username"} name={"username"} type={"text"} />
				</div>
				<div>
					<label for={"pwd"}>{"password"}</label>
					<input id={"pwd"} name={"pwd"} type={"password"}/>
				</div>
				<button type={"submit"}>{"login"}</button>
			</form>
		</> }
	}
}
