mod anilist;

use once_cell::sync::OnceCell;
use log::info;
use tokio::{runtime::{self, Handle}, sync::oneshot::{channel as oneshot, Sender}};

use std::thread;

pub use anilist::search;

pub static HANDLE: OnceCell<Handle> = OnceCell::new();

pub fn run() -> Sender<()> {
	let (sender, receiver) = oneshot::<()>();
	let mut runtime = runtime::Builder::new()
		.enable_all()
		.basic_scheduler()
		.build()
		.unwrap();
	info!("Runtime built");
	// Handle can only be set here
	HANDLE.set(runtime.handle().clone()).unwrap();
	thread::spawn(move || {
		info!("Starting");
		runtime
			.block_on(receiver)
			.expect("Error running the core runtime");
		info!("Shutting down")
	});
	sender
}
