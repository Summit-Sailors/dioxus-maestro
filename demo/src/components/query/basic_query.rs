use {
  crate::models::user::{Role, User}, 
  async_std::task::sleep, 
  dioxus::{fullstack::once_cell::sync::Lazy, prelude::*}, 
  maestro_query::prelude::*, 
  maestro_toast::{ctx::use_toast, toast_code::EToastCode, toast_info::ToastInfo, toast_position::EToastPosition}, 
  maestro_ui::button::Button, 
  std::{collections::HashMap, fmt::Error, sync::{Arc, RwLock}, time::Duration}
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
  sleep(Duration::from_millis(500)).await; // network delay sim
  
  match USERS.read() {
    Ok(users) => match users.get(&username) {
      Some(user) => QueryResult::Ok(user.clone()),
      None => QueryResult::Err("User not found".into()),
    },
    Err(_) => QueryResult::Err("Failed to read users".into()),
  }
}

async fn fetch_users_by_role(role: Role) -> QueryResult<Vec<User>, String> {
  sleep(Duration::from_millis(500)).await;
  
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
  sleep(Duration::from_millis(500)).await;
  
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
pub fn QueryDemo() -> Element {
  let mut toast = use_toast();
  let query_client: UseQueryClient<Vec<User>, Error, String> = use_init_query_client();
  
  // demonstrate type-safe query registry and caching
  let admin_query = use_get_query(["admin"], |keys| async move {
    fetch_user(keys[0].to_string()).await
  });
  
  // demonstrate multi-key query system and query freshness
  let moderators = use_get_query([Role::Moderator], |_| async move {
    fetch_users_by_role(Role::Moderator).await
  });
  
  // demonstrate both manual and automatic mutations
  let update_mutation = use_mutation(|user: User| async move {
    update_user(user).await
  });

  let admin_query_clone = admin_query.clone();
  
  // demonstration of silent vs regular mutations
  let mut handle_role_update = move |username: String, new_role: Role| {
    let username_clone = username.clone();
    // properly extract the User from QueryResult
    match admin_query_clone.result().value() {
      QueryResult::Ok(user) => {
        let mut updated = user.clone();
        updated.role = new_role;
        
        // now we're passing a User directly
        let _ = update_mutation.mutate_silent(updated);
        
        // invalidate queries after mutation
        query_client.invalidate_queries(&[username, Role::Moderator.to_string()]);
        
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
      }
    }
  };

  rsx! {
    div { class: "p-4 space-y-4",
    h3 { class: "text-2xl text-gray-800 text-center font-bold mb-4", "Main Dioxus Query Demo" }
      // demonstrate intelligent loading states
      div { class: "space-y-2",
        h2 { class: "text-xl font-bold", "Admin User" }
        {match admin_query.result().value().to_owned() {
          QueryResult::Loading(Some(prev)) => rsx! {
            div { class: "opacity-50", // show previous data while loading
              "Loading... Previous data:"
              div { "Username: {prev.username}" }
              div { "Role: {prev.role}" }
            }
          },
          QueryResult::Loading(None) => rsx! { div { "Loading..." } },
          QueryResult::Ok(user) => rsx! {
            div {
              div { "Username: {user.username}" }
              div { "Role: {user.role}" }
              Button {
                class: "px-4 py-2 bg-blue-500 text-white rounded",
                on_click: move |_| handle_role_update(user.username.clone(), Role::Moderator),
                "Change to Moderator"
              }
            }
          },
          QueryResult::Err(err) => rsx! {
            div { class: "text-red-500", "Error: {err}" }
          }
        }}
      }

    // demonstrate multi-key query system
    div { class: "space-y-2",
      h2 { class: "text-xl font-bold", "Moderators" }
      {match moderators.result().value() {
        QueryResult::Loading(_) => rsx! { div { "Loading moderators..." } },
        QueryResult::Ok(users) => rsx! {
          div { class: "space-y-2",
            {users.iter().map(|user| rsx! {
              div { key: "{user.username}",
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
