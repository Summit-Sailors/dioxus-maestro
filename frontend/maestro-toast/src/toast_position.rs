#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum EToastPosition {
	BottomLeft,
	#[default]
	BottomRight,
	TopLeft,
	TopRight,
}
