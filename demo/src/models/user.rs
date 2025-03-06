use {
	serde::{Deserialize, Serialize},
	strum_macros::{Display, EnumString, VariantNames},
	validator::{Validate, ValidationError},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString, VariantNames)]
#[serde(rename_all = "lowercase")]
pub enum Role {
	#[strum(serialize = "admin")]
	Admin,
	#[strum(serialize = "user")]
	User,
	#[strum(serialize = "moderator")]
	Moderator,
}

fn validate_word_count(text: &str) -> Result<(), ValidationError> {
	let word_count = text.split_whitespace().count();
	if word_count < 3 {
		return Err(ValidationError::new("invalid_word_count"));
	}
	Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct User {
	#[validate(length(min = 3, max = 20, message = "Username must be between 3 and 20 characters"))]
	pub username: String,
	#[validate(email(message = "Invalid email format"))]
	pub email: String,
	#[validate(custom(function = "validate_word_count"))]
	pub bio: String,
	#[validate(range(min = 18, max = 150, message = "Age must be between 18 and 150"))]
	pub age: i32,
	pub role: Role,
}

impl Default for User {
	fn default() -> Self {
		Self { username: String::new(), email: String::new(), bio: String::new(), age: 18, role: Role::Admin }
	}
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAttrs {
	pub gender: String,
	pub favorite_color: String,
}
