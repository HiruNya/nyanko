use once_cell::sync::OnceCell;

use nyanko_core::{HANDLE, Nyanko};

pub static CORE: OnceCell<Nyanko> = OnceCell::new();

pub fn core() -> &'static Nyanko {
	CORE.get_or_init(|| Nyanko::with_handle(HANDLE.get().unwrap().clone()))
}
