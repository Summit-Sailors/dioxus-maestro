use {async_std::task::sleep, dioxus::prelude::*, std::time::Duration};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AvatarImageLoadingStatus {
	Idle,
	Success,
	Error,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AvatarContext {
	pub status: Signal<AvatarImageLoadingStatus>,
	pub on_status_change: Option<Callback<bool>>,
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct AvatarProps {
	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Element,
	#[props(default = None)]
	pub onload: Option<Callback<bool>>,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
	use_context_provider::<AvatarContext>(|| AvatarContext { status: Signal::new(AvatarImageLoadingStatus::Idle), on_status_change: props.onload });

	rsx! {
		div { ..props.attributes,{props.children} }
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct AvatarImageProps {
	#[props(extends = GlobalAttributes, extends = image)]
	pub attributes: Vec<Attribute>,
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
	let mut avatar_context = use_context::<AvatarContext>();
	match *avatar_context.status.read() {
		AvatarImageLoadingStatus::Success => rsx! {
			img {
				onload: move |_| {
						avatar_context.status.set(AvatarImageLoadingStatus::Success);
						if let Some(callback) = avatar_context.on_status_change {
								callback.call(true);
						}
				},
				onerror: move |_| {
						avatar_context.status.set(AvatarImageLoadingStatus::Error);
						if let Some(callback) = avatar_context.on_status_change {
								callback.call(false);
						}
				},
				..props.attributes,
			}
		},
		_ => rsx! {},
	}
}

#[derive(Props, PartialEq, Debug, Clone)]
pub struct AvatarFallbackProps {
	#[props(extends = GlobalAttributes, extends = span)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Element,
	#[props(default = 0)]
	pub delayMs: u32,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
	let avatar_context = use_context::<AvatarContext>();
	let mut can_load = use_signal(|| false);
	use_effect(move || {
		spawn(async move {
			if props.delayMs > 0 {
				sleep(Duration::from_millis(props.delayMs.into())).await;
				can_load.set(true);
			}
		});
	});
	if *avatar_context.status.read() != AvatarImageLoadingStatus::Success && can_load() {
		rsx! {
			span { ..props.attributes,{props.children} }
		}
	} else {
		rsx! {}
	}
}
