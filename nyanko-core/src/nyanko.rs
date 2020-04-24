use tokio::{runtime::Handle, task::JoinHandle};

use nyanko_anilist::{Client as AniClient, SearchEntry, Token, Viewer};

use crate::accounts::{Accounts, AniListAccount};
use crate::settings::Settings;

pub struct Nyanko {
	pub accounts: Accounts,
	pub client: AniClient,
	pub handle: Handle,
	pub settings: Settings,
}
impl Default for Nyanko {
	fn default() -> Self {
		Self::with_handle(Handle::current())
	}
}
impl Nyanko {
	pub fn with_handle(handle: Handle) -> Self {
		Self {
			accounts: Accounts::load(),
			client: AniClient::new(env!("ANILIST_CLIENT_ID").to_string()),
			handle,
			settings: Settings::load(),
		}
	}
	pub fn shutdown(&mut self) {
		self.settings.save();
	}
	// This blocks
	pub fn anilist_create_account(&mut self, token: Token) -> Option<&AniListAccount> {
		let account = self.accounts.create(token)?;
		let id = account.id.clone();
		let token = account.token.token.clone();
		let user = futures::executor::block_on(async { self.user(token).await }).ok()??;
		let account = self.accounts.as_mut().get_mut(&id)?;
		account.update(user);
		Some(account)
	}
	pub fn anilist_login_link(&self) -> String { self.client.auth_link() }
	pub fn current_account(&self) -> Option<&AniListAccount> { self.accounts.current(&self.settings) }
	pub fn search(&self, query: String) -> JoinHandle<Option<Vec<SearchEntry>>> {
		let client = self.client.clone();
		self.handle.spawn(async move { client.search(query).await.ok() })
	}
	pub fn set_current_account(&mut self, current: String) -> bool {
		self.accounts.set_current(&mut self.settings, current)
	}
	pub async fn update_user(&mut self) -> Option<()> {
		let token = self.accounts.current(&self.settings)?.token.token.clone();
		let user = self.user(token).await.ok()??;
		let account = self.accounts.current_mut(&self.settings)?;
		account.update(user);
		Some(())
	}
	pub fn user(&self, token: String) -> JoinHandle<Option<Viewer>> {
		let client = self.client.clone();
		self.handle.spawn(async move { client.user(&token).await.ok() })
	}
}
