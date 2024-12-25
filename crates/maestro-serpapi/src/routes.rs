#[derive(Debug)]
pub enum SerpApiRoute {
	Search,
	Locations,
	Account,
}

impl SerpApiRoute {
	pub fn url_path(&self) -> String {
		match self {
			SerpApiRoute::Search => "https://serpapi.com/search".to_string(),
			SerpApiRoute::Locations => "https://serpapi.com/locations.json".to_string(),
			SerpApiRoute::Account => "https://serpapi.com/account".to_string(),
		}
	}
}
