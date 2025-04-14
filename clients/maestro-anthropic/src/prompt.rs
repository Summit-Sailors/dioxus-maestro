use std::num::NonZeroU16;

use serde::{Deserialize, Serialize};

use super::{Model, Tool, chat_message::ChatMessage, tool};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, bon::Builder)]
pub struct Prompt {
	#[builder(finish_fn)]
	pub stream: bool,

	pub messages: Vec<ChatMessage>,
	#[builder(default = Model::Sonnet37)]
	pub model: Model,
	#[builder(default = NonZeroU16::new(8192).unwrap())]
	pub max_tokens: NonZeroU16,
	#[builder(default = 0.2)]
	pub temperature: f32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_k: Option<NonZeroU16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stop_sequences: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<tool::Choice>,
	#[builder(default)]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub tools: Vec<Tool>,

	#[builder(default)]
	#[serde(skip_serializing_if = "serde_json::Map::is_empty")]
	pub metadata: serde_json::Map<String, serde_json::Value>,
}

pub use prompt_builder::*;
