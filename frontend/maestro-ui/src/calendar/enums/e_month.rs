use std::ops::{Add, Sub};

use chrono::{Datelike, NaiveDate};
use num_traits::AsPrimitive;
use serde::{Deserialize, Serialize};

use super::e_week_day::ECalendarDay;

#[derive(
	Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, strum_macros::EnumIter, strum_macros::Display, strum_macros::FromRepr, strum_macros::IntoStaticStr,
)]
#[repr(u8)]
pub enum ECalendarMonth {
	January = 1,
	February = 2,
	March = 3,
	April = 4,
	May = 5,
	June = 6,
	July = 7,
	August = 8,
	September = 9,
	October = 10,
	November = 11,
	December = 12,
}

impl<T> From<T> for ECalendarMonth
where
	T: AsPrimitive<u8>,
{
	fn from(prim: T) -> Self {
		let month_num = ((prim.as_() - 1) % 12) + 1;
		Self::from_repr(month_num).unwrap()
	}
}

impl ECalendarMonth {
	pub fn num_days(&self, year: i32) -> u8 {
		let month = *self as u32;
		NaiveDate::from_ymd_opt(year, month, 1)
			.unwrap()
			.checked_add_months(chrono::Months::new(1))
			.unwrap()
			.signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
			.num_days() as u8
	}

	pub fn first_day(&self, year: i32) -> ECalendarDay {
		(NaiveDate::from_ymd_opt(year, *self as u32, 1).unwrap().weekday() as u8).into()
	}

	pub fn last_day(&self, year: i32) -> ECalendarDay {
		(NaiveDate::from_ymd_opt(year, *self as u32, self.num_days(year) as u32).unwrap().weekday() as u8).into()
	}

	pub fn next(&self) -> Self {
		let next_month = (*self as u8 % 12) + 1;
		Self::from_repr(next_month).unwrap()
	}

	pub fn prev(&self) -> Self {
		let prev_month = if *self as u8 == 1 { 12 } else { *self as u8 - 1 };
		Self::from_repr(prev_month).unwrap()
	}
}

impl Add<ECalendarMonth> for ECalendarMonth {
	type Output = Self;

	fn add(self, rhs: ECalendarMonth) -> Self::Output {
		self + rhs as u8
	}
}

impl Sub<ECalendarMonth> for ECalendarMonth {
	type Output = Self;

	fn sub(self, rhs: ECalendarMonth) -> Self::Output {
		self - rhs as u8
	}
}

impl<T> Add<T> for ECalendarMonth
where
	T: AsPrimitive<u8>,
{
	type Output = Self;

	fn add(self, rhs: T) -> Self::Output {
		let result = if self as u8 == 11 { 12 } else { (self as u8 + rhs.as_()) % 12 };
		Self::from_repr(result).unwrap()
	}
}

impl<T> Sub<T> for ECalendarMonth
where
	T: AsPrimitive<u8>,
{
	type Output = Self;

	fn sub(self, rhs: T) -> Self::Output {
		let result = if self as u8 == 1 { 12 } else { (self as u8 + 12 - (rhs.as_() % 12)) % 12 };
		Self::from_repr(result).unwrap()
	}
}
