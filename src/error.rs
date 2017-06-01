use packets::Packet;
use std::net::ToSocketAddrs;

pub fn error<A: ToSocketAddrs>(packet: Packet, addr: A)
{
	unimplemented!();
}
