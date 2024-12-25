#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum EToastPosition {
	BottomLeft,
	#[default]
	BottomRight,
	TopLeft,
	TopRight,
}
