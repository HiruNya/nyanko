use qmetaobject::{execute_async, QObject, QPointer, SimpleListModel};

use std::cell::RefCell;

use super::model::SearchEntry;

#[derive(QObject, Default)]
#[allow(non_snake_case)]
pub struct Gui {
	base: qt_base_class!(trait QObject),
	search: qt_method!(fn(&self, query: String)),
	search_results: qt_property!(RefCell<SimpleListModel<SearchEntry>>; NOTIFY searchResultsChanged),
	searchResultsChanged: qt_signal!(),
}
impl Gui {
	fn search(&self, query: String) {
		let ptr = QPointer::from(&*self);
		execute_async(async move {
			let result = if let Some(result) = core::search(query).await.unwrap() {
				result.into_iter()
					.map(|entry| SearchEntry::from(entry))
					.collect()
			} else { SimpleListModel::default() };
			ptr.as_pinned().map(|ptr|{
				let ptr = ptr.borrow();
				*(ptr.search_results.borrow_mut()) = result;
				ptr.searchResultsChanged();
			});
		})
	}
}
