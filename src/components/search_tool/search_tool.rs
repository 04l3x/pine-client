use super::record_view::RecordView;
use crate::models::record::Record;
use crate::utils::{log, new_style, parser};
use material_yew::{
	icon_button::MatIconButton,
	text_inputs::{MatTextField, TextFieldType},
};
use web_sys::MouseEvent;
use yew::{html, Callback, Component, Context, Html, Properties};

pub struct SearchTool {
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

	fn create(ctx: &Context<Self>) -> Self {
		ctx.link().send_message(Msg::Init);
		Self {
			query: String::from(""),
			results: vec![],
		}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Init => {
				ctx.link().send_future(async {
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

				ctx.link().send_future(async {
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
				ctx.props().toggle.emit(e);
			}

			Msg::UpdateResults(results) => {
				self.results = results;
			}
		}
		true
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
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

		let li = parser(new_style(
			"li",
			r#"
				&:active {
					background: red;
				}
				&:hover {
					background: blue;
				}
			"#,
		));

		let on_input = ctx.link().callback(|e: String| Msg::Search(e));

		let toggle = ctx.link().callback(|e| Msg::Toggle(e));

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

			{	if self.results.is_empty() {
					html! {
						<div>{"none results to view"}</div>
					}
				}
				else {
					html! {
						<div>
							<ul>
								{
									for self.results.iter().map( |rec| {
										html!{ <li class={li.clone()}>
											<RecordView record={rec.clone()} />
										</li> }
									})
								}
							</ul>
						</div>
					}
				}
			}
		</> }
	}
}
