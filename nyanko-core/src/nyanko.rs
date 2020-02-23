use tokio::{runtime::Handle, task::JoinHandle};

use nyanko_anilist::{Client as AniClient, SearchEntry};

pub struct Nyanko {
	client: AniClient,
	handle: Handle,
}
impl Default for Nyanko {
	fn default() -> Self {
		Self {
			client: AniClient::new(),
			handle: Handle::current(),
		}
	}
}
impl Nyanko {
	pub fn with_handle(handle: Handle) -> Self {
		Self { client: AniClient::new(), handle }
	}
	pub fn search(&self, query: String) -> JoinHandle<Option<Vec<SearchEntry>>> {
		let client = self.client.clone();
		self.handle.spawn(async move { client.search(query).await.ok() })
	}
}
