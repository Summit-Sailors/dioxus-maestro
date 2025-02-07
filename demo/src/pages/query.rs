use {
  crate::{
    components::forms::form_field_wrapper::FormFieldWrapper, 
    models::user::{Role, User}
  }, async_std::sync::RwLock, dioxus::{fullstack::once_cell, prelude::*}, maestro_forms::fields::{
    form::{
      Form, 
      InnerComponentProps
    }, 
    select::SelectFormField, 
    text::TextFormInput, 
    textarea::TextArea
  }, maestro_query::prelude::*, maestro_ui::button::{
    Button, ButtonSize, ButtonType, ButtonVariant}, std::{
    collections::HashMap, fmt::Error, sync::Arc
  }, strum::VariantNames, validator::Validate
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
    delete_mutation.mutate(username);
    if delete_mutation.result().is_ok() {
      query_client.invalidate_query(String::from("users"));
    }
  };

  let mut show_form = use_signal(|| false);

  rsx! {
    div { class: "container mx-auto p-4",
      h2 { class: "text-2xl font-bold mb-4", "Users List" }

      Button {
        class: "bg-blue-500 text-white px-4 py-2 rounded mb-4",
        on_click: move |_| {
          let current_show_form = show_form();
          show_form.set(!current_show_form);
        },
        if show_form() { "Hide Form" } else { "Add User" }
      }

      if show_form() {
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
                  button {
                    class: "mt-2",
                    r#type: "button",
                    onclick: move |_| handle_delete(user.username.clone()),
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

#[component]
pub fn FormContent(props: InnerComponentProps<User>) -> Element {
  let loading = use_signal(|| false);
  
  let role_values = Role::VARIANTS.iter().map(|&s| s.to_string()).collect::<Vec<_>>();
  let role_labels = Role::VARIANTS.iter().map(|&s| s.to_string()).collect::<Vec<_>>();

  let input_class = "w-full p-2 rounded-md border border-gray-300 shadow-sm focus:ring-2 focus:ring-blue-400 focus:outline-none placeholder-opacity-50";
  
  rsx! {
    div { 
      class: "space-y-6",
      FormFieldWrapper {
        label: "Username",
        field: props.form.get_form_field("username".to_string()),
        show_validation: true,
        TextFormInput::<User> {
          name: "username",
          placeholder: "Enter your username",
          class: "{input_class}",
        }
      }

      FormFieldWrapper {
        label: "Email",
        field: props.form.get_form_field("email".to_string()),
        show_validation: true,
        TextFormInput::<User> {
          name: "email",
          placeholder: "Enter your email address",
          class: "{input_class}",
        }
      }

      FormFieldWrapper {
        label: "Bio",
        field: props.form.get_form_field("bio".to_string()),
        show_validation: true,
        TextArea::<User> {
          name: "bio",
          placeholder: "Tell us about yourself...",
          rows: 4,
          class: "{input_class}",
        }
      }

      FormFieldWrapper {
        label: "Role",
        field: props.form.get_form_field("role".to_string()),
        show_validation: true,
        SelectFormField::<User, String> {
          name: "role",
          values: role_values,
          labels: Some(role_labels),
          class: "w-full p-2 rounded-md border border-gray-300 focus:ring focus:ring-blue-300 focus:outline-none",
        }
      }

      Button {
				button_type: ButtonType::Submit,
				disabled: loading(),
				prevent_default: false,
				size: ButtonSize::Default,
				variant: ButtonVariant::Default,
				class: format!(
          "mt-4 py-2 rounded-md text-white font-semibold transition-all duration-200 {}",
          if loading() {
            "bg-gray-400 cursor-not-allowed opacity-70"
          } else {
            "bg-blue-500 hover:bg-blue-600 transform hover:scale-105"
          },
				),
				if loading() {
					"Loading..."
				} else {
					"Submit"
				}
			}
    }
  }
}


#[component]
pub fn OptimisticUserForm(on_success: Option<EventHandler>) -> Element {
  let query_client: UseQueryClient<Vec<User>, UserError, String> = use_init_query_client();
  let mut error_message = use_signal(|| String::new());
  
  let create_mutation = use_mutation(|new_user: User| async move {
    let mut users = USERS.write().await;
    
    if let Err(e) = new_user.validate() {
      return MutationResult::Err(UserError::ValidationError(e.to_string()));
    }

    log::info!("Adding user: {:?}", new_user);
    users.insert(new_user.username.clone(), new_user.clone());
    MutationResult::Ok(new_user)
  });

  let handle_submit = move |(_event, (user, is_valid)): (Event<FormData>, (User, bool))| {
    if !is_valid {
      error_message.set("Form validation failed".to_string());
      return;
    }

    create_mutation.mutate(user);
    
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
        log::error!("Mutation failed: {:?}", create_mutation.result());
        // error message setting
      }
    });
  };

  rsx! {
    div {
      if !error_message().is_empty() {
        div { class: "text-red-500 mb-4", "{error_message}" }
      }
      Form {
        initial_values: User::default(),
        onsubmit: handle_submit,
        inner: FormContent
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
      
      div { class: "space-x-2",
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
    div { 
      class: "min-h-screen w-full flex flex-col items-center justify-start",
      
      div { 
        class: "w-full max-w-4xl px-4 py-8",
        
        div { 
          class: "text-center mb-8",
          h1 { 
            class: "text-3xl font-bold",
            "Maestro Query Demonstrations" 
          }
        }

        div {
          class: "space-y-8",
          
          // Users Demo
          div { 
            class: "w-full",
            BasicQueryDemo {}
          }
          
          // Cache Demo
          div { 
            class: "w-full",
            CacheDemo {}
          }
          
          // Silent Mutation Demo
          div { 
            class: "w-full",
            SilentMutationDemo {}
          }
          
          // Manual Mutation Demo
          div { 
            class: "w-full",
            ManualMutationDemo {}
          }
        }
      }
    }
  }
}
