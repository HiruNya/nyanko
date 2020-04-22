use jsonwebtoken::{dangerous_unsafe_decode, errors::Result };
use serde::{Deserialize, Serialize};

pub fn auth_link(client_id: &str) -> String {
	format!("https://anilist.co/api/v2/oauth/authorize?client_id={}&response_type=token", client_id)
}

#[derive(Deserialize, Serialize)]
pub struct Token {
	pub token: String,
	pub exp: u64,
}
impl Token {
	pub fn new(token: String) -> Result<Self> {
		let claim: Claim = dangerous_unsafe_decode(&token)?.claims;
		Ok(Token { token, exp: claim.exp })
	}
}

#[derive(Deserialize)]
pub struct Claim {
	exp: u64
}
