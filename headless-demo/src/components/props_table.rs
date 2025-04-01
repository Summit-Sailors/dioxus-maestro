use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TableBody {
	pub prop: String,
	pub prop_type: String,
	pub prop_default: String,
}

#[derive(Clone, PartialEq, Props)]
pub struct TableProps {
	#[props(extends = table, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	pub content: Vec<TableBody>,
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
				for TableBody { prop , prop_type , prop_default } in props.content.iter() {
					tr { class: "*:px-3 *:py-2 *:not-last:border-r *:not-last:border-r-neutral-800",
						td { "{prop}" }
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
