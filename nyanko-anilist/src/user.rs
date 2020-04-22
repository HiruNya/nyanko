use reqwest::Result as ReqwestResult;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{GraphQlResponsePayload, URL};

const QUERY_VIEWER: &str = r#"
query {
    Viewer {
        name,
        avatar {
            large,
            medium,
        }
    }
}
"#;

pub async fn viewer(client: &reqwest::Client, token: &str) -> ReqwestResult<Viewer> {
	let viewer = client.post(URL)
		.header("Authorization", format!("Bearer {}", token))
		.json(&super::GraphQlQueryPayload {
			query: QUERY_VIEWER,
			variables: json!({}),
		})
		.send()
		.await?
		.json::<GraphQlResponsePayload<ViewerPayload>>()
		.await?
		.unwrap()
		.unwrap();
	Ok(viewer)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Viewer {
	pub name: String,
	pub avatar: Avatar,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Avatar {
	pub large: Option<String>,
	pub medium: Option<String>,
}

#[derive(Deserialize)]
pub struct ViewerPayload {
	#[serde(rename = "Viewer")]
	viewer: Viewer,
}
impl ViewerPayload {
	fn unwrap(self) -> Viewer {
		self.viewer
	}
}
