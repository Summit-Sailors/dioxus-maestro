// TODO: under dev

use crate::models::user::User;
use dioxus::prelude::*;
use maestro_query::prelude::*;
use maestro_sqlx::pool::create_db_pool_sqlx;
use sqlx::PgPool;
use std::sync::Arc;


pub fn HooksPage(cx: Scope) -> Element {
  let client = use_init_query_client::<Vec<User>, String, String>();
    
  render! {
    div { class: "container mx-auto p-6",
      h1 { class: "text-3xl font-bold mb-8", "Maestro Query Demo" }
      
      div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
        // query 
        { QueryExamples {} }
        
        // mutation 
        { MutationExamples {} }
      }
    }
  }
}

fn QueryExamples(cx: Scope) -> Element {
  let pool = use_signal(cx, || None::<Arc<PgPool>>);
  
  use_effect(cx, (), |_| {
    to_owned![pool];
    async move {
      let db_pool = create_db_pool_sqlx().await;
      pool.set(Some(Arc::new(db_pool)));
    }
  });

  // basic users query
  let users_query = use_get_query(["all_users"], move |_| {
    let pool = pool.clone();
    async move {
      match &*pool.get() {
        Some(pool) => {
          match sqlx::query_as!(
            User,
            "SELECT username, email, bio, age, role FROM users"
          )
          .fetch_all(pool)
          .await {
            Ok(users) => QueryResult::Ok(users),
            Err(e) => QueryResult::Err(e.to_string()),
          }
        }
        None => QueryResult::Loading(None),
      }
    }
  });

  // filtered users query
  let role_filter = use_state(cx, || "admin".to_string());
  let filtered_users = use_get_query([role_filter.get().clone()], move |keys| {
    let pool = pool.clone();
    let role = &keys[0];
    async move {
      match &*pool.get() {
        Some(pool) => {
          match sqlx::query_as!(
            User,
            "SELECT username, email, bio, age, role FROM users WHERE role = $1",
            role
          )
          .fetch_all(pool)
          .await {
            Ok(users) => QueryResult::Ok(users),
            Err(e) => QueryResult::Err(e.to_string()),
          }
        }
        None => QueryResult::Loading(None),
      }
    }
  });

  render! {
    div { class: "bg-white rounded-lg shadow p-6",
      h2 { class: "text-2xl font-semibold mb-4", "Query Examples" }

      // all users section
      section { class: "mb-8",
        h3 { class: "text-xl font-medium mb-3", "All Users" }
        
        div { class: "space-y-2",
          if users_query.result().is_loading() {
            div { class: "text-gray-500", "Loading users..." }
          } else if let QueryResult::Ok(users) = &users_query.result().value() {
            users.iter().map(|user| {
              rsx! {
                div { class: "border rounded p-3",
                  p { class: "font-medium", "{user.username}" }
                  p { class: "text-sm text-gray-600", "Role: {user.role}" }
                  p { class: "text-sm text-gray-600", "Email: {user.email}" }
                }
              }
            })
          } else if let QueryResult::Err(error) = &users_query.result().value() {
            div { class: "text-red-500", "Error: {error}" }
          }
        }
      }

      // filtered users section
      section {
        h3 { class: "text-xl font-medium mb-3", "Filtered Users" }
        
        div { class: "mb-4",
          select {
            class: "border rounded px-3 py-2",
            value: "{role_filter}",
            onchange: move |evt| role_filter.set(evt.value.clone()),
            option { value: "admin", "Admin" }
            option { value: "user", "User" }
            option { value: "moderator", "Moderator" }
          }
        }

        div { class: "space-y-2",
          if filtered_users.result().is_loading() {
            div { class: "text-gray-500", "Loading filtered users..." }
          } else if let QueryResult::Ok(users) = &filtered_users.result().value() {
            users.iter().map(|user| {
              rsx! {
                div { class: "border rounded p-3",
                  p { class: "font-medium", "{user.username}" }
                  p { class: "text-sm text-gray-600", "Email: {user.email}" }
                }
              }
            })
          } else if let QueryResult::Err(error) = &filtered_users.result().value() {
              div { class: "text-red-500", "Error: {error}" }
          }
        }
      }
    }
  }
}

