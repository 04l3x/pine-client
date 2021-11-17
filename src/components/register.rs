use crate::utils::{new_style, parser};
use material_yew::{
	button::MatButton,
	text_inputs::{MatTextField, TextFieldType},
};
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
		let ctn_style = parser(new_style(
			"div",
			r#"
					display: flex;
					width: 100%;
					height: 100vh;
					justify-content: center;
					align-items: center;
				"#,
		));
		html! { <>
			<div class={ctn_style} >
				<form>
					<div>
						<MatTextField
							label = {"email"}
							field_type = {TextFieldType::Email}
						/>
					</div>
					<div>
						<MatTextField
							label = {"username"}
							field_type = {TextFieldType::Text}
						/>
					</div>

					<div>
						<MatTextField
							label = {"password"}
							field_type = {TextFieldType::Password}
						/>
					</div>

					<div>
						<MatTextField
							label = {"confirm password"}
							field_type = {TextFieldType::Password}
						/>
					</div>
					<MatButton label = {"login"} />
				</form>
			</div>
		</> }
	}
}
