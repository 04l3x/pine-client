use crate::models::record::Record;
use yew::{html, Component, Context, Html, Properties};

pub struct RecordView;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub record: Record,
}

impl Component for RecordView {
	type Message = ();
	type Properties = Props;

	fn create(_: &Context<Self>) -> Self {
		Self
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		html! { <>
			<div>
				{&ctx.props().record.name}
			</div>
		</> }
	}
}
