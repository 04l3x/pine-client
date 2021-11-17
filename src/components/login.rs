use crate::auth;
use crate::utils::{new_style, parser};
use error::Result;
use material_yew::{
	button::MatButton,
	text_inputs::{MatTextField, TextFieldType},
};
use yew::prelude::*;

pub struct Login {
	link: ComponentLink<Self>,
	username: String,
	password: String,
}

pub enum Msg {
	SetUsername(String),
	SetPassword(String),
	Login,
}

impl Component for Login {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			username: String::from(""),
			password: String::from(""),
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::SetUsername(username) => {
				self.username = username;
			}
			Msg::SetPassword(password) => {
				self.password = password;
			}
			Msg::Login => match Self::login(self.username.clone(), self.password.clone()) {
				_ => {}
			},
		}
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

		let input_usrn = self.link.callback(|e: InputData| Msg::SetUsername(e.value));
		let input_pwd = self.link.callback(|e: InputData| Msg::SetPassword(e.value));

		let on_login = self.link.callback(|_| Msg::Login);

		html! { <>
			<div class={ctn_style} >
				<form>
					<div>
						<MatTextField
							label = {"username"}
							field_type = {TextFieldType::Text}
							oninput = { input_usrn }
							value = { self.username.clone() }
						/>
					</div>
					<div>
						<MatTextField
							label = {"password"}
							field_type = {TextFieldType::Password}
							oninput = { input_pwd }
							value = { self.password.clone() }
						/>
					</div>
					<span onclick = { on_login }>
						<MatButton label={"login"}/>
					</span>
				</form>
			</div>
		</> }
	}
}

impl Login {
	fn login(username: String, password: String) -> Result<()> {
		Ok(auth::Credentials::new(username, password).login())
	}
}
