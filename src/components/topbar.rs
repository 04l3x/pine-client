use crate::components::{login_button::LoginButton, register_button::RegisterButton};
use crate::utils::{new_style, parser};
use material_yew::icon_button::MatIconButton;
use material_yew::top_app_bar::*;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew::Callback;

pub struct TopBar;

pub enum Msg {
	Toggle(MouseEvent),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub toggle: Callback<MouseEvent>,
	pub drawer_opened: bool,
}

impl Component for TopBar {
	type Message = Msg;
	type Properties = Props;

	fn create(_: &Context<Self>) -> Self {
		Self {}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Toggle(e) => {
				ctx.props().toggle.emit(e);
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let toggle = ctx.link().callback(|e| Msg::Toggle(e));

		let spacer_class = match ctx.props().drawer_opened {
			true => parser(new_style(
				"div",
				r#"
					width: var(--mdc-drawer-width);
				"#,
			)),
			false => parser(new_style(
				"div",
				r#"
					width: 0px;
				"#,
			)),
		};

		html! { <>
			<MatTopAppBar>
				<MatTopAppBarNavigationIcon>
					{if !ctx.props().drawer_opened {
						html! { <span onclick={toggle}> <MatIconButton icon={"search"} > </MatIconButton> </span> }
					} else {
						html! { }
					}}
				</MatTopAppBarNavigationIcon>

				<MatTopAppBarActionItems>
					<LoginButton/>
				</MatTopAppBarActionItems>
				<MatTopAppBarActionItems>
					<RegisterButton/>
				</MatTopAppBarActionItems>
				<MatTopAppBarActionItems>
					<div class={spacer_class}></div>
				</MatTopAppBarActionItems>
			</MatTopAppBar>

		</> }
	}
}
