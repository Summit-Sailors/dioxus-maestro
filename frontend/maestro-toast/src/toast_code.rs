#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum EToastCode {
	Success,
	Warning,
	Error,
	#[default]
	Info,
}
