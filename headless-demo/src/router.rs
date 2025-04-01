use {
	crate::{
		layout::Layout,
		pages::{AccordionPage, AspectRatioPage, AvatarPage, ButtonPage, CheckboxPage, Home},
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
		}
	}
}
