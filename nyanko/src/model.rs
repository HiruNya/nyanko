use qmetaobject::*;

use nyanko_anilist::SearchEntry as NyankoSearchEntry;
use nyanko_core::AniListAccount;

#[derive(Default, SimpleListItem)]
pub struct AccountEntry {
	pub avatar: QString,
	pub name: String,
	pub id: String,
	pub account_type: String,
}
impl From<&AniListAccount> for AccountEntry {
	fn from(al: &AniListAccount) -> Self {
		Self {
			name: al.name.clone(),
			id: al.id.clone(),
			avatar: al.avatar.clone().unwrap_or_default().into(),
			account_type: "AniList".to_string(),
		}
	}
}

#[derive(SimpleListItem)]
pub struct SearchEntry {
	pub anime_id: i64,
	pub description: String,
	pub image: QString,
	pub title: String,
}
impl Default for SearchEntry {
	fn default() -> Self {
		Self {
			anime_id: 0,
			description: Default::default(),
			image: Default::default(),
			title: "???".into(),
		}
	}
}
impl From<NyankoSearchEntry> for SearchEntry {
	fn from(entry: NyankoSearchEntry) -> Self {
		let title = entry.title.user_preferred.or(entry.title.english)
			.unwrap_or_else(|| String::from("???"));
		Self {
			anime_id: entry.id,
			description: entry.description.unwrap_or_default(),
			image: entry.cover_image.large.into(),
			title,
		}
	}
}
