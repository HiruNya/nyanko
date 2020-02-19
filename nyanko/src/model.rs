use qmetaobject::*;

use nyanko_anilist::SearchEntry as NyankoSearchEntry;

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
