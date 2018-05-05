use std::env::home_dir;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use toml;

#[derive(Clone, Serialize, Deserialize)]
pub struct Index {
	pub entries: Option<Vec<Entry>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Entry {
	pub name: String,
	pub versions: Vec<String>,
	pub author: String,
	pub repository: Option<String>,
}

#[allow(dead_code)]
impl Index {
	pub fn read() -> Result<Index, toml::de::Error> {
		let mut me = match File::open("data/index") {
			Ok(f) => f,
			Err(_) => {
				println!("  error: failed to open index");
				exit(-1);
			}
		};
		let mut contents = String::new();
		if me.read_to_string(&mut contents).is_err() {
			println!("  error: failed to read index");
			exit(-1);
		}
		toml::from_str(contents.as_ref())
	}

	pub fn write(&self) -> Result<String, toml::ser::Error> {
		toml::to_string(&self)
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserDB {
	pub users: Option<Vec<User>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
	pub name: String,
	pub hash: String,
}

#[allow(dead_code)]
impl UserDB {
	pub fn read() -> Result<UserDB, toml::de::Error> {
		let path = match home_dir() {
			Some(mut d) => {
				d.push("pebble_users");
				d
			}
			None => {
				println!("  error: failed to open user db");
				exit(-1);
			}
		};

		let mut me = match File::open(path) {
			Ok(f) => f,
			Err(_) => {
				println!("  error: failed to open user db");
				exit(-1);
			}
		};
		let mut contents = String::new();
		if me.read_to_string(&mut contents).is_err() {
			println!("  error: failed to read user");
			exit(-1);
		}
		toml::from_str(contents.as_ref())
	}

	pub fn write(&self) -> Result<String, toml::ser::Error> {
		toml::to_string(&self)
	}
}
