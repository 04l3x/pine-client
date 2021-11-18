use crate::app::AppRoute;
use crate::auth::Credentials;
use crate::utils::{log, new_style, parser};
use material_yew::{
	button::MatButton,
	text_inputs::{MatTextField, TextFieldType},
};
use yew::prelude::*;
use yewtil::future::LinkFuture;

pub struct Login {
	link: ComponentLink<Self>,
	username: String,
	password: String,
}

pub enum Msg {
	Clean,
	Login,
	SetUsername(String),
	SetPassword(String),
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
			Msg::Clean => {
				self.username = String::from("");
				self.password = String::from("");
				true
			}
			Msg::SetUsername(username) => {
				self.username = username;
				false
			}
			Msg::SetPassword(password) => {
				self.password = password;
				false
			}
			Msg::Login => {
				let credentials = Credentials::new(self.username.clone(), self.password.clone());

				self.link.send_future(async {
					match credentials.login().await {
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

				false
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
