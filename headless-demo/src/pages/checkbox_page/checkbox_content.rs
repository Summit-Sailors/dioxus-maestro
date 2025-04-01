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
pub fn CheckboxContent() -> Element {
	rsx! {
		p { class: "container flex flex-col gap-3 lg:py-6 py-4 text-neutral-300",
			"A control that allows the user to toggle between checked and not checked."
		}
		div { class: "grow flex flex-col justify-center items-center overflow-hidden rounded-md border border-neutral-800 bg-neutral-950",
			div { class: "p-6 flex flex-col gap-4 items-start",
				div { class: "flex justify-center items-center gap-3",
					Checkbox {
						id: "maestro-box",
						class: "w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none",
						value: "some",
						name: "box",
						CheckboxIndicator { class: "text-neutral-100 " }
					}
					label { class: "text-slate-100", r#for: "maestro-box", "Check Me" }
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
use maestro_headless::checkbox::{{Checkbox, CheckboxIndicator}};

rsx! {{
div {{ 
  class: 'flex justify-center items-center gap-3',
  Checkbox {{
    id: 'maestro-box',
    class: 'w-6 h-6 rounded flex items-center justify-center border border-neutral-100 transition-colors hover:border-orange-600 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none',
    value: 'some',
    name: 'box',
    CheckboxIndicator {{ class: 'text-neutral-100' }}
  }}
  label {{ 
    class: 'text-slate-100', r#for: 'maestro-box', 'Check Me' 
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
			}
		}
		DescriptionSection {
			title: "Usage and Anatomy",
			description: "Import all parts and piece them together. Each part may be styled separately, accept own properties and additional attributes, e.g. \"data\" or \"aria\" (althought they are provided by default).",
			div { class: "grow flex flex-col rounded-md border border-neutral-800 bg-neutral-950 p-6",
				code { class: "font-mono whitespace-pre text-xs text-neutral-300",
					pre { "Checkbox {{
	CheckboxIndicator {{ }}
}}" }
				}
			}
		}
		DescriptionSection { title: "Api Reference",
			div { class: "flex flex-col space-y-4",
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Root Component" }
					p {
						"Wrapps all parts af the accordion and manages state. Contains all the parts of a checkbox. Props "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"checked"
						}
						" and "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"on_change"
						}
						span { class: "font-medium", "must go in pair" }
						"if use "
						span { class: "font-medium", "controllable state" }
						". In other case may be used "
						span { class: "px-1.5 py-0.5 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-600 inline-flex items-center justify-center",
							"default_checked"
						}
						"."
					}
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "checked".into(),
											prop_default: "None".into(),
											prop_type: "Option<bool>".into(),
									},
									TableBody {
											prop: "on_value_change".into(),
											prop_default: "None".into(),
											prop_type: "Callback<bool>".into(),
									},
									TableBody {
											prop: "default_checked".into(),
											prop_default: "false".into(),
											prop_type: "bool".into(),
									},
									TableBody {
											prop: "disabled".into(),
											prop_default: "false".into(),
											prop_type: "bool".into(),
									},
									TableBody {
											prop: "required".into(),
											prop_default: "false".into(),
											prop_type: "bool".into(),
									},
									TableBody {
											prop: "name".into(),
											prop_default: "-".into(),
											prop_type: "String".into(),
									},
									TableBody {
											prop: "value".into(),
											prop_default: "-".into(),
											prop_type: "String".into(),
									},
									TableBody {
											prop: "extra_attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
									},
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
									},
									TableBody {
											prop: "children".into(),
											prop_default: "-".into(),
											prop_type: "Element".into(),
									},
							]),
						}
					}
				}
				div { class: "flex flex-col gap-4",
					h4 { class: "font-medium text-lg text-orange-300", "Indicator Component" }
					p { "Appears when the checkbox is checked. May accept checked component." }
					div { class: "overflow-hidden rounded-sm border border-neutral-700",
						PropsTable {
							content: Vec::from([
									TableBody {
											prop: "attributes".into(),
											prop_default: "[]".into(),
											prop_type: "Vec<Attribute>".into(),
									},
									TableBody {
											prop: "children".into(),
											prop_default: "-".into(),
											prop_type: "Element".into(),
									},
							]),
						}
					}
				}
			}
		}
	}
}
