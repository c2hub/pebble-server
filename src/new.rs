use packets::Packet;
use std::net::ToSocketAddrs;

pub fn new<A: ToSocketAddrs>(packet: Packet, addr: A) {
	unimplemented!();
}
