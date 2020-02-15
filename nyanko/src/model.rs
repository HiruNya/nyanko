use qmetaobject::*;

use nyanko_anilist::SearchEntry as NyankoSearchEntry;

#[derive(SimpleListItem)]
pub struct SearchEntry {
	pub id: i64,
	pub image: QString,
	pub title: String,
}
impl Default for SearchEntry {
	fn default() -> Self {
		Self {
			id: 0,
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
			id: entry.id,
			image: entry.cover_image.large.into(),
			title,
		}
	}
}
