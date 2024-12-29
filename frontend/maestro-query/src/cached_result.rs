use {
	crate::result::QueryResult,
	instant::Instant,
	std::{fmt::Debug, ops::Deref, time::Duration},
};

const STALE_TIME: u64 = 100;

#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct CachedResult<T, E> {
	pub(crate) value: QueryResult<T, E>,
	pub(crate) instant: Option<Instant>,
	#[builder(default = false)]
	pub(crate) has_been_queried: bool,
}

impl<T, E> CachedResult<T, E> {
	pub fn value(&self) -> &QueryResult<T, E> {
		&self.value
	}

	pub fn is_fresh(&self) -> bool {
		if let Some(instant) = self.instant {
			instant.elapsed().as_millis() < Duration::from_millis(STALE_TIME).as_millis()
		} else {
			false
		}
	}

	pub(crate) fn has_been_queried(&self) -> bool {
		self.has_been_queried
	}

	pub(crate) fn set_to_loading(&mut self) {
		self.value.set_loading();
		self.instant = Some(Instant::now());
		self.has_been_queried = true;
	}
}

impl<T, E> Deref for CachedResult<T, E> {
	type Target = QueryResult<T, E>;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}
