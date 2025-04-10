use {
	crate::theme::{
		context::{THEME_ATOM, ThemeContext, set_document_theme},
		storage::get_storage,
		system::get_system_theme_detector,
		types::{ResolvedTheme, Theme},
	},
	dioxus::prelude::*,
	std::{cell::RefCell, rc::Rc},
};

#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
	pub children: Element,
	#[props(default)]
	default_theme: Option<Theme>,
}

#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
	let storage = get_storage();

	let system_theme_detector = get_system_theme_detector();

	// Initialize theme from storage or default
	let initial_theme = storage.get_theme().or_else(|| props.default_theme.clone()).unwrap_or(Theme::System);

	let initial_system_dark = system_theme_detector.prefers_dark_mode();

	let mut theme = use_signal(|| initial_theme);
	let mut system_prefers_dark = use_signal(|| initial_system_dark);
	let mut resolved_theme = use_signal(|| theme().resolve(&system_prefers_dark()));

	// Set theme when changed (Will find an alternative to RefCell later during the real implementation)
	let set_theme = Rc::new(RefCell::new(move |new_theme: Theme| {
		theme.set(new_theme.clone());
		storage.set_theme(new_theme.clone());
		resolved_theme.set(new_theme.resolve(&system_prefers_dark()));
	}));

	let theme_ctx = ThemeContext {
		theme: theme.clone(),
		resolved_theme: resolved_theme.clone(),
		system_prefers_dark: system_prefers_dark.clone(),
		set_theme: set_theme.clone(),
	};

	// persist theme to storage when it changes
	use_effect(move || {
		let theme_class = resolved_theme().as_class();
		set_document_theme(theme_class);
	});

	// apply theme to document
	use_effect(move || {
		to_owned![system_prefers_dark, theme, resolved_theme];
		system_theme_detector.listen_for_theme_changes(move |prefers_dark| {
			system_prefers_dark.set(prefers_dark);
			if them() == Theme::System {
				resolved_theme.set(theme().resolve(prefers_dark));
			}
		});
	});

	provide_context(theme_ctx);

	// Render the children with the theme
	let theme_class = resolved_theme().as_class();

	rsx! {
		div {
			class: "{theme_class}",
			props.children
		}
	}
}
