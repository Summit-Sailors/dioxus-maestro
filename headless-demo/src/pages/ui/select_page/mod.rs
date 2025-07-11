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
	maestro_headless::switch::{SwitchIndicator, SwitchRoot},
	maestro_ui::{
		select::{Select, SelectDropdown, SelectOption, SelectTrigger, SelectValue},
		shared::ESide,
	},
};

mod consts;

#[component]
pub fn SelectStyledPage() -> Element {
	let mut selected = use_signal(Vec::new);
	let mut disabled = use_signal(|| false);
	let mut is_multiple = use_signal(|| false);

	rsx! {
		DescriptionSection {
			class: "[&>h3]:lg:text-2xl [&>h3]:text-xl",
			title: "Select",
			description: "A UI component that allows to choose one or more options from a dropdown list, providing a compact and efficient way to select values from a predefined set of choices.",
		}
		section { class: "container flex flex-col px-4 lg:py-6 py-4 ",
			div { class: "flex flex-wrap gap-5 items-center mb-4",
				div { class: "flex flex-wrap items-center gap-2 text-neutral-100 mb-4 mt-5",
					SwitchRoot {
						checked: disabled(),
						on_toggle_change: move |v| disabled.set(v),
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Disable"
				}
				div { class: "flex items-center gap-2 text-neutral-100 mb-4 mt-5",
					SwitchRoot {
						checked: is_multiple(),
						on_toggle_change: move |v| is_multiple.set(v),
						class: "flex items-center px-1 py-1 rounded-full h-6 w-12 bg-neutral-500 data-[state=checked]:bg-neutral-100 border border-neutral-700",
						SwitchIndicator { class: "relative data-[state=checked]:translate-x-5 transition ease-linear rounded-full w-5 h-5 bg-neutral-900" }
					}
					"Multiple"
				}
			}
			div { class: "grow flex flex-col justify-center items-center rounded-md border border-neutral-800 bg-neutral-950 overflow-hidden",
				div { class: "p-6 flex gap-4 items-start w-full max-w-96",
					Select {
						value: selected(),
						on_value_change: move |value: Vec<String>| { selected.set(value) },
						multi: is_multiple(),
						disabled,
						SelectTrigger {
							SelectValue { placeholder: "Chose something..." }
						}
						SelectDropdown {
							side: ESide::Bottom,
							side_offset: 10.0,
							class: "w-60",
							SelectOption { key: 1, value: "apple", "Apple" }
							SelectOption { key: 2, value: "banana", "Banana" }
							SelectOption { key: 3, value: "ice-cream", "Ice-Cream" }
							SelectOption { key: 4, value: "coffee", "Coffee" }
							SelectOption { key: 5, value: "salt", disabled: true, "Salt" }
							SelectOption { key: 6, value: "chocolatte", "Chocolatte" }
						}
					}
				}
				ExampleCodeCollapsible { code: EXAMPLE }
			}
		}
		DescriptionSection { title: "Usage and Anatomy",
			ExampleCodeAnatomy { code: EXAMPLE_ANATOMY }
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "SelectRoot" }
					p {
						"Wrapps all parts of the select and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"value"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_value_change"
						}
						span { class: "font-medium", " must go in pair" }
						" if use "
						span { class: "font-medium", "controllable state" }
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
										prop_type: "Option<Vec<String>>".into(),
										tooltip_text: Some(
												"Must be used in pair with on_value_change callback".into(),
										),
								},
								PropsStruct {
										prop: "on_value_change".into(),
										prop_default: "None".into(),
										prop_type: "Callback<Vec<String>>".into(),
										tooltip_text: Some("Must be used in pair with value prop".into()),
								},
								PropsStruct {
										prop: "default_value".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<String>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "open".into(),
										prop_default: "None".into(),
										prop_type: "Option<bool>".into(),
										tooltip_text: Some(
												"Must be used in pair with on_open_change callback".into(),
										),
								},
								PropsStruct {
										prop: "on_open_change".into(),
										prop_default: "None".into(),
										prop_type: "Callback<bool>".into(),
										tooltip_text: Some("Must be used in pair with open prop".into()),
								},
								PropsStruct {
										prop: "default_open".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "disabled".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Prevents select from any interactions".into()),
								},
								PropsStruct {
										prop: "required".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "multi".into(),
										prop_default: "false".into(),
										prop_type: "bool".into(),
										tooltip_text: Some("Used to indicate than the select is multiple".into()),
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
										attr: "aria-haspopup".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if the select is disabled".into(),
								},
								AttrsStruct {
										attr: "aria-expanded".into(),
										value: "<bool>".into(),
										description: "Indicates if the select is opened".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the select is disabled".into(),
								},
								AttrsStruct {
										attr: "data-role".into(),
										value: "select".into(),
										description: "".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "SelectTrigger" }
					p {
						"Opens/closes select dropdown. Must be a child of "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"SelectRoot"
						}
						" component."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "onclick".into(),
										prop_default: "None".into(),
										prop_type: "Callback<Event<MouseData>>".into(),
										tooltip_text: None,
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
										attr: "data-state".into(),
										value: "open | closed".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-expanded".into(),
										value: "true".into(),
										description: "Appears if current item is opened".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "SelectIcon" }
					p {
						"A small icon displayed next to the value and used for the visualization of the fact select is opened/closed. Must be a child of "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"SelectTrigger"
						}
						" component. Optional."
					}
					PropsTable {
						content: Vec::from([
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'span' attribules".into()),
								},
								PropsStruct {
										prop: "children".into(),
										prop_default: "None".into(),
										prop_type: "Element".into(),
										tooltip_text: None,
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "SelectValue" }
					p {
						"Displays selected value or the placeholder. Must live in "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"SelectTrigger"
						}
						" component."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "placeholder".into(),
										prop_default: "''".into(),
										prop_type: "Option<String>".into(),
										tooltip_text: None,
								},
								PropsStruct {
										prop: "attributes".into(),
										prop_default: "[]".into(),
										prop_type: "Vec<Attribute>".into(),
										tooltip_text: Some("Extends 'global' and 'span' attribules".into()),
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
										attr: "data-state".into(),
										value: "value | placeholder".into(),
										description: "".into(),
								},
						]),
					}
				}
				div {
					h4 { class: "font-medium text-lg text-orange-300", "Dropdown Component" }
					p { class: "mb-4",
						"The dropdown component that appears when the trigger pressed. Must be a child of "
						span { class: "px-1.5 py-0.5 font-mono text-orange-400 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"SelectRoot"
						}
						" component."
					}
					PageTabs {
						props_list: Vec::from([
								PropsStruct {
										prop: "side".into(),
										prop_default: "ESide::Bottom".into(),
										prop_type: "ESide".into(),
										tooltip_text: Some(
												"The preferred side of the trigger to render against when open".into(),
										),
								},
								PropsStruct {
										prop: "side_offset".into(),
										prop_default: "0.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some("The distance from the trigger (px)".into()),
								},
								PropsStruct {
										prop: "align".into(),
										prop_default: "EAlign::Center".into(),
										prop_type: "EAlign".into(),
										tooltip_text: Some("The preferred alignment against the trigger".into()),
								},
								PropsStruct {
										prop: "align_offset".into(),
										prop_default: "0.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some(
												"The offset from the 'start' or 'end' alignment options (px)".into(),
										),
								},
								PropsStruct {
										prop: "avoid_collisions".into(),
										prop_default: "true".into(),
										prop_type: "bool".into(),
										tooltip_text: Some(
												"Overrides the side preferences to prevent collisions with boundary edges."
														.into(),
										),
								},
								PropsStruct {
										prop: "collision_padding".into(),
										prop_default: "0.0".into(),
										prop_type: "f32".into(),
										tooltip_text: Some(
												"The distance in pixels from the boundary edges where collision detection should occur"
														.into(),
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
										attr: "aria-hidden".into(),
										value: "true".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-state".into(),
										value: "open | closed".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-side".into(),
										value: "left | right | bottom | top".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-align".into(),
										value: "start | end | center".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "--maestro-select-anchor-width".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-select-anchor-height".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-select-content-width".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
								AttrsStruct {
										attr: "--maestro-select-content-height".into(),
										value: "<>px".into(),
										description: "CSS variable".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "SelectOption" }
					p { "The component that contains the select option." }
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
										tooltip_text: Some("Prevents toggling current item".into()),
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
										attr: "data-role".into(),
										value: "option".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-disabled".into(),
										value: "true".into(),
										description: "Appears only if the current item is disabled".into(),
								},
								AttrsStruct {
										attr: "data-disabled".into(),
										value: "true".into(),
										description: "Appears only if the current item is disabled".into(),
								},
								AttrsStruct {
										attr: "data-selected".into(),
										value: "<bool>".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-selected".into(),
										value: "accordion-item".into(),
										description: "".into(),
								},
						]),
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "SelectOptionIndicator" }
					p { class: "flex gap-1 mb-4", "Renders when the item is selected. Optional." }
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
										prop_type: "Element".into(),
										tooltip_text: None,
								},
						]),
						attrs_list: Vec::from([
								AttrsStruct {
										attr: "data-hidden".into(),
										value: "<bool>".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "aria-hidden".into(),
										value: "<bool>".into(),
										description: "".into(),
								},
								AttrsStruct {
										attr: "data-role".into(),
										value: "indicator".into(),
										description: "".into(),
								},
						]),
					}
				}
			}
		}
	}
}
