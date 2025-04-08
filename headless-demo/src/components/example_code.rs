use {
	dioxus::prelude::*,
	maestro_headless::collapsible::{CollapsibleRoot, CollapsibleContent, CollapsibleTrigger},
};

#[component]
pub fn ExampleCodeCollapsible(code: &'static str) -> Element {
	let mut open = use_signal(|| false);
	rsx! {
		div { class: "py-3 w-full px-6 border-t border-neutral-800 bg-neutral-950 overflow-hidden",
			CollapsibleRoot {
				open: open(),
				on_open_change: move |current_open| open.set(current_open),
				class: "flex flex-col  max-h-[640px] h-full ",
				div { class: "flex items-center gap-3 py-3",
					CollapsibleTrigger { class: "flex items-center justify-center px-3 py-2 font-medium rounded bg-orange-600 border-2 border-transparent hover:border-orange-600 text-neutral-50 hover:bg-neutral-950 focus-visible:ring-2 ring-orange-600 ring-offset-neutral-950 focus-visible:ring-offset-2 focus-visible:outline-none transition-colors ease-linear",
						if open() {
							"Collapse"
						} else {
							"Expand Code"
						}
					}
				}
				CollapsibleContent { class: "data-[state=closed]:animate-slide-out data-[state=open]:animate-slide-in overflow-auto",
					code { class: "font-mono whitespace-pre text-xs text-neutral-300",
						pre { "{code}" }
					}
				}
			}
		}
	}
}

#[component]
pub fn ExampleCodeAnatomy(code: &'static str) -> Element {
	rsx! {
		div { class: "grow flex flex-col rounded-md border border-neutral-800 bg-neutral-950 p-6",
			code { class: "font-mono whitespace-pre text-xs text-neutral-300",
				pre { "{code}" }
			}
		}
	}
}
