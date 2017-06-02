use std::fs::File;
use std::io::Read;
use packets::Packet;
use std::process::exit;
use std::net::UdpSocket;
use std::net::ToSocketAddrs;

use toml;

use index::*;

pub fn find<A: ToSocketAddrs>(packet: Packet, addr: A)
{
	let sock = match UdpSocket::bind("0.0.0.0:9002")
	{
		Ok(s) => s,
		Err(_) =>
		{
			println!("  error: failed to bind to socket");
			exit(-1);
		}
	};
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
	let mut index = String::new();
	let mut f = match File::open("data/index")
	{
		Ok(f) => f,
		Err(_) =>
		{
			if let Err(_) = File::create("data/index")
			{
				println!("  error: failed to create index file");
				exit(-1);
			}
			else
			{
				Packet::find(&name, "none")
					.send(addr);
				return;
			}
		}
	};
	if let Err(_) = f.read_to_string(&mut index)
	{
		println!("  error: failed to read index");
		exit(-1);
	};
	let index: Index = match toml::from_str(&index)
	{
		Ok(i) =>
		{
			match i
			{
				Some(ind) => ind,
				None =>
				{
					Packet::find(&name, "none")
						.send(addr);
					return;
				}
			}
		},
		Err(_) =>
		{
			println!("  error: failed to parse index");
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

	if let Some(entry) = index.iter()
		.find(|ref ent| ent.name == name)
	{
		Packet::find(&name, &entry.versions[0])
			.send(addr);
		return;
	}
}
