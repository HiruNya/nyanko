use tokio::{runtime::Handle, task::JoinHandle};

use nyanko_anilist::{Client as AniClient, SearchEntry};

pub struct Nyanko {
	client: AniClient,
	handle: Handle,
}
impl Default for Nyanko {
	fn default() -> Self {
		Self {
			client: AniClient::new(env!("ANILIST_CLIENT_ID").to_string()),
			handle: Handle::current(),
		}
	}
}
impl Nyanko {
	pub fn with_handle(handle: Handle) -> Self {
		Self { client: AniClient::new(env!("ANILIST_CLIENT_ID").to_string()), handle }
	}
	pub fn anilist_login_link(&self) -> String { self.client.auth_link() }
	pub fn search(&self, query: String) -> JoinHandle<Option<Vec<SearchEntry>>> {
		let client = self.client.clone();
		self.handle.spawn(async move { client.search(query).await.ok() })
	}
}
