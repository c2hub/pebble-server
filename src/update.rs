use packets::Packet;

use std::fs::File;
use std::io::Read;
use std::net::ToSocketAddrs;

pub fn update<A: ToSocketAddrs>(packet: Packet, addr: A) {
	Packet::update(
		if let Ok(mut f) = File::open("data/index") {
			let mut s = String::new();
			f.read_to_string(&mut s).expect("failed to read index");
			s
		} else {
			Packet::error("failed to open index").send(addr);
			return;
		}.as_ref(),
	).send(addr)
}
