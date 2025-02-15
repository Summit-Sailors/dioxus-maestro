use {
  crate::models::user::{Role, User}, 
  async_std::task::sleep, 
  dioxus::{fullstack::once_cell::sync::Lazy, prelude::*}, 
  maestro_query::prelude::*, 
  maestro_toast::{ctx::use_toast, toast_code::EToastCode, toast_info::ToastInfo, toast_position::EToastPosition}, 
  maestro_ui::button::Button, 
  std::{collections::HashMap, sync::{Arc, RwLock}, time::Duration}, tailwind_fuse::tw_join
};

// mock backend store
static USERS: Lazy<Arc<RwLock<HashMap<String, User>>>> = Lazy::new(|| {
  let mut map = HashMap::new();
  map.insert(
    "admin".to_string(),
    User {
      username: "admin".to_string(),
      email: "admin@example.com".to_string(),
      age: 30,
      bio: "this is the admin's bio".to_string(),
      role: Role::Admin,
    },
  );
  map.insert(
    "mod1".to_string(),
    User {
      username: "mod1".to_string(),
      email: "mod1@example.com".to_string(),
      age: 25,
      bio: "moderator number one".to_string(),
      role: Role::Moderator,
    },
  );
  map.insert(
    "user1".to_string(),
    User {
      username: "user1".to_string(),
      email: "user1@example.com".to_string(),
      age: 22,
      bio: "regular user here".to_string(),
      role: Role::User,
    },
  );
  Arc::new(RwLock::new(map))
});

// mock api functions
async fn fetch_user(username: String) -> QueryResult<User, String> {
  sleep(Duration::from_millis(2000)).await; // network delay sim
  
  match USERS.read() {
    Ok(users) => match users.get(&username) {
      Some(user) => QueryResult::Ok(user.clone()),
      None => QueryResult::Err("User not found".into()),
    },
    Err(_) => QueryResult::Err("Failed to read users".into()),
  }
}

async fn fetch_users_by_role(role: Role) -> QueryResult<Vec<User>, String> {
  sleep(Duration::from_millis(2000)).await;
  
  match USERS.read() {
    Ok(users) => {
      let filtered = users
        .values()
        .filter(|user| user.role == role)
        .cloned()
        .collect();
      QueryResult::Ok(filtered)
    },
    Err(_) => QueryResult::Err("Failed to read users".into()),
  }
}

async fn update_user(user: User) -> MutationResult<User, String> {
  sleep(Duration::from_millis(2000)).await;
  
  match USERS.write() {
    Ok(mut users) => {
      if let Some(existing) = users.get_mut(&user.username) {
        *existing = user.clone();
        MutationResult::Ok(user)
      } else {
        MutationResult::Err("User not found".into())
      }
    },
    Err(_) => MutationResult::Err("Failed to update user".into()),
  }
}

#[component]
pub fn QueryDemoWrapper() -> Element {
  rsx! {
    // provider for single user queries
    SingleUserQueryProvider {
      // provider for user list queries (moderators)
      UserListQueryProvider {
        QueryDemo {}
      }
    }
  }
}

// provider for single user queries
#[component]
fn SingleUserQueryProvider(children: Element) -> Element {
  let _query_client: UseQueryClient<User, String, String> = use_init_query_client();
  
  rsx! {
    {children}
  }
}

// provider for user list queries
#[component]
fn UserListQueryProvider(children: Element) -> Element {
  let _query_client: UseQueryClient<Vec<User>, String, Role> = use_init_query_client();
  
  rsx! {
    {children}
  }
}

