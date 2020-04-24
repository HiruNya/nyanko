use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use toml::{from_slice, to_vec};

use std::{fs::File, io::{Read, Write}};

const SETTINGS_PATH: &str = "./settings.toml";

#[derive(Default, Deserialize, Serialize)]
pub struct Settings {
	pub current: Option<String>,
	#[serde(skip)]
	pub synced: bool,
}
impl Settings {
	pub fn load() -> Settings {
		let func = ||{
			let mut buffer = Vec::new();
			File::open(SETTINGS_PATH)
				.map_err(|_| warn!("Cannot find settings at '{}'... using defaults", SETTINGS_PATH)).ok()?
				.read_to_end(&mut buffer).ok()?;
			let mut settings: Settings = from_slice(buffer.as_slice())
				.map_err(|err| error!("Could not deserialize the settings: {:?}", err)).ok()?;
			settings.synced = true;
			info!("Read settings");
			Some(settings)
		};
		func().unwrap_or_default()
	}
	pub fn save(&self) {
		if self.synced { return }
		let func = || {
			let buf = to_vec(&self).map_err(|error| error!("Serialising settings: {:?}", error)).ok()?;
			let mut file = File::create(SETTINGS_PATH).map_err(|err| error!("Opening settings file, `{}`: {:?}", SETTINGS_PATH, err)).ok()?;
			file.write_all(&buf).map_err(|err| error!("Saving settings into file: {:?}", err)).ok()?;
			info!("Successfully saved settings to `{}`", SETTINGS_PATH);
			Some(())
		};
		func();
	}
	pub fn set_current(&mut self, current: String) {
		self.current = Some(current);
		self.synced = false;
	}
}
