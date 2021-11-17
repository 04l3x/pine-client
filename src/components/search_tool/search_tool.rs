use crate::models::record::Record;
use crate::utils::{log, new_style, parser};
use material_yew::text_inputs::{MatTextField, TextFieldType};
use yew::prelude::*;
use yewtil::future::LinkFuture;

use super::record_view::RecordView;

pub struct SearchTool {
	link: ComponentLink<Self>,
	query: String,
	results: Vec<Record>,
}

pub enum Msg {
	Init,
	Search(String),
	UpdateResults(Vec<Record>),
}

impl Component for SearchTool {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		link.send_message(Msg::Init);
		Self {
			link,
			query: String::from(""),
			results: vec![],
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Init => {
				self.link.send_future(async {
					match Record::get_public_record(1).await {
						Ok(res) => {
							log(&format!("Ok: {:?}", res));
							Msg::UpdateResults(res.content())
						}
						Err(err) => {
							log(&format!("Err: {:?}", err));
							Msg::UpdateResults(vec![])
						}
					}
				});
			}

			Msg::Search(query) => {
				self.query = query.clone();

				self.link.send_future(async {
					match Record::search(query, 1).await {
						Ok(res) => {
							log(&format!("Ok: {:?}", res));
							Msg::UpdateResults(res.content())
						}
						Err(err) => {
							log(&format!("Err: {:?}", err));
							Msg::UpdateResults(vec![])
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
					<MatTextField
						label="search"
						field_type={TextFieldType::Search}
						icon_trailing="search"
						oninput={on_input}
						value={self.query.clone()}
					/>
				</div>
			</div>

			{	if !self.results.len() > 0  {
					html! {
						<div>{"none results to view"}</div>
					}
				}
				else {
					html! {
						<div>
							{
								for self.results.iter().map( |rec| {
									html!{ <div>
										<RecordView record={rec.clone()} />
									</div> }
								})
							}
						</div>
					}
				}
			}



		</> }
	}
}
