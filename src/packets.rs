#![allow(dead_code)]
use serde_cbor;
use ansi_term::Colour::{Yellow, Green, Red};

use std::net::ToSocketAddrs;
use std::process::exit;
use std::fmt;

use SOCKET;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Packet
{
	Publish { uname: String, hash: String, file: Vec<u8>, name: String, version: String },
	Update { data: String },
	Find { name: String, version: String },
	Upload { uname: String, hash: String, file: Vec<u8>, name: String, version: String },
	Error { msg: String },
	Register { name: String, hash: String },
	Login { name: String, hash: String },
	New,
}

impl Packet
{
	pub fn new() -> Packet
	{
		Packet::New
	}
	/*
	** Types
	*/
	pub fn error(msg: &str) -> Packet
	{
		Packet::Error { msg: msg.to_owned() }
	}

	pub fn register(name: &str, hash: &str) -> Packet
	{
		Packet::Register
		{
			name: name.to_owned(),
			hash: hash.to_owned(),
		}
	}

	pub fn login(name: &str, hash: &str) -> Packet
	{
		Packet::Login
		{
			name: name.to_owned(),
			hash: hash.to_owned(),
		}
	}

	pub fn update(data: &str) -> Packet
	{
		Packet::Update
		{
			data: data.to_owned(),
		}
	}

	pub fn find(name: &str, version: &str) -> Packet
	{
		Packet::Find
		{
			name: name.to_owned(),
			version: version.to_owned(),
		}
	}

	pub fn upload(uname: &str, hash: &str, file: Vec<u8>, name: &str, version: &str) -> Packet
	{
		Packet::Upload
		{
			uname: uname.to_owned(),
			hash: hash.to_owned(),
			file: file,
			name: name.to_owned(),
			version: version.to_owned(),
		}
	}

	pub fn publish(uname: &str, hash: &str, file: Vec<u8>, name: &str, version: &str) -> Packet
	{
		let lib_name = "lib".to_string() + name;
		Packet::Publish
		{
			name: lib_name.to_owned(),
			hash: hash.to_owned(),
			uname: uname.to_owned(),
			file: file,
			version: version.to_owned()
		}
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

		match *self
		{
			Packet::New =>
				write!(f, "new"),
			Packet::Find { ref name, .. } =>
				write!(f, "find {}",
					Green.bold().paint(name.clone())
				),
			Packet::Error { ref msg } =>
				write!(f, "error {}",
					Red.bold().paint(msg.clone())
				),
			Packet::Upload { ref name, ref version, .. } =>
				write!(f,  "upload [{}] {}",
					Green.bold().paint(name.clone()),
					Red.bold().paint(version.clone())
				),
			Packet::Publish { ref name, ref version, .. } =>
				write!(f,  "publish [{}] {}",
					Green.bold().paint(name.clone()),
					Red.bold().paint(version.clone())
				),
			// there is something wrong in here, TODO
			Packet::Update { ref data } =>
				write!(f,  "update {}",
					Red.bold().paint(data.clone())
				),
			Packet::Register { ref name, ref hash } =>
				write!(f,  "register {} {}",
					Green.bold().paint(name.clone()),
					Red.bold().paint(hash.clone())
				),
			Packet::Login { ref name, ref hash } =>
				write!(f,  "login {} {}",
					Green.bold().paint(name.clone()),
					Red.bold().paint(hash.clone())
				),
		}
	}
}
