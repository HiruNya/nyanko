use log::error;
use tokio::sync::oneshot::Sender;

use nyanko_core::{Nyanko, run_runtime};

#[no_mangle]
pub extern "C" fn init() -> *mut Init {
	#[cfg(target_os = "android")]
		{
			use log::info;

			android_logger::init_once(android_logger::Config::default()
				.with_min_level(log::Level::Info));
			info!("Logger initialised");
		}
	let (shutdown, handle) = run_runtime();
	let nyanko = Nyanko::with_handle(handle);
	Box::into_raw(Box::new(Init {
		nyanko,
		shutdown,
	}))
}

#[no_mangle]
#[repr(C)]
pub struct Init {
	pub nyanko: Nyanko,
	pub shutdown: Sender<()>,
}

#[no_mangle]
pub unsafe extern "C" fn free_init(init: *mut Init) {
	let init = Box::from_raw(init);
	let _ = init.shutdown.send(()).map_err(|e| error!("Shutting down: {:?}", e));
}
