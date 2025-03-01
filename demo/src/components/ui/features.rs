#![allow(non_snake_case)]
use {
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::{fa_solid_icons::FaCheck, io_icons::IoChevronForward},
		Icon,
	},
	tailwind_fuse::tw_join,
};

#[derive(Debug, Props, Clone, PartialEq)]
pub struct FeatureProps {
	pub title: String,
	pub features: Vec<String>,
}

#[component]
pub fn Features(props: FeatureProps) -> Element {
	let mut is_expanded = use_signal(|| false);

	rsx! {
		div { class: "mb-4 rounded-lg hover:shadow-lg",
			div {
				class: "inline-flex items-center cursor-pointer px-3 py-2",
				onclick: move |_| is_expanded.toggle(),
				span { class: "font-semibold text-md text-gray-100",
					{format!("{} Features", props.title)}
				}
				span {
					Icon {
						width: 16,
						height: 16,
						icon: IoChevronForward,
						class: tw_join!(
								"transition-transform duration-200 text-gray-50", (is_expanded())
								.then_some("rotate-180")
						),
					}
				}
			}

			div {
				class: tw_join!(
						"px-3 pb-3 w-full transition-all duration-300", if is_expanded() {
						"block opacity-100 max-h-full scale-y-100" } else {
						"hidden opacity-0 max-h-0 scale-y-0" }
				),
				div { class: "mt-2 p-4 rounded-lg shadow-lg",
					ul {
						{
								props
										.features
										.iter()
										.map(|feature| {
												let (_feature, description) = match feature.find(":") {
														Some(idx) => {
																let (b, n) = feature.split_at(idx + 1);
																(b.to_string(), n.to_string())
														}
														None => (String::new(), feature.clone()),
												};
												rsx! {
													li { class: "flex flex-wrap items-start mb-2 space-x-2",
														span {
															Icon {
																width: 16,
																height: 16,
																icon: FaCheck,
																class: "text-green-500 mt-0.5",
															}
														}

														if !_feature.is_empty() {
															span { class: "text-gray-100 font-medium border rounded-md border-gray-700 p-1",
																"{_feature}"
															}
															span { class: "text-gray-400", "{description}" }
														} else {
															span { class: "text-gray-100", "{feature}" }
														}
													}
												}
										})
						}
					}
				}
			}
		}
	}
}
