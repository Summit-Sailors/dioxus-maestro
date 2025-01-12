#![allow(clippy::disallowed_types)]
use {
	crate::{cached_result::CachedResult, result::QueryResult},
	dioxus_lib::prelude::*,
	futures_util::{
		stream::{FuturesUnordered, StreamExt},
		Future,
	},
	instant::Instant,
	std::{
		any::TypeId,
		collections::{HashMap, HashSet},
		hash::Hash,
		sync::{Arc, RwLock},
	},
};

pub fn use_init_query_client<T, E, K>() -> UseQueryClient<T, E, K>
where
	T: 'static,
	E: 'static,
	K: 'static,
{
	use_context_provider(|| UseQueryClient { queries_registry: Signal::default(), scheduler: Signal::new(schedule_update_any()) })
}

/// Get access to the [UseQueryClient].
pub fn use_query_client<T, E, K>() -> UseQueryClient<T, E, K>
where
	T: 'static,
	E: 'static,
	K: 'static,
{
	use_context()
}

pub(crate) type QueryFn<T, E, K> = dyn Fn(Vec<K>) -> Box<dyn Future<Output = QueryResult<T, E>>>;

pub(crate) type QueryValue<T> = Arc<RwLock<T>>;

#[derive(Clone)]
pub(crate) struct QueryListeners<T, E, K> {
	pub(crate) value: QueryValue<CachedResult<T, E>>,
	pub(crate) listeners: HashSet<ScopeId>,
	pub(crate) query_fn: Arc<Box<QueryFn<T, E, K>>>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct RegistryEntry<K> {
	pub(crate) query_keys: Vec<K>,
	pub(crate) query_fn_id: TypeId,
}

pub(crate) type QueriesRegistry<T, E, K> = HashMap<RegistryEntry<K>, QueryListeners<T, E, K>>;

pub struct UseQueryClient<T, E, K>
where
	T: 'static,
	E: 'static,
	K: 'static,
{
	pub(crate) queries_registry: Signal<QueriesRegistry<T, E, K>>,
	pub(crate) scheduler: Signal<Arc<dyn Fn(ScopeId)>>,
}

impl<T, E, K> Clone for UseQueryClient<T, E, K> {
	fn clone(&self) -> Self {
		*self
	}
}
impl<T, E, K> Copy for UseQueryClient<T, E, K> {}

impl<T, E, K> UseQueryClient<T, E, K>
where
	T: 'static + Clone,
	E: 'static + Clone,
	K: 'static + PartialEq + Eq + Hash + Clone,
{
	pub(crate) fn get_entry(&self, entry: &RegistryEntry<K>) -> QueryListeners<T, E, K> {
		self.queries_registry.peek().get(entry).unwrap().clone()
	}

	pub(crate) async fn run_new_query(&self, entry: &RegistryEntry<K>) {
		let QueryListeners { value, query_fn, listeners, .. } = self.get_entry(entry);
		let is_fresh = value.read().unwrap().is_fresh();
		let is_loading = value.read().unwrap().is_loading();
		let has_been_queried = value.read().unwrap().has_been_queried();
		if (!is_fresh && !is_loading) || !has_been_queried {
			if has_been_queried {
				value.write().unwrap().set_to_loading();
				for listener in listeners {
					(self.scheduler.peek())(listener);
				}
			}
			value.write().unwrap().has_been_queried = true;
			*value.write().unwrap() =
				CachedResult { value: Box::into_pin((query_fn)(entry.query_keys.clone())).await, instant: Some(Instant::now()), has_been_queried: true };
			let QueryListeners { listeners, .. } = self.get_entry(entry);
			for listener in listeners {
				(self.scheduler.peek())(listener);
			}
		}
	}

	pub(crate) async fn invalidate_queries_inner(
		queries_registry: Signal<QueriesRegistry<T, E, K>>,
		scheduler: Signal<Arc<dyn Fn(ScopeId)>>,
		keys_to_invalidate: &[K],
	) {
		let tasks = FuturesUnordered::new();
		for (RegistryEntry { query_keys, .. }, QueryListeners { value, listeners, query_fn }) in queries_registry.peek().iter() {
			if query_keys.iter().any(|k| keys_to_invalidate.contains(k)) {
				value.write().unwrap().set_to_loading();
				listeners.iter().for_each(|listener| (scheduler.peek())(*listener));
				to_owned![query_fn, query_keys, listeners, value];
				tasks.push(Box::pin(async move {
					*value.write().unwrap() =
						CachedResult { value: Box::into_pin((query_fn)(query_keys.clone())).await, instant: Some(Instant::now()), has_been_queried: true };
					listeners.iter().for_each(|listener| (scheduler.peek())(*listener));
				}));
			}
		}
		tasks.count().await;
	}

	pub fn invalidate_query(&self, key_to_invalidate: K) {
		let queries_registry = self.queries_registry;
		let scheduler = self.scheduler;
		spawn(async move {
			Self::invalidate_queries_inner(queries_registry, scheduler, &[key_to_invalidate]).await;
		});
	}

	pub fn invalidate_queries(&self, keys_to_invalidate: &[K]) {
		let queries_registry = self.queries_registry;
		let scheduler = self.scheduler;
		let keys_to_invalidate = keys_to_invalidate.to_vec();
		spawn(async move {
			Self::invalidate_queries_inner(queries_registry, scheduler, &keys_to_invalidate).await;
		});
	}
}
