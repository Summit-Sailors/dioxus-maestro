use {
	crate::{
		layout::Layout,
		pages::{calendar::CalendarDemo, form::FormsDemo, home::HomePage, hooks::HooksDemo, plotters::PlottersDemo, toast::ToastDemo, ui::UIDemo},
	},
	dioxus::prelude::*,
	strum_macros::EnumIter,
};

#[derive(Clone, PartialEq, EnumIter, Routable)]
pub enum Route {
	#[layout(Layout)]
	#[route("/")]
	HomePage {},
	#[route("/form")]
	FormsDemo {},
	#[route("/hooks")]
	HooksDemo {},
	#[route("/plotters")]
	PlottersDemo {},
	#[route("/toast")]
	ToastDemo {},
	#[route("/ui")]
	UIDemo {},
	#[route("/calendar")]
	CalendarDemo {},
}

impl Route {
	pub fn name(&self) -> &'static str {
		match self {
			Route::HomePage {} => "Home",
			Route::FormsDemo {} => "Form",
			Route::HooksDemo {} => "Hooks",
			Route::PlottersDemo {} => "Plotters",
			Route::ToastDemo {} => "Toast",
			Route::UIDemo {} => "UI",
			Route::CalendarDemo {} => "Calendar",
		}
	}

	pub fn description(&self) -> &'static str {
		match self {
			Route::HomePage {} => "Home",
			Route::FormsDemo {} => "A powerful, type-safe form management solution for Dioxus applications that brings the best of Formik's paradigms to Rust.",
			Route::HooksDemo {} => "Enhanced hooks collection for Dioxus applications that provides type safety, cross-platform compatibility, and optimized performance.",
			Route::PlottersDemo {} => "A powerful, flexible, and reactive charting library for Dioxus applications built on top of the plotters crate.",
			Route::ToastDemo {} => "A powerful, flexible, and intuitive toast notification system designed specifically for Dioxus applications. ",
			Route::UIDemo {} => "A comprehensive, type-safe, and highly customizable UI component library for Dioxus, designed to provide developers with powerful, flexible, and elegant UI building blocks.",
      Route::CalendarDemo {} => "A  highly customizable UI calendar and date picker utility.",
		}
	}
}
