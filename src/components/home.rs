use crate::app::AppRoute;
use crate::components::{
	dashboard::Dashboard, login::Login, register::Register, search_tool::search_tool::SearchTool,
	topbar::TopBar,
};
use crate::utils::log;
use material_yew::drawer::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::service::RouteService;

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
		Self::current_route();
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

		let opened = self.drawer_opened;

		html! { <>
			<MatDrawer open={opened} drawer_type={"dismissible"}>

				<SearchTool toggle={toggle.clone()} />

				<MatDrawerAppContent>
					<TopBar toggle={toggle} drawer_opened={opened} />
					<Router<AppRoute, ()>
						render = Router::render(Self::switch)
					/>
				</MatDrawerAppContent>
			</MatDrawer>
		</> }
	}
}

impl Home {
	fn switch(switch: AppRoute) -> Html {
		match switch {
			AppRoute::Login => html! { <Login/> },
			AppRoute::Register => html! { <Register/> },
			AppRoute::Home => html! { <Dashboard/> },
		}
	}

	fn current_route() -> Route {
		let route_service: RouteService<()> = RouteService::new();
		let route = route_service.get_route();
		log(&format!("{:?}", route));
		route
	}
}
