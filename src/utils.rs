use css_in_rust::Style;
use gloo_console;
use yew::html::Classes;

enum Env {
	Dev,
	_Prod,
	_Testing,
}

pub fn log(s: &str) {
	match check_env() {
		Env::Dev => gloo_console::log!(s),
		_ => (),
	}
}

fn check_env() -> Env {
	Env::Dev
}

pub fn parser(style: Style) -> Classes {
	Classes::from(style.to_string())
}

pub fn new_style(class_name: &'static str, css: &'static str) -> Style {
	Style::create(class_name, css).expect("no created css")
}
