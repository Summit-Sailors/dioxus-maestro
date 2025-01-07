use {
	super::e_week_day::ECalendarDay,
	chrono::{Datelike, NaiveDate},
	num_traits::AsPrimitive,
	serde::{Deserialize, Serialize},
	std::ops::{Add, Sub},
};

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
		(((prim.as_()) % 12) + 1).into()
	}
}

impl ECalendarMonth {
	pub fn num_days(&self, year: i32) -> u8 {
		let month = *self as u32;
		NaiveDate::from_ymd_opt(year, month, 1)
			.unwrap() // safe since we know 1st of any month exists
			.checked_add_months(chrono::Months::new(1))
			.unwrap() // safe since adding 1 month to valid date is always valid
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
		*self + 1
	}

	pub fn prev(&self) -> Self {
		*self - 1
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
		Self::from_repr(self as u8 + rhs.as_()).unwrap()
	}
}

impl<T> Sub<T> for ECalendarMonth
where
	T: AsPrimitive<u8>,
{
	type Output = Self;

	fn sub(self, rhs: T) -> Self::Output {
		Self::from_repr(self as u8 - rhs.as_()).unwrap()
	}
}
