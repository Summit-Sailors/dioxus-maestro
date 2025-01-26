use {
  crate::{
    components::forms::form_field_wrapper::FormFieldWrapper, 
    models::user::User
  }, async_std::sync::RwLock, dioxus::{fullstack::once_cell, prelude::*}, maestro_forms::fields::{
    form::{
      Form, 
      InnerComponentProps
    }, 
    select::SelectFormField, 
    text::TextFormInput, 
    textarea::TextArea
  }, 
  maestro_query::prelude::*, 
  maestro_ui::button::{
    Button, ButtonSize, ButtonType, ButtonVariant
  }, 
  std::{
    collections::HashMap, fmt::Error, sync::Arc
  }, validator::Validate,
};

// Simulated backend storage
pub static USERS: once_cell::sync::Lazy<Arc<RwLock<HashMap<String, User>>>> =
  once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

#[derive(Debug, Clone, PartialEq)]
pub enum UserError {
  NotFound,
  ValidationError(String),
  DatabaseError(String),
}

const AVAILABLE_ROLES: &[&str] = &["admin", "user", "moderator"];

#[component]
pub fn QueryDemo() -> Element {
  let query_client: UseQueryClient<Vec<User>, Error, String> = use_init_query_client();
  
  // query result in a signal to avoid temporary value issue
  let users_query = use_get_query([String::from("users")], |_| async move {
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
    let delete_mutation = delete_mutation.clone();
    let query_client = query_client.clone();
    async move {
      delete_mutation.mutate(username.clone());
      query_client.invalidate_query(String::from("users"));
    }
  };

  let mut show_form = use_signal(|| false);

  rsx! {
    div { class: "container mx-auto p-4",
      h2 { class: "text-2xl font-bold mb-4", "Users List" }
      
      Button {
        class: "bg-blue-500 text-white px-4 py-2 rounded mb-4",
        on_click: move |_| show_form.set(!show_form()),
        if *show_form.read() { "Hide Form" } else { "Add User" }
      }

      if *show_form.read() {
        UserForm {
          on_success: move |_| {
            show_form.set(false);
            query_client.invalidate_query(String::from("users"));
          }
        }
      }

      {match users_query.result().value() {
        QueryResult::Loading(_) => rsx!{ div { class: "text-gray-500", "Loading users..." } },
        QueryResult::Err(e) => rsx!{ div { class: "text-red-500", "Error: {e}" } },
        QueryResult::Ok(users) => rsx!{
          div { class: "grid gap-4",
            {users.iter().map(|user| {
              let user = user.clone();
              rsx!(
                div { 
                  key: "{user.username}",
                  class: "border p-4 rounded shadow-md bg-white",
                  p { class: "font-semibold", "Username: {user.username}" }
                  p { "Email: {user.email}" }
                  p { "Age: {user.age}" }
                  p { "Role: {user.role}" }
                  Button {
                    class: "mt-2",
                    variant: ButtonVariant::Destructive,
                    on_click: move |_| handle_delete(user.username.clone()),
                    "Delete User"
                  }
                }
              )
            })}
          }
        }
      }}
    }
  }
}

pub fn form_content(props: InnerComponentProps<User>) -> Element {
  let loading = use_signal(|| false);
  let error_message = use_signal(|| String::new());

  rsx! {
    div { class: "space-y-4 bg-white p-6 rounded-lg shadow",
      if !error_message().is_empty() {
        div { 
          class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative", 
          role: "alert",
          "{error_message()}"
        }
      }

      FormFieldWrapper {
        label: "Username",
        field: props.form.get_form_field("username".to_string()),
        TextFormInput::<User> {
          name: "username",
          placeholder: "Enter your username",
          class: "w-full p-2 rounded-md border border-gray-300 focus:ring focus:ring-blue-300 focus:outline-none",
        }
      }

      FormFieldWrapper {
        label: "Email",
        field: props.form.get_form_field("email".to_string()),
        TextFormInput::<User> {
          name: "email",
          placeholder: "Enter your email address",
          class: "w-full p-2 rounded-md border border-gray-300 focus:ring focus:ring-blue-300 focus:outline-none",
        }
      }

      FormFieldWrapper {
        label: "Bio",
        field: props.form.get_form_field("bio".to_string()),
        TextArea::<User> {
          name: "bio",
          placeholder: "Tell us about yourself...",
          rows: 4,
          class: "w-full p-2 rounded-md border border-gray-300 focus:ring focus:ring-blue-300 focus:outline-none",
        }
      }

      FormFieldWrapper {
        label: "Role",
        field: props.form.get_form_field("role".to_string()),
        SelectFormField::<User, String> {
          name: "role",
          values: AVAILABLE_ROLES.iter().map(|&s| s.to_string()).collect(),
          class: "w-full p-2 rounded-md border border-gray-300 focus:ring focus:ring-blue-300 focus:outline-none",
        }
      }

      Button {
        class: format!(
          "mt-4 py-2 rounded-md text-white font-semibold {} transition-opacity duration-200",
          if loading() { "bg-gray-400 cursor-not-allowed opacity-70" } else { "bg-blue-500 hover:bg-blue-600" }
        ),
        button_type: ButtonType::Submit,
        disabled: loading(),
        size: ButtonSize::Default,
        variant: ButtonVariant::Default,
        if loading() { "Loading..." } else { "Submit" }
      }
    }
  }
}

#[component]
pub fn UserForm(on_success: Option<EventHandler>) -> Element {
  let create_mutation = use_mutation(|new_user: User| async move {
    if let Err(e) = new_user.validate() {
      return MutationResult::Err(UserError::ValidationError(e.to_string()));
    }
    let mut users = USERS.write().await;
    users.insert(new_user.username.clone(), new_user.clone());
    MutationResult::Ok(new_user)
  });

  let mut error_message = use_signal(|| String::new());
  let mut loading = use_signal(|| false);

  let on_submit = Some(Callback::new(
    move |(event, (_submitted_user, is_valid)): (Event<FormData>, (User, bool))| {
      event.prevent_default();
      loading.set(true);

      spawn(async move {
        if is_valid {
          let result = create_mutation.result();
          match *result {
            MutationResult::Ok(_) => {
              error_message.set(String::new());
              loading.set(false);
              if let Some(handler) = on_success {
                handler.call(());
              }
            }
            MutationResult::Err(ref e) => {
              match e {
                UserError::ValidationError(msg) => error_message.set(msg.clone()),
                UserError::NotFound => {
                  error_message.set("User not found".to_string())
                }
                UserError::DatabaseError(msg) => error_message.set(msg.clone()),
              }
              loading.set(false);
            }
            _ => {}
          }
        } else {
            error_message.set("Form validation failed. Please check your inputs.".to_string());
            loading.set(false);
        }
      });
    },
  ));

  rsx! {
    Form {
      initial_value: User::default(),
      onsubmit: on_submit,
      inner: form_content
    }
  }
}
