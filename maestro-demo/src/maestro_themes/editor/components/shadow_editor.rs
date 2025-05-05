// Shadow settings editor component with color picker
use std::{fmt, str::FromStr};

use dioxus::prelude::*;

use crate::maestro_themes::editor::state::{DesignerState, ThemedesignerAction};

#[derive(Debug, Clone, PartialEq)]
struct ShadowPart {
	x_offset: i32,
	y_offset: i32,
	blur: u32,
	spread: i32,
	color: String,
	opacity: u8,
}

impl ShadowPart {
	fn shadow_string(&self) -> String {
		format!(
			"{}px {}px {}px {}px rgb({} / {}%)",
			self.x_offset,
			self.y_offset,
			self.blur,
			self.spread,
			self.color.trim_start_matches("rgb(").trim_end_matches(')'),
			self.opacity
		)
	}

	fn from_string(s: &str) -> Option<Self> {
		// match pattern like: "0 1px 2px 0 rgb(0 0 0 / 0.05)"
		let s = s.trim();

		// basic shadow components parsing
		let parts: Vec<&str> = s.split_whitespace().collect();
		if parts.len() < 4 {
			return None;
		}

		// x offset
		let x_offset = if parts[0].ends_with("px") { i32::from_str(&parts[0][..parts[0].len() - 2]).ok()? } else { i32::from_str(parts[0]).ok()? };

		// y offset
		let y_offset = if parts[1].ends_with("px") { i32::from_str(&parts[1][..parts[1].len() - 2]).ok()? } else { i32::from_str(parts[1]).ok()? };

		// blur radius
		let blur_str = parts[2];
		let blur = if let Some(s) = blur_str.strip_suffix("px") { u32::from_str(s).ok()? } else { u32::from_str(blur_str).ok()? };

		// spread radius
		let spread_str = parts[3];
		let spread = if let Some(s) = spread_str.strip_suffix("px") { i32::from_str(s).ok()? } else { i32::from_str(spread_str).ok()? };

		// extract color
		let color = if s.contains("rgb(") {
			let start = s.find("rgb(").unwrap_or(0);
			let end = s[start..].find(" /").map(|e| start + e).unwrap_or_else(|| s.find(")").map(|e| e + 1).unwrap_or(s.len()));
			s[start..end].to_string()
		} else {
			"rgb(0 0 0)".to_string() // default color
		};

		// opacity from color
		let opacity_str = if s.contains("/ 0.") {
			// find the opacity value like "0.05"
			let start = s.find("/ 0.").unwrap_or(0) + 3;
			let end = s[start..].find(")").map(|e| start + e).unwrap_or(s.len());
			let opacity_float = f32::from_str(&s[start..end]).unwrap_or(0.05);
			(opacity_float * 100.0) as u8
		} else if s.contains("/ ") {
			// find the opacity percentage like "5%"
			let start = s.find("/ ").unwrap_or(0) + 2;
			let end = s[start..].find("%").map(|e| start + e).unwrap_or(s.len());
			u8::from_str(&s[start..end]).unwrap_or(5)
		} else {
			5 // default 5% opacity
		};

		Some(Self { x_offset, y_offset, blur, spread, color, opacity: opacity_str })
	}
}

impl fmt::Display for ShadowPart {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.shadow_string())
	}
}

fn parse_shadow_string(shadow: &str) -> Vec<ShadowPart> {
	// split multiple shadows
	let shadow_parts: Vec<&str> = shadow.split(',').map(|s| s.trim()).collect();

	shadow_parts.iter().filter_map(|part| ShadowPart::from_string(part)).collect()
}

fn build_shadow_string(parts: &[ShadowPart]) -> String {
	parts.iter().map(|part| part.to_string()).collect::<Vec<String>>().join(", ")
}

// Hex to RGB
fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
	let hex = hex.trim_start_matches('#');

	if hex.len() != 6 {
		return None;
	}

	let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
	let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
	let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

	Some((r, g, b))
}

// RGB to Hex
fn rgb_to_hex(rgb: &str) -> Option<String> {
	// RGB values from format like "rgb(0 0 0)"
	let content = rgb.trim_start_matches("rgb(").trim_end_matches(')');
	let parts: Vec<&str> = content.split_whitespace().collect();

	if parts.len() != 3 {
		return None;
	}

	let r = u8::from_str(parts[0]).ok()?;
	let g = u8::from_str(parts[1]).ok()?;
	let b = u8::from_str(parts[2]).ok()?;

	Some(format!("#{:02x}{:02x}{:02x}", r, g, b))
}

