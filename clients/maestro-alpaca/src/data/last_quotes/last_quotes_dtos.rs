use {
	crate::data::enums::feed::Feed,
	chrono::{DateTime, Utc},
	serde::{Deserialize, Serialize},
	std::collections::HashMap,
};

#[derive(bon::Builder, Clone, Debug, Deserialize, Serialize)]
pub struct LatestQuotesRequestDTO {
	#[serde(serialize_with = "serialize_vec_to_csv")]
	symbols: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	feed: Option<Feed>,
	#[serde(skip_serializing_if = "Option::is_none")]
	currency: Option<String>,
}

pub fn serialize_vec_to_csv<S>(vec: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	vec.join(",").serialize(serializer)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuoteDTO {
	#[serde(rename = "t")]
	pub time: DateTime<Utc>,
	#[serde(rename = "ax")]
	pub ask_exchange: String,
	#[serde(rename = "ap")]
	pub ask_price: f32,
	#[serde(rename = "as")]
	pub ask_size: u32,
	#[serde(rename = "bx")]
	pub bid_exchange: String,
	#[serde(rename = "bp")]
	pub bid_price: f32,
	#[serde(rename = "bs")]
	pub bid_size: u32,
	#[serde(rename = "c")]
	pub conditions: Vec<String>,
	#[serde(rename = "z")]
	pub tape: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LatestQuotesResponseDTO {
	quotes: HashMap<String, QuoteDTO>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuoteResponseDTO {
	pub symbol: String,
	#[serde(flatten)]
	pub data: QuoteDTO,
}

impl From<LatestQuotesResponseDTO> for Vec<QuoteResponseDTO> {
	fn from(response: LatestQuotesResponseDTO) -> Self {
		response.quotes.into_iter().map(|(symbol, data)| QuoteResponseDTO { symbol, data }).collect()
	}
}
