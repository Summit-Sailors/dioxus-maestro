use {
	async_std::task::sleep,
	dioxus::prelude::*,
	std::time::Duration,
	web_sys::{
		HtmlImageElement,
		wasm_bindgen::{JsCast, closure::Closure},
		window,
	},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ImageLoadingStatus {
	Idle,
	Loading,
	Loaded,
	Error,
}

#[derive(Clone, Debug, PartialEq)]
struct AvatarContextValue {
	image_loading_status: Signal<ImageLoadingStatus>,
}

#[derive(Props, PartialEq, Clone)]
pub struct AvatarRootProps {
	#[props(extends = GlobalAttributes, extends = div)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	pub children: Element,
}

#[component]
pub fn AvatarRoot(props: AvatarRootProps) -> Element {
	let image_loading_status = use_signal(|| ImageLoadingStatus::Idle);
	use_context_provider::<AvatarContextValue>(|| AvatarContextValue { image_loading_status });

	rsx! {
		div { ..props.attributes, ..props.extra_attributes,{props.children} }
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct AvatarImageProps {
	#[props(default = ReadOnlySignal::new(Signal::new(String::new())))]
	pub src: ReadOnlySignal<String>,
	#[props(extends = GlobalAttributes, extends = img)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
	let mut context = use_context::<AvatarContextValue>();
	let mut onload_ref = use_signal(|| None::<Closure<dyn FnMut()>>);
	let mut onerror_ref = use_signal(|| None::<Closure<dyn FnMut()>>);

	use_effect(move || {
		if props.src.read().clone().is_empty() {
			context.image_loading_status.set(ImageLoadingStatus::Error);
			return;
		}

		context.image_loading_status.set(ImageLoadingStatus::Loading);

		let image = window()
			.expect("Window should exist.")
			.document()
			.expect("Document should exist.")
			.create_element("img")
			.map(|element| element.unchecked_into::<HtmlImageElement>())
			.expect("Image element should be created.");

		if onload_ref.peek().is_none() {
			let onload = Closure::wrap(Box::new(move || {
				context.image_loading_status.set(ImageLoadingStatus::Loaded);
			}) as Box<dyn FnMut()>);
			onload_ref.set(Some(onload));
		}

		if onerror_ref.peek().is_none() {
			let onerror = Closure::wrap(Box::new(move || {
				context.image_loading_status.set(ImageLoadingStatus::Loaded);
			}) as Box<dyn FnMut()>);
			onerror_ref.set(Some(onerror));
		}

		if let (Some(load_closure), Some(error_closure)) = (&*onerror_ref.read(), &*onload_ref.read()) {
			image.set_onload(Some(load_closure.as_ref().unchecked_ref()));
			image.set_onerror(Some(error_closure.as_ref().unchecked_ref()));
			image.set_src(&*props.src.peek());
		}
	});

	if *context.image_loading_status.read() == ImageLoadingStatus::Loaded {
		rsx! {
			img {
				src: props.src.read().clone(),
				..props.attributes,
				..props.extra_attributes,
			}
		}
	} else {
		rsx! {}
	}
}

#[derive(Props, PartialEq, Clone)]
pub struct AvatarFallbackProps {
	#[props(default = None)]
	pub delay_ms: Option<u32>,

	#[props(extends = GlobalAttributes, extends = span)]
	pub attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	#[props(default = None)]
	pub children: Element,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
	let context = use_context::<AvatarContextValue>();

	let mut can_render = use_signal(|| props.delay_ms.is_none());

	use_effect(move || {
		if let Some(delay_ms) = props.delay_ms {
			spawn(async move {
				sleep(Duration::from_millis(delay_ms.into())).await;
				can_render.set(true);
			});
		}
	});

	if can_render() && *context.image_loading_status.read() != ImageLoadingStatus::Loaded {
		rsx! {
			span { ..props.attributes, ..props.extra_attributes,{props.children} }
		}
	} else {
		rsx! {}
	}
}
