#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum EToastCode {
	Success,
	Warning,
	Error,
	#[default]
	Info,
}
