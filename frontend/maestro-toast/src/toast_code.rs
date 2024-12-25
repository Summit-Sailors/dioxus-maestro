#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum EToastCode {
	Success,
	Warning,
	Error,
	#[default]
	Info,
}
