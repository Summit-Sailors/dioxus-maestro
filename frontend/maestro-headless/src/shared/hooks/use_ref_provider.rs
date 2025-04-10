use {dioxus::prelude::*, std::rc::Rc};

pub fn use_ref_provider() -> Signal<Option<Rc<MountedData>>> {
	let current_ref = use_signal(|| None::<Rc<MountedData>>);

	use_context_provider::<Signal<Option<Rc<MountedData>>>>(|| current_ref)
}
