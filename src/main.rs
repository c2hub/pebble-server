#![allow(unused_variables)]
#[macro_use]
extern crate serde_derive;
extern crate serde_cbor;
extern crate ansi_term;
extern crate serde;
extern crate toml;

use std::thread;
use std::net::UdpSocket;
use std::process::exit;
use std::fs::create_dir;
use std::path::Path;

mod packets;
mod index;

mod publish;
mod update;
mod find;
mod upload;
mod error;
mod register;
mod login;
mod new;

use packets::{Packet, PacketType};
use index::{Index, Entry};

use publish::publish;
use update::update;
use find::find;
use upload::upload;
use error::error;
use register::register;
use login::login;
use new::new;

use std::io::Write;
use std::fs::File;

fn main()
{

	if !Path::new("data").exists()
	{
		if let Err(_) = create_dir("data")
		{
			println!("  error: failed to create data folder");
			exit(-1);
		}
		let mut index = match File::create("data/index")
		{
			Ok(f) => f,
			Err(_) =>
			{
				println!("  error: failed to create index file");
				exit(-1);
			}
		};

		let _ = write!(index, "{}",
			toml::to_string(&Index
				{
					entries: Some
					(
						vec!
						[
							Entry
							{
								name: "dummy".to_string(),
								versions: vec!["1.0.0".to_string()],
								author: "lukas".to_string(),
								repository: None,
							},
							Entry
							{
								name: "dummy2".to_string(),
								versions: vec!["1.0.0".to_string()],
								author: "lukas".to_string(),
								repository: None,
							}
						]
					)
				}
			).unwrap()
		);
	}

	let sock = match UdpSocket::bind("0.0.0.0:9001")
	{
		Ok(s) => s,
		Err(_) =>
		{
			println!(" error: failed to bind to socket");
			exit(-1);
		}
	};

	loop
	{
		let mut res = [0; 2 * 1024 * 1024];
		let (amt, src) = match sock.recv_from(&mut res)
		{
			Ok((a,s)) => (a,s),
			Err(_) =>
			{
				println!("  error: failed to receive packet");
				exit(-1);
			}
		};

		let res = &mut res[..amt];
		let packet: Packet = match serde_cbor::de::from_slice(res)
		{
			Ok(p) => p,
			Err(_) =>
			{
				println!("  error: failed to deserialize packet");
				exit(-1);
			}
		};

		thread::spawn( move ||
			{
				match packet.ptype
				{
					PacketType::Publish => publish(packet, src),
					PacketType::Update => update(packet, src),
					PacketType::Find => find(packet, src),
					PacketType::Upload => upload(packet, src),
					PacketType::Error => error(packet, src),
					PacketType::Register => register(packet, src),
					PacketType::Login => login(packet, src),
					PacketType::New => new(packet, src),
				}
			}
		);
	}
}
