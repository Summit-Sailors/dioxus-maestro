use dioxus::prelude::*;
use strum_macros::EnumIter;

use crate::{
	clients::{
		db::{diesel_demo::DieselDemo, sqlx_demo::SqlxDemo},
		utilities::{alpaca_demo::AlpacaDemo, anthropic_demo::AnthropicDemo, apalis_demo::ApalisDemo, serpapi_demo::SerpApiDemo},
	},
	layout::Layout,
	pages::{
		calendar::CalendarDemo, form::FormsDemo, home::HomePage, hooks::HooksDemo, not_found::NotFound, plotters::PlottersDemo, query::CompleteQueryDemo,
		radio::RadioDemo, toast::ToastDemo, ui::UIDemo,
	},
};

#[derive(Clone, PartialEq, EnumIter, Routable)]
#[rustfmt::skip]
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
    CalendarDemo {},
    #[nest("/clients")]
      #[nest("/db")]
        #[route("/diesel")]
        DieselDemo {},
        #[route("/sqlx")]
        SqlxDemo {},
      #[end_nest]
      #[nest("/utilities")]
        #[route("/apalis")]
        ApalisDemo {},
        #[route("/serpapi")]
        SerpApiDemo {},
        #[route("/alpaca")]
        AlpacaDemo {},
        #[route("/anthropic")]
        AnthropicDemo {},
      #[end_nest]
    #[end_nest]
  #[end_layout]
  #[route("/:..route")]
  NotFound {route: Vec<String>}
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
			Route::CalendarDemo {} => "Calendar",
			Route::DieselDemo {} => "Diesel",
			Route::SqlxDemo {} => "Sqlx",
			Route::SerpApiDemo {} => "SerpAPI",
			Route::ApalisDemo {} => "Apalis",
			Route::AlpacaDemo {} => "Alpaca",
			Route::AnthropicDemo {} => "Anthropic",
			Route::NotFound { route: _ } => "Not Found",
		}
	}

	pub fn description(&self) -> &'static str {
		match self {
			Route::HomePage {} => "Home",
			Route::FormsDemo {} => "A powerful, type-safe form management solution for Dioxus applications that brings the best of Formik's paradigms to Rust.",
			Route::HooksDemo {} =>
				"Enhanced hooks collection for Dioxus applications that provides type safety, cross-platform compatibility, and optimized performance.",
			Route::PlottersDemo {} => "A powerful, flexible, and reactive charting library for Dioxus applications built on top of the plotters crate.",
			Route::CompleteQueryDemo {} =>
				"A powerful and flexible query management system for Dioxus applications that provides advanced caching, synchronization, and state management capabilities.",
			Route::RadioDemo {} => " A state management utility for Dioxus that takes reactive state to the next level with its innovative channel-based approach. ",
			Route::ToastDemo {} => "A powerful, flexible, and intuitive toast notification system designed specifically for Dioxus applications. ",
			Route::UIDemo {} =>
				"A comprehensive, type-safe, and highly customizable UI component library for Dioxus, designed to provide developers with powerful, flexible, and elegant UI building blocks.",
			Route::CalendarDemo {} => "A  highly customizable UI calendar and date picker utility.",
			Route::DieselDemo {} => "A diesel utility equipped with both sync and async database connection/pool creation and an extension for paginated queries",
			Route::SqlxDemo {} => "A sqlx utility equipped with both sync and async database connection/pool creation",
			Route::SerpApiDemo {} => "A serpapi utility designed to make your experience pleasant when integrating SerpAPI into your Dioxus app",
			Route::ApalisDemo {} => "A Apalis utility designed to make integrating Apalis into your Dioxus app easier",
			Route::AlpacaDemo {} => "A Alpaca utility designed to make connecting to and using Alpaca with your Dioxus apps easier",
			Route::AnthropicDemo {} => "A utility designed to make connecting to and using Anthropic with your Dioxus apps easier",
			Route::NotFound { route: _ } => "Page was not Found",
		}
	}
}
