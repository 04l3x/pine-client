use crate::models::record::Record;
use crate::utils::{log, new_style, parser};
use material_yew::{
	icon_button::MatIconButton,
	text_inputs::{MatTextField, TextFieldType},
};
use yew::prelude::*;
use yewtil::future::LinkFuture;

use super::record_view::RecordView;

pub struct SearchTool {
	link: ComponentLink<Self>,
	props: Props,
	query: String,
	results: Vec<Record>,
}

pub enum Msg {
	Init,
	Search(String),
	Toggle(MouseEvent),
	UpdateResults(Vec<Record>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub toggle: Callback<MouseEvent>,
}

impl Component for SearchTool {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		link.send_message(Msg::Init);
		Self {
			link,
			props,
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

			Msg::Toggle(e) => {
				self.props.toggle.emit(e);
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
				justify-content: space-evenly;
			"#,
		));

		let text_field = parser(new_style(
			"mwc-textfield",
			r#"
				margin-left: 32px;
			"#,
		));

		let toggle_style = parser(new_style(
			"mwc-textfield",
			r#"
				color: var(--colors-white);
			"#,
		));

		let on_input = self.link.callback(|e: InputData| Msg::Search(e.value));

		let toggle = self.link.callback(|e| Msg::Toggle(e));

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

				<span class={toggle_style} onclick={toggle}> <MatIconButton icon={"close"} > </MatIconButton> </span>
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
