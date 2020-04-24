mod accounts;
mod nyanko;
mod settings;

pub use accounts::AniListAccount;
pub use nyanko::Nyanko;

use once_cell::sync::OnceCell;
use log::info;
use tokio::{runtime::{self, Handle}, sync::oneshot::{channel as oneshot, Sender}};

use std::thread;

/// The handle to a Tokio runtime.
/// This will only be set if [`run_runtime`] is called before using this function.
pub static HANDLE: OnceCell<Handle> = OnceCell::new();

/// Runs a Tokio runtime in a separate thread.
///
/// Use this *only* if you can't build your own runtime in the current thread.
///
/// The Tokio Handle for the runtime can be taken from [`HANDLE`].
/// However only one handle can be set so this function will panic if run more than once.
pub fn run_runtime() -> Sender<()> {
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
