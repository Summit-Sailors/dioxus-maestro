use {
	dioxus_lib::prelude::*,
	futures_util::Future,
	std::{fmt::Debug, mem, sync::Arc},
};

pub type MutationFn<T, E, A> = dyn Fn(A) -> Box<dyn Future<Output = MutationResult<T, E>>>;

pub struct UseMutation<T, E, A>
where
	T: 'static,
	E: 'static,
	A: 'static,
{
	value: Signal<MutationResult<T, E>>,
	mutation_fn: Signal<Arc<Box<MutationFn<T, E, A>>>>,
	scheduler: Signal<Arc<dyn Fn(ScopeId)>>,
	scope_id: ScopeId,
}

impl<T, E, A> Clone for UseMutation<T, E, A> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T, E, A> Copy for UseMutation<T, E, A> {}

impl<T, E, A> UseMutation<T, E, A>
where
	T: 'static,
	E: 'static,
{
	pub fn result(&self) -> ReadableRef<Signal<MutationResult<T, E>>> {
		self.value.peek()
	}

	async fn inner_mutate(
		arg: A,
		mut value: Signal<MutationResult<T, E>>,
		scheduler: Signal<Arc<dyn Fn(ScopeId)>>,
		scope_id: ScopeId,
		mutation_fn: Signal<Arc<Box<MutationFn<T, E, A>>>>,
	) {
		// Set state to loading and notify
		value.write().set_loading();

		// TODO optimization: Check if the value was already loading
		// to decide to call the scheduler or not
		(scheduler.peek())(scope_id);

		// Trigger the mutation function
		let fut = (mutation_fn.peek())(arg);
		let fut = Box::into_pin(fut);
		let new_value = fut.await;

		// Set state to the new value and notify
		value.set(new_value);

		// TODO optimization: Check if the previous and new value are
		// different to decide to call the scheduler or not
		(scheduler.peek())(scope_id);
	}

	async fn inner_silent_mutate(arg: A, mut value: Signal<MutationResult<T, E>>, mutation_fn: Signal<Arc<Box<MutationFn<T, E, A>>>>) {
		value.write().set_loading();

		// Trigger the mutation function
		let fut = (mutation_fn.peek())(arg);
		let fut = Box::into_pin(fut);
		let new_value = fut.await;

		// Set state to the new value
		value.set(new_value);
	}

	/// Call the mutation function with a set of arguments, in the **background**.
	pub fn mutate(&self, arg: A)
	where
		T: 'static,
		E: 'static,
		A: 'static,
	{
		let value = self.value;
		let scheduler = self.scheduler;
		let scope_id = self.scope_id;
		let mutation_fn = self.mutation_fn;
		spawn(async move { Self::inner_mutate(arg, value, scheduler, scope_id, mutation_fn).await });
	}

	/// Call the mutation function with a set of arguments.
	pub async fn manual_mutate(&self, arg: A) {
		Self::inner_mutate(arg, self.value, self.scheduler, self.scope_id, self.mutation_fn).await;
	}

	/// Call the mutation function silently with a set of arguments, in the **background**.
	/// This will not make the component re run.
	pub async fn mutate_silent(&self, arg: A)
	where
		T: 'static,
		E: 'static,
		A: 'static,
	{
		let value = self.value;
		let mutation_fn = self.mutation_fn;
		spawn(async move {
			Self::inner_silent_mutate(arg, value, mutation_fn).await;
		});
	}

	/// Call the mutation function silently with a set of arguments.
	/// This will not make the component re run.
	pub async fn manual_mutate_silent(&self, arg: A) {
		Self::inner_silent_mutate(arg, self.value, self.mutation_fn).await;
	}
}

#[derive(Debug, PartialEq)]
pub enum MutationResult<T, E> {
	Ok(T),
	Err(E),
	Loading(Option<T>),
	Pending,
}

impl<T, E> MutationResult<T, E> {
	pub fn is_ok(&self) -> bool {
		matches!(self, MutationResult::Ok(..))
	}

	pub fn is_err(&self) -> bool {
		matches!(self, MutationResult::Err(..))
	}

	pub fn is_loading(&self) -> bool {
		matches!(self, MutationResult::Loading(..))
	}

	pub fn is_pending(&self) -> bool {
		matches!(self, MutationResult::Pending)
	}

	pub fn set_loading(&mut self) {
		if let Some(v) = mem::replace(self, Self::Pending).into() {
			*self = Self::Loading(Some(v))
		}
	}
}

impl<T, E> From<Result<T, E>> for MutationResult<T, E> {
	fn from(value: Result<T, E>) -> Self {
		match value {
			Ok(v) => MutationResult::Ok(v),
			Err(e) => MutationResult::Err(e),
		}
	}
}

impl<T, E> From<MutationResult<T, E>> for Option<T> {
	fn from(result: MutationResult<T, E>) -> Self {
		match result {
			MutationResult::Ok(v) => Some(v),
			MutationResult::Err(_) => None,
			MutationResult::Loading(v) => v,
			MutationResult::Pending => None,
		}
	}
}

pub fn use_mutation<T, E, A, M, F>(mutation_fn: M) -> UseMutation<T, E, A>
where
	T: 'static + PartialEq,
	E: 'static,
	A: 'static,
	M: Fn(A) -> F + 'static,
	F: Future<Output = MutationResult<T, E>> + 'static,
{
	use_hook(|| UseMutation {
		value: Signal::new(MutationResult::Pending),
		mutation_fn: Signal::new(Arc::new(Box::new(move |p| {
			let fut = mutation_fn(p);
			Box::new(fut)
		}))),
		scheduler: Signal::new(schedule_update_any()),
		scope_id: current_scope_id().unwrap(),
	})
}
