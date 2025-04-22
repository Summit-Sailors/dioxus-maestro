use dioxus::prelude::*;

use crate::components::maestro_themes::theme::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
	pub children: Element,
	#[props(default)]
	default_theme: Option<Theme>,
}

#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
	#[cfg(feature = "server")]
	return rsx! {
		div { {props.children} }
	};

	#[cfg(all(any(feature = "web", feature = "desktop"), not(feature = "server")))]
	{
		let storage = get_storage();
		let system_theme_detector = get_system_theme_detector();

		let initial_theme = storage.get_theme().or_else(|_| Ok::<Option<Theme>, ThemeStorageError>(props.default_theme.clone())).unwrap_or(Some(Theme::Auto));

		let initial_system_dark = system_theme_detector.prefers_dark_mode();

		let mut theme = use_signal(|| initial_theme);
		let system_prefers_dark = use_signal(|| initial_system_dark);
		let mut resolved_theme = use_signal(|| theme().as_ref().map(|t| t.resolve(system_prefers_dark())).unwrap_or(ResolvedTheme::Dark));

		let set_theme = Callback::new(move |new_theme: Theme| {
			theme.set(Some(new_theme.clone()));
			let _ = storage.set_theme(&new_theme);
			resolved_theme.set(new_theme.resolve(system_prefers_dark()));
		});

		provide_context(ThemeContext { theme, resolved_theme, system_prefers_dark, set_theme });

		use_effect(move || {
			let theme_class = resolved_theme().to_string().to_lowercase();
			crate::components::maestro_themes::theme::context::set_document_theme(theme_class.as_str());
		});

		use_effect(move || {
			to_owned![system_prefers_dark, theme, resolved_theme];

			system_theme_detector.listen_for_theme_changes(Box::new(move |prefers_dark| {
				system_prefers_dark.set(prefers_dark);

				if let Some(t) = theme() {
					if t == Theme::Auto {
						resolved_theme.set(t.resolve(prefers_dark));
					}
				}
			}));
		});

		let theme_class = resolved_theme().to_string();

		rsx! {
			div { "data-theme": "{theme_class}", {props.children} }
		}
	}
}
