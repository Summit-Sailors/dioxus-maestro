use crate::{toast_code::EToastCode, toast_position::EToastPosition};

#[derive(Debug, Clone, bon::Builder)]
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
