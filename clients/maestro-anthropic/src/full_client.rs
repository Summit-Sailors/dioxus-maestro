use {
	futures::StreamExt,
	reqwest::{
		header::{HeaderMap, HeaderValue, CONTENT_TYPE},
		Client, StatusCode,
	},
	serde::{Deserialize, Serialize},
	std::num::NonZeroU16,
};

pub type AnthropicResult<T> = std::result::Result<T, AnthropicClientError>;

#[derive(Clone)]
pub struct AnthropicClient {
	inner: Client,
}

impl AnthropicClient {
	pub const ANTHROPIC_VERSION: &'static str = "2023-06-01";
	pub const DEFAULT_URL: &'static str = "https://api.anthropic.com/v1/messages";
	pub const USER_AGENT: &'static str = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION"));

	pub fn new(key: &str) -> Self {
		let mut val = HeaderValue::from_bytes(key.as_bytes()).unwrap();
		val.set_sensitive(true);
		let mut headers = HeaderMap::new();
		headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
		headers.insert("anthropic-version", HeaderValue::from_static(Self::ANTHROPIC_VERSION));
		headers.insert("x-api-key", val);
		Self { inner: Client::builder().default_headers(headers).build().unwrap() }
	}

	pub async fn chat(&self, request: ChatRequest) -> AnthropicResult<ChatResponse> {
		let response = self.inner.post(Self::DEFAULT_URL).json(&request).send().await?;
		if response.status() != StatusCode::OK {
			return Err(response.json::<AnthropicErrorWrapper>().await?.error.into());
		}
		Ok(serde_json::from_slice(&response.bytes().await?)?)
	}

	pub async fn stream_chat(&self, request: ChatRequest) -> AnthropicResult<impl futures::Stream<Item = AnthropicResult<ChatStreamEvent>> + Send + 'static> {
		use {eventsource_stream::Eventsource, futures::TryStreamExt};
		let client = self.inner.clone();
		let stream = futures::stream::once(async move {
			let response = client.post(Self::DEFAULT_URL).json(&request).send().await?;
			if response.status() != StatusCode::OK {
				return Err(AnthropicClientError::Anthropic(response.json::<AnthropicErrorWrapper>().await?.error));
			}
			Ok::<_, AnthropicClientError>(response.bytes_stream().eventsource().map(|event| match event {
				Ok(event) => match serde_json::from_str::<ChatStreamEvent>(&event.data) {
					Ok(event) => Ok(event),
					Err(error) => Err(AnthropicClientError::Parse(error)),
				},
				Err(error) => Err(AnthropicClientError::Stream(error)),
			}))
		})
		.try_flatten();

		Ok(stream)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatRequest {
	pub model: String,
	pub messages: Vec<Message>,
	pub max_tokens: Option<u32>,
	pub stream: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
	pub role: String,
	pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
	pub content: Vec<ContentBlock>,
	pub stop_reason: Option<String>,
	pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct ContentBlock {
	#[serde(rename = "type")]
	pub block_type: String,
	pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
	pub input_tokens: u32,
	pub output_tokens: u32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ChatStreamEvent {
	#[serde(rename = "content_block_start")]
	ContentBlockStart { index: usize, content_block: ContentBlock },
	#[serde(rename = "content_block_delta")]
	ContentBlockDelta { index: usize, delta: ContentDelta },
	#[serde(rename = "content_block_stop")]
	ContentBlockStop { index: usize },
	#[serde(rename = "message_start")]
	MessageStart { message: ChatResponse },
	#[serde(rename = "message_delta")]
	MessageDelta { delta: MessageDelta },
	#[serde(rename = "message_stop")]
	MessageStop,
}

#[derive(Debug, Deserialize)]
pub struct ContentDelta {
	pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MessageDelta {
	pub stop_reason: Option<String>,
	pub usage: Option<Usage>,
}

#[derive(Debug, thiserror::Error)]
pub enum AnthropicClientError {
	#[error("HTTP error: {0}")]
	#[allow(clippy::upper_case_acronyms)]
	HTTP(#[from] reqwest::Error),
	#[error("Parse error: {0}")]
	Parse(#[from] serde_json::Error),
	#[error("Anthropic error: {0}")]
	Anthropic(#[from] AnthropicError),
	#[error("Stream error: {0}")]
	Stream(#[from] eventsource_stream::EventStreamError<reqwest::Error>),
}

#[derive(Debug, Deserialize, thiserror::Error, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum AnthropicError {
	#[error("invalid request (400): {message}")]
	#[serde(rename = "invalid_request_error")]
	InvalidRequest { message: String },
	#[error("authentication (401): {message}")]
	#[serde(rename = "authentication_error")]
	Authentication { message: String },
	#[error("permission (403): {message}")]
	#[serde(rename = "permission_error")]
	Permission { message: String },
	#[error("not found (404): {message}")]
	#[serde(rename = "not_found_error")]
	NotFound { message: String },
	#[error("request too large (413): {message}")]
	RequestTooLarge { message: String },
	#[error("rate limit (429): {message}")]
	#[serde(rename = "rate_limit_error")]
	RateLimit { message: String },
	#[error("api error (500): {message}")]
	#[serde(rename = "api_error")]
	#[allow(clippy::upper_case_acronyms)]
	API { message: String },
	#[error("overloaded (529): {message}")]
	#[serde(rename = "overloaded_error")]
	Overloaded { message: String },
	#[error("unknown error ({code}): {message}")]
	Unknown { code: NonZeroU16, message: String },
}

impl AnthropicError {
	pub fn status(&self) -> NonZeroU16 {
		match self {
			Self::InvalidRequest { .. } => NonZeroU16::new(400).unwrap(),
			Self::Authentication { .. } => NonZeroU16::new(401).unwrap(),
			Self::Permission { .. } => NonZeroU16::new(403).unwrap(),
			Self::NotFound { .. } => NonZeroU16::new(404).unwrap(),
			Self::RequestTooLarge { .. } => NonZeroU16::new(413).unwrap(),
			Self::RateLimit { .. } => NonZeroU16::new(429).unwrap(),
			Self::API { .. } => NonZeroU16::new(500).unwrap(),
			Self::Overloaded { .. } => NonZeroU16::new(529).unwrap(),
			Self::Unknown { code, .. } => *code,
		}
	}
}

#[derive(Deserialize)]
struct AnthropicErrorWrapper {
	error: AnthropicError,
}
