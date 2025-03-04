use {dioxus::prelude::*, tailwind_fuse::tw_merge};

#[component]
pub fn Backdrop(show: Signal<bool>) -> Element {
	rsx! {
		div {
			class: tw_merge!(
					"fixed inset-0 bg-slate-900/30 backdrop-blur-sm -z-10 transition-all duration-300",
					if show() { "opacity-100 z-40" } else { "pointer-events-none" }
			),
			onclick: move |_| show.set(false),
		}
	}
}
