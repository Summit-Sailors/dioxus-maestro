use {crate::cached_result::CachedResult, std::mem};

#[derive(Debug, Clone, PartialEq)]
pub enum QueryResult<T, E> {
	Ok(T),
	Err(E),
	Loading(Option<T>),
}

impl<T, E> QueryResult<T, E> {
	pub fn is_ok(&self) -> bool {
		matches!(self, QueryResult::Ok(..))
	}

	pub fn is_err(&self) -> bool {
		matches!(self, QueryResult::Err(..))
	}

	pub fn is_loading(&self) -> bool {
		matches!(self, QueryResult::Loading(..))
	}

	pub fn set_loading(&mut self) {
		let result = mem::replace(self, Self::Loading(None));
		if let Self::Ok(v) = result {
			*self = Self::Loading(Some(v))
		}
	}
}

impl<T, E> Default for QueryResult<T, E> {
	fn default() -> Self {
		Self::Loading(None)
	}
}

impl<T, E> From<CachedResult<T, E>> for Option<T> {
	fn from(result: CachedResult<T, E>) -> Self {
		match result.value {
			QueryResult::Ok(v) => Some(v),
			QueryResult::Err(_) => None,
			QueryResult::Loading(v) => v,
		}
	}
}

impl<T, E> From<Result<T, E>> for QueryResult<T, E> {
	fn from(value: Result<T, E>) -> Self {
		match value {
			Ok(v) => QueryResult::Ok(v),
			Err(e) => QueryResult::Err(e),
		}
	}
}

impl<T, E> From<T> for QueryResult<T, E> {
	fn from(value: T) -> Self {
		QueryResult::Ok(value)
	}
}
