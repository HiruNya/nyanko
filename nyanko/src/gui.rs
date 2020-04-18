use qmetaobject::{execute_async, QObject, QPointer, QString, SimpleListModel};

use std::cell::RefCell;

use super::{core::core, model::SearchEntry};

#[derive(QObject, Default)]
#[allow(non_snake_case)]
pub struct Gui {
	base: qt_base_class!(trait QObject),
	anilist_login: qt_method!(fn(&self, link: String)),
	anilist_login_link: qt_method!(fn(&self) -> QString),
	search: qt_method!(fn(&self, query: String)),
	search_results: qt_property!(RefCell<SimpleListModel<SearchEntry>>; CONST),
}
impl Gui {
	fn anilist_login(&self, link: String) {
		info!("Successfully Logged in: {}", link)
	}
	fn anilist_login_link(&self) -> QString { core().anilist_login_link().into() }
	fn search(&self, query: String) {
		let ptr = QPointer::from(&*self);
		execute_async(async move {
			let list = if let Some(result) = core().search(query).await.unwrap() {
				result.into_iter()
					.map(|entry| SearchEntry::from(entry))
					.collect()
			} else { Vec::default() };
			ptr.as_pinned().map(|ptr| {
				ptr.borrow().search_results.borrow_mut().reset_data(list);
			});
		})
	}
}
