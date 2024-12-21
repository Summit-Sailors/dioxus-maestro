#![allow(non_snake_case)]

use crate::{toast_code::EToastCode, toast_position::EToastPosition};

#[derive(Debug, Clone, bon::Builder)]
pub struct ToastInfo {
	pub heading: Option<String>,
	pub context: String,
	pub allow_toast_close: bool,
	pub position: EToastPosition,
	pub icon: Option<EToastCode>,
	pub hide_after: Option<usize>,
}