#[component]
fn MutationExamples(cx: Scope) -> Element {
  let pool = use_state(cx, || None::<Arc<PgPool>>);
  
  use_effect(cx, (), |_| {
    to_owned![pool];
    async move {
      let db_pool = create_db_pool_sqlx().await;
      pool.set(Some(Arc::new(db_pool)));
    }
  });

  // Create user mutation
  let create_user = use_mutation(move |user: User| {
    let pool = pool.clone();
    async move {
      match &*pool.get() {
        Some(pool) => {
          match sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, bio, age, role) 
              VALUES ($1, $2, $3, $4, $5)
              RETURNING username, email, bio, age, role",
            user.username,
            user.email,
            user.bio,
            user.age,
            user.role
          )
          .fetch_one(pool)
          .await {
            Ok(user) => MutationResult::Ok(user),
            Err(e) => MutationResult::Err(e.to_string()),
          }
        }
        None => MutationResult::Err("Database connection not available".to_string()),
      }
    }
  });

  let user = use_state(cx, User::default);

  render! {
    div { class: "bg-white rounded-lg shadow p-6",
      h2 { class: "text-2xl font-semibold mb-4", "Mutation Examples" }

      form {
        class: "space-y-4",
        onsubmit: move |ev| {
          ev.prevent_default();
          create_user.mutate(user.get().clone());
        },

        div {
          label { class: "block text-sm font-medium mb-1", "Username" }
          input {
            class: "border rounded px-3 py-2 w-full",
            value: "{user.get().username}",
            oninput: move |ev| {
              let mut new_user = user.get().clone();
              new_user.username = ev.value.clone();
              user.set(new_user);
            }
          }
        }

        div {
          label { class: "block text-sm font-medium mb-1", "Email" }
          input {
            class: "border rounded px-3 py-2 w-full",
            r#type: "email",
            value: "{user.get().email}",
            oninput: move |ev| {
              let mut new_user = user.get().clone();
              new_user.email = ev.value.clone();
              user.set(new_user);
            }
          }
        }

        div {
          label { class: "block text-sm font-medium mb-1", "Bio" }
          textarea {
            class: "border rounded px-3 py-2 w-full",
            value: "{user.get().bio}",
            oninput: move |ev| {
              let mut new_user = user.get().clone();
              new_user.bio = ev.value.clone();
              user.set(new_user);
            }
          }
        }

        div {
          label { class: "block text-sm font-medium mb-1", "Age" }
          input {
            class: "border rounded px-3 py-2 w-full",
            r#type: "number",
            value: "{user.get().age}",
            oninput: move |ev| {
              let mut new_user = user.get().clone();
              new_user.age = ev.value.parse().unwrap_or(18);
              user.set(new_user);
            }
          }
        }

        div {
          label { class: "block text-sm font-medium mb-1", "Role" }
          select {
            class: "border rounded px-3 py-2 w-full",
            value: "{user.get().role}",
            onchange: move |ev| {
              let mut new_user = user.get().clone();
              new_user.role = ev.value.clone();
              user.set(new_user);
            },
            option { value: "", "Select a role" }
            option { value: "admin", "Admin" }
            option { value: "user", "User" }
            option { value: "moderator", "Moderator" }
          }
        }

        button {
          class: "bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 disabled:opacity-50",
          r#type: "submit",
          disabled: create_user.result().is_loading(),
          if create_user.result().is_loading() {
            "Creating..."
          } else {
            "Create User"
          }
        }

        // mutation status messages
        {match create_user.result() {
          MutationResult::Ok(_) => render! {
            div { class: "text-green-500 mt-2", "User created successfully!" }
          },
          MutationResult::Err(error) => render! {
            div { class: "text-red-500 mt-2", "Error: {error}" }
          },
          _ => None
        }}
      }
    }
  }
}
