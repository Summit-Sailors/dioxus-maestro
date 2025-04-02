use {
	crate::components::{
		description_section::DescriptionSection,
		props_table::{PropsTable, TableBody},
	},
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsCheckLg},
	dioxus_logger::tracing::info,
	maestro_headless::{
		accordion::{Accordion, AccordionContent, AccordionHeader, AccordionItem, AccordionTrigger, AccordionVariant},
		checkbox::{Checkbox, CheckboxIndicator},
		checkbox_group::{CheckboxGroup, CheckboxGroupItem},
		collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
		shared::EOrientation,
		tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
	},
};

#[component]
pub fn CheckboxGroupContent() -> Element {
	let mut values = use_signal::<Vec<String>>(Vec::new);

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
					name: "favorites",
					div { class: "flex justify-center items-center gap-3",
						CheckboxGroupItem {
							id: "chocolate",
							class: "w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50",
							value: "chocolate",
						}
						label { class: "text-slate-100", r#for: "chocolate", "Chocolate" }
					}
					div { class: "flex justify-center items-center gap-3",
						CheckboxGroupItem {
							id: "banana",
							disabled: true,
							class: "w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50",
							value: "banana",
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
						}
						label { class: "text-slate-100", r#for: "coffee", "Coffee" }
					}
					div { class: "flex justify-center items-center gap-3",
						CheckboxGroupItem {
							id: "ice-cream",
							class: "w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50",
							value: "ice-cream",
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

			div { class: "py-3 w-full px-6 border-t border-neutral-800 bg-neutral-950 overflow-hidden",
				Collapsible { class: "flex flex-col  max-h-[640px] h-full ",
					div { class: "flex items-center gap-3 py-3",
						CollapsibleTrigger { class: "flex items-center justify-center px-3 py-2 font-medium rounded bg-orange-600 border-2 border-transparent hover:border-orange-600 text-neutral-50 hover:bg-neutral-950 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none transition-colors ease-linear",
							"Open Code"
						}
					}
					CollapsibleContent { class: "data-[state=closed]:animate-slide-out data-[state=open]:animate-slide-in overflow-auto",
						code { class: "font-mono whitespace-pre text-xs text-neutral-300",
							pre {
								"use dioxus::prelude::*;
use maestro_headless::checkbox::CheckboxIndicator;
use maestro_headless::checkbox_group::{{CheckboxGroup, CheckboxGroupItem}}

rsx! {{
  div {{ 
  class: 'p-6 flex flex-col gap-4 items-start',
  CheckboxGroup {{
    class: 'flex items-center gap-4 md:flex-row flex-col',
    value: values(),
    on_value_change: move |v: Vec<String>| {{
      values.set(v);
    }},
    name: 'favorites',
    div {{ 
      class: 'flex justify-center items-center gap-3',
      CheckboxGroupItem {{
        id: 'chocolate',
        class: 'w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50',
        value: 'chocolate',
      }}
      label {{
        class: 'text-slate-100',
        r#for: 'chocolate',
        'Chocolate'
      }}
    }}
    div {{ 
      class: 'flex justify-center items-center gap-3',
      CheckboxGroupItem {{
        id: 'banana',
        disabled: true,
        class: 'w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50',
        value: 'banana',
      }}
      label {{
        class: 'text-slate-100',
        r#for: 'banana',
        'Banana'
      }}
    }}
    div {{ 
      class: 'flex justify-center items-center gap-3',
      CheckboxGroupItem {{
        id: 'coffee',
        disabled: true,
        class: 'w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50',
        value: 'coffee',
      }}
      label {{
        class: 'text-slate-100',
        r#for: 'coffee',
        'Coffee'
      }}
    }}
    div {{ 
      class: 'flex justify-center items-center gap-3',
      CheckboxGroupItem {{
        id: 'ice-cream',
        disabled: true,
        class: 'w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50',
        value: 'ice-cream',
      }}
      label {{
        class: 'text-slate-100',
        r#for: 'ice-cream',
        'Ice-cream'
      }}
    }}
  }}
  ul {{ 
    class: 'space-y-2 list-disc list-inside',
    for value in values().iter() {{
      li {{ '{{value}}' }}
    }}
  }}
  }}		
}}"
							}
						}
					}
				}
			}
		}
		DescriptionSection { title: "Supports",
			ul { class: "flex flex-col gap-2 *:flex *:items-center *:gap-2",
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"Controlled/uncontrolled state"
				}
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"Keyboard navigation"
				}
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"Group manages checked state of each checkbox"
				}
				li {
					Icon { icon: BsCheckLg {}, class: "text-orange-600" }
					"Group manages list of checked values"
				}
			}
		}
		DescriptionSection {
			title: "Usage and Anatomy",
			description: "Import all parts and piece them together. Each part may be styled separately, accept own properties and additional attributes, e.g. \"data\" or \"aria\" (althought they are provided by default).",
			div { class: "grow flex flex-col rounded-md border border-neutral-800 bg-neutral-950 p-6",
				code { class: "font-mono whitespace-pre text-xs text-neutral-300",
					pre { "CheckboxGroup {{
	CheckboxGroupItem {{ }}
}}" }
				}
			}
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-6",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Root Component" }
					p {
						"Wrapps all checkboxes af the checkbox group and manages state. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"value"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_value_change"
						}
						span { class: "font-medium", "must go in pair" }
						"if use "
						span { class: "font-medium", "controllable state" }
						". In other case may be used "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"default_value"
						}
						"."
					}
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "value".into(),
											prop_default: "None".into(),
											prop_type: "Option<Vec<String>>".into(),
											tooltip_text: Some("Must go in pair with 'on_value_change' callback".into()),
									},
									TableBody {
											prop: "on_value_change".into(),
											prop_default: "None".into(),
											prop_type: "Callback<Vec<String>>".into(),
											tooltip_text: Some("Must go in pair with 'value' prop".into()),
									},
									TableBody {
											prop: "default_value".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<String>".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "orientation".into(),
											prop_default: "EOrientation::Vertical".into(),
											prop_type: "EOrientation::Vertical | EOrientation::Horizontal".into(),
											tooltip_text: None,
									},
									TableBody {
											prop: "disabled".into(),
											prop_default: "false".into(),
											prop_type: "bool".into(),
											tooltip_text: Some("Prevents from toggling any item in the group".into()),
									},
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'div' attribules".into()),
									},
									TableBody {
											prop: "children".into(),
											prop_default: "-".into(),
											prop_type: "Element".into(),
											tooltip_text: Some("Required".into()),
									},
							]),
						}
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Item Component" }
					p { "Indicates checkbox input." }
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "value".into(),
											prop_default: "-".into(),
											prop_type: "String".into(),
											tooltip_text: Some("Required".into()),
									},
									TableBody {
											prop: "disabled".into(),
											prop_default: "false".into(),
											prop_type: "bool".into(),
											tooltip_text: Some(
													"Prevents checkbox from selecting/deselecting current item".into(),
											),
									},
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
											tooltip_text: Some("Extends 'global' and 'button' attribules".into()),
									},
									TableBody {
											prop: "children".into(),
											prop_default: "None".into(),
											prop_type: "Element".into(),
											tooltip_text: Some("Custom checked indicator if specified".into()),
									},
							]),
						}
					}
				}
			}
		}
	}
}
