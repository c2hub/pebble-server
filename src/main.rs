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

use publish::publish;
use update::update;
use find::find;
use upload::upload;
use error::error;
use register::register;
use login::login;
use new::new;

fn main()
{
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
