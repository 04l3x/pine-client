use uuid::Uuid;
use yew::prelude::*;

use crate::components::repository_viewer::RepositoryViewer;

pub struct Dashboard {}

pub enum Msg {}

impl Component for Dashboard {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {}
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		let id = Uuid::parse_str("83501ab1-de6f-45a1-b1b1-6b6c5f1048b8").unwrap();
		html! { <>
			<div>
				<p>{"welcome"}</p>
				<RepositoryViewer repo_id={id} />
			</div>
		</> }
	}
}
