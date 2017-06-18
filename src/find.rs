use packets::Packet;
use std::process::exit;
use std::net::ToSocketAddrs;

use data::*;

use version_compare::Version;

pub fn find<A: ToSocketAddrs>(packet: Packet, addr: A)
{
	let name = match packet.name
	{
		Some(s) => s,
		None =>
		{
			Packet::error("find socket requires a name and version")
				.send(addr);
			return;
		}
	};

	let version = match packet.data
	{
		Some(s) => s,
		None =>
		{
			Packet::error("find socket requires a name and version")
				.send(addr);
			return;
		}
	};

	let version = match Version::from(&version)
	{
		Some(s) => s,
		None =>
		{
			Packet::error("invalid version string")
				.send(addr);
			return;
		}
	};

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

	let index = match index.entries
	{
		Some(i) => i,
		None =>
		{
			Packet::find(&name, "none")
				.send(addr);
			return;
		}
	};

	if let Some(entry) = index.clone().iter()
		.find(
			|ref ent| ent.name == name
				&& ent.versions
					.iter()
					.find(
						|ref ver| version == Version::from(ver).unwrap()
					).is_some()
			)
	{
		Packet::find(&name, &entry.versions[0])
			.send(addr);
		return;
	}
	else
	{
		Packet::find(&name, "none")
			.send(addr);
	}
}
