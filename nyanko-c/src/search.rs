use log::error;

use std::{ffi::{CStr, CString}, os::raw::{c_char, c_int}};

use super::Init;

#[no_mangle]
pub unsafe extern "C" fn search(nyanko: *mut Init, query: *mut c_char) -> *mut SearchEntries {
	let nyanko = nyanko.as_ref().unwrap();
	let query = CStr::from_ptr(query);
	let query = query.to_str().map_err(|e| error!("Can't verify given string is UTF8: {:?}", e))
		.unwrap().into();
	let join = nyanko.nyanko.search(query);
	let vec = nyanko.nyanko.handle.block_on(join) // blocks
		.map_err(|e| error!("Searching: {:?}", e))
		.unwrap_or_default()
		.unwrap_or_default()
		.into_iter()
		.map(SearchEntry::from)
		.collect::<Vec<_>>();
	let ptr = Box::into_raw(Box::new(SearchEntries::from(vec)));
	ptr

}

#[no_mangle]
pub unsafe extern "C" fn free_search(entries: *mut SearchEntries) {
	let SearchEntries { ptr, len } = *Box::from_raw(entries);
	Vec::from_raw_parts(ptr, len as usize, len as usize)
		.into_iter()
		.for_each(|entry| {
			let SearchEntry { title, image } = entry;
			CString::from_raw(title);
			CString::from_raw(image);
		})
}

#[no_mangle]
#[repr(C)]
pub struct SearchEntries {
	pub ptr: *mut SearchEntry,
	pub len: c_int,
}
impl From<Vec<SearchEntry>> for SearchEntries {
	fn from(o: Vec<SearchEntry>) -> Self {
		let len = o.len() as c_int;
		let ptr = Box::into_raw(o.into_boxed_slice()) as *mut SearchEntry;
		Self {
			ptr,
			len
		}
	}
}

#[no_mangle]
#[repr(C)]
pub struct SearchEntry {
	title: *mut c_char,
	image: *mut c_char,
}
impl From<nyanko_anilist::SearchEntry> for SearchEntry {
	fn from(o: nyanko_anilist::SearchEntry) -> Self {
		let title = o.title.english.or(o.title.user_preferred).unwrap_or_default();
		let image = o.cover_image.large;
		Self {
			title: CString::new(title).unwrap().into_raw(),
			image: CString::new(image).unwrap().into_raw(),
		}
	}
}
