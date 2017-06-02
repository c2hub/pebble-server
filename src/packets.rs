#![allow(dead_code)]
use serde_cbor;

use std::net::ToSocketAddrs;
use std::process::exit;

use SOCKET;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Packet
{
	pub ptype: PacketType,
	pub name: Option<String>,
	pub list: Option<Vec<String>>,
	pub data: Option<String>,
	pub raw_data: Option<Vec<u8>>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum PacketType
{
	Publish,
	Update,
	Find,
	Upload,
	Error,
	Register,
	Login,
	New,
}

impl Packet
{
	/*
	** Yay, builder pattern!
	*/
	pub fn new() -> Packet
	{
		Packet
		{
			ptype: PacketType::New,
			name: None,
			list: None,
			data: None,
			raw_data: None
		}
	}

	pub fn name(mut self, name: String) -> Packet
	{
		self.name = Some(name);
		self
	}

	pub fn ptype(mut self, ptype: PacketType) -> Packet
	{
		self.ptype = ptype;
		self
	}

	pub fn list(mut self, list: Vec<String>) -> Packet
	{
		self.list = Some(list);
		self
	}

	pub fn data(mut self, data: String) -> Packet
	{
		self.data = Some(data);
		self
	}

	pub fn raw_data(mut self, raw_data: Vec<u8>) -> Packet
	{
		self.raw_data = Some(raw_data);
		self
	}

	/*
	** Types
	*/
	pub fn error(msg: &str) -> Packet
	{
		Packet::new()
			.ptype(PacketType::Error)
			.name(msg.to_owned())
	}

	pub fn register(name: &str, hash: &str) -> Packet
	{
		Packet::new()
			.ptype(PacketType::Register)
			.name(name.to_owned())
			.data(hash.to_owned())
	}

	pub fn login(name: &str, hash: &str) -> Packet
	{
		Packet::new()
			.ptype(PacketType::Login)
			.name(name.to_owned())
			.data(hash.to_owned())
	}

	pub fn update(name: &str, version: &str) -> Packet
	{
		Packet::new()
			.ptype(PacketType::Update)
			.name(name.to_owned())
			.data(version.to_owned())
	}

	pub fn find(name: &str, version: &str) -> Packet
	{
		Packet::new()
			.ptype(PacketType::Find)
			.name(name.to_owned())
			.data(version.to_owned())
	}

	pub fn upload(name: &str, file: Vec<u8>, version: &str) -> Packet
	{
		Packet::new()
			.ptype(PacketType::Upload)
			.name(name.to_owned())
			.raw_data(file)
			.data(version.to_owned())
	}

	pub fn publish(name: &str, file: Vec<u8>, version: &str) -> Packet
	{
		let lib_name = "lib".to_string() + name;
		Packet::new()
			.ptype(PacketType::Publish)
			.name(lib_name)
			.raw_data(file)
			.data(version.to_owned())
	}

	/*
	** Reading
	*/
	pub fn read(source: &[u8]) -> Result<Packet, serde_cbor::error::Error>
	{
		serde_cbor::de::from_slice(source)
	}

	pub fn make(self) -> Result<Vec<u8>, serde_cbor::Error>
	{
		serde_cbor::ser::to_vec(&self)
	}

	/*
	** Sending
	*/
	pub fn send<A: ToSocketAddrs>(self, addr: A)
	{
		let bytes = match self.clone().make()
		{
			Ok(b) => b,
			Err(_) =>
			{
				println!("  error: failed to serialize packet. {:?}", self);
				exit(-1);
			}
		};

		if let Err(e) = SOCKET.send_to(&bytes, addr)
		{
			println!("{}", e);
		}
	}
}
