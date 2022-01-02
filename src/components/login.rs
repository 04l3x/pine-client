//use crate::app::AppRoute;
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
	username: Option<String>,
	password: Option<String>,
}

pub enum Msg {
	Clean,
	Login,
	SetUsername(Option<String>),
	SetPassword(Option<String>),
}

impl Component for Login {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			username: None,
			password: None,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Clean => {
				self.username = None;
				self.password = None;
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
				match &self.username {
					Some(username) => match &self.password {
						Some(password) => {
							let credentials = Credentials::new(username.clone(), password.clone());

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
						}
						None => (),
					},
					None => (),
				}

				//are you here? really?
				drop(self);
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

		let input_usrn = self
			.link
			.callback(|e: InputData| Msg::SetUsername(Some(e.value)));
		let input_pwd = self
			.link
			.callback(|e: InputData| Msg::SetPassword(Some(e.value)));

		let on_login = self.link.callback(|_| Msg::Login);
		//let on_login = self.link.callback(|_| drop(input_pwd));

		html! { <>
			<div class={ctn_style} >
				<form>
					<div>
						<MatTextField
							label = {"username"}
							field_type = {TextFieldType::Text}
							oninput = { input_usrn }
							//value = { match &self.username {
							//	Some(username) => username.clone(),
							//	None => String::from(""),
							//} }
						/>
					</div>
					<div>
						<MatTextField
							label = {"password"}
							field_type = {TextFieldType::Password}
							oninput = { input_pwd }
							//value = { match &self.password {
							//	Some(password) => password.clone(),
							//	None => String::from(""),
							//} }
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
