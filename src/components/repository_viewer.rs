use crate::models::repository::{Repository, Tree};
use crate::utils::log;
use uuid::Uuid;
use yew::prelude::*;

pub struct RepositoryViewer {
	tree: Option<Tree>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub repo_id: Uuid,
}

pub enum Msg {
	FetchTree,
	UpdateTree(Tree),
}

impl Component for RepositoryViewer {
	type Message = Msg;
	type Properties = Props;

	fn create(ctx: &Context<Self>) -> Self {
		ctx.link().send_message(Msg::FetchTree);
		Self { tree: None }
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::FetchTree => {
				let id = ctx.props().repo_id.clone();

				ctx.link().send_future(async move {
					match Repository::get_tree(id).await {
						Ok(res) => {
							log(&format!("Ok: {:?}", res));
							Msg::UpdateTree(res.content())
						}
						Err(err) => {
							log(&format!("Err: {:?}", err));
							Msg::UpdateTree(Tree::default())
						}
					}
				});
			}

			Msg::UpdateTree(tree) => self.tree = Some(tree),
		}
		true
	}

	fn view(&self, _: &Context<Self>) -> Html {
		html! { <>
			{"A cool visor for each repo"}
		</> }
	}
}
