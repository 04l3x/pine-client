use crate::components::{
	repository_viewer::RepositoryViewer, topbar::TopBar,
	search_tool::{
		search_tool::SearchTool
	}
};
//use crate::utils::{new_style, parser};

use material_yew::drawer::*;
use yew::prelude::*;

pub struct Home {
	pub drawer_opened: bool,
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

		let opened = self.drawer_opened;

		html! { <>
			<MatDrawer open={opened} drawer_type={"dismissible"}>

				<SearchTool />

				<MatDrawerAppContent>
					<TopBar toggle={toggle} drawer_opened={opened} />

					<super::tree::Viewer/>

					<RepositoryViewer/>
				</MatDrawerAppContent>
			</MatDrawer>
		</> }
	}
}
