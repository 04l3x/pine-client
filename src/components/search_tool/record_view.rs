use crate::models::record::Record;
use yew::prelude::*;

pub struct RecordView {
	props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub record: Record,
}

impl Component for RecordView {
	type Message = ();
	type Properties = Props;

	fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
		Self { props }
	}

	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		if self.props != props {
			self.props = props;
			true
		} else {
			false
		}
	}

	fn view(&self) -> Html {
		html! { <>
			<div>
				{self.props.record.name.clone()}
			</div>
		</> }
	}
}