#[derive(Props, PartialEq, Clone)]
struct ShadowPartEditorProps {
	pub label: String,
	pub part: ShadowPart,
	pub index: usize,
	pub on_change: EventHandler<(usize, ShadowPart)>,
	pub on_add: EventHandler<()>,
	pub on_remove: EventHandler<usize>,
	pub can_remove: bool,
}

#[component]
fn NumberInput(label: String, value: i32, min: i32, max: i32, step: i32, on_change: EventHandler<i32>) -> Element {
	rsx! {
		div { class: "flex flex-col",
			label { class: "text-xs text-gray-500 mb-1", "{label}" }
			div { class: "flex items-center",
				input {
					class: "w-16 h-8 border rounded px-2 text-sm",
					r#type: "number",
					value: "{value}",
					min: "{min}",
					max: "{max}",
					step: "{step}",
					oninput: move |evt| {
							if let Ok(val) = i32::from_str(&evt.value()) {
									on_change.call(val);
							}
					},
				}
			}
		}
	}
}

#[component]
fn SliderInput(label: String, value: u8, min: u8, max: u8, on_change: EventHandler<u8>) -> Element {
	rsx! {
		div { class: "flex flex-col",
			label { class: "text-xs text-gray-500 mb-1", "{label}" }
			div { class: "flex items-center gap-2",
				input {
					class: "flex-1",
					r#type: "range",
					min: "{min}",
					max: "{max}",
					value: "{value}",
					oninput: move |evt| {
							if let Ok(val) = u8::from_str(&evt.value()) {
									on_change.call(val);
							}
					},
				}
				span { class: "w-8 text-sm", "{value}%" }
			}
		}
	}
}

#[component]
fn ColorInput(label: String, color: String, on_change: EventHandler<String>) -> Element {
	let hex_color = use_memo(move || rgb_to_hex(&color).unwrap_or_else(|| "#000000".to_string()));

	rsx! {
		div { class: "flex flex-col",
			label { class: "text-xs text-gray-500 mb-1", "{label}" }
			div { class: "flex items-center gap-2",
				input {
					class: "h-8 cursor-pointer",
					r#type: "color",
					value: "{hex_color}",
					oninput: move |evt| {
							let hex_value = evt.value();
							if let Some((r, g, b)) = hex_to_rgb(&hex_value) {
									let rgb_color = format!("rgb({} {} {})", r, g, b);
									on_change.call(rgb_color);
							}
					},
				}
				span { class: "text-xs font-mono", "{hex_color}" }
			}
		}
	}
}

#[component]
fn ShadowPartEditor(props: ShadowPartEditorProps) -> Element {
	let mut part = use_signal(|| props.part.clone());

	rsx! {
		div { class: "p-4 border rounded-md bg-[color:var(--card-bg)] mb-4",
			div { class: "flex justify-between items-center mb-4",
				h4 { class: "font-medium text-sm", "{props.label}" }
				div { class: "flex gap-2",
					button {
						class: "text-sm px-2 py-1 bg-[color:var(--accent-bg)] text-[color:var(--accent-text)] rounded hover:opacity-90",
						onclick: move |_| props.on_add.call(()),
						"Add Layer"
					}
					button {
						class: "text-sm px-2 py-1 bg-[color:var(--destructive)] text-[color:var(--destructive-foreground)] rounded hover:opacity-90",
						disabled: "{!props.can_remove}",
						onclick: move |_| props.on_remove.call(props.index),
						style: if !props.can_remove { "opacity: 0.5; cursor: not-allowed;" } else { "" },
						"Remove"
					}
				}
			}

			div { class: "mb-4 p-4 bg-[color:var(--muted)] rounded flex justify-center items-center",
				div {
					style: "box-shadow: {part().to_string()};",
					class: "shadow-preview w-20 h-20 bg-[color:var(--muted)] rounded",
				}
			}

			div { class: "grid grid-cols-2 md:grid-cols-3 gap-4 mb-4",
				NumberInput {
					label: "X Offset (px)".to_string(),
					value: part().x_offset,
					min: -50,
					max: 50,
					step: 1,
					on_change: move |val| {
							part.with_mut(|p| p.x_offset = val);
							props.on_change.call((props.index, part()));
					},
				}

				NumberInput {
					label: "Y Offset (px)".to_string(),
					value: part().y_offset,
					min: -50,
					max: 50,
					step: 1,
					on_change: move |val| {
							part.with_mut(|p| p.y_offset = val);
							props.on_change.call((props.index, part()));
					},
				}

				NumberInput {
					label: "Blur (px)".to_string(),
					value: part().blur as i32,
					min: 0,
					max: 100,
					step: 1,
					on_change: move |val: i32| {
							part.with_mut(|p| p.blur = val.max(0) as u32);
							props.on_change.call((props.index, part()));
					},
				}

				NumberInput {
					label: "Spread (px)".to_string(),
					value: part().spread,
					min: -25,
					max: 50,
					step: 1,
					on_change: move |val| {
							part.with_mut(|p| p.spread = val);
							props.on_change.call((props.index, part()));
					},
				}

				ColorInput {
					label: "Color".to_string(),
					color: part().color.clone(),
					on_change: move |val| {
							part.with_mut(|p| p.color = val);
							props.on_change.call((props.index, part()));
					},
				}

				SliderInput {
					label: "Opacity".to_string(),
					value: part().opacity,
					min: 0,
					max: 100,
					on_change: move |val| {
							part.with_mut(|p| p.opacity = val);
							props.on_change.call((props.index, part()));
					},
				}
			}
			div { class: "text-xs text-gray-500 mt-2",
				"Preview: "
				code { class: "font-mono bg-[color:var(--muted)] px-1 py-0.5 rounded text-xs break-all",
					"{part().to_string()}"
				}
			}
		}
	}
}

