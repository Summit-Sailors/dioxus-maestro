use serde::{Deserialize, Serialize};
use validator::Validate;
use apalis_core::storage::Job;

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

impl Job for User {
  const NAME: &'static str = "maestro::User";
}

impl Default for User {
  fn default() -> Self {
    Self {
      username: String::new(),
      email: String::new(),
      bio: String::new(),
      age: 18,
      role: String::new(),
    }
  }
}

