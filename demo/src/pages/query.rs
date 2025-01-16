use async_std::sync::RwLock;
use dioxus::fullstack::once_cell;
use dioxus::prelude::*;
use maestro_query::prelude::*;
use validator::Validate;
use std::collections::HashMap;
use std::fmt::Error;
use std::sync::Arc;

use crate::models::user::User;

// simulated backend storage
static USERS: once_cell::sync::Lazy<Arc<RwLock<HashMap<String, User>>>> = 
  once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

#[derive(Debug, Clone, PartialEq)]
pub enum UserError {
  NotFound,
  ValidationError(String),
  DatabaseError(String),
}

// component to display user list
#[component]
pub fn UserList() -> Element {
    let query_client: UseQueryClient<Vec<User>, Error, String> = use_init_query_client();
    
    let users = use_get_query([String::from("users")], |_| async move {
      let users = USERS.read().await;
      QueryResult::Ok::<Vec<User>, Error>(users.values().cloned().collect::<Vec<User>>())
    });

    let delete_mutation = use_mutation(|username: String| async move {
      let mut users = USERS.write().await;
      match users.remove(&username) {
        Some(_) => MutationResult::Ok(()),
        None => MutationResult::Err(UserError::NotFound),
      }
    });

    let handle_delete = move |username: String| {
      let delete_mutation = delete_mutation;
      let query_client = query_client;
      async move {
        delete_mutation.mutate(username.clone());
        query_client.invalidate_query(String::from("users"));
      }
    };

    rsx! {
      div {
        h2 { "Users List" }
        match users.result().value() {
          QueryResult::Loading(_) => rsx!{ div { "Loading users..." } },
          QueryResult::Err(e) => rsx!{ div { "Error: {e}" } },
          QueryResult::Ok(users) => rsx!{
              div {
                {
                  users.iter().map(|user| rsx!(
                    div {
                      key: "{user.username}",
                      class: "user-item",
                      p { "Username: {user.username}" }
                      p { "Email: {user.email}" }
                      p { "Age: {user.age}" }
                      p { "Role: {user.role}" }
                      button {
                        onclick: move |_| handle_delete(user.username.clone()),
                        "Delete User"
                      }
                    }
                ))
                }
              }
          }
        }
      }
    }
}

// component to create/edit user
#[component]
pub fn UserForm(initial_user: Option<User>, on_success: EventHandler) -> Element {
    let mut user = use_signal(|| initial_user.unwrap_or_default());
    
    let create_mutation = use_mutation(|new_user: User| async move {
      if let Err(e) = new_user.validate() {
        return MutationResult::Err(UserError::ValidationError(e.to_string()));
      }

      let mut users = USERS.write().await;
      users.insert(new_user.username.clone(), new_user.clone());
      MutationResult::Ok(new_user)
    });

    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        to_owned![create_mutation, on_success, user];
        
        async move {
            create_mutation.mutate(user.read().to_owned());
            if create_mutation.result().is_ok() {
                on_success.call(());
            }
        }
    };

    rsx! {
        form {
            onsubmit: handle_submit,
            div {
                label { "Username:" }
                input {
                    value: "{user.read().username}",
                    oninput: move |e| user.set(User {
                        username: e.value().clone(),
                        ..*user.read()
                    })
                }
            }
            div {
                label { "Email:" }
                input {
                    value: "{user.read().email}",
                    oninput: move |e| user.set(User {
                        email: e.value().clone(),
                        ..*user.read()
                    })
                }
            }
            div {
                label { "Bio:" }
                textarea {
                    value: "{user.read().bio}",
                    oninput: move |e| user.set(User {
                        bio: e.value().clone(),
                        ..*user.read()
                    })
                }
            }
            div {
                label { "Age:" }
                input {
                    r#type: "number",
                    value: "{user.read().age}",
                    oninput: move |e| user.set(User {
                        age: e.value().parse().unwrap_or(18),
                        ..*user.read()
                    })
                }
            }
            div {
                label { "Role:" }
                select {
                    value: "{user.read().role}",
                    onchange: move |e| user.set(User {
                        role: e.value().clone(),
                        ..*user.read()
                    }),
                    option { value: "", "Select Role" }
                    option { value: "user", "User" }
                    option { value: "admin", "Admin" }
                }
            }
            button {
                r#type: "submit",
                disabled: create_mutation.result().is_loading(),
                {if create_mutation.result().is_loading() { "Saving..." } else { "Save User" }}
            }
            {match *create_mutation.result() {
                MutationResult::Err(UserError::ValidationError(ref e)) => Some(rsx!{
                    div { class: "error", "{e}" }
                }),
                _ => None
            }}
        }
    }
}
