use {
	num_traits::AsPrimitive,
	serde::{Deserialize, Serialize},
	std::ops::{Add, Sub},
};

#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, strum_macros::EnumIter, strum_macros::Display, strum_macros::FromRepr, strum_macros::IntoStaticStr,
)]
#[repr(u8)]
pub enum ECalendarDay {
	Sun = 0,
	Mon = 1,
	Tue = 2,
	Wed = 3,
	Thu = 4,
	Fri = 5,
	Sat = 6,
}

impl ECalendarDay {
	pub fn is_weekend(&self) -> bool {
		matches!(self, ECalendarDay::Sat | ECalendarDay::Sun)
	}

	pub fn days_from_sunday(&self) -> u8 {
		*self as u8
	}

	pub fn days_until_sunday(&self) -> u8 {
		7 - self.days_from_sunday()
	}

	pub fn next(&self) -> Self {
		*self + 1
	}

	pub fn prev(&self) -> Self {
		*self - 1
	}
}

impl<T> From<T> for ECalendarDay
where
	T: AsPrimitive<u32>,
{
	fn from(day: T) -> Self {
		((day.as_()) % 7).into()
	}
}

impl Add<ECalendarDay> for ECalendarDay {
	type Output = Self;

	fn add(self, rhs: ECalendarDay) -> Self::Output {
		self + rhs as u8
	}
}

impl Sub<ECalendarDay> for ECalendarDay {
	type Output = Self;

	fn sub(self, rhs: ECalendarDay) -> Self::Output {
		self - rhs as u8
	}
}

impl<T> Add<T> for ECalendarDay
where
	T: AsPrimitive<u8>,
{
	type Output = Self;

	fn add(self, rhs: T) -> Self::Output {
		Self::from_repr(self as u8 + rhs.as_()).unwrap()
	}
}

impl<T> Sub<T> for ECalendarDay
where
	T: AsPrimitive<u8>,
{
	type Output = Self;

	fn sub(self, rhs: T) -> Self::Output {
		Self::from_repr(self as u8 - rhs.as_()).unwrap()
	}
}
