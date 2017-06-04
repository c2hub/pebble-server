use data::UserDB;
use packets::Packet;

use std::process::exit;
use std::net::ToSocketAddrs;

pub fn login<A: ToSocketAddrs>(packet: Packet, addr: A)
{
	let name = match packet.name
	{
		Some(n) => n,
		None =>
		{
			println!("  error: we don't login nameless people");
			Packet::error("missing username")
				.send(addr);
			return;
		}
	};

	let hash = match packet.data
	{
		Some(h) => h,
		None =>
		{
			println!("  error: we don't login people with no password");
			Packet::error("missing password")
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

	if user_db.users
		.unwrap()
		.iter()
		.find(|x| x.name == name && x.hash == hash)
		.is_some()
	{
		Packet::register("hello", "there")
			.send(addr);
	}
	else
	{
		Packet::error("couldn't login - invalid username or password")
			.send(addr);
	}
}
