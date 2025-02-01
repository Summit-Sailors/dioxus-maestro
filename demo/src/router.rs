use {
	crate::{
		layout::Layout,
    pages::{
      forms::FormsDemo,
      home::HomePage,
      hooks::HooksDemo,
      plotters::PlottersDemo,
      query::CompleteQueryDemo,
      radio::RadioDemo,
      toast::ToastDemo,
      ui::UIDemo,
      calendar::CalendarDemo
    }
	},
	dioxus::prelude::*,
	strum_macros::EnumIter,
};

#[derive(Routable, Clone, PartialEq, EnumIter)]
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
	#[route("/query")]
	CompleteQueryDemo {},
	#[route("/radio")]
	RadioDemo {},
	#[route("/toast")]
	ToastDemo {},
	#[route("/ui")]
	UIDemo {},
	#[route("/calendar")]
	CalendarDemo {}
}

impl Route {
	pub fn name(&self) -> &'static str {
		match self {
			Route::HomePage {} => "Home",
			Route::FormsDemo {} => "Form",
			Route::HooksDemo {} => "Hooks",
			Route::PlottersDemo {} => "Plotters",
			Route::CompleteQueryDemo {} => "Query",
			Route::RadioDemo {} => "Radio",
			Route::ToastDemo {} => "Toast",
			Route::UIDemo {} => "UI",
      Route::CalendarDemo {} => "Calendar"
		}
	}
}
