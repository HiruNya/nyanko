
pub fn auth_link(client_id: &str) -> String {
	format!("https://anilist.co/api/v2/oauth/authorize?client_id={}&response_type=token", client_id)
}
