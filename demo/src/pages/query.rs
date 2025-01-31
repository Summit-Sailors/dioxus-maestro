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

// simulated backend storage
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
pub fn BasicQueryDemo() -> Element {
  let query_client: UseQueryClient<Vec<User>, UserError, String> = use_init_query_client();

  let users_query = use_get_query([String::from("users")], |_| async move {
    let users = USERS.read().await;
    QueryResult::Ok::<Vec<User>, UserError>(users.values().cloned().collect::<Vec<User>>())
  });

  let delete_mutation = use_mutation(|username: String| async move {
    let mut users = USERS.write().await;
    match users.remove(&username) {
      Some(_) => MutationResult::Ok::<(), UserError>(()),
      None => MutationResult::Err(UserError::NotFound),
    }
  });

  let handle_delete = move |username: String| {
    let delete_mutation = delete_mutation.clone();
    let query_client = query_client.clone();

    {
      delete_mutation.mutate(username);

      if delete_mutation.result().is_ok() {
        query_client.invalidate_query(String::from("users"));
      }
    };
  };

  let mut show_form = use_signal(|| false);

  rsx! {
    div { class: "container mx-auto p-4",
      h2 { class: "text-2xl font-bold mb-4", "Users List" }

      Button {
        class: "bg-blue-500 text-white px-4 py-2 rounded mb-4",
        on_click: move |_| {
          let current_show_form = *show_form.read();
          show_form.set(!current_show_form);
        },
        if *show_form.read() { "Hide Form" } else { "Add User" }
      }

      if *show_form.read() {
        {log::info!("Rendering form");}
        OptimisticUserForm {
          on_success: move |_| {
            show_form.set(false);
            query_client.invalidate_query(String::from("users"));
          }
        }
      }

      {match users_query.result().value() {
        QueryResult::Loading(_) => rsx!{ div { class: "text-gray-500", "Loading users..." } },
        QueryResult::Err(e) => rsx!{ div { class: "text-red-500", "Error: {e:?}" } },
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
pub fn OptimisticUserForm(on_success: Option<EventHandler>) -> Element {
  let query_client: UseQueryClient<Vec<User>, UserError, String> = use_init_query_client();

  let mut error_message = use_signal(|| String::new());
  
  let create_mutation = use_mutation(|new_user: User| async move {
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    let mut users = USERS.write().await;
    
    if let Err(e) = new_user.validate() {
      return MutationResult::Err(UserError::ValidationError(e.to_string()));
    }
    
    users.insert(new_user.username.clone(), new_user.clone());
    MutationResult::Ok(new_user)
  });

  let handle_submit = move |(event, (user, is_valid)): (Event<FormData>, (User, bool))| {
    println!("user, is_valid");
    event.prevent_default();
    if !is_valid {
      error_message.set("Form validation failed".to_string());
      return;
    }

    println!("{:?}, {}", user, is_valid);

    // start mutation
    create_mutation.mutate(user.clone());
    
    // handle mutation result and invalidate query on success
    spawn(async move {
      while create_mutation.result().is_loading() {
        async_std::task::sleep(std::time::Duration::from_millis(50)).await;
      }
      
      if create_mutation.result().is_ok() {
        query_client.invalidate_query(String::from("users"));
        if let Some(handler) = on_success {
          handler.call(());
        }
      }
    });
  };

  rsx! {
    div {
      if !error_message().is_empty() {
        div { class: "text-red-500 mb-4", "{error_message}" }
      }
      Form {
        initial_value: User::default(),
        onsubmit: handle_submit,
        inner: form_content
      }
    }
  }
}

#[component]
pub fn CacheDemo() -> Element {
  let query_client: UseQueryClient<String, Error, String> = use_init_query_client();

  let cached_query = use_get_query([String::from("cached-data")], |_| async move {
    async_std::task::sleep(std::time::Duration::from_secs(2)).await;
    QueryResult::Ok::<String, Error>("This data is cached!".to_string())
  });

  let force_refetch = move |_| { 
    query_client.invalidate_query(String::from("cached-data"));  
  };

  rsx! {
    div { class: "p-4 border rounded mt-4",
      h3 { class: "text-xl font-bold mb-4", "Cache Demonstration" }
      
      div { class: "mb-4",
        p { "Cache Status: ",
          if cached_query.result().is_fresh() {
            span { class: "text-green-500", "Fresh" }
          } else {
            span { class: "text-yellow-500", "Stale" }
          }
        }
        p { "Query Status: ",
          if cached_query.result().is_loading() {
            span { class: "text-blue-500", "Loading..." }
          } else {
            span { class: "text-green-500", "Ready" }
          }
        }
      }

      div { class: "mb-4",
        match cached_query.result().value() {
          QueryResult::Loading(_) => rsx!{ "Fetching data..." },
          QueryResult::Ok(data) => rsx!{ "Data: {data}" },
          QueryResult::Err(e) => rsx!{ "Error: {e}" }
        }
      }

      Button {
        class: "bg-blue-500 text-white px-4 py-2 rounded",
        on_click: force_refetch,
        "Force Refetch"
      }
    }
  }
}


#[component]
pub fn SilentMutationDemo() -> Element {
  let mut counter = use_signal(|| 0);

  let silent_mutation = use_mutation(|value: i32| async move {
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    MutationResult::Ok::<i32, Error>(value)
  });

  let handle_normal_mutation = move |_| {
    let new_value = counter() + 1;
    counter.set(new_value);
    silent_mutation.mutate(new_value);
  };

  let handle_silent_mutation = move |_| {
    let new_value = counter() + 1;
    counter.set(new_value);
    let silent_mutation = silent_mutation.clone(); 
    spawn(async move {
      silent_mutation.mutate_silent(new_value).await;
    });
  };

  rsx! {
    div { class: "p-4 border rounded mt-4",
      h3 { class: "text-xl font-bold mb-4", "Silent vs Normal Mutations" }
      
      p { class: "mb-4", "Counter: {counter}" }
      
      div { class: "space-x-4",
          Button {
            class: "bg-blue-500 text-white px-4 py-2 rounded",
            on_click: handle_normal_mutation,
            "Normal Mutation"
          }
          
          Button {
            class: "bg-green-500 text-white px-4 py-2 rounded",
            on_click: handle_silent_mutation,
            "Silent Mutation"
          }
      }

      div { class: "mt-4",
        "Mutation Status: ",
        match *silent_mutation.result() {
          MutationResult::Loading(_) => "Loading...",
          MutationResult::Ok(_) => "Success",
          MutationResult::Err(_) => "Error",
          MutationResult::Pending => "Pending"
        }
      }
    }
  }
}


#[component]
pub fn ManualMutationDemo() -> Element {
  let mut status = use_signal(|| "Idle");
  
  let manual_mutation = use_mutation(|value: String| async move {
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    MutationResult::<std::string::String, Error>::Ok(value)
  });

  let handle_manual_mutation = move |_| {
    let mutation = manual_mutation.clone();
    status.set("Starting...");
    
    spawn(async move {
      status.set("Processing...");
      mutation.manual_mutate("Test".to_string()).await;
      status.set("Completed!");
    });
  };

  rsx! {
    div { class: "p-4 border rounded mt-4",
      h3 { class: "text-xl font-bold mb-4", "Manual Mutation Control" }
      
      p { class: "mb-4", "Status: {status}" }
      
      Button {
        class: "bg-blue-500 text-white px-4 py-2 rounded",
        on_click: handle_manual_mutation,
        "Trigger Manual Mutation"
      }
    }
  }
}


#[component]
pub fn ParallelQueriesDemo() -> Element {
  let query_client: UseQueryClient<Vec<String>, Error, String> = use_init_query_client();
  
  // first parallel query
  let departments_query = use_get_query([String::from("departments")], |_| async move {
    async_std::task::sleep(std::time::Duration::from_millis(500)).await;
    QueryResult::<Vec<std::string::String>, Error>::Ok(vec!["Engineering".to_string(), "Marketing".to_string()])
  });

  // second parallel query depending on departments
  let department_names = match departments_query.result().value() {
    QueryResult::Ok(deps) => deps.clone(),
    _ => Vec::new(),
  };

  let employees_query = use_get_query(
    [String::from("employees"), department_names.join(",")],
    |_| async move {
      async_std::task::sleep(std::time::Duration::from_millis(500)).await;
      QueryResult::<Vec<std::string::String>, Error>::Ok(vec!["John".to_string(), "Jane".to_string()])
    }
  );

  rsx! {
    div { class: "p-4 border rounded mt-4",
      h3 { class: "text-xl font-bold mb-4", "Parallel Queries with Dependencies" }
      
      div { class: "grid grid-cols-2 gap-4",
        div { class: "p-4 bg-gray-100 rounded",
          h4 { class: "font-bold mb-2", "Departments" }
          match departments_query.result().value() {
            QueryResult::Loading(_) => rsx!{ "Loading departments..." },
            QueryResult::Ok(deps) => rsx!{
              ul {
                {deps.iter().map(|dep| rsx!(
                  li { key: "{dep}", "{dep}" }
                ))}
              }
            },
            QueryResult::Err(_) => rsx!{ "Error loading departments" }
          }
        }

        div { class: "p-4 bg-gray-100 rounded",
          h4 { class: "font-bold mb-2", "Employees" }
          match employees_query.result().value() {
            QueryResult::Loading(_) => rsx!{ "Loading employees..." },
            QueryResult::Ok(emps) => rsx!{
              ul {
                {emps.iter().map(|emp| rsx!(
                  li { key: "{emp}", "{emp}" }
                ))}
              }
            },
            QueryResult::Err(_) => rsx!{ "Error loading employees" }
          }
        }
      }

      Button {
        class: "mt-4 bg-blue-500 text-white px-4 py-2 rounded",
        on_click: move |_| query_client.invalidate_query(String::from("departments")),
        "Refresh Departments"
      }
    }
  }
}


#[component]
pub fn CompleteQueryDemo() -> Element {
  rsx! {
    div { class: "container mx-auto p-4",
      h1 { class: "text-3xl font-bold mb-8", "Maestro Query Demonstrations" }
      
      // Users Demo
      BasicQueryDemo {}
      
      // Cache Demo
      CacheDemo {}
      
      // Silent Mutation Demo
      SilentMutationDemo {}
      
      // Manual Mutation Demo
      ManualMutationDemo {}
    }
  }
}
