use yew::prelude::*;
use crate::utils::{new_style, parser};

pub struct Viewer;
impl Component for Viewer {
    type Message = ();
    type Properties = ();

	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		Self { }
	}
    
	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}
    
	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}
    
	fn view(&self) -> Html {
		let container = parser( 
			new_style( "img",
				r#"
					margin: 100px;
					width: 600px;
					height: 800px;
					background: silver;
				"#
			)
		);
		
		let top_bar = parser( 
			new_style( "img",
				r#"
					margin: 0px;
					padding: 0px;
					width: 100%;
					height: 48px;
					background: blue;
				"#
			)
		);

		let content = parser( 
			new_style( "img",
				r#"
					margin: 0px;
					padding: px;
					width: 100%;
					height: 100%;
					background: red;
				"#
			)
		);
		html!{
			<div class={container} >
				<div class={top_bar} >
					<div></div>
					<div></div>
				</div>
				
				<div class={content} >
				</div>
			</div>
		}
	}
}

pub struct TreeViewer;
impl  Component for TreeViewer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		Self { }
	}
    
	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}
    
	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}
    
	fn view(&self) -> Html {
		html!{

		}
	}
}

#[allow(dead_code)]
enum EntryType {
	Folder,
	File,
}

#[allow(dead_code)]
enum ViewMode{
	Row,
	Card,
}
