use {
	crate::{
		components::{
			description_section::DescriptionSection,
			example_code::{ExampleCodeAnatomy, ExampleCodeCollapsible},
			features_list::Features,
			tables::{AttrsStruct, PropsStruct},
			tabs::PageTabs,
		},
		pages::headless::checkbox_page::consts::{EXAMPLE, EXAMPLE_ANATOMY},
	},
	dioxus::prelude::*,
	maestro_headless::checkbox::{CheckboxIndicator, CheckboxRoot},
};

#[component]
pub fn CheckboxContent() -> Element {
	let features_list: Vec<&str> = Vec::from(["Controlled/uncontrolled state", "Keyboard navigation"]);

	rsx! {
		p { class: "container flex flex-col gap-3 lg:py-6 py-4 text-neutral-300",
			"A control that allows the user to toggle between checked and not checked."
		}
		div { class: "grow flex flex-col justify-center items-center overflow-hidden rounded-md border border-neutral-800 bg-neutral-950",
			div { class: "p-6 flex flex-col gap-4 items-start",
				div { class: "flex justify-center items-center gap-3",
					CheckboxRoot {
						id: "maestro-box",
						class: "w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none",
						value: "some",
						CheckboxIndicator { class: "text-neutral-100 " }
					}
					label { class: "text-slate-100", r#for: "maestro-box", "Check Me" }
				}
			}
			ExampleCodeCollapsible { code: EXAMPLE }
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
					h4 { class: "font-medium text-lg text-orange-300", "CheckboxRoot" }
					p {
						"Indicates a checkbox input. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"checked"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_change"
						}
						span { class: "font-medium", " must go in pair" }
						"in case of usage "
						span { class: "font-medium", "controllable state" }
						". In other case may be used "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"default_checked"
						}
						"."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "checked".into(),
										prop_default: "None".into(),
										prop_type: "Option<bool>".into(),
										tooltip_text: Some("Must go in pair with 'on_value_change' callback".into()),
								},
								PropsStruct {
										prop: "on_value_change".into(),
										prop_default: "None".into(),
										prop_type: "Callback<bool>".into(),
										tooltip_text: Some("Must go in pair with 'checked' prop".into()),
								},
								PropsStruct {
										prop: "default_checked".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Prevents checkbox from selecting/deselecting".into()),
								},
								PropsStruct {
										prop: "required".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "value".into(),
										prop_default: "-".into(),
										prop_type: "String".into(),
										tooltip_text: Some("Required".into()),
								},
								PropsStruct {
										prop: "extra_attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some(
												"Helps provide attributes to the checkbox if it is nested into other components"
														.into(),
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
										prop_default: "-".into(),
										prop_type: "Element".into(),
										tooltip_text: Some("Required".into()),
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
										description: "Appears only if the checkbox is disabled".into(),
								},
								AttrsStruct {
										attr: "aria-required".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the checkbox is disabled".into(),
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
					h4 { class: "font-medium text-lg text-orange-300", "CheckboxIndicator" }
					p {
						"Appears when the checkbox is checked. May accept checked component. Must be wrapped by "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"CheckboxRoot"
						}
						"."
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
										description: "Appears only if the checkbox is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the checkbox is disabled".into(),
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
