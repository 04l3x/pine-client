use crate::utils::{new_style, parser};
use material_yew::button::MatButton;
use material_yew::drawer::*;
use material_yew::icon_button::MatIconButton;
use material_yew::text_inputs::MatTextField;
use material_yew::top_app_bar::*;
use yew::prelude::*;

pub struct Home {
	drawer_opened: bool,
	link: ComponentLink<Self>,
}

pub enum Msg {
	Toggle,
}

impl Component for Home {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			drawer_opened: false,
			link,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Toggle => {
				self.drawer_opened = !self.drawer_opened;
				true
			}
		}
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		let toggle = self.link.callback(|_| Msg::Toggle);

		let spacer = match self.drawer_opened {
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
			<MatDrawer open={self.drawer_opened} drawer_type={"dismissible"}>

				<RepositoryLister />

				<MatDrawerAppContent>
					<MatTopAppBar>
						<MatTopAppBarNavigationIcon>
							<span onclick={toggle}> <MatIconButton icon={"menu"} > </MatIconButton> </span>
						</MatTopAppBarNavigationIcon>

						<MatTopAppBarActionItems>
							<LoginButton/>
						</MatTopAppBarActionItems>
						<MatTopAppBarActionItems>
							<RegisterButton/>
						</MatTopAppBarActionItems>
						<MatTopAppBarActionItems>
							<div class={spacer} ></div>
						</MatTopAppBarActionItems>
					</MatTopAppBar>

					<super::tree::Viewer/>

					<RepositoryViewer/>

				</MatDrawerAppContent>
			</MatDrawer>
		</> }
	}
}

pub struct LoginButton;
impl Component for LoginButton {
	type Message = ();
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
			<MatButton label="login" raised=true/>
		</> }
	}
}

pub struct RegisterButton;
impl Component for RegisterButton {
	type Message = ();
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
			<MatButton label="register" raised=true/>
		</> }
	}
}

pub struct RepositoryViewer;
impl Component for RepositoryViewer {
	type Message = ();
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
			{"A cool visor for each repo"}
		</> }
	}
}

pub struct RepositoryLister;
impl Component for RepositoryLister {
	type Message = ();
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
			<SearchTool />
			<div>
				{"Repo lister"}
			</div>
		</> }
	}
}

pub struct SearchTool;
impl Component for SearchTool {
	type Message = ();
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
		let container = parser(new_style(
			"img",
			r#"
					height: 64px;
					background: var(--mdc-theme-primary);
					display: flex;
					align-items: center;
				"#,
		));

		let text_field = parser(new_style(
			"mwc-textfield",
			r#"
					margin-left: 32px;
				"#,
		));

		html! { <>
			<div class={container}>
				<div class={text_field}>
					<MatTextField  label="search" icon_trailing="search" />
				</div>
			</div>
		</> }
	}
}
