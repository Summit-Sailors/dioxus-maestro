use {
	crate::{
		components::{
			description_section::DescriptionSection,
			example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
			features_list::Features,
			tables::{AttrsStruct, PropsStruct},
			tabs::PageTabs,
		},
		pages::checkbox_page::consts::{EXAMPLE_GROUP, EXAMPLE_GROUP_ANATOMY},
	},
	dioxus::prelude::*,
	maestro_headless::{
		checkbox_group::{CheckboxGroup, CheckboxGroupIndicator, CheckboxGroupItem},
		shared::EOrientation,
	},
};

#[component]
pub fn CheckboxGroupContent() -> Element {
	let mut values = use_signal::<Vec<String>>(Vec::new);

	let features_list: Vec<&str> =
		Vec::from(["Controlled/uncontrolled state", "Keyboard navigation", "Group manages checked state of each checkbox", "Group manages list of checked values"]);

	rsx! {
		p { class: "container flex flex-col gap-3 lg:py-6 py-4 text-neutral-300",
			"A group of checkboxes where several items can be checked at a time."
		}
		div { class: "grow flex flex-col justify-center items-center overflow-hidden rounded-md border border-neutral-800 bg-neutral-950",
			div { class: "p-6 flex flex-col gap-4 items-start",
				CheckboxGroup {
					class: "flex items-center gap-4 md:flex-row flex-col",
					value: values(),
					on_value_change: move |v: Vec<String>| {
							values.set(v);
					},
					div { class: "flex justify-center items-center gap-3",
						CheckboxGroupItem {
							id: "chocolate",
							class: "w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50",
							value: "chocolate",
							CheckboxGroupIndicator {}
						}
						label { class: "text-slate-100", r#for: "chocolate", "Chocolate" }
					}
					div { class: "flex justify-center items-center gap-3",
						CheckboxGroupItem {
							id: "banana",
							disabled: true,
							class: "w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50",
							value: "banana",
							CheckboxGroupIndicator {}
						}
						label {
							class: "text-slate-100 pointer-events-none opacity-50",
							r#for: "banana",
							"Banana"
						}
					}
					div { class: "flex justify-center items-center gap-3",
						CheckboxGroupItem {
							id: "coffee",
							class: "w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50",
							value: "coffee",
							CheckboxGroupIndicator {}
						}
						label { class: "text-slate-100", r#for: "coffee", "Coffee" }
					}
					div { class: "flex justify-center items-center gap-3",
						CheckboxGroupItem {
							id: "ice-cream",
							class: "w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50",
							value: "ice-cream",
							CheckboxGroupIndicator {}
						}
						label { class: "text-slate-100", r#for: "ice-cream", "Ice-cream" }
					}
				}
				ul { class: "space-y-2 list-disc list-inside",
					for value in values().iter() {
						li { "{value}" }
					}
				}
			}
			ExampleCodeCollapsible { code: EXAMPLE_GROUP }
		}
		DescriptionSection { title: "Supports",
			Features { features: features_list.clone() }
		}
		DescriptionSection { title: "Usage and Anatomy",
			ExampleCodeAnatomy { code: EXAMPLE_GROUP_ANATOMY }
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "CheckboxGroup" }
					p {
						"Wrapps all checkboxes of the checkbox group and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"value"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_value_change"
						}
						" in case of usage the "
						span { class: "font-medium", "controllable state" }
						" must go in pair. In other case may be used "
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
										prop_type: "Option<Vec<String>>".into(),
										tooltip_text: Some("Must go in pair with 'on_value_change' callback".into()),
								},
								PropsStruct {
										prop: "on_value_change".into(),
										prop_default: "None".into(),
										prop_type: "Callback<Vec<String>>".into(),
										tooltip_text: Some("Must go in pair with 'value' prop".into()),
								},
								PropsStruct {
										prop: "default_value".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<String>".into(),
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
										description: "Appears only if the group is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the group is disabled".into(),
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
					h4 { class: "font-medium text-lg text-orange-300", "CheckboxGroupItem" }
					p { "Indicates checkbox input as part of the group." }
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "value".into(),
										prop_default: "-".into(),
										prop_type: "String".into(),
										tooltip_text: Some("Required".into()),
								},
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some(
												"Prevents checkbox from selecting/deselecting current item".into(),
										),
								},
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
										attr: "aria-checked".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if the group or current item is disabled".into(),
								},
								AttrsStruct {
										attr: "aria-required".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the group or current item is disabled".into(),
								},
								AttrsStruct {
										attr: "data-state".into(),
										value: "checked | unchecked".into(),
										description: "".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "CheckboxGroupIndicator" }
					p {
						"Appears when the checkbox is checked. Must be placed in the "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"CheckboxGroupItem"
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
										attr: "aria-checked".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if the group is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the group is disabled".into(),
								},
								AttrsStruct {
										attr: "data-state".into(),
										value: "checked | unchecked".into(),
										description: "".into(),
								},
						]),
					}
				}
			}
		}
	}
}
