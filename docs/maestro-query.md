# Maestro Query

A powerful and flexible query management system for Dioxus applications that provides advanced caching, synchronization, and state management capabilities.

## Features

- 🚀 **Automatic Cache Management**: Built-in stale-time tracking and cache invalidation
- 🔄 **Smart Re-rendering**: Only updates components when their specific query data changes
- 🎯 **Type-safe Query Keys**: Strongly typed query keys for compile-time safety
- 📦 **Built-in Loading States**: Sophisticated handling of loading, error, and success states
- 🔍 **Query Deduplication**: Automatically batches and deduplicates identical queries
- 🎮 **Fine-grained Control**: Silent mutations and manual query execution when needed
- 🔄 **Background Updates**: Supports automatic background data refreshing
- ⚡ **Zero Configuration**: Works out of the box with sensible defaults

## Complete Example

Here's a real-world example showing how to build a user management system with Maestro Query:

```rust
#[component]
pub fn UsersList() -> Element {
  // Initialize the query client
  let query_client: UseQueryClient<Vec<User>, Error, String> = use_init_query_client();
  
  // Query to fetch users
  let users_query = use_get_query([String::from("users")], |_| async move {
    let users = USERS.read().await;
    QueryResult::Ok(users.values().cloned().collect())
  });

  // Mutation to delete users
  let delete_mutation = use_mutation(|username: String| async move {
    let mut users = USERS.write().await;
    match users.remove(&username) {
      Some(_) => MutationResult::Ok(()),
      None => MutationResult::Err(UserError::NotFound),
    }
  });

  // Automatic cache invalidation after deletion
  let handle_delete = move |username: String| {
    let delete_mutation = delete_mutation.clone();
    let query_client = query_client.clone();
    async move {
      delete_mutation.mutate(username);
      query_client.invalidate_query(String::from("users"));
      // or
      query_client.invalidate_queries(["multiple", "query", "keys"])
    }
  };

  rsx! {
    div {
      {match users_query.result().value() {
        QueryResult::Loading(_) => rsx!{ "Loading..." },
        QueryResult::Err(e) => rsx!{ "Error: {e}" },
        QueryResult::Ok(users) => rsx!{
          // Render users list
        }
      }}
    }
  }
}
```

## Key Features in Detail

### 1. Query Client Setup

```rust
// Initialize in your root or parent component
let query_client: UseQueryClient<Data, Error, Key> = use_init_query_client();
```

### 2. Data Fetching with Loading States

```rust
let query = use_get_query([query_key], |_| async move {
  match fetch_data().await {
    Ok(data) => QueryResult::Ok(data),
    Err(e) => QueryResult::Err(e),
  }
});

// Access the result with built-in loading states
match query.result().value() {
  QueryResult::Loading(_) => render_loading(),
  QueryResult::Ok(data) => render_data(data),
  QueryResult::Err(error) => render_error(error),
}
```

### 3. Mutations with Form Handling

```rust
#[component]
pub fn UserForm(on_success: EventHandler) -> Element {
  let mut user = use_signal(|| User::default());
  let form = use_init_form_ctx(initial_values);
  
  let create_mutation = use_mutation(|new_user: User| async move {
    // Validation support
    if let Err(e) = new_user.validate() {
      return MutationResult::Err(UserError::ValidationError(e.to_string()));
    }

    // Perform mutation
    save_user(new_user).await
  });

  let handle_submit = move |event: FormEvent| {
    create_mutation.mutate(user.read().clone());
  };

  rsx! {
    Form {
      form: form // form context
      onsubmit: handle_submit,
      // Form fields...
      inner: button {
        r#type: "submit",
        disabled: create_mutation.result().is_loading(),
        {if create_mutation.result().is_loading() { 
          "Saving..." 
        } else { 
          "Save" 
        }}
      }
    }
  }
}
```

### 4. Automatic Cache Invalidation

```rust
// Invalidate queries after mutations
let handle_update = move |data| {
  update_mutation.mutate(data);
  query_client.invalidate_query("cache_key");
  // or
  query_client.invalidate_queries(["multiple", "query", "keys"])
};
```

### 5. Error Handling

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum UserError {
  NotFound,
  ValidationError(String),
  DatabaseError(String),
}

// In your component
match *mutation.result() {
  MutationResult::Err(UserError::ValidationError(ref e)) => {
    rsx!{ div { class: "error", "{e}" } }
  },
  MutationResult::Loading(_) => rsx!{ "Processing..." },
  MutationResult::Ok(_) => rsx!{ "Success!" },
  _ => None,
}
```

## State Management Comparison

### Without Maestro Query

```rust
// Manual state management
let users = use_signal(Vec::new);
let loading = use_signal(false);
let error = use_signal(None);

use_effect(move || {
  loading.set(true);
  spawn(async move {
    match fetch_users().await {
      Ok(data) => users.set(data),
      Err(e) => error.set(Some(e)),
    }
    loading.set(false);
  });
});
```

### With Maestro Query

```rust
// Automatic state management
let users_query = use_get_query(["users"], |_| async move {
  fetch_users().await
});

// All states handled automatically
match users_query.result().value() {
  QueryResult::Loading(_) => render_loading(),
  QueryResult::Ok(users) => render_users(users),
  QueryResult::Err(error) => render_error(error),
}
```

## Getting Started

1. Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-maestro = "0.1.0"
```

2.Initialize the query client in your root component

3.Start using queries and mutations in your components

## Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

## License

MIT or Apache-2.0, at your option.
