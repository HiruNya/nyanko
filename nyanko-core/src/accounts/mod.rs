use log::{error, info};
use nosqlite::{Connection, KeyTable};
use uuid::Uuid;

use nyanko_anilist::{Token, Viewer};

use std::{fs::{create_dir, read_dir}, path::Path, sync::Mutex};

#[derive(Default)]
pub struct Accounts {
	accounts: Vec<AniListAccount>,
	current: usize,
}
impl Accounts {
	pub fn load() -> Self {
		read_dir("./accounts").map(|dir| {
			dir.filter_map(|entry| Some(entry.ok()?.path()))
				.filter_map(|path| Some((Connection::open(&path).ok()?, path)))
				.filter_map(|(connection, path)| {
					let table: KeyTable<String> = connection.key_table("account").ok()?;
					let users = table.as_ref().get("user".into());
					let user = users.data::<Viewer, _>(&connection).unwrap().unwrap();
					let avatar = user.avatar.large.or(user.avatar.medium);
					let name = user.name;
					let account = AniListAccount {
						avatar,
						name,
						id: path.file_name()?.to_str()?.trim_end_matches(".db").into(),
						token: table.as_ref().get("token".into()).data(&connection).ok()??,
						table: Mutex::new(connection),
					};
					Some(account)
				})
				.inspect(|account| info!("Found account: {} ~ {}", account.name, account.id))
				.collect::<Vec<_>>()
		}).map(|accounts| Accounts { accounts, current: 0 }).unwrap_or_default()
	}
	pub fn create(&mut self, token: Token) -> Option<&mut AniListAccount> {
		AniListAccount::create(token)
			.map(|account| self.accounts.push(account))
			.and_then(move |()| self.accounts.last_mut())
	}
	pub fn current(&self) -> Option<&AniListAccount> { self.accounts.get(self.current) }
	pub fn current_mut(&mut self) -> Option<&mut AniListAccount> { self.accounts.get_mut(self.current) }
}
impl AsRef<Vec<AniListAccount>> for Accounts {
	fn as_ref(&self) -> &Vec<AniListAccount> { &self.accounts }
}
impl AsMut<Vec<AniListAccount>> for Accounts {
	fn as_mut(&mut self) -> &mut Vec<AniListAccount> { &mut self.accounts }
}

pub struct AniListAccount {
	pub avatar: Option<String>,
	pub name: String,
	pub id: String,
	pub table: Mutex<Connection>,
	pub token: Token,
}
impl AniListAccount {
	pub fn create(token: Token) -> Option<Self> {
		let mut connection = None;
		let mut id = String::default();
		if !Path::new("./accounts").exists() {
			if let Err(e) = create_dir("./accounts") {
				error!("Error creating `accounts` dir: {}", e);
				return None
			}
		}
		for _ in 0..4 {
			id = Uuid::new_v4().to_simple().to_string();
			match Connection::open(format!("./accounts/{}.db", id)) {
				Ok(c) => {
					connection = Some(c);
					break;
				}
				Err(e) => error!("{:?}", e),
			}
		}
		connection
			.and_then(|connection| {
				let table: KeyTable<String> = connection.key_table("account").ok()?;
				table.insert("user".into(), Viewer{ name: id.clone(), avatar: Default::default() }, &connection).ok()?;
				table.insert("token".into(), &token, &connection).ok()?;
				Some(connection)
			})
			.map(|connection| {
				AniListAccount {
					avatar: None,
					name: id.clone(),
					id,
					table: Mutex::new(connection),
					token,
				}
			})
	}
	pub fn update(&mut self, user: Viewer) -> Option<()> {
		let connection = self.table.get_mut().ok()?;
		connection.key_table::<String, _>("account").ok()?.as_ref()
			.iter().patch(&user, &connection).ok()?;
		self.name = user.name;
		self.avatar = user.avatar.large.or(user.avatar.medium);
		Some(())
	}
}
