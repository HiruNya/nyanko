use once_cell::sync::OnceCell;
use tokio::task::JoinHandle;

use super::HANDLE;
use nyanko_anilist::{Client, SearchEntry};

pub fn client() -> Client {
	static CLIENT: OnceCell<Client> = OnceCell::new();
	CLIENT.get_or_init(|| Client::new()).clone()
}

pub fn search(query: String) -> JoinHandle<Option<Vec<SearchEntry>>> {
	HANDLE.get().unwrap().spawn(async { client().search(query).await.ok() })
}
