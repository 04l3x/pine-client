use yew::prelude::*;
//use crate::components::search_tool::SearchTool;

use crate::models::record::Record;

pub struct RepositoryLister;

pub struct Props {
	
}

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
			//<SearchTool />
			<div>
				{"Repo lister"}
			</div>
		</> }
	}
}


