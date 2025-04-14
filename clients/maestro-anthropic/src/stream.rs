use std::{pin::Pin, task::Poll};

use futures::StreamExt;
use serde::{Deserialize, Serialize};

use super::{
	client::AnthropicError,
	response::{Block, ResponseMessage, StopReason, Usage},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Event {
	Ping,
	MessageStart { message: ResponseMessage },
	ContentBlockStart { index: usize, content_block: Block },
	ContentBlockDelta { index: usize, delta: Delta },
	ContentBlockStop { index: usize },
	MessageStats { delta: MessageStats },
	MessageStop,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ApiResult {
	Event {
		#[serde(flatten)]
		event: Event,
	},
	Error {
		error: AnthropicError,
	},
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Delta {
	#[serde(alias = "text_delta")]
	Text { text: String },

	#[serde(rename = "input_json_delta")]
	Json { partial_json: String },
}

#[derive(Debug, thiserror::Error, Serialize)]
#[error("`Delta::{from:?}` canot be applied to `{to}`.")]
pub struct ContentMismatch {
	pub from: Delta,
	pub to: &'static str,
}

#[derive(Debug, thiserror::Error, Serialize)]
#[error("Index {index} out of bounds. Max index is {max}.")]
pub struct OutOfBounds {
	pub index: usize,
	pub max: usize,
}

#[derive(Debug, thiserror::Error, Serialize, derive_more::From)]
pub enum DeltaError {
	#[error("Cannot apply delta because: {error}")]
	ContentMismatch { error: ContentMismatch },
	#[error("Cannot apply delta because: {error}")]
	OutOfBounds { error: OutOfBounds },
	#[error("Cannot apply delta because deserialization failed because: {error}")]
	Parse { error: String },
}

impl Delta {
	pub fn merge(mut self, delta: Delta) -> Result<Self, ContentMismatch> {
		match (&mut self, delta) {
			(Delta::Text { text }, Delta::Text { text: delta }) => {
				text.push_str(&delta);
			},
			(Delta::Json { partial_json }, Delta::Json { partial_json: delta }) => {
				partial_json.push_str(&delta);
			},
			(to, from) => {
				return Err(ContentMismatch {
					from,
					to: match to {
						Delta::Text { .. } => stringify!(Delta::Text),
						Delta::Json { .. } => stringify!(Delta::Json),
					},
				});
			},
		}

		Ok(self)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStats {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stop_reason: Option<StopReason>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stop_sequence: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage: Option<Usage>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("HTTP error: {error}")]
	Stream {
		#[from]
		error: eventsource_stream::EventStreamError<reqwest::Error>,
	},
	#[error("JSON error: {error}")]
	Parse { error: serde_json::Error, event: eventsource_stream::Event },
	#[error("API error: {error}")]
	Anthropic { error: AnthropicError, event: eventsource_stream::Event },
}

pub struct Stream {
	inner: Pin<Box<dyn futures::Stream<Item = Result<Event, Error>> + Send + 'static>>,
}

static_assertions::assert_impl_all!(Stream: futures::Stream, Send);

impl Stream {
	pub fn new<S>(stream: S) -> Self
	where
		S: futures::Stream<Item = Result<eventsource_stream::Event, eventsource_stream::EventStreamError<reqwest::Error>>> + Send + 'static,
	{
		Self {
			inner: Box::pin(stream.map(|event| match event {
				Ok(event) => match serde_json::from_str::<ApiResult>(&event.data) {
					Ok(ApiResult::Event { event }) => Ok(event),
					Ok(ApiResult::Error { error }) => Err(Error::Anthropic { error, event }),
					Err(error) => Err(Error::Parse { error, event }),
				},
				Err(error) => Err(Error::Stream { error }),
			})),
		}
	}
}

impl futures::Stream for Stream {
	type Item = Result<Event, Error>;

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Option<Self::Item>> {
		self.inner.as_mut().poll_next(cx)
	}
}

pub trait FilterExt: futures::stream::Stream<Item = Result<Event, Error>> + Sized + Send {
	/// Filter out rate limit and overload errors. Because the server sends
	/// these events there isn't a need to retry or backoff. The stream will
	/// continue when ready.
	///
	/// This is recommended for most use cases.
	fn filter_rate_limit(self) -> impl futures::Stream<Item = Result<Event, Error>> + Send {
		self.filter_map(|result| async move {
			match result {
				Ok(event) => Some(Ok(event)),
				Err(Error::Anthropic { error: AnthropicError::Overloaded { .. } | AnthropicError::RateLimit { .. }, .. }) => None,
				Err(error) => Some(Err(error)),
			}
		})
	}

	/// Filter out everything but [`Event::ContentBlockDelta`]. This can include
	/// text, JSON, and tool use.
	fn deltas(self) -> impl futures::Stream<Item = Result<Delta, Error>> + Send {
		self.filter_map(|result| async move {
			match result {
				Ok(Event::ContentBlockDelta { delta, .. }) => Some(Ok(delta)),
				_ => None,
			}
		})
	}

	/// Filter out everything but text pieces.
	fn text(self) -> impl futures::Stream<Item = Result<String, Error>> + Send {
		self.deltas().filter_map(|result| async move {
			match result {
				Ok(Delta::Text { text }) => Some(Ok(text)),
				_ => None,
			}
		})
	}
}

impl<S> FilterExt for S where S: futures::Stream<Item = Result<Event, Error>> + Send {}
