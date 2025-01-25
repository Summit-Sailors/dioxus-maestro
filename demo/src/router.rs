use {
	crate::{
		layout::Layout,
    pages::{
      forms::FormsDemo,
      home::HomePage,
      hooks::HooksDemo,
      plotters::PlottersDemo,
      query::QueryDemo,
      radio::RadioDemo,
      toast::ToastDemo,
      ui::UIDemo
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
	QueryDemo {},
	#[route("/radio")]
	RadioDemo {},
	#[route("/toast")]
	ToastDemo {},
	#[route("/ui")]
	UIDemo {},
}

impl Route {
	pub fn name(&self) -> &'static str {
		match self {
			Route::HomePage {} => "Home",
			Route::FormsDemo {} => "Form",
			Route::HooksDemo {} => "Hooks",
			Route::PlottersDemo {} => "Plotters",
			Route::QueryDemo {} => "Query",
			Route::RadioDemo {} => "Radio",
			Route::ToastDemo {} => "Toast",
			Route::UIDemo {} => "UI",
		}
	}
}
