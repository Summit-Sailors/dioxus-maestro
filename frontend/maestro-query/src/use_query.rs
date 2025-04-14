#![allow(clippy::disallowed_types)]
use std::{
	any::TypeId,
	collections::HashSet,
	hash::Hash,
	sync::{Arc, RwLock, RwLockReadGuard},
};

use dioxus_lib::prelude::*;
use futures_util::Future;
mod warnings {
	pub use warnings::Warning;
}
use maestro_hooks::explicit_memo::use_explicit_memo;
pub use warnings::Warning;

use crate::{
	cached_result::CachedResult,
	result::QueryResult,
	use_query_client::{QueryFn, QueryListeners, QueryValue, RegistryEntry, UseQueryClient, use_query_client},
};

// #[derive(Clone)]
pub struct UseQuery<T, E, K>
where
	T: 'static,
	E: 'static,
	K: 'static + Eq + Hash,
{
	cleaner: Signal<UseQueryCleaner<T, E, K>>,
	client: UseQueryClient<T, E, K>,
	value: QueryValue<CachedResult<T, E>>,
	scope_id: ScopeId,
}

impl<T, E, K> Clone for UseQuery<T, E, K>
where
	K: Eq + Hash + Clone,
{
	fn clone(&self) -> Self {
		Self { cleaner: self.cleaner, client: self.client, value: self.value.clone(), scope_id: self.scope_id }
	}
}

impl<T, E, K> UseQuery<T, E, K>
where
	K: Eq + Hash + Clone,
{
	pub fn result(&self) -> RwLockReadGuard<CachedResult<T, E>> {
		self.value.read().expect("Query value is already borrowed")
	}
}

pub struct UseQueryCleaner<T, E, K>
where
	T: 'static,
	E: 'static,
	K: 'static + Eq + Hash,
{
	client: UseQueryClient<T, E, K>,
	registry_entry: RegistryEntry<K>,
	scope_id: ScopeId,
}

impl<T, E, K: Eq + Hash> Drop for UseQueryCleaner<T, E, K> {
	fn drop(&mut self) {
		dioxus_lib::prelude::warnings::signal_write_in_component_body::allow(|| {
			let mut queries_registry = match self.client.queries_registry.try_write_unchecked() {
				Err(dioxus_lib::prelude::BorrowMutError::Dropped(_)) => return, // noted as safe
				Err(e) => panic!("Unexpected error: {e}"),
				Ok(v) => v,
			};
			if let Some(query_listeners) = queries_registry.get_mut(&self.registry_entry) {
				query_listeners.listeners.remove(&self.scope_id);
				if query_listeners.listeners.is_empty() {
					queries_registry.remove(&self.registry_entry);
				}
			}
		});
	}
}

pub struct Query<T, E, K> {
	query_fn: Arc<Box<QueryFn<T, E, K>>>,
	initial_value: Option<QueryResult<T, E>>,
	registry_entry: RegistryEntry<K>,
}

impl<T, E, K> Query<T, E, K> {
	pub fn new<Q, F>(query_fn: Q) -> Self
	where
		Q: 'static + Fn(Vec<K>) -> F,
		F: 'static + Future<Output = QueryResult<T, E>>,
		K: Clone,
	{
		Self {
			query_fn: Arc::new(Box::new(move |query_keys| Box::new(query_fn(query_keys)))),
			initial_value: None,
			registry_entry: RegistryEntry { query_keys: Vec::new(), query_fn_id: TypeId::of::<F>() },
		}
	}
}

pub fn use_query<T, E, K, const N: usize>(query_keys: [K; N], query: impl FnOnce() -> Query<T, E, K>) -> UseQuery<T, E, K>
where
	T: 'static + PartialEq + Clone,
	E: 'static + Clone,
	K: 'static + Eq + Hash + Clone,
{
	let client = use_query_client();
	use_explicit_memo(query_keys.clone(), || {
		let mut query = query();
		query.registry_entry.query_keys = query_keys.to_vec();
		let registry_entry = query.registry_entry;
		dioxus_lib::prelude::warnings::signal_write_in_component_body::allow(|| {
			let mut queries_registry = client.queries_registry.write_unchecked();
			let query_listeners = queries_registry.entry(registry_entry.clone()).or_insert(QueryListeners {
				listeners: HashSet::default(),
				value: QueryValue::new(RwLock::new(CachedResult::builder().value(query.initial_value.unwrap_or_default()).build())),
				query_fn: query.query_fn,
			});
			query_listeners.listeners.insert(current_scope_id().unwrap());
			let value = query_listeners.value.clone();
			spawn({
				to_owned![registry_entry];
				async move {
					client.run_new_query(&registry_entry).await;
				}
			});
			UseQuery {
				client,
				value,
				scope_id: current_scope_id().unwrap(),
				cleaner: Signal::new(UseQueryCleaner { client, registry_entry, scope_id: current_scope_id().unwrap() }),
			}
		})
	})
}

pub fn use_get_query<TValue, TError, TKey, TQueryFn, F, const N: usize>(query_keys: [TKey; N], query_fn: TQueryFn) -> UseQuery<TValue, TError, TKey>
where
	TValue: 'static + PartialEq + Clone,
	TError: 'static + Clone,
	TKey: 'static + Eq + Hash + Clone,
	TQueryFn: 'static + Fn(Vec<TKey>) -> F,
	F: 'static + Future<Output = QueryResult<TValue, TError>>,
{
	use_query(query_keys, || Query::new(query_fn))
}
