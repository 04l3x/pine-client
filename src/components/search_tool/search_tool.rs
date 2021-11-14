use crate::models::record::Record;
use crate::utils::{new_style, parser};
use material_yew::text_inputs::MatTextField;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::virtual_dom::VNode;
use yewtil::future::LinkFuture;

pub struct SearchTool {
	link: ComponentLink<Self>,
	query: String,
	results: Option<Vec<Record>>,
}

pub enum Msg {
	Init,
	Search(String),
	UpdateResults(Option<Vec<Record>>),
}

impl Component for SearchTool {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		link.send_message(Msg::Init);
		Self {
			link,
			query: String::from(""),
			results: None,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Init => {
				self.link.send_future(async {
					match Record::get_public_record().await {
						Ok(res) => {
							ConsoleService::log(&format!("Ok: {:?}", res));
							Msg::UpdateResults(res.content())
						}
						Err(err) => {
							ConsoleService::log(&format!("Err: {:?}", err));
							Msg::UpdateResults(None)
						}
					}
				});
			}

			Msg::Search(query) => {
				self.query = query.clone();

				self.link.send_future(async {
					match Record::search(query, 1).await {
						Ok(res) => {
							ConsoleService::log(&format!("Ok: {:?}", res));
							Msg::UpdateResults(res.content())
						}
						Err(err) => {
							ConsoleService::log(&format!("Err: {:?}", err));
							Msg::UpdateResults(None)
						}
					}
				});
			}

			Msg::UpdateResults(results) => {
				self.results = results;
			}
		}
		true
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

		let on_input = self.link.callback(|e: InputData| Msg::Search(e.value));

		html! { <>
			<div class={container}>
				<div class={text_field}>
					<MatTextField  label="search" icon_trailing="search" oninput={on_input} value={self.query.clone()}/>
				</div>
			</div>

			{match &self.results {
				Some(results) => html! { 
					<div>
						{
							for results.iter().map( |rec| {
								html!{ <div>{"record"}<p>{rec.name.clone()}</p></div> }
							})
						}
					</div> 
				},
				None => html! { 
					<div>{"none results to view"}</div> 
				}
			}}
		</> }
	}
}
