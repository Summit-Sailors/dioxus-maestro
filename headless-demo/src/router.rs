use {
	crate::{
		layout::Layout,
		pages::{
			AccordionPage, AspectRatioPage, AvatarPage, ButtonPage, CheckboxPage, CollapsiblePage, DialogPage, Home, HoverCardPage, PopoverPage, ProgressBarPage,
			RadioPage,
		},
	},
	dioxus::prelude::*,
	strum_macros::EnumIter,
};

#[derive(Clone, PartialEq, EnumIter, Routable)]
pub enum Route {
	#[layout(Layout)]
	#[route("/")]
	Home {},
	#[nest("/components")]
	#[route("/accordion")]
	AccordionPage {},
	#[route("/aspect-ratio")]
	AspectRatioPage {},
	#[route("/avatar")]
	AvatarPage {},
	#[route("/button")]
	ButtonPage {},
	#[route("/checkbox-&-group")]
	CheckboxPage,
	#[route("/collapsible")]
	CollapsiblePage {},
	#[route("/dialog")]
	DialogPage {},
	#[route("/hover-card")]
	HoverCardPage {},
	#[route("/popover")]
	PopoverPage {},
	#[route("/progress")]
	ProgressBarPage {},
	#[route("/radion-group")]
	RadioPage {},
}

impl Route {
	pub fn name(&self) -> &'static str {
		match self {
			Route::Home {} => "Home",
			Route::AccordionPage {} => "Accordion",
			Route::AspectRatioPage {} => "Aspect Ratio",
			Route::AvatarPage {} => "Avatar",
			Route::ButtonPage {} => "Button",
			Route::CheckboxPage {} => "Checkbox & CheckboxGroup",
			Route::CollapsiblePage {} => "Collapsible",
			Route::DialogPage {} => "Dialog",
			Route::HoverCardPage {} => "Hover Card",
			Route::PopoverPage {} => "Popover",
			Route::ProgressBarPage {} => "Progress Bar",
			Route::RadioPage {} => "Radio Group",
		}
	}
}
