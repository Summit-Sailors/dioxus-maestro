use {
	super::{
		prompt::{IsComplete, PromptBuilder},
		AnthropicStream,
	},
	crate::prompt::{SetMessages, SetSystem, SetToolChoice, SetTools},
	eventsource_stream::Eventsource,
	reqwest::{
		header::{HeaderMap, HeaderValue, CONTENT_TYPE},
		Client, Method, StatusCode,
	},
	serde::{Deserialize, Serialize},
	std::{env, num::NonZeroU16},
};

pub type AnthropicResult<T> = std::result::Result<T, AnthropicClientError>;

#[derive(Clone)]
pub struct AnthropicClient {
	pub(crate) inner: Client,
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

	pub async fn message<S: IsComplete>(&self, prompt: PromptBuilder<S>) -> AnthropicResult<super::response::ResponseMessage> {
		let response = self.inner.request(Method::POST, Self::DEFAULT_URL).json(&prompt.build(false)).send().await?;
		if response.status() != StatusCode::OK {
			return Err(response.json::<AnthropicErrorWrapper>().await?.error.into());
		}
		Ok(serde_json::from_slice(&response.bytes().await?)?)
	}

	pub async fn stream<S: IsComplete>(&self, prompt: PromptBuilder<S>) -> AnthropicResult<AnthropicStream> {
		let response = self.inner.request(Method::POST, Self::DEFAULT_URL).json(&prompt.build(true)).send().await?;
		if response.status() != StatusCode::OK {
			return Err(response.json::<AnthropicErrorWrapper>().await?.error.into());
		}
		Ok(AnthropicStream::new(response.bytes_stream().eventsource()))
	}

	pub async fn call_tool(&self, prompt: PromptBuilder<SetToolChoice<SetTools<SetSystem<SetMessages>>>>) -> AnthropicResult<serde_json::Value> {
		let raw_resp = self.message(prompt).await?;
		println!("{raw_resp:#?}");
		raw_resp
			.message
			.content
			.last()
			.and_then(|block| block.tool_use())
			.map(|tool_use| tool_use.input.to_owned())
			.ok_or(AnthropicClientError::UnexpectedResponse { message: "No tool use result found" })
	}
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
	#[error("Unexpected response: {message}")]
	#[allow(missing_docs)]
	UnexpectedResponse { message: &'static str },
}

#[derive(Debug, thiserror::Error, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[allow(missing_docs)]
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
	// This inconsistency is in the API.
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
	// Anthropic's API specifies they can add more error codes in the future.
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
#[serde(tag = "error")]
pub(crate) struct AnthropicErrorWrapper {
	pub(crate) error: AnthropicError,
}
