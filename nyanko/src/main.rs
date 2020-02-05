#[macro_use] extern crate cstr;
#[macro_use] extern crate log;
#[macro_use] extern crate qmetaobject;
extern crate nyanko_core as core;
mod gui;
mod model;

use qmetaobject::{QmlEngine, qml_register_type};

qrc!(declare_resources,
	"qml" {
		"./qml/AniGrid.qml" as "AniGrid.qml",
		"./qml/main.qml" as "main.qml",
		"./qml/Search.qml" as "Search.qml",
		"./qml/SearchBar.qml" as "SearchBar.qml",
	}
);

fn main() {
	if std::env::var("RUST_LOG").is_err() { std::env::set_var("RUST_LOG", "INFO") }
	pretty_env_logger::init();
	info!("Starting");
	let mut engine = QmlEngine::new();
	let finish = core::run();
	declare_resources();
	qml_register_type::<gui::Gui>(cstr!("core"), 1, 0, cstr!("Nyanko"));
	engine.load_file("qrc:/qml/main.qml".into());

	info!("Starting GUI");
	engine.exec();

	info!("Shutting down");
	finish.send(()).expect("Could not close the core runtime");
}
