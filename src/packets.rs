#![allow(dead_code)]
use serde_cbor;
use ansi_term::Colour::{Yellow, Green, Red};

use std::net::ToSocketAddrs;
use std::process::exit;
use std::fmt;

use SOCKET;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Packet
{
	pub ptype: PacketType,
	pub name: Option<String>,
	pub extra: Option<String>,
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
			extra: None,
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

	pub fn extra(mut self, extra: String) -> Packet
	{
		self.extra = Some(extra);
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

#[allow(unused_must_use)]
impl fmt::Display for Packet
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}",
			Yellow.bold().paint("packet ")
		);

		match self.ptype
		{
			PacketType::New =>
				write!(f, "new"),
			PacketType::Find =>
				write!(f, "find {}",
					Green.bold().paint(match self.name
					{
						Some(ref n) => n.clone(),
						None => "none".to_string(),
					})
				),
			PacketType::Error =>
				write!(f, "error {}",
					Red.bold().paint(match self.name
					{
						Some(ref n) => n.clone(),
						None => "none".to_string(),
					})
				),
			PacketType::Upload =>
				write!(f,  "upload [{}] {}",
					Green.bold().paint(match self.name
					{
						Some(ref n) => n.clone(),
						None => "none".to_string(),
					}),
					Red.bold().paint(match self.data
					{
						Some(ref n) => n.clone(),
						None => "none".to_string(),
					})
				),
			PacketType::Publish =>
				write!(f,  "publish [{}] {}",
					Green.bold().paint(match self.name
					{
						Some(ref n) => n.clone(),
						None => "none".to_string(),
					}),
					Red.bold().paint(match self.data
					{
						Some(ref n) => n.clone(),
						None => "none".to_string(),
					})
				),
			PacketType::Update =>
				write!(f,  "update [{}] {}",
					Green.bold().paint(match self.name
					{
						Some(ref n) => n.clone(),
						None => "none".to_string(),
					}),
					Red.bold().paint(match self.data
					{
						Some(ref n) => n.clone(),
						None => "none".to_string(),
					})
				),
			PacketType::Register =>
				write!(f,  "register {} {}",
					Green.bold().paint(match self.name
					{
						Some(ref n) => n.clone(),
						None => "none".to_string(),
					}),
					Red.bold().paint(match self.data
					{
						Some(ref n) => n.clone(),
						None => "none".to_string(),
					})
				),
			PacketType::Login =>
				write!(f,  "login {} {}",
					Green.bold().paint(match self.name
					{
						Some(ref n) => n.clone(),
						None => "none".to_string(),
					}),
					Red.bold().paint(match self.data
					{
						Some(ref n) => n.clone(),
						None => "none".to_string(),
					})
				),
		}
	}
}
