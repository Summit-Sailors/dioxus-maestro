use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
	#[serde(rename = "new")]
	New,
	#[serde(rename = "replaced")]
	Replaced,
	#[serde(rename = "order_replace_rejected")]
	ReplaceRejected,
	#[serde(rename = "partial_fill")]
	PartialFill,
	#[serde(rename = "fill")]
	Filled,
	#[serde(rename = "done_for_day")]
	DoneForDay,
	#[serde(rename = "canceled")]
	Canceled,
	#[serde(rename = "order_cancel_rejected")]
	CancelRejected,
	#[serde(rename = "expired")]
	Expired,
	#[serde(rename = "pending_cancel")]
	PendingCancel,
	#[serde(rename = "stopped")]
	Stopped,
	#[serde(rename = "rejected")]
	Rejected,
	#[serde(rename = "suspended")]
	Suspended,
	#[serde(rename = "pending_new")]
	PendingNew,
	#[serde(rename = "pending_replace")]
	PendingReplace,
	#[serde(rename = "calculated")]
	Calculated,
}
