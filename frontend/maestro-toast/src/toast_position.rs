#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum EToastPosition {
	BottomLeft,
	#[default]
	BottomRight,
	TopLeft,
	TopRight,
}
