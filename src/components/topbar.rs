use crate::utils::{new_style, parser};
use material_yew::icon_button::MatIconButton;
use material_yew::top_app_bar::*;
use yew::prelude::*;
use yew::Callback;

use crate::components::{login_button::LoginButton, register_button::RegisterButton};

pub struct TopBar {
	props: Props,
	link: ComponentLink<Self>,
}

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

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { link, props }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Toggle(e) => {
				self.props.toggle.emit(e);
				true
			}
		}
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		if self.props != props {
			self.props = props;
			true
		} else {
			false
		}
	}

	fn view(&self) -> Html {
		let toggle = self.link.callback(|e| Msg::Toggle(e));

		let spacer_class = match self.props.drawer_opened {
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
					{if !self.props.drawer_opened {
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
