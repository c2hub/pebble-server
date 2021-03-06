use data::*;
use packets::Packet;

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::net::ToSocketAddrs;
use std::process::exit;

use toml;
use version_compare::Version;

pub fn publish<A: ToSocketAddrs>(packet: Packet, addr: A) {
	if let Packet::Publish {
		uname,
		hash,
		file,
		name,
		version,
	} = packet
	{
		let user_db = match UserDB::read() {
			Ok(d) => d,
			Err(_) => {
				println!("  error: failed to read user db");
				Packet::error("database error").send(addr);
				exit(-1);
			}
		};

		// println!("{} {} {} {}", &uname, &hash, &name, &version);

		if user_db
			.users
			.unwrap()
			.iter()
			.find(|x| x.name == uname && x.hash == hash)
			.is_none()
		{
			println!("  error: incorrect username or password");
			Packet::error("incorrect username or password").send(addr);
			return;
		}

		let index = match Index::read() {
			Ok(i) => i,
			Err(e) => {
				println!("  error: failed to parse index, {}", e);
				Packet::error("failed to parse index").send(addr);
				exit(-1);
			}
		};

		let mut index = match index.entries {
			Some(i) => i,
			None => {
				println!("  error: can't load index");
				Packet::error("can't load index").send(addr);
				return;
			}
		};

		let found = if let Some(ref mut entry) = index.iter_mut().find(|ref ent| ent.name == name) {
			if entry.author == uname {
				let old_ver_str = entry.versions[0].clone();
				let old_ver = match Version::from(&old_ver_str) {
					Some(o) => o,
					None => {
						println!("  error: failed to recognize old version");
						Packet::error("failed to recognize old version").send(addr);
						return;
					}
				};

				let new_ver = match Version::from(&version) {
					Some(n) => n,
					None => {
						println!("  error: invalid new version");
						Packet::error("invalid new version").send(addr);
						return;
					}
				};

				if old_ver > new_ver {
					println!("  error: old version is newer than new version, rejecting...");
					Packet::error("old version is newer than new version").send(addr);
					return;
				} else if old_ver == new_ver {
					println!("  error: old version is the same as new version, rejecting...");
					Packet::error("old version is the same as new version").send(addr);
					return;
				}

				entry.versions.insert(0, version.clone());

				if create_dir_all("data/".to_string() + name.as_ref() + "/" + version.as_ref())
					.is_err()
				{
					println!("  error: failed to create directory to store pebble");
					Packet::error("couldn't store pebble, failed to create directory").send(addr);
					return;
				}

				match File::create(
					"data/".to_string() + name.as_ref() + "/" + version.as_ref() + "libpackage.zip",
				) {
					Ok(mut f) => if f.write_all(&file).is_err() {
						println!("  error: failed to write bytes to zip");
						Packet::error("failed to write bytes to zip").send(addr);
						return;
					},
					Err(_) => {
						println!("  error: failed to create zip");
						Packet::error("couldn't store pebble, failed to save zip").send(addr);
						return;
					}
				}
			} else {
				println!("  error: pebble doesn't belong to user!");
				Packet::error("the pebble doesn't belong to you!").send(addr);
				return;
			}
			true
		} else {
			false
		};

		if !found {
			if create_dir_all("data/".to_string() + name.as_ref() + "/" + version.as_ref()).is_err()
			{
				println!("  error: failed to create directory to store pebble");
				Packet::error("couldn't store pebble, failed to create directory").send(addr);
				return;
			}

			match File::create(
				"data/".to_string() + name.as_ref() + "/" + version.as_ref() + "/"
					+ "libpackage.zip",
			) {
				Ok(mut f) => if f.write_all(&file).is_err() {
					println!("  error: failed to write bytes to zip");
					Packet::error("failed to write bytes to zip").send(addr);
					return;
				},
				Err(_) => {
					println!("  error: failed to create zip");
					Packet::error("couldn't store pebble, failed to save zip").send(addr);
					return;
				}
			}

			index.push(Entry {
				name: name,
				versions: vec![version],
				author: uname,
				repository: None, //TODO
			});
		}

		let mut index_f = match File::create("data/index") {
			Ok(f) => f,
			Err(_) => {
				println!("  error: failed to open index file");
				Packet::error("failed to open index file").send(addr);
				return;
			}
		};

		if write!(
			index_f,
			"{}",
			toml::to_string(&Index {
				entries: Some(index)
			}).unwrap()
		).is_err()
		{
			println!("  error: failed to write to index");
			Packet::error("failed to write to index").send(addr);
			return;
		}

		Packet::publish("hello", "there", Vec::new(), "hello", "there") // Obi-Wan
			.send(addr);
	}
}
