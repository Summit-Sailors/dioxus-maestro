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
	#[props(default = None)]
	pub onload: Option<Callback<bool>>,

	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Element,
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
	let mut context = use_context::<AvatarContext>();
	match *context.status.read() {
		AvatarImageLoadingStatus::Success => rsx! {
			img {
				onload: move |_| {
						context.status.set(AvatarImageLoadingStatus::Success);
						if let Some(callback) = context.on_status_change {
								callback.call(true);
						}
				},
				onerror: move |_| {
						context.status.set(AvatarImageLoadingStatus::Error);
						if let Some(callback) = context.on_status_change {
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
	#[props(default = 0)]
	pub delayMs: u32,

	#[props(extends = GlobalAttributes, extends = span)]
	pub attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Element,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
	let context = use_context::<AvatarContext>();
	let mut can_load = use_signal(|| false);

	use_effect(move || {
		spawn(async move {
			if props.delayMs > 0 {
				sleep(Duration::from_millis(props.delayMs.into())).await;
				can_load.set(true);
			}
		});
	});

	if *context.status.read() != AvatarImageLoadingStatus::Success && can_load() {
		rsx! {
			span { ..props.attributes,{props.children} }
		}
	} else {
		rsx! {}
	}
}
