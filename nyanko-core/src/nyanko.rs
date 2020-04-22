use tokio::{runtime::Handle, task::JoinHandle};

use nyanko_anilist::{Client as AniClient, SearchEntry, Token, Viewer};

use crate::accounts::{Accounts, AniListAccount};

pub struct Nyanko {
	pub accounts: Accounts,
	pub client: AniClient,
	pub handle: Handle,
}
impl Default for Nyanko {
	fn default() -> Self {
		Self {
			accounts: Accounts::load(),
			client: AniClient::new(env!("ANILIST_CLIENT_ID").to_string()),
			handle: Handle::current(),
		}
	}
}
impl Nyanko {
	pub fn with_handle(handle: Handle) -> Self {
		Self {
			accounts: Accounts::load(),
			client: AniClient::new(env!("ANILIST_CLIENT_ID").to_string()),
			handle
		}
	}
	// This blocks
	pub fn anilist_create_account(&mut self, token: Token) -> Option<&AniListAccount> {
		self.accounts.create(token)?;
		let account = self.accounts.as_ref().last()?;
		let user = futures::executor::block_on(async { self.user(account.token.token.clone()).await }).ok()??;
		let account = self.accounts.as_mut().last_mut()?;
		account.update(user);
		Some(account)
	}
	pub fn anilist_login_link(&self) -> String { self.client.auth_link() }
	pub fn search(&self, query: String) -> JoinHandle<Option<Vec<SearchEntry>>> {
		let client = self.client.clone();
		self.handle.spawn(async move { client.search(query).await.ok() })
	}
	pub async fn update_user(&mut self) -> Option<()> {
		let token = self.accounts.current()?.token.token.clone();
		let user = self.user(token).await.ok()??;
		let account = self.accounts.current_mut()?;
		account.update(user);
		Some(())
	}
	pub fn user(&self, token: String) -> JoinHandle<Option<Viewer>> {
		let client = self.client.clone();
		self.handle.spawn(async move { client.user(&token).await.ok() })
	}
}
