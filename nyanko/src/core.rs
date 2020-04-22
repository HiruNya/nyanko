use once_cell::sync::OnceCell;

use std::sync::RwLock;

use nyanko_core::{HANDLE, Nyanko};

pub static CORE: OnceCell<RwLock<Nyanko>> = OnceCell::new();

pub fn core() -> &'static RwLock<Nyanko> {
	CORE.get_or_init(|| RwLock::new(Nyanko::with_handle(HANDLE.get().unwrap().clone())))
}
