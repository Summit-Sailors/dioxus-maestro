#![allow(non_snake_case)]

use crate::{toast_code::EToastCode, toast_position::EToastPosition};

#[derive(bon::Builder, Clone, Debug)]
pub struct ToastInfo {
	pub heading: Option<String>,
	pub context: String,
	#[builder(default = true)]
	pub allow_toast_close: bool,
	#[builder(default)]
	pub position: EToastPosition,
	pub icon: Option<EToastCode>,
	#[builder(default = 10)]
	pub hide_after: usize,
}