#[derive(Props, PartialEq, Clone)]
struct ShadowSizeEditorProps {
	pub label: String,
	pub size_key: String,
	pub value: String,
}

#[component]
fn ShadowSizeEditor(props: ShadowSizeEditorProps) -> Element {
	let mut shadow_parts = use_signal(|| parse_shadow_string(&props.value));
	let mut state = use_context::<Signal<DesignerState>>();

	let size_key = use_signal(|| props.size_key.clone());

	let mut update_shadow_part = move |index: usize, part: ShadowPart| {
		let mut parts = shadow_parts();
		if index < parts.len() {
			parts[index] = part;
			shadow_parts.set(parts.clone());

			let shadow_string = build_shadow_string(&parts);
			state.with_mut(|s| s.apply_action(ThemedesignerAction::UpdateShadow { key: size_key(), value: shadow_string }));
		}
	};

	let add_shadow_part = move |_| {
		let mut parts = shadow_parts();
		// a new part with the current theme's text color (defaulting to black)
		parts.push(ShadowPart { x_offset: 0, y_offset: 4, blur: 8, spread: 0, color: "rgb(0 0 0)".to_string(), opacity: 5 });
		shadow_parts.set(parts.clone());

		let shadow_string = build_shadow_string(&parts);
		state.with_mut(|s| s.apply_action(ThemedesignerAction::UpdateShadow { key: size_key(), value: shadow_string }));
	};

	let remove_shadow_part = move |index: usize| {
		let mut parts = shadow_parts();
		if parts.len() > 1 && index < parts.len() {
			parts.remove(index);
			shadow_parts.set(parts.clone());

			let shadow_string = build_shadow_string(&parts);
			state.with_mut(|s| s.apply_action(ThemedesignerAction::UpdateShadow { key: size_key(), value: shadow_string }));
		}
	};

	rsx! {
		div { class: "shadow-size-editor mb-6",
			h4 { class: "text-base font-medium mb-2", "{props.label}" }

			div { class: "shadow-parts space-y-2 ",
				{
						shadow_parts
								.iter()
								.enumerate()
								.map(|(i, part)| {
										rsx! {
											ShadowPartEditor {
												label: format!("Layer {}", i + 1),
												part: part.clone(),
												index: i,
												on_change: move |(idx, part)| update_shadow_part(idx, part),
												on_add: add_shadow_part,
												on_remove: remove_shadow_part,
												can_remove: shadow_parts.read().len() > 1,
											}
										}
								})
				}
			}

			div { class: "mt-2 text-xs text-gray-500",
				"Combined shadow: "
				code { class: "font-mono bg-[color:var(--muted)] px-1 py-0.5 rounded text-xs overflow-auto max-w-full block",
					"{build_shadow_string(&shadow_parts.read())}"
				}
			}
		}
	}
}

