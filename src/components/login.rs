//use crate::app::AppRoute;
use crate::auth::Credentials;
use crate::utils::{log, new_style, parser};
use material_yew::{
	button::MatButton,
	text_inputs::{MatTextField, TextFieldType},
};
use yew::prelude::*;

pub struct Login {
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

	fn create(_: &Context<Self>) -> Self {
		Self {
			username: String::new(),
			password: String::new(),
		}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Clean => {
				self.username = String::new();
				self.password = String::new();
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
				if !self.username.eq("") {
					if !self.password.eq("") {
						let credentials =
							Credentials::new(self.username.clone(), self.password.clone());

						ctx.link().send_future(async {
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
					}
				}

				//are you here?
				drop(self);
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
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

		let input_usrn = ctx.link().callback(|e: String| Msg::SetUsername(e));

		let input_pwd = ctx.link().callback(|e: String| Msg::SetPassword(e));

		let on_login = ctx.link().callback(|_| Msg::Login);

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
