use dioxus::prelude::*;
use dioxus_free_icons::{
	Icon,
	icons::bs_icons::{BsGearFill, BsMoon, BsSun},
};
use tailwind_fuse::tw_merge;

use crate::maestro_themes::theme::types::Theme;

#[component]
pub fn ThemeSelect() -> Element {
	let theme_ctx = crate::maestro_themes::theme::context::use_theme();
	let current_theme = (*theme_ctx.theme.read()).unwrap_or(Theme::Auto);

	let cycle_theme = move |_| {
		let new_theme = match current_theme {
			Theme::Light => Theme::Dark,
			Theme::Dark => Theme::Auto,
			Theme::Auto => Theme::Light,
		};
		theme_ctx.set_theme.call(new_theme);
	};

	let get_theme_content = |theme: Theme| match theme {
		Theme::Light => rsx! {
			Icon {
				icon: BsSun,
				width: 16,
				height: 16,
				class: "text-gray-900",
			}
			span { class: "ml-2", "Light" }
		},
		Theme::Dark => rsx! {
			Icon {
				icon: BsMoon,
				width: 16,
				height: 16,
				class: "text-blue-500",
			}
			span { class: "ml-2", "Dark" }
		},
		Theme::Auto => rsx! {
			Icon {
				icon: BsGearFill,
				width: 16,
				height: 16,
				class: "text-gray-500",
			}
			span { class: "ml-2", "Auto" }
		},
	};

	let get_theme_label = |theme: Theme| match theme {
		Theme::Light => "Light Mode",
		Theme::Dark => "Dark Mode",
		Theme::Auto => "Auto Theme",
	};

	rsx! {
		button {
			class: tw_merge!(
					"flex items-center justify-center p-2 rounded-full transition-all duration-200",
					"hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500",
					"text-[color:var(--text-color)] bg-[color:var(--bg-secondary-color)]"
			),
			onclick: cycle_theme,
			title: get_theme_label(current_theme),
			aria_label: get_theme_label(current_theme),
			{get_theme_content(current_theme)}
		}
	}
}
