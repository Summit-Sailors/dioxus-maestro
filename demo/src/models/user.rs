#[cfg(feature = "server")]
use apalis_core::storage::Job;
use {
	serde::{Deserialize, Serialize},
	validator::Validate
};

pub const AVAILABLE_ROLES: &[&str] = &["admin", "user", "moderator"];

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct User {
	#[validate(length(min = 3, max = 20, message = "Username must be between 3 and 20 characters"))]
	pub username: String,
	#[validate(email(message = "Invalid email format"))]
	pub email: String,
	#[validate(length(min = 5, message = "Bio must be at least 5 characters"))]
	pub bio: String,
	#[validate(range(min = 18, max = 150, message = "Age must be between 18 and 150"))]
	pub age: i32,
	#[validate(length(min = 2, message = "Please select a role"))]
	pub role: String,
}

impl Default for User {
  fn default() -> Self {
    Self {
      username: String::new(),
      email: String::new(),
      bio: String::new(),
      age: 18,
      role: AVAILABLE_ROLES[0].to_string(),
    }
  }
}


#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAttrs {
  pub gender: String,
  pub favorite_color: String,
}

impl Default for UserAttrs {
  fn default() -> Self {
    Self {
      gender: String::new(),
      favorite_color: String::new(),
    }
  }
}
