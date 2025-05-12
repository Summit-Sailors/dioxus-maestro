use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// Request DTO for Condition Codes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionCodesRequestDTO {
	// The type of ticks (required)
	pub ticktype: String,
	// The one character name of the tape (required)
	pub tape: String,
}

// Response DTO for Condition Codes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionCodesResponseDTO {
	// Mapping of condition codes to their names
	pub conditions: HashMap<String, String>,
}

// Request DTO for Exchange Codes (though this endpoint doesn't seem to require parameters)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ExchangeCodesRequestDTO {}

// Response DTO for Exchange Codes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeCodesResponseDTO {
	// Mapping of exchange codes to their names
	pub exchanges: HashMap<String, String>,
}
