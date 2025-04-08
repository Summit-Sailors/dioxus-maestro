use {
	crate::components::{
		description_section::DescriptionSection,
		example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
		features_list::Features,
		tables::{AttrsStruct, PropsStruct, PropsTable},
		tabs::PageTabs,
	},
	consts::{EXAMPLE, EXAMPLE_ANATOMY},
	dioxus::prelude::*,
	maestro_headless::{
		range::{Range, RangeRoot, RangeThumb, RangeTrack},
		shared::EOrientation,
	},
};

mod consts;

#[component]
pub fn RangePage() -> Element {
	let mut value = use_signal(|| Vec::from([0.0_f32]));
	let features_list: Vec<&str> = Vec::from([
		"Controlled/uncontrolled state",
		"Keyboard navigation",
		"Multiple thumbs",
		"Minimum value between thumbs",
		"Supports vertical / horizontal orientation",
	]);

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Range",
			description: "An input where the user selects a value from within a given range.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex grow items-center justify-center w-full",
					RangeRoot {
						class: "w-52 flex items-center",
						value: value(),
						on_value_change: move |v| { value.set(v) },
						RangeTrack { class: "flex-1 bg-neutral-600 rounded-full h-1",
							Range { class: "flex-1 bg-orange-600 rounded-full h-1" }
						}
						RangeThumb { class: "w-6 h-6 rounded-full bg-orange-600 flex items-center justify-center text-neutral-300 text-xs cursor-pointer transition-colors hover:text-neutral-100 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-orange-600 focus-visible:ring-offset-2 focus-visible:ring-offset-neutral-900",
							"{value().get(0).unwrap_or(&0.0):.0}"
						}
					}
				}
				ExampleCodeCollapsible { code: EXAMPLE }
			}
		}
		DescriptionSection { title: "Supports",
			Features { features: features_list.clone() }
		}
		DescriptionSection { title: "Usage and Anatomy",
			ExampleCodeAnatomy { code: EXAMPLE_ANATOMY }
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "RangeRoot" }
					p {
						"Wrapps all parts of the range input. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"value"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_value_change"
						}
						" if "
						span { class: "font-medium", "controllable state" }
						" is used "
						span { class: "font-medium", "must go in pair" }
						". In other case may be used "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"default_value"
						}
						"."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "value".into(),
										prop_default: "None".into(),
										prop_type: "Option<Vec<f32>>".into(),
										tooltip_text: Some("Must go in pair with 'on_value_change' callback".into()),
								},
								PropsStruct {
										prop: "on_value_change".into(),
										prop_default: "None".into(),
										prop_type: "Callback<Vec<f32>>".into(),
										tooltip_text: Some("Must go in pair with 'value' prop".into()),
								},
								PropsStruct {
										prop: "default_value".into(),
										prop_default: "0".into(),
										prop_type: "f32".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "orientation".into(),
										prop_default: "EOrientation::Vertical".into(),
										prop_type: "EOrientation::Vertical | EOrientation::Horizontal".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Prevents from toggling any item in the group".into()),
								},
								PropsStruct {
										prop: "required".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "min".into(),
										prop_default: "0.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some("Minimum allowed value".into()),
								},
								PropsStruct {
										prop: "max".into(),
										prop_default: "100.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some("Maximum allowed value".into()),
								},
								PropsStruct {
										prop: "step".into(),
										prop_default: "1.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some("The step interval".into()),
								},
								PropsStruct {
										prop: "min_steps_between_thumbs".into(),
										prop_default: "0.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some(
												"The minimum permitted steps between multiple thumbs".into(),
										),
								},
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'div' attribules".into()),
								},
								PropsStruct {
										prop: "children".into(),
										prop_default: "-".into(),
										prop_type: "Element".into(),
										tooltip_text: Some("Required".into()),
								},
						]),
						attrs_list: Vec::from([
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if is disabled".into(),
								},
								AttrsStruct {
										attr: "aria-required".into(),
										value: "true".into(),
										description: "Appears only if is required".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if is disabled".into(),
								},
								AttrsStruct {
										attr: "data-required".into(),
										value: "true".into(),
										description: "Appears only if is required".into(),
								},
								AttrsStruct {
										attr: "aria-roledescription".into(),
										value: "range slider".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-label".into(),
										value: "<String>".into(),
										description: "Current value".into(),
								},
								AttrsStruct {
										attr: "aria-orientation".into(),
										value: "horizontal | vartical".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-orientation".into(),
										value: "horizontal | vartical".into(),
										description: "".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "RangeTrack" }
					p {
						"The track that contains the "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"Range"
						}
						"."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'button' attribules".into()),
								},
								PropsStruct {
										prop: "children".into(),
										prop_default: "None".into(),
										prop_type: "Element".into(),
										tooltip_text: Some("Custom checked indicator if specified".into()),
								},
						]),
						attrs_list: Vec::from([
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if the range is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the range is disabled".into(),
								},
								AttrsStruct {
										attr: "aria-orientation".into(),
										value: "horizontal | vartical".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-orientation".into(),
										value: "horizontal | vartical".into(),
										description: "".into(),
								},
						]),
					}
				}

				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Range" }
					p {
						"The range itself. Must be a child of the "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"RangeTrack"
						}
						" component."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'button' attribules".into()),
								},
								PropsStruct {
										prop: "children".into(),
										prop_default: "None".into(),
										prop_type: "Element".into(),
										tooltip_text: Some("Custom checked indicator if specified".into()),
								},
						]),
						attrs_list: Vec::from([
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if the range is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the range is disabled".into(),
								},
								AttrsStruct {
										attr: "aria-orientation".into(),
										value: "horizontal | vartical".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-orientation".into(),
										value: "horizontal | vartical".into(),
										description: "".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "RangeThumb" }
					p {
						"A draggable thumb. Must be the child of "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"RangeRoot"
						}
						" component."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'span' attribules".into()),
								},
								PropsStruct {
										prop: "children".into(),
										prop_default: "None".into(),
										prop_type: "Option<Element>".into(),
										tooltip_text: None,
								},
						]),
						attrs_list: Vec::from([
								AttrsStruct {
										attr: "aria-valuemin".into(),
										value: "<f32>".into(),
										description: "Min value".into(),
								},
								AttrsStruct {
										attr: "aria-valuenow".into(),
										value: "<f32>".into(),
										description: "Current value of the thumb".into(),
								},
								AttrsStruct {
										attr: "aria-valuemax".into(),
										value: "<f32>".into(),
										description: "Max valiue".into(),
								},
								AttrsStruct {
										attr: "aria-orientation".into(),
										value: "horizontal | vartical".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-orientation".into(),
										value: "horizontal | vartical".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if the range is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the range is disabled".into(),
								},
						]),
					}
				}
			}
		}
	}
}
