mod search;
use log::{info, error};
use reqwest::Result as ReqwestResult;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

const URL: &str = "https://graphql.anilist.co";

pub use search::{SearchEntry, SearchTitle, CoverImage};

#[derive(Clone)]
pub struct Client {
	client: reqwest::Client,
}
impl Client {
	pub fn new() -> Self {
		Self {
			client: reqwest::Client::new(),
		}
	}

	pub async fn search(&self, query: String) -> ReqwestResult<Vec<SearchEntry>> {
		info!("Searching for `{}`", query);
		match search::search(&self.client, &query).await {
			Ok(result) => {
				info!("Got {} results", result.len());
				Ok(result)
			}
			Err(error) => {
				error!("{:?}", error);
				Err(error)
			}
		}
	}
}

#[derive(Serialize)]
pub(crate) struct GraphQlQueryPayload {
	pub query: &'static str,
	pub variables: JsonValue,
}

#[derive(Deserialize)]
pub(crate) struct GraphQlResponsePayload<T> {
	data: Option<T>,
	#[serde(default)]
	errors: Vec<GraphQlError>
}
impl<T> GraphQlResponsePayload<T> {
	pub fn unwrap(self) -> T {
		match self.data {
			Some(data) => data,
			None => panic!("GraphQl Error: {:?}", self.errors),
		}
	}
}

#[derive(Deserialize)]
pub(crate) struct GraphQlPage<T> {
	#[serde(rename = "Page")]
	pub page: T,
}

#[derive(Deserialize)]
pub(crate) struct GraphQlMedia<T> {
	pub media: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GraphQlError {
	message: String,
	status: u16,
	locations: Vec<GraphQlErrorLocation>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GraphQlErrorLocation {
	line: u32,
	column: u32,
}
