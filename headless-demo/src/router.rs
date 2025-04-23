use {
	crate::{
		layout::Layout,
		pages::{
			AccordionPage, AccordionStyledPage, AspectRatioPage, AspectRatioStyledPage, AvatarPage, AvatarStyledPage, ButtonPage, ButtonStyledPage, CheckboxPage,
			CheckboxStyledPage, CollapsiblePage, CollapsibleStyledPage, DialogPage, DialogStyledPage, HeadlessHome, Home, HoverCardPage, HoverCardStyledPage,
			PopoverPage, PopoverStyledPage, ProgressBarPage, ProgressBarStyledPage, RadioPage, RadioStyledPage, RangePage, RangeStyledPage, SelectPage,
			SelectStyledPage, SeparatorPage, SeparatorStyledPage, SheetStyledPage, StyledHome, SwitchPage, SwitchStyledPage, TabsPage, TabsStyledPage, TogglePage,
			ToggleStyledPage, TooltipPage, TooltipStyledPage,
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
	#[end_nest]
	#[end_nest]
	#[nest("/ui")]
	#[route("/")]
	StyledHome {},
	#[nest("/components")]
	#[route("/button")]
	ButtonStyledPage,
	#[route("/accordion")]
	AccordionStyledPage,
	#[route("/aspect-ratio")]
	AspectRatioStyledPage {},
	#[route("/avatar")]
	AvatarStyledPage {},
	#[route("/checkbox-&-group")]
	CheckboxStyledPage,
	#[route("/collapsible")]
	CollapsibleStyledPage {},
	#[route("/dialog")]
	DialogStyledPage {},
	#[route("/hover-card")]
	HoverCardStyledPage {},
	#[route("/popover")]
	PopoverStyledPage {},
	#[route("/progress")]
	ProgressBarStyledPage {},
	#[route("/radio-group")]
	RadioStyledPage {},
	#[route("/range")]
	RangeStyledPage {},
	#[route("/select")]
	SelectStyledPage {},
	#[route("/separator")]
	SeparatorStyledPage {},
	#[route("/sheet")]
	SheetStyledPage,
	#[route("/switch")]
	SwitchStyledPage {},
	#[route("/tabs")]
	TabsStyledPage {},
	#[route("/toggle-&-group")]
	ToggleStyledPage,
	#[route("/tooltip")]
	TooltipStyledPage,
}

impl Route {
	pub fn name(&self) -> &'static str {
		match self {
			Route::Home {} => "Home",
			Route::HeadlessHome {} => "Headless Lib",
			Route::StyledHome {} => "UI Lib",
			Route::AccordionPage {} | Route::AccordionStyledPage {} => "Accordion",
			Route::AspectRatioPage {} | Route::AspectRatioStyledPage {} => "Aspect Ratio",
			Route::AvatarPage {} | Route::AvatarStyledPage {} => "Avatar",
			Route::ButtonPage {} | Route::ButtonStyledPage {} => "Button",
			Route::CheckboxPage {} | Route::CheckboxStyledPage {} => "Checkbox & CheckboxGroup",
			Route::CollapsiblePage {} | Route::CollapsibleStyledPage {} => "Collapsible",
			Route::DialogPage {} | Route::DialogStyledPage {} => "Dialog",
			Route::HoverCardPage {} | Route::HoverCardStyledPage {} => "Hover Card",
			Route::PopoverPage {} | Route::PopoverStyledPage {} => "Popover",
			Route::ProgressBarPage {} | Route::ProgressBarStyledPage {} => "Progress Bar",
			Route::RadioPage {} | Route::RadioStyledPage {} => "Radio Group",
			Route::RangePage {} | Route::RangeStyledPage {} => "Range",
			Route::SelectPage {} | Route::SelectStyledPage {} => "Select",
			Route::SeparatorPage {} | Route::SeparatorStyledPage {} => "Separator",
			Route::SwitchPage {} | Route::SwitchStyledPage {} => "Switch",
			Route::TabsPage {} | Route::TabsStyledPage {} => "Tabs",
			Route::TogglePage {} | Route::ToggleStyledPage {} => "Toggle & ToggleGroup",
			Route::TooltipPage {} | Route::TooltipStyledPage {} => "Tooltip",
			Route::SheetStyledPage {} => "Sheet",
		}
	}

	pub fn get_headless_routes() -> Vec<Route> {
		Vec::from([
			Route::Home {},
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

	pub fn get_styled_routes() -> Vec<Route> {
		Vec::from([
			Route::Home {},
			Route::AccordionStyledPage {},
			Route::AspectRatioStyledPage {},
			Route::AvatarStyledPage {},
			Route::ButtonStyledPage {},
			Route::CheckboxStyledPage {},
			Route::CollapsibleStyledPage {},
			Route::DialogStyledPage {},
			Route::HoverCardStyledPage {},
			Route::PopoverStyledPage {},
			Route::ProgressBarStyledPage {},
			Route::RadioStyledPage {},
			Route::RangeStyledPage {},
			Route::SelectStyledPage {},
			Route::SeparatorStyledPage {},
			Route::SheetStyledPage {},
			Route::SwitchStyledPage {},
			Route::TabsStyledPage {},
			Route::ToggleStyledPage {},
			Route::TooltipStyledPage {},
		])
	}

	pub fn get_home_routes() -> Vec<Route> {
		Vec::from([Route::HeadlessHome {}, Route::StyledHome {}])
	}
}
