use yew::prelude::*;

pub struct Register {}

pub enum Msg {}

impl Component for Register {
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
					<label for={"email"} >{"email"}</label>
					<input id={"email"} name={"email"} type={"text"} />
				</div>

				<div>
					<label for={"username"} >{"username"}</label>
					<input id={"username"} name={"username"} type={"text"} />
				</div>

				<div>
					<label for={"pwd"}>{"password"}</label>
					<input id={"pwd"} name={"pwd"} type={"password"}/>
				</div>

				<div>
					<label for={"pwd2"}>{"confirm password"}</label>
					<input id={"pwd2"} name={"pwd2"} type={"password"}/>
				</div>

				<button type={"submit"}>{"register"}</button>
			</form>
		</> }
	}
}