#[component]
pub fn QueryDemo() -> Element {
  let mut toast = use_toast();
  let mut is_loading = use_signal(|| false);
  // appropriate query clients for each type
  let single_user_client = use_query_client::<User, String, String>();
  let user_list_client = use_query_client::<Vec<User>, String, Role>();
  
  // single user query
  let admin_query = use_get_query([String::from("admin")], |keys| async move {
    fetch_user(keys[0].clone()).await
  });
  
  // user list query (moderators)
  let moderators = use_get_query([Role::Moderator], |_| async move {
    fetch_users_by_role(Role::Moderator).await
  });
  
  let update_mutation = use_mutation(|user: User| async move {
    update_user(user).await
  });

  let admin_query_clone = admin_query.clone();
  
  let handle_role_update = move |username: String, current_role: Role| {
    let username_clone = username.clone();

    let new_role = if current_role == Role::Admin { Role::Moderator } else { Role::Admin };

    spawn(async move {
      is_loading.set(true);
      match admin_query_clone.result().value() {
        QueryResult::Ok(user) => {
          let mut updated = user.clone();
          updated.role = new_role;
          
          let _ = update_mutation.mutate_silent(updated).await;

          is_loading.set(false);
          
          single_user_client.invalidate_query(username);
          user_list_client.invalidate_query(Role::Moderator);
          
          toast.write().popup(ToastInfo {
            heading: Some("Role Updated".into()),
            context: format!("Updated role for {}", username_clone),
            icon: Some(EToastCode::Success),
            position: EToastPosition::TopRight,
            allow_toast_close: true,
            hide_after: 5,
          });
        },
        _ => {
          toast.write().popup(ToastInfo {
            heading: Some("Error".into()),
            context: "Failed to update user role: User data not available".into(),
            icon: Some(EToastCode::Error),
            position: EToastPosition::BottomRight,
            allow_toast_close: true,
            hide_after: 8,
          });

          // revert UI on failure
        }
      }
    });
  };

  rsx! {
    div {
      class: "flex justify-center items-center rounded-lg shadow-lg bg-gray-100",
      div {
        class: "flex flex-col items-center bg-white rounded-lg shadow-lg w-full p-4 max-w-lg",
        
        h3 { 
          class: "text-2xl text-gray-800 text-center font-bold mb-4", 
          "Main Dioxus Query Demo" 
        }

        div { 
          class: "w-full text-center text-gray-700 p-4",
          h2 { class: "text-xl font-bold", "Admin User:" }

          {match admin_query.result().value().to_owned() {
            QueryResult::Loading(Some(prev)) => rsx! {
              div { 
                class: "opacity-50",
                "Loading... Previous data:"
                div { "Username: {prev.username}" }
                div { "Role: {prev.role}" }
              }
            },
            QueryResult::Loading(None) => rsx! { 
              div { class: "text-gray-500", "Loading..." } 
            },
            QueryResult::Ok(user) => rsx! {
              div {
                div { "Username: {user.username}" }
                div { "Role: {user.role}" }
                Button {
                  class: tw_join!(
                    "bg-blue-500 text-white px-4 py-2 rounded mt-2",
                    if user.role == Role::Admin { "bg-red-500 text-white" } else { "bg-blue-500 text-white" }
                  ),
                  on_click: move |_| handle_role_update.clone()(user.username.clone(), user.clone().role),
                  if is_loading() {
                    "Updating..."
                  } else {
                    "Toggle Admin Role"
                  }
                }
              }
            },
            QueryResult::Err(err) => rsx! {
              div { class: "text-red-500", "Error: {err}" }
            }
          }}
        }

        div { 
          class: "w-full text-center mt-4 p-4",
          h2 { class: "text-xl font-bold text-gray-700", "Moderators:" }

          {match moderators.result().value() {
            QueryResult::Loading(_) => rsx! { 
              div { class: "text-gray-500", "Loading moderators..." } 
            },
            QueryResult::Ok(users) => rsx! {
                div { class: "space-y-2",
                  {users.iter().map(|user| rsx! {
                    div {
                      class: "text-gray-700 font-medium",
                      key: "{user.username}",
                      "Username: {user.username}"
                    }
                  })}
                }
            },
            QueryResult::Err(err) => rsx! {
              div { class: "text-red-500", "Error loading moderators: {err}" }
            }
          }}
        }
      }
    }
  }
}
