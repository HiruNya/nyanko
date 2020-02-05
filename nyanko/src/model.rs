use qmetaobject::*;

use nyanko_anilist::SearchEntry as NyankoSearchEntry;

#[derive(Clone, SimpleListItem)]
pub struct SearchEntry {
	pub image: QVariant, // Todo: Change this to QUrl when possible
	pub title: String,
}
impl Default for SearchEntry {
	fn default() -> Self {
		Self {
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
			image: QString::from(entry.cover_image.large).into(),
			title,
		}
	}
}
