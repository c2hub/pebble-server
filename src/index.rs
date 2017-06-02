#[derive(Clone, Serialize, Deserialize)]
pub struct Index
{
	pub entries: Option<Vec<Entry>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Entry
{
	pub name: String,
	pub versions: Vec<String>,
	pub author: String,
	pub repository: Option<String>,
}