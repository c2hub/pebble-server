use fehler::Fehler;
use packets::Packet;
use std::net::ToSocketAddrs;

use data::*;

use version_compare::Version;

pub fn find<A: ToSocketAddrs>(packet: Packet, addr: A) {
	if let Packet::Find { name, version } = packet {
		let is_any = version == "*";

		let version = match Version::from(&version) {
			Some(s) => s,
			None => {
				if !is_any {
					Packet::error("invalid version string").send(addr);
					return;
				} else {
					Version::from("1.0.0").unwrap()
				}
			}
		};

		let index = Index::read().fehler("failed to parse index", &addr);

		let index = match index.entries {
			Some(i) => i,
			None => {
				Packet::find(&name, "none").send(addr);
				return;
			}
		};

		if let Some(entry) = index.clone().iter().find(|ref ent| {
			ent.name == name
				&& ent.versions
					.iter()
					.find(|ref ver| version == Version::from(ver).unwrap() || is_any)
					.is_some()
		}) {
			Packet::find(&name, &entry.versions[0]).send(addr);
			return;
		} else {
			Packet::find(&name, "none").send(addr);
		}
	}
}
