
use {
  crate::{components::form::form_content::FormContent, models::user::User}, 
  async_std::sync::RwLock, 
  dioxus::{fullstack::once_cell, prelude::*}, 
  maestro_forms::fields::form::{Form, FormResult},
  maestro_query::prelude::*, 
  maestro_ui::button::{Button, ButtonType, ButtonVariant}, 
  std::{collections::HashMap, sync::Arc}, 
  validator::Validate
};

// simulated backend storage
pub static USERS: once_cell::sync::Lazy<Arc<RwLock<HashMap<String, User>>>> =
  once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

#[derive(Debug, Clone, PartialEq)]
pub enum CustomError {
  NetworkError,
  NotFound,
  ValidationError(String),
  DatabaseError(String),
}

#[component]
pub fn BasicQueryDemo() -> Element {
  let query_client: UseQueryClient<Vec<User>, CustomError, String> = use_init_query_client();

  let users_query = use_get_query([String::from("users")], |_| async move {
    let users = USERS.read().await;
    QueryResult::Ok::<Vec<User>, CustomError>(users.values().cloned().collect::<Vec<User>>())
  });

  let delete_mutation = use_mutation(|username: String| async move {
    let mut users = USERS.write().await;
    match users.remove(&username) {
      Some(_) => MutationResult::Ok::<(), CustomError>(()),
      None => MutationResult::Err(CustomError::NotFound),
    }
  });

  let handle_delete = move |username: String| {
    delete_mutation.mutate(username);
    if delete_mutation.result().is_ok() {
      println!("invalidating cache");
      query_client.invalidate_query(String::from("users"));
    }
  };

  let mut show_form = use_signal(|| false);

  rsx! {
    div { 
      class: "border bg-white rounded shadow-md p-4",
      h2 { class: "text-2xl text-gray-800 text-center font-bold mb-4", "Users List" }

      div {
        class: "grid flex justify-center mt-2",
        Button {
          class: "bg-blue-500 text-white rounded mb-4",
          on_click: move |_| {
            let current_show_form = show_form();
            show_form.set(!current_show_form);
          },
          if show_form() { "Hide Form" } else { "Add User" }
        }
      }

      div {
        if show_form() {
          OptimisticUserForm {
            on_success: move |_| {
              show_form.set(false);
              query_client.invalidate_query(String::from("users"));
            }
          }
        }
      }

      div {
        class: "grid flex grid-cols-1 justify-center h-96 text-center gap-2 text-gray-800 bg-white", 
        {match users_query.result().value() {
          QueryResult::Loading(_) => rsx! { 
            div { class: "text-gray-500", "Loading users..." } 
          },
          QueryResult::Err(e) => rsx! { 
            div { class: "text-red-500", "Error: {e:?}" } 
          },
          QueryResult::Ok(users) => rsx! {
            
              {if users.is_empty() {
                rsx! { 
                  div { 
                    "No users found."
                  } 
                }
              } else {
                rsx! {
                  {users.iter().map(|user| {
                    let user = user.clone();
                    rsx!(
                      div {
                        key: "{user.username}",
                        class: "border bg-gray-200 p-4 rounded shadow-md bg-white",
                        p { class: "font-semibold", "Username: {user.username}" }
                        p { "Email: {user.email}" }
                        p { "Age: {user.age}" }
                        p { "Role: {user.role}" }
                        Button {
                          class: "mt-2 bg-red-500 hover:bg-red-700 rounded border",
                          disabled: delete_mutation.result().is_loading(),
                          variant: ButtonVariant::Destructive,
                          button_type: ButtonType::Button,
                          on_click: move |_| handle_delete(user.username.clone()),
                          { if delete_mutation.result().is_loading() {"Deleting..."} else {"Delete User"} }
                        }
                      }
                    )
                  })}
                }
              }}
            }
          }
        }
      }
    }
  }
}


#[component]
pub fn OptimisticUserForm(on_success: Option<EventHandler>) -> Element {
  let query_client: UseQueryClient<Vec<User>, CustomError, String> = use_init_query_client();
  let mut error_message = use_signal(|| String::new());
  
  let create_mutation = use_mutation(|new_user: User| async move {
    let mut users = USERS.write().await;
    
    if let Err(e) = new_user.validate() {
      return MutationResult::Err(CustomError::ValidationError(e.to_string()));
    }

    users.insert(new_user.username.clone(), new_user.clone());
    MutationResult::Ok(new_user)
  });

  let handle_submit = move |(_event, (submitted_user, is_valid)): (FormEvent, FormResult<User>)| {
    async move {
      if !is_valid {
        error_message.set("Form validation failed".to_string());
        return;
      }

      create_mutation.mutate(submitted_user.clone());
      
      spawn(async move {
        while create_mutation.result().is_loading() {
          async_std::task::sleep(std::time::Duration::from_millis(50)).await;
        }
        
        if create_mutation.result().is_ok() {
          query_client.invalidate_query(String::from("users"));
          if let Some(handler) = on_success {
            handler.call(());
          }
        } else {
          // rollback
          let mut users = USERS.write().await;
          users.remove(&submitted_user.username);
          log::error!("Mutation failed: {:?}", create_mutation.result());
        }
      });
    }
  };

  rsx! {
    div {
      if !error_message().is_empty() {
        div { class: "text-red-500 text-center mb-4", "{error_message}" }
      }
      Form {
        initial_values: User::default(),
        onsubmit: handle_submit,
        inner: FormContent
      }
    }
  }
}
