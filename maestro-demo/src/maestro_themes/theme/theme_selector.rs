use dioxus::prelude::*;
use dioxus_free_icons::{
	Icon,
	icons::bs_icons::{BsCaretDownFill, BsGearFill, BsMoon, BsSun},
};
use tailwind_fuse::tw_merge;

use crate::maestro_themes::theme::types::Theme;

#[component]
pub fn ThemeSelect() -> Element {
	let theme_ctx = crate::maestro_themes::theme::context::use_theme();
	let current_theme = theme_ctx.theme.read().clone().unwrap_or(Theme::Auto);
	let mut is_open = use_signal(|| false);

	let mut set_theme = move |new_theme: Theme| {
		theme_ctx.set_theme.call(new_theme);
		is_open.set(false);
	};

	let get_label = |theme: Theme| {
		rsx!(
      match theme {
          Theme::Light => rsx! {
            Icon { icon: BsSun, width: 16, height: 16 }
            span { "Light" }
          },
          Theme::Dark => rsx! {
            Icon { icon: BsMoon, width: 16, height: 16 }
            span { "Dark" }
          },
          Theme::Auto => rsx! {
            Icon { icon: BsGearFill, width: 16, height: 16 }
            span { "Auto" }
          },
      }
    )
	};

	rsx! {
    div { class: "relative inline-block w-24",
      button {
        class: tw_merge!(
            "w-full flex items-center justify-between p-2.5 text-sm text-left rounded-lg border shadow-sm transition",
            "bg-[color:var(--bg-color)]", "text-[color:var(--text-color)]",
            "border-gray-300 dark:border-gray-600",
            "hover:shadow focus:outline-none focus:ring-2 focus:ring-blue-500"
        ),
        onclick: move |_| {
            is_open.set(!is_open());
        },
        {get_label(current_theme)}
        Icon {
          icon: BsCaretDownFill,
          class: tw_merge!(
              "ml-2 transition-transform", if is_open() { "rotate-180" } else { "rotate-0" }
          ),
        }
      }

      {is_open().then(|| rsx! {
        div {
          class: tw_merge!(
              "absolute z-10 mt-2 w-full rounded-lg shadow-lg border",
              "bg-[color:var(--bg-color)]", "border-gray-300 dark:border-gray-600"
          ),
          button {
            class: tw_merge!(
                "flex items-center w-full px-4 py-2 text-sm text-left",
                "hover:bg-gray-100 dark:hover:bg-gray-600"
            ),
            onclick: move |_| set_theme(Theme::Light),
            Icon { icon: BsSun, width: 16, height: 16 }
            span { " Light" }
          }
          button {
            class: tw_merge!(
                "flex items-center w-full px-4 py-2 text-sm text-left",
                "hover:bg-gray-100 dark:hover:bg-gray-600"
            ),
            onclick: move |_| set_theme(Theme::Dark),
            Icon { icon: BsMoon, width: 16, height: 16 }
            span { " Dark" }
          }
          button {
            class: tw_merge!(
                "flex items-center w-full px-4 py-2 text-sm text-left",
                "hover:bg-gray-100 dark:hover:bg-gray-600"
            ),
            onclick: move |_| set_theme(Theme::Auto),
            Icon { icon: BsGearFill, width: 16, height: 16 }
            span { " Auto" }
          }
        }
      })}
    }
  }
}
