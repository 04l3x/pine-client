use crate::auth::User;
use crate::utils::{log, new_style, parser};
use material_yew::{
    button::MatButton,
    text_inputs::{MatTextField, TextFieldType},
};
use yew::prelude::*;

pub struct Register {
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

    fn create(_: &Context<Self>) -> Self {
        Self {
            email: String::from(""),
            username: String::from(""),
            password: String::from(""),
            password2: String::from(""),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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

                    ctx.link().send_future(async {
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

        let input_email = ctx.link().callback(|e: String| Msg::SetEmail(e));
        let input_usrn = ctx.link().callback(|e: String| Msg::SetUsername(e));
        let input_pwd = ctx.link().callback(|e: String| Msg::SetPassword(e));
        let input_pwd2 = ctx.link().callback(|e: String| Msg::SetPassword2(e));

        let on_register = ctx.link().callback(|_| Msg::Register);

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
