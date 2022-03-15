use crate::app::AppRoute;
use crate::components::{
	dashboard::Dashboard, login::Login, register::Register, search_tool::search_tool::SearchTool,
	topbar::TopBar,
};
use material_yew::drawer::*;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Home {
	drawer_opened: bool,
}

pub enum Msg {
	Toggle,
}

impl Component for Home {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {
			drawer_opened: false,
		}
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Toggle => {
				self.drawer_opened = !self.drawer_opened;
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let toggle = ctx.link().callback(|_| Msg::Toggle);

		let opened = self.drawer_opened;

		html! { <>
			<MatDrawer open={opened} drawer_type={"dismissible"}>

				<SearchTool toggle={toggle.clone()} />

				<MatDrawerAppContent>
					<BrowserRouter>
						<TopBar toggle={toggle} drawer_opened={opened} />
						<Switch<AppRoute> render={Switch::render(Self::switch)} />
					</BrowserRouter>
				</MatDrawerAppContent>
			</MatDrawer>
		</> }
	}
}

impl Home {
	fn switch(switch: &AppRoute) -> Html {
		match switch {
			AppRoute::Login => html! { <Login/> },
			AppRoute::Register => html! { <Register/> },
			AppRoute::Home => html! { <Dashboard/> },
			AppRoute::NotFound => html! { <span>{"not found"}</span> },
		}
	}
}
