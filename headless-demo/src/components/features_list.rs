use {
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsCheckLg},
};

#[derive(Clone, PartialEq, Props)]
pub struct FeaturesProps {
	#[props(extends = ul, extends = GlobalAttributes)]
	pub attributes: Vec<Attribute>,
	pub features: Vec<&'static str>,
}

pub fn Features(props: FeaturesProps) -> Element {
	rsx! {
		ul {
			class: "flex flex-col gap-2 *:flex *:items-center *:gap-2",
			..props.attributes,
			for feature in props.features.iter() {
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"{feature}"
				}
			}
		}
	}
}
