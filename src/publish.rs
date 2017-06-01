use packets::Packet;
use std::net::ToSocketAddrs;

pub fn publish<A: ToSocketAddrs>(packet: Packet, addr: A)
{
	unimplemented!();
}
