use packets::Packet;
use std::net::ToSocketAddrs;
use std::env::home_dir;
use std::process::exit;
use std::fs::File;
use std::io::Write;

use toml;

use data::*;

pub fn register<A: ToSocketAddrs>(packet: Packet, addr: A)
{
	let mut user_db = match UserDB::read()
	{
		Ok(u) => u,
		Err(_) =>
		{
			println!("  error: failed to parse user database");
			Packet::error("failed to parse user db")
				.send(addr);
			return;
		}
	};

	if user_db.users.clone().is_none()
	{
		let uv = Vec::new();
		user_db.users = Some(uv);
	};

	let name = match packet.name
	{
		Some(n) => n,
		None =>
		{
			println!("  error: we don't register nameless people");
			Packet::error("you need a name to be registered")
				.send(addr);
			return;
		}
	};

	let hash = match packet.data
	{
		Some(h) => h,
		None =>
		{
			println!("  error: we don't register passwordless people either");
			Packet::error("you need a password to register")
				.send(addr);
			return;
		}
	};

	let usr = User
	{
		name: name.clone(),
		hash: hash,
	};

	match user_db.users
	{
		Some(ref mut u) =>
		{
			if u.clone().iter().find(|x| x.name == name).is_none()
				{u.push(usr);}
			else
			{
				Packet::error("user already exists")
					.send(addr);
				return;
			}
		},
		None => unreachable!()
	}

	let path = match home_dir()
	{
		Some(mut d) => {d.push("pebble_users"); d},
		None =>
		{
			println!("  error: failed to get user db path");
			exit(-1);
		}
	};

	let mut db_file = match File::open(&path)
	{
		Ok(f) => f,
		Err(_) =>
		{
			println!("  error: failed to create user db");
			exit(-1);
		}
	};

	let _ = write!(db_file, "{}",
		toml::to_string(&user_db).unwrap()
	);

	Packet::register("hello", "there") // Kenobi
		.send(addr);
}
