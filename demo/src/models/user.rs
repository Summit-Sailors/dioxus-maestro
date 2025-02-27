
use {
  serde::{Deserialize, Serialize}, 
  strum_macros::{Display, EnumString, VariantNames}, 
  validator::Validate
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct User {
  #[validate(length(min = 3, max = 20, message = "Username must be between 3 and 20 characters"))]
  pub username: String,
  #[validate(email(message = "Invalid email format"))]
  pub email: String,
  #[validate(length(min = 3, message = "Bio must be at least 3 words"))]
  pub bio: String,
  #[validate(range(min = 18, max = 150, message = "Age must be between 18 and 150"))]
  pub age: i32,
  pub role: Role,
}

impl Default for User {
  fn default() -> Self {
    Self {
      username: String::new(),
      email: String::new(),
      bio: String::new(),
      age: 18,
      role: Role::Admin,
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
