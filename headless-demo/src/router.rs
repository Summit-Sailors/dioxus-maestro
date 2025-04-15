use {
	crate::{
		layout::Layout,
		pages::{
			AccordionPage, AspectRatioPage, AvatarPage, ButtonPage, CheckboxPage, CollapsiblePage, DialogPage, HeadlessHome, Home, HoverCardPage, PopoverPage,
			ProgressBarPage, RadioPage, RangePage, SelectPage, SeparatorPage, SwitchPage, TabsPage, TogglePage, TooltipPage,
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
	#[nest("/headless")]
	#[route("/")]
	HeadlessHome {},
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
	#[route("/radio-group")]
	RadioPage {},
	#[route("/range")]
	RangePage {},
	#[route("/select")]
	SelectPage {},
	#[route("/separator")]
	SeparatorPage {},
	#[route("/switch")]
	SwitchPage {},
	#[route("/tabs")]
	TabsPage {},
	#[route("/toggle-&-group")]
	TogglePage,
	#[route("/tooltip")]
	TooltipPage,
}

impl Route {
	pub fn name(&self) -> &'static str {
		match self {
			Route::Home {} => "Home",
			Route::HeadlessHome {} => "Headless",
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
			Route::RangePage {} => "Range",
			Route::SelectPage {} => "Select",
			Route::SeparatorPage {} => "Separator",
			Route::SwitchPage {} => "Switch",
			Route::TabsPage {} => "Tabs",
			Route::TogglePage {} => "Toggle & ToggleGroup",
			Route::TooltipPage {} => "Tooltip",
		}
	}

	pub fn get_headless_routes() -> Vec<Route> {
		Vec::from([
			Route::AccordionPage {},
			Route::AspectRatioPage {},
			Route::AvatarPage {},
			Route::ButtonPage {},
			Route::CheckboxPage {},
			Route::CollapsiblePage {},
			Route::DialogPage {},
			Route::HoverCardPage {},
			Route::PopoverPage {},
			Route::ProgressBarPage {},
			Route::RadioPage {},
			Route::RangePage {},
			Route::SelectPage {},
			Route::SeparatorPage {},
			Route::SwitchPage {},
			Route::TabsPage {},
			Route::TogglePage {},
			Route::TooltipPage {},
		])
	}
}
