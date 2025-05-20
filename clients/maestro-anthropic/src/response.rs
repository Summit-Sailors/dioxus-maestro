use {
	crate::{
		stream::{ContentMismatch, Delta, DeltaError, MessageStats},
		Model,
	},
	serde::{Deserialize, Serialize},
};

#[derive(strum_macros::AsRefStr, Clone, Copy, Debug, Deserialize, strum_macros::Display, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
	User,
	Assistant,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
	pub role: Role,
	pub content: Content,
}

#[derive(Clone, Debug, Deserialize, derive_more::IsVariant, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Content {
	SinglePart(String),
	MultiPart(Vec<Block>),
}

impl Content {
	pub fn text<T>(text: T) -> Self
	where
		T: Into<String>,
	{
		Self::SinglePart(text.into())
	}

	pub fn unwrap_single_part(self) -> Block {
		match self {
			Self::SinglePart(text) => Block::Text { text },
			Self::MultiPart(_) => {
				panic!("Content is MultiPart, not SinglePart");
			},
		}
	}

	pub fn push<P>(&mut self, part: P)
	where
		P: Into<Block>,
	{
		if self.is_single_part() {
			let mut old = Content::MultiPart(vec![]);
			std::mem::swap(self, &mut old);
			self.push(old.unwrap_single_part());
		}

		if let Content::MultiPart(parts) = self {
			parts.push(part.into());
		}
	}

	pub fn last(&self) -> Option<&Block> {
		match self {
			Self::SinglePart(_) => None,
			Self::MultiPart(parts) => parts.last(),
		}
	}
}

impl std::fmt::Display for Content {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::SinglePart(string) => write!(f, "{}", string),
			// This could be derived but the `Join` trait is not stable. Neither
			// is `Iterator::intersperse`. This also has fewer allocations.
			Self::MultiPart(parts) => {
				let mut iter = parts.iter();
				if let Some(part) = iter.next() {
					write!(f, "{}", part)?;
					for part in iter {
						write!(f, "{}{}", Self::SEP, part)?;
					}
				}
				Ok(())
			},
		}
	}
}

impl Content {
	pub const SEP: &'static str = "\n\n";
}

#[derive(Clone, Debug, Deserialize, strum_macros::Display, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Block {
	#[serde(alias = "text_delta")]
	Text { text: String },
	ToolUse {
		#[serde(flatten)]
		call: Use,
	},
	ToolResult {
		#[serde(flatten)]
		call: ToolBlock,
	},
}

impl Block {
	pub fn text<T>(text: T) -> Self
	where
		T: Into<String>,
	{
		Self::Text { text: text.into() }
	}

	pub fn merge_deltas<TDeltas>(&mut self, deltas: TDeltas) -> Result<(), DeltaError>
	where
		TDeltas: IntoIterator<Item = Delta>,
	{
		let mut it = deltas.into_iter();
		let acc: Delta = match it.next() {
			Some(delta) => delta,
			None => return Ok(()),
		};
		let acc: Delta = it.try_fold(acc, |acc, delta| acc.merge(delta))?;
		match (self, acc) {
			(Block::Text { text, .. }, Delta::Text { text: delta }) => {
				text.push_str(&delta);
			},
			(Block::ToolUse { call: Use { input, .. } }, Delta::Json { partial_json }) => {
				use serde_json::Value::Object;
				let partial_json: serde_json::Value = serde_json::from_str(&partial_json)
					.map_err(|e| DeltaError::Parse { error: format!("Could not merge partial json `{}` into `{}` because {}", partial_json, input, e) })?;
				if let (Object(new), Object(old)) = (partial_json, input) {
					old.extend(new);
				}
			},
			(this, acc) => {
				let variant_name = match this {
					Block::Text { .. } => stringify!(Block::Text),
					Block::ToolUse { .. } => stringify!(Block::ToolUse),
					Block::ToolResult { .. } => stringify!(Block::ToolResult),
				};
				return Err(ContentMismatch { from: acc, to: variant_name }.into());
			},
		}
		Ok(())
	}

	pub fn tool_use(&self) -> Option<&Use> {
		match self {
			Self::ToolUse { call, .. } => Some(call),
			_ => None,
		}
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ResponseMessage {
	pub id: String,
	#[serde(flatten)]
	pub message: Message,
	pub model: Model,
	pub stop_reason: Option<StopReason>,
	pub stop_sequence: Option<String>,
	pub usage: Usage,
}

impl ResponseMessage {
	pub fn apply_delta(&mut self, delta: MessageStats) {
		self.stop_reason = delta.stop_reason;

		self.stop_sequence = delta.stop_sequence;
		if let Some(usage) = delta.usage {
			self.usage = usage;
		}
	}

	pub fn tool_use(&self) -> Option<&Use> {
		if !matches!(self.stop_reason, Some(StopReason::ToolUse)) {
			return None;
		}

		self.message.content.last()?.tool_use()
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
	EndTurn,
	MaxTokens,
	StopSequence,
	ToolUse,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Usage {
	pub input_tokens: u64,
	pub output_tokens: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Use {
	pub id: String,
	pub name: String,
	pub input: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ToolBlock {
	pub tool_use_id: String,
	pub content: Content,
	pub is_error: bool,
}