#[component]
pub fn ShadowEditor() -> Element {
	let state = use_context::<Signal<DesignerState>>();

	rsx! {
		div { class: "shadow-editor p-4 bg-[color:var(--card-bg)] rounded-lg",
			h3 { class: "text-lg font-medium mb-4 pb-2 border-b", "Shadow Settings" }

			div { class: "shadow-preview-grid mb-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
				div { class: "shadow-preview-item  flex flex-col items-center justify-center p-4 bg-[color:var(--muted)] text-[color:var(--text-color)] rounded-md",
					div { class: "text-sm font-medium mb-2", "Small (sm)" }
					div {
						class: "shadow-preview w-16 h-16 bg-[color:var(--muted)] rounded-md",
						style: "box-shadow: {state().shadow.sm};",
					}
				}

				div { class: "shadow-preview-item flex flex-col items-center justify-center p-4 bg-[color:var(--muted)] text-[color:var(--text-color)] rounded-md",
					div { class: "text-sm font-medium mb-2", "Medium (md)" }
					div {
						class: "shadow-preview w-16 h-16 bg-[color:var(--muted)] rounded-md",
						style: "box-shadow: {state().shadow.md};",
					}
				}

				div { class: "shadow-preview-item flex flex-col items-center justify-center p-4 bg-[color:var(--muted)] text-[color:var(--text-color)] rounded-md",
					div { class: "text-sm font-medium mb-2 text-[color:var(--text-color)]",
						"Large (lg)"
					}
					div {
						class: "shadow-preview w-16 h-16 bg-[color:var(--muted)] rounded-md",
						style: "box-shadow: {state().shadow.lg};",
					}
				}

				div { class: "shadow-preview-item flex flex-col items-center justify-center p-4 bg-[color:var(--muted)] text-[color:var(--text-color)] rounded-md",
					div { class: "text-sm font-medium mb-2", "Extra Large (xl)" }
					div {
						class: "shadow-preview w-16 h-16 bg-[color:var(--muted)] rounded-md",
						style: "box-shadow: {state().shadow.xl};",
					}
				}

				div { class: "shadow-preview-item flex flex-col items-center justify-center p-4 bg-[color:var(--muted)] text-[color:var(--text-color)] rounded-md",
					div { class: "text-sm font-medium mb-2", "2x Large (xxl)" }
					div {
						class: "shadow-preview w-16 h-16 bg-[color:var(--muted)] rounded-md",
						style: "box-shadow: {state().shadow.xxl};",
					}
				}
			}

			details { class: "shadow-editor-details mb-4 border rounded-md",
				summary { class: "p-3 font-medium text-sm cursor-pointer hover:bg-[color:var(--hover-bg)]",
					"Small Shadow (sm)"
				}
				div { class: "p-4 border-t",
					ShadowSizeEditor {
						label: "Small Shadow".to_string(),
						size_key: "sm".to_string(),
						value: state().shadow.sm.clone(),
					}
				}
			}

			details { class: "shadow-editor-details mb-4 border rounded-md",
				summary { class: "p-3 font-medium text-sm cursor-pointer hover:bg-[color:var(--hover-bg)]",
					"Medium Shadow (md)"
				}
				div { class: "p-4 border-t",
					ShadowSizeEditor {
						label: "Medium Shadow".to_string(),
						size_key: "md".to_string(),
						value: state().shadow.md.clone(),
					}
				}
			}

			details { class: "shadow-editor-details mb-4 border rounded-md",
				summary { class: "p-3 font-medium text-sm cursor-pointer hover:bg-[color:var(--hover-bg)]",
					"Large Shadow (lg)"
				}
				div { class: "p-4 border-t",
					ShadowSizeEditor {
						label: "Large Shadow".to_string(),
						size_key: "lg".to_string(),
						value: state().shadow.lg.clone(),
					}
				}
			}

			details { class: "shadow-editor-details mb-4 border rounded-md",
				summary { class: "p-3 font-medium text-sm cursor-pointer hover:bg-[color:var(--hover-bg)]",
					"Extra Large Shadow (xl)"
				}
				div { class: "p-4 border-t",
					ShadowSizeEditor {
						label: "Extra Large Shadow".to_string(),
						size_key: "xl".to_string(),
						value: state().shadow.xl.clone(),
					}
				}
			}

			details { class: "shadow-editor-details mb-4 border rounded-md",
				summary { class: "p-3 font-medium text-sm cursor-pointer hover:bg-[color:var(--hover-bg)]",
					"2x Large Shadow (xxl)"
				}
				div { class: "p-4 border-t",
					ShadowSizeEditor {
						label: "2x Large Shadow".to_string(),
						size_key: "xxl".to_string(),
						value: state().shadow.xxl.clone(),
					}
				}
			}
		}
	}
}
