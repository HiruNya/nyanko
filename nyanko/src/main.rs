#[macro_use] extern crate cstr;
#[macro_use] extern crate log;
#[macro_use] extern crate qmetaobject;
mod core;
mod gui;
mod model;
mod resource;

use qmetaobject::{QmlEngine, qml_register_type};

use std::env;

fn main() {
	if env::var("RUST_LOG").is_err() { env::set_var("RUST_LOG", "INFO") }
	pretty_env_logger::init();
	info!("Starting");
	let mut engine = QmlEngine::new();
	let finish = nyanko_core::run_runtime();
	resource::declare_resources();
	qml_register_type::<gui::Gui>(cstr!("core"), 1, 0, cstr!("Nyanko"));
	engine.load_file("qrc:/qml/main.qml".into());

	info!("Starting GUI");
	engine.exec();
	core::core().write().unwrap().shutdown();

	info!("Shutting down");
	finish.send(()).expect("Could not close the core runtime");
}
