#[derive(Clone, Serialize, Deserialize)]
pub struct Packet
{
	pub ptype: PacketType,
	pub name: Option<String>,
	pub list: Option<Vec<String>>,
	pub data: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PacketType
{
	Publish,
	Update,
	Find,
	Receive
}
