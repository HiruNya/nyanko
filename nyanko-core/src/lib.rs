mod accounts;
mod nyanko;
mod settings;

pub use accounts::AniListAccount;
pub use nyanko::Nyanko;

use log::info;
use tokio::{runtime::{self, Handle}, sync::oneshot::{channel as oneshot, Sender}};

use std::thread;

/// Runs a Tokio runtime in a separate thread.
///
/// Use this *only* if you can't build your own runtime in the current thread.
pub fn run_runtime() -> (Sender<()>, Handle) {
	let (sender, receiver) = oneshot::<()>();
	let mut runtime = runtime::Builder::new()
		.enable_all()
		.basic_scheduler()
		.build()
		.unwrap();
	info!("Runtime built");
	let handle = runtime.handle().clone();
	thread::spawn(move || {
		info!("Starting");
		runtime
			.block_on(receiver)
			.expect("Error running the core runtime");
		info!("Shutting down")
	});
	(sender, handle)
}
