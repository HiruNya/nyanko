use futures::FutureExt;
#[macro_use] extern crate log;
use web_view::{Content, WebViewBuilder};

use std::env;

mod server;

fn main() {
	if env::var("RUST_LOG").is_err() { env::set_var("RUST_LOG", "INFO") }
	pretty_env_logger::init();
	let (shutdown, shutdown_recv) = tokio::sync::oneshot::channel();

	let socket = server::serve(shutdown_recv.map(|_| ()));
	info!("Hosted on ethereal port: {}", socket);

	WebViewBuilder::new()
		.title("Nyanko")
		.content(Content::Url(format!("http://{}/index.html", socket)))
		.invoke_handler(|_, _| Ok(()))
		.user_data(())
		.resizable(true)
		.debug(cfg!(debug_assertions))
		.build()
		.expect("Could not build web view GUI")
		.run()
		.expect("Error running the web view GUI");

	info!("Shutting down");
	shutdown.send(()).expect("Could not close the core runtime");
}
