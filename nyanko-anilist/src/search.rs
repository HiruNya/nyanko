use reqwest::Result as ReqwestResult;
use serde::Deserialize;

use super::{GraphQlResponsePayload, GraphQlPage, GraphQlMedia};

const QUERY: &str = r#"
query($search: String, $page: Int) {
	Page(page: $page, perPage: 20) {
		media(search: $search) {
			id,
			description,
			title {
				english
				userPreferred
			},
			coverImage {
				large
			}
		}
	}
}
"#;

pub async fn search(client: &reqwest::Client, query: &str) -> ReqwestResult<Vec<SearchEntry>> {
	Ok(
		client.post(super::URL)
			.json(&super::GraphQlQueryPayload {
				query: QUERY,
				variables: serde_json::json!({
					"search": query,
				}),
			})
			.send()
			.await?
			.json::<GraphQlResponsePayload<GraphQlPage<GraphQlMedia<SearchEntry>>>>()
			.await?
			.unwrap()
			.page
			.media
	)
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchEntry {
	pub id: i64,
	pub description: Option<String>,
	pub title: SearchTitle,
	pub cover_image: CoverImage,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTitle {
	pub english: Option<String>,
	pub user_preferred: Option<String>,
}

#[derive(Deserialize)]
pub struct CoverImage {
	pub large: String,
}
