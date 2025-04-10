use {
	dioxus::{prelude::*, web::WebEventExt},
	dioxus_logger::tracing::info,
	std::rc::Rc,
	web_sys::window,
};

#[derive(Props, PartialEq, Clone)]
pub struct PortalProps {
	#[props(extends = div, extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	#[props(default = Vec::new())]
	pub extra_attributes: Vec<Attribute>,
	children: Element,
}

#[component]
pub fn Portal(props: PortalProps) -> Element {
	let PortalProps { attributes, extra_attributes, children } = props;

	let mut attrs = attributes.clone();
	attrs.extend(extra_attributes);
	let mut current_ref = use_context::<Signal<Option<Rc<MountedData>>>>();
	// let mut current_ref = use_signal(|| None::<Rc<MountedData>>);

	info!("REF {:?}", current_ref());
	use_effect(move || {
		info!("Effect");
		if let Some(element) = current_ref().and_then(|x| x.try_as_web_event()) {
			info!("Effect IF");
			let window = window().expect("No global window exists");
			let document = window.document().expect("No document exists");
			document.body().expect("No body element exists").append_child(&element).expect("Failed to append portal container");
		}
	});

	use_drop(move || {
		info!("Drop");
		if let Some(container) = current_ref().and_then(|x| x.try_as_web_event()) {
			if let Some(parent) = container.parent_node() {
				info!("Drop if");
				parent.remove_child(&container).ok();
			}
		}
	});

	rsx! {
		div {
			onmounted: move |event| { current_ref.set(Some(event.data())) },
			..attrs,
			{children}
		}
	}
}
