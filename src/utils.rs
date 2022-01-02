use css_in_rust::Style;
use yew::html::Classes;
use yew::services::console::ConsoleService;
use yew::services::storage::{Area, StorageService};

pub fn local_storage() -> StorageService {
	StorageService::new(Area::Local).expect("not storage available")
}

enum Env {
	Dev,
	_Prod,
	_Testing,
}

pub fn log(s: &str) {
	match check_env() {
		Env::Dev => ConsoleService::log(s),
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
	/*{
		Ok(style) => style,
		/*Err(e) => {
			panic!("css error: {}", e);
		}*/
	}*/
}
