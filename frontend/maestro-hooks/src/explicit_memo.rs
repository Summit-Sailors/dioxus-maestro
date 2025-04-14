use std::{cell::RefCell, rc::Rc};

use dioxus_lib::prelude::*;

struct MemoizedSynced<TValue, TDeps> {
	value: TValue,
	deps: TDeps,
}

/// Alternative to [use_memo]
/// Benefits:
///   - No unnecessary rerenders
///
/// Downsides:
///   - T needs to be Clone (cannot be avoided)
pub fn use_explicit_memo<TValue, TDeps>(deps: TDeps, init: impl FnOnce() -> TValue) -> TValue
where
	TValue: 'static + Clone,
	TDeps: PartialEq + 'static,
{
	let value = use_hook::<Rc<RefCell<Option<MemoizedSynced<TValue, TDeps>>>>>(Rc::default);
	let mut memoized_value = value.borrow_mut();
	if memoized_value.as_ref().map(|memoized_value| &memoized_value.deps) != Some(&deps) {
		*memoized_value = Some(MemoizedSynced { value: init(), deps });
	}
	memoized_value.as_ref().unwrap().value.clone()
}
