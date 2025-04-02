use {
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsInfo},
	maestro_headless::{
		popover::{Popover, PopoverArrow, PopoverContent, PopoverTrigger},
		shared::{EAlign, ESide},
	},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PropsStruct {
	pub prop: String,
	pub prop_type: String,
	pub prop_default: String,
	pub tooltip_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttrsStruct {
	pub attr: String,
	pub description: String,
	pub value: String,
}

#[derive(Clone, PartialEq, Props)]
pub struct TableProps {
	#[props(extends = table, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub content: Vec<PropsStruct>,
}

pub fn PropsTable(props: TableProps) -> Element {
	rsx! {
		table { class: "w-full overflow-auto",
			thead {
				tr { class: "border-b border-b-neutral-700 bg-neutral-800/40 *:py-2 *:not-last:border-r *:not-last:border-r-neutral-800 *:px-3 *:font-medium *:text-base",
					th { "Prop" }
					th { "Type" }
					th { "Default" }
				}
			}
			tbody { class: "*:not-last:border-b *:not-last:border-b-neutral-700",
				for PropsStruct { prop , prop_type , prop_default , tooltip_text } in props.content.iter() {
					tr { class: "*:px-3 *:py-2 *:not-last:border-r *:not-last:border-r-neutral-800",
						td { class: "flex items-center gap-2",
							"{prop}"
							if let Some(text) = tooltip_text {
								{
										rsx! {
											Popover { class: "w-fit",
												PopoverTrigger { class: "w-4 h-4 bg-neutral-900 text-orange-600 rounded-full border border-neutral-100 flex items-center justify-center focus-visible:outline-none",
													Icon { icon: BsInfo {} }
												}
												PopoverContent {
													side: ESide::Top,
													side_offset: 4.0,
													align: EAlign::Center,
													class: "z-10 data-[state=open]:opacity-100 data-[state=closed]:opacity-0 bg-neutral-700 text-neutral-100 text-xs text-center rounded-sm p-2 transition-opacity ease-linear",
													div { class: "max-w-32", "{text}" }
													PopoverArrow { width: 16.0, height: 8.0, class: "text-neutral-700" }
												}
											}
										}
								}
							}
						}
						td {
							span { class: "px-1 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-800 inline-flex items-center justify-center",
								"{prop_type}"
							}
						}
						td {
							span { class: "px-1 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-800 inline-flex items-center justify-center",
								"{prop_default}"
							}
						}
					}
				}

			}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct AttrsProps {
	#[props(extends = table, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub content: Vec<AttrsStruct>,
}

pub fn AttrsTable(props: AttrsProps) -> Element {
	rsx! {
		table { class: "w-full overflow-auto",
			thead {
				tr { class: "border-b border-b-neutral-700 bg-neutral-800/40 *:py-2 *:not-last:border-r *:not-last:border-r-neutral-800 *:px-3 *:font-medium *:text-base",
					th { "Attribute" }
					th { "Value" }
					th { "Description" }
				}
			}
			tbody { class: "*:not-last:border-b *:not-last:border-b-neutral-700",
				for AttrsStruct { attr , description, value } in props.content.iter() {
					tr { class: "*:px-3 *:py-2 *:not-last:border-r *:not-last:border-r-neutral-800",
						td { class: "flex items-center gap-2", "{attr}" }
						td {
							span { class: "px-1 font-mono text-neutral-300 font-light text-xs rounded-xs bg-neutral-800 inline-flex items-center justify-center",
								"{value}"
							}
						}
						td { class: "flex items-center gap-2", "{description}" }
					}
				}

			}
		}
	}
}
