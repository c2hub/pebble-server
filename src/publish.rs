use data::*;
use packets::Packet;

use std::io::Write;
use std::process::exit;
use std::net::ToSocketAddrs;
use std::fs::{File, create_dir_all};

use version_compare::Version;
use toml;

pub fn publish<A: ToSocketAddrs>(packet: Packet, addr: A)
{
	let name = match packet.name
	{
		Some(u) => u,
		None =>
		{
			println!("  error: an author needs to have a name");
			Packet::error("missing username")
				.send(addr);
			return;
		}
	};

	let hash = match packet.extra
	{
		Some(h) => h,
		None =>
		{
			println!("  error: an author needs a password, huh");
			Packet::error("missing password, are you logged in?")
				.send(addr);
			return;
		}
	};

	let bytes = match packet.raw_data
	{
		Some(b) => b,
		None =>
		{
			println!("  error: can't upload when there is nothing to upload");
			Packet::error("can't upload nothing")
				.send(addr);
			return;
		}
	};

	let version = match packet.data
	{
		Some(v) => v,
		None =>
		{
			println!("  error: version-less pebbles are not allowed");
			Packet::error("version-less pebbles are not allowed")
				.send(addr);
			return;
		}
	};

	let uname = match packet.data2
	{
		Some(n) => n,
		None =>
		{
			println!("  error: nameless pebbles are not allowed");
			Packet::error("nameless pabbles are not allowed")
				.send(addr);
			return;
		}
	};

	let user_db = match UserDB::read()
	{
		Ok(d) => d,
		Err(_) =>
		{
			println!("  error: failed to read user db");
			Packet::error("database error")
				.send(addr);
			exit(-1);
		}
	};

	println!("{} {} {} {}", &uname, &hash, &name, &version);

	if user_db.users
		.unwrap()
		.iter()
		.find(|x| x.name == uname && x.hash == hash)
		.is_none()
	{
		println!("  error: incorrect username or password");
		Packet::error("incorrect username or password")
			.send(addr);
		return;
	}

	let index = match Index::read()
	{
		Ok(i) => i,
		Err(e) =>
		{
			println!("  error: failed to parse index, {}", e);
			Packet::error("failed to parse index")
				.send(addr);
			exit(-1);
		}
	};

	let mut index = match index.entries
	{
		Some(i) => i,
		None =>
		{
			println!("  error: can't load index");
			Packet::error("can't load index")
				.send(addr);
			return;
		}
	};

	let found = if let Some(ref mut entry) = index.iter_mut().find(|ref ent| ent.name == name)
	{
		if entry.author == uname
		{
			let old_ver_str = entry.versions[0].clone();
			let old_ver = match Version::from(&old_ver_str)
			{
				Some(o) => o,
				None =>
				{
					println!("  error: failed to recognize old version");
					Packet::error("failed to recognize old version")
						.send(addr);
					return;
				}
			};

			let new_ver = match Version::from(&version)
			{
				Some(n) => n,
				None =>
				{
					println!("  error: invalid new version");
					Packet::error("invalid new version")
						.send(addr);
					return;
				}
			};

			if old_ver > new_ver
			{
				println!("  error: old version is newer than new version, rejecting...");
				Packet::error("old version is newer than new version")
					.send(addr);
				return;
			}
			else if old_ver == new_ver
			{
				println!("  error: old version is the same as new version, rejecting...");
				Packet::error("old version is the same as new version")
					.send(addr);
				return;
			}

			entry.versions.insert(0, version.clone());

			if create_dir_all(
				  "data/".to_string()
				+ name.as_ref()
				+ "/"
				+ version.as_ref()).is_err()
			{
				println!("  error: failed to create directory to store pebble");
				Packet::error("couldn't store pebble, failed to create directory")
					.send(addr);
				return;
			}

			match File::create(
				  "data/".to_string()
				+ name.as_ref()
				+ "/"
				+ version.as_ref()
				+ "libpackage.zip")
			{
				Ok(mut f) => if f.write_all(&bytes).is_err()
				{
					println!("  error: failed to write bytes to zip");
					Packet::error("failed to write bytes to zip")
						.send(addr);
					return;
				},
				Err(_) =>
				{
					println!("  error: failed to create zip");
					Packet::error("couldn't store pebble, failed to save zip")
						.send(addr);
					return;
				}
			}
		}
		else
		{
			println!("  error: pebble doesn't belong to user!");
			Packet::error("the pebble doesn't belong to you!")
				.send(addr);
			return;
		}
		true
	}
	else
		{false};

	if !found
	{
		if create_dir_all(
			  "data/".to_string()
			+ name.as_ref()
			+ "/"
			+ version.as_ref()).is_err()
		{
			println!("  error: failed to create directory to store pebble");
			Packet::error("couldn't store pebble, failed to create directory")
				.send(addr);
			return;
		}

		match File::create(
			  "data/".to_string()
			+ name.as_ref()
			+ "/"
			+ version.as_ref()
			+ "/"
			+ "libpackage.zip")
		{
			Ok(mut f) => if f.write_all(&bytes).is_err()
			{
				println!("  error: failed to write bytes to zip");
				Packet::error("failed to write bytes to zip")
					.send(addr);
				return;
			},
			Err(_) =>
			{
				println!("  error: failed to create zip");
				Packet::error("couldn't store pebble, failed to save zip")
					.send(addr);
				return;
			}
		}

		index.push(Entry
		{
			name: uname,
			versions: vec![version],
			author: name,
			repository: None, //TODO
		});
	}

	let mut index_f = match File::create("data/index")
	{
		Ok(f) => f,
		Err(_) =>
		{
			println!("  error: failed to open index file");
			Packet::error("failed to open index file")
				.send(addr);
			return;
		}
	};

	if write!(index_f, "{}",
			toml::to_string(&Index {entries: Some(index)}).unwrap()
		).is_err()
	{
		println!("  error: failed to write to index");
		Packet::error("failed to write to index")
			.send(addr);
		return;
	}

	Packet::publish("hello", "there", Vec::new(), "hello", "there") // Obi-Wan
		.send(addr);
}
