#![allow(non_snake_case)]
use {
	dioxus::prelude::*,
	dioxus_free_icons::{
		icons::{fa_solid_icons::FaCheck, io_icons::IoChevronForward},
		Icon,
	},
	tailwind_fuse::tw_join,
};

#[derive(Clone, Debug, PartialEq, Props)]
pub struct FeatureProps {
	pub title: String,
	pub features: Vec<String>,
}

#[component]
pub fn Features(props: FeatureProps) -> Element {
	let mut is_expanded = use_signal(|| false);

	rsx! {
		div { class: "overflow-hidden",
			div {
				class: "flex items-center cursor-pointer gap-3",
				onclick: move |_| is_expanded.toggle(),
				span { class: "font-medium text-2xl text-slate-300 transition-colors hover:text-slate-100",
					"{props.title}"
				}
				span {
					Icon {
						width: 16,
						height: 16,
						icon: IoChevronForward,
						class: tw_join!(
								"fill-none transition-transform duration-200", (is_expanded())
								.then_some("rotate-90")
						),
					}
				}
			}

			div {
				class: tw_join!(
						"w-full transition-all duration-300", if is_expanded() {
						"block opacity-100 h-full scale-y-100 mt-3" } else {
						"opacity-0 h-0 scale-y-0 mt-0" }
				),
				ul { class: "flex flex-col gap-2.5",
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
												li { class: "flex items-start gap-2 text-slate-100",
													span { class: "w-5 h-5 flex items-center justify-center bg-indigo-500 rounded-full",
														Icon {
															width: 16,
															height: 16,
															icon: FaCheck,
															class: "text-slate-200 w-2 h-2",
														}
													}
													span {
														if !_feature.is_empty() {
															span { class: "font-medium", "{_feature}" }
															"{description}"
														} else {
															"{feature}"
														}
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
