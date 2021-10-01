use css_in_rust::Style;
use yew::html::Classes;

pub fn parser(style: Style) -> Classes {
	Classes::from(style.to_string())
}

pub fn new_style(class_name: &'static str, css: &'static str) -> Style {
	Style::create(class_name, css).expect("no created css")
	/*{
		Ok(style) => style,
		/*Err(e) => {
			panic!("css error: {}", e);
		}*/
	}*/
}
