use qmetaobject::{execute_async, QAbstractListModel, QObject, QPointer, QString, SimpleListModel};

use nyanko_anilist::Token;

use std::cell::RefCell;

use super::{core::core, model::{AccountEntry, SearchEntry}};

#[derive(QObject, Default)]
#[allow(non_snake_case)]
pub struct Gui {
	base: qt_base_class!(trait QObject),
	accounts: qt_property!(RefCell<SimpleListModel<AccountEntry>>; CONST),
	anilist_login: qt_method!(fn(&self, link: String) -> bool),
	anilist_login_link: qt_method!(fn(&self) -> QString),
	init: qt_method!(fn(&self)),
	search: qt_method!(fn(&self, query: String)),
	search_results: qt_property!(RefCell<SimpleListModel<SearchEntry>>; CONST),
	#[allow(dead_code)]
	current_account: qt_property!(i32; READ get_current_account WRITE set_current_account NOTIFY current_account_changed),
	current_account_changed: qt_signal!(),
	update_user: qt_method!(fn(&self)),
}
impl Gui {
	fn anilist_login(&self, link: String) -> bool {
		Token::new(link).ok()
			.and_then(|token| Some(AccountEntry::from(core().write().unwrap().anilist_create_account(token)?)))
			.map(|account| self.accounts.borrow_mut().push(account))
			.is_some()
	}
	fn anilist_login_link(&self) -> QString { core().read().unwrap().anilist_login_link().into() }
	fn get_current_account(&self) -> i32 {
		let func = || {
			let core = core().read().ok()?;
			let id = &core.accounts.current(&core.settings)?.id;
			self.accounts.borrow()
				.iter().enumerate()
				.find(|(_, account)| &account.id == id)
				.map(|(i, _)| i as i32)
		};
		func().unwrap_or(-1)
	}
	fn init(&self) {
		let mut accounts = self.accounts.borrow_mut();
		core().read().unwrap().accounts.as_ref().values().for_each(|account| {
			accounts.push(AccountEntry::from(account))
		})
	}
	fn search(&self, query: String) {
		let ptr = QPointer::from(&*self);
		execute_async(async move {
			let list = if let Some(result) = core().read().unwrap().search(query).await.unwrap() {
				result.into_iter()
					.map(|entry| SearchEntry::from(entry))
					.collect()
			} else { Vec::default() };
			ptr.as_pinned().map(|ptr| {
				ptr.borrow().search_results.borrow_mut().reset_data(list);
			});
		})
	}
	fn set_current_account(&mut self, index: i32) {
		let accounts = self.accounts.borrow();
		if index < accounts.row_count() {
			let id = accounts[index as usize].id.clone();
			if core().write().unwrap().set_current_account(id) {
				self.current_account_changed()
			}
		}
	}
	fn update_user(&self) {
		let _ = core().write().unwrap().update_user();
	}
}
