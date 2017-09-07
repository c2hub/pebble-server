use std::net::ToSocketAddrs;
use std::fmt;

use packets::Packet;

pub trait Fehler<T>
{
	fn fehler<A: ToSocketAddrs>(self, msg: &str, addr: &A) -> T;
}

impl<T, E: fmt::Debug> Fehler<T> for Result<T, E>
{
	fn fehler<A: ToSocketAddrs>(self, msg: &str, addr: &A) -> T
	{
		match self
		{
			Ok(res) => res,
			Err(_) =>
			{
				Packet::error(msg).send(addr);
				extern { fn __rust_start_panic() -> !; }
				unsafe { __rust_start_panic(); }
			}
		}
	}
}

impl<T> Fehler<T> for Option<T>
{
	fn fehler<A: ToSocketAddrs>(self, msg: &str, addr: &A) -> T
	{
		match self
		{
			Some(res) => res,
			None =>
			{
				println!("  error: {}", msg);
				Packet::error(msg).send(addr.clone());
				extern { fn __rust_start_panic() -> !; }
				unsafe { __rust_start_panic(); }
			}
		}
	}
}
