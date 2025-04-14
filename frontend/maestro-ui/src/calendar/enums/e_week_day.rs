use std::ops::{Add, Sub};

use num_traits::AsPrimitive;
use serde::{Deserialize, Serialize};

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
		(7 - self.days_from_sunday()) % 7
	}

	pub fn next(&self) -> Self {
		let next_day = (*self as u8 + 1) % 7;
		Self::from_repr(next_day).unwrap()
	}

	pub fn prev(&self) -> Self {
		let prev_day = (*self as u8 + 6) % 7;
		Self::from_repr(prev_day).unwrap()
	}
}

impl<T> From<T> for ECalendarDay
where
	T: AsPrimitive<u32>,
{
	fn from(day: T) -> Self {
		let day_num = day.as_() % 7;
		Self::from_repr(day_num as u8).unwrap()
	}
}

impl Add<ECalendarDay> for ECalendarDay {
	type Output = Self;

	fn add(self, rhs: ECalendarDay) -> Self::Output {
		let new_day = (self as u8 + rhs as u8) % 7;
		Self::from_repr(new_day).unwrap()
	}
}

impl Sub<ECalendarDay> for ECalendarDay {
	type Output = Self;

	fn sub(self, rhs: ECalendarDay) -> Self::Output {
		let new_day = (self as u8 + 7 - rhs as u8) % 7;
		Self::from_repr(new_day).unwrap()
	}
}

impl<T> Add<T> for ECalendarDay
where
	T: AsPrimitive<u8>,
{
	type Output = Self;

	fn add(self, rhs: T) -> Self::Output {
		let new_day = (self as u8 + rhs.as_()) % 7;
		Self::from_repr(new_day).unwrap()
	}
}

impl<T> Sub<T> for ECalendarDay
where
	T: AsPrimitive<u8>,
{
	type Output = Self;

	fn sub(self, rhs: T) -> Self::Output {
		let new_day = (self as u8 + 7 - (rhs.as_() % 7)) % 7;
		Self::from_repr(new_day).unwrap()
	}
}
