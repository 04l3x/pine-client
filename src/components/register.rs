use crate::auth::User;
use crate::utils::{log, new_style, parser};
use material_yew::{
	button::MatButton,
	text_inputs::{MatTextField, TextFieldType},
};
use yew::prelude::*;
use yewtil::future::LinkFuture;

pub struct Register {
	link: ComponentLink<Self>,
	email: String,
	username: String,
	password: String,
	password2: String,
}

pub enum Msg {
	Clean,
	SetEmail(String),
	SetUsername(String),
	SetPassword(String),
	SetPassword2(String),
	Register,
}

impl Component for Register {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			email: String::from(""),
			username: String::from(""),
			password: String::from(""),
			password2: String::from(""),
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Clean => {
				self.email = String::from("");
				self.username = String::from("");
				self.password = String::from("");
				self.password2 = String::from("");
				true
			}
			Msg::SetEmail(email) => {
				self.email = email;
				false
			}
			Msg::SetUsername(username) => {
				self.username = username;
				false
			}
			Msg::SetPassword(password) => {
				self.password = password;
				false
			}
			Msg::SetPassword2(password2) => {
				self.password2 = password2;
				false
			}
			Msg::Register => {
				if self.password == self.password2 {
					let new_user = User::new(
						self.email.clone(),
						self.username.clone(),
						self.password.clone(),
					);

					self.link.send_future(async {
						match new_user.register().await {
							Ok(_) => {
								log(&format!("ok"));
								Msg::Clean
							}
							Err(e) => {
								log(&format!("err: {:?}", e));
								Msg::Clean
							}
						}
					});
				}
				true
			}
		}
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

		let input_email = self.link.callback(|e: InputData| Msg::SetEmail(e.value));
		let input_usrn = self.link.callback(|e: InputData| Msg::SetUsername(e.value));
		let input_pwd = self.link.callback(|e: InputData| Msg::SetPassword(e.value));
		let input_pwd2 = self
			.link
			.callback(|e: InputData| Msg::SetPassword2(e.value));

		let on_register = self.link.callback(|_| Msg::Register);

		html! { <>
			<div class={ctn_style} >
				<form>
					<div>
						<MatTextField
							label = {"email"}
							field_type = {TextFieldType::Email}
							oninput = { input_email }
							value = { self.email.clone() }
						/>
					</div>
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

					<div>
						<MatTextField
							label = {"confirm password"}
							field_type = {TextFieldType::Password}
							oninput = { input_pwd2 }
							value = { self.password2.clone() }
						/>
					</div>
					<span onclick = { on_register }>
						<MatButton label = {"register"} />
					</span>
				</form>
			</div>
		</> }
	}
}
