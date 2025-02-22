use {
  diesel::prelude::*, diesel_async::RunQueryDsl, dioxus::prelude::*, maestro_hooks::pagination::use_pagination, serde::{Deserialize, Serialize}, std::str::FromStr
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct User {
  username: String,
  email: String,
  bio: String,
  age: i32,
  role: Role,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
enum Role {
  Admin,
  User,
  Moderator,
}

// illustration, representing a user record in the database
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = users)]
struct UserRecord {
  id: i32,
  username: String,
  email: String,
  bio: String,
  age: i32,
  role: String,
}

#[allow(non_snake_case)]
#[component]
fn DieselDemo() -> Element {
  let users = use_signal(|| Vec::<User>::new());
  let loading = use_signal(|| true);
  let error = use_signal(|| None::<String>);
  let total_items = use_signal(|| 0);

  // desired page size
  let page_size = 10;
  let (pagination, (mut next_idx, mut prev_idx, mut next_page, mut prev_page, mut set_page_size)) =
    use_pagination(use_memo(move || total_items()), page_size);
  
  // to fetch data when page changes
  use_effect(move || {
    to_owned![users, loading, error, total_items];
    async move {
      loading.set(true);
      
      match fetch_users_paginated(pagination.page() + 1, pagination.page_size()).await {
        Ok(paginated_result) => {
          users.set(paginated_result.records);
          // updating total items count - this could come from total_records in the response
          total_items.set(paginated_result.total_records);
          loading.set(false);
        }
        Err(err) => {
          error.set(Some(format!("Error: {}", err)));
          loading.set(false);
        }
      }
    }
  });

  // to get the currently focused user
  let current_user = use_memo(move || {
    let users_array = users.current();
    let current_idx = (pagination.idx)() % (pagination.page_size)();
    
    if users_array.len() > current_idx as usize {
      Some(users_array[current_idx as usize].clone())
    } else {
      None
    }
  });

  rsx! {
    div {
      class: "container mx-auto p-4",
      h1 { class: "text-2xl font-bold mb-4", "Users with Pagination" }
      
      if *loading() {
        div { class: "text-blue-500", "Loading users..." }
      } else if let Some(err) = error() {
        div { class: "text-red-500", "Error: {err}" }
      } else {
        div {
          // pagination info
          h2 { 
            class: "text-xl font-semibold mb-2", 
            format!("Users ({})", pagination.counter_label) 
          }
          
          // item navigation controls
          div {
            class: "flex items-center space-x-2 mb-4",
            button {
              class: tw_join!(
                "px-3 py-1 border rounded"
                if pagination.prev_idx_disabled() { "opacity-50 cursor-not-allowed" } else { "hover:bg-gray-100" }
              ),
              disabled: pagination.prev_idx_disabled(),
              onclick: move |_| prev_idx(),
              "Previous Item"
            }
            
            span { class: "text-sm", "Item: {pagination.idx() + 1}" }
            
            button {
              class: tw_join!(
                "px-3 py-1 border rounded",
                if pagination.next_idx_disabled() { "opacity-50 cursor-not-allowed" } else { "hover:bg-gray-100" }
              ),
              disabled: pagination.next_idx_disabled(),
              onclick: move |_| next_idx(),
              "Next Item"
            }
          }
          
          // currently focused user (if available)
          if let Some(user) = current_user() {
            div {
              class: "bg-yellow-50 p-4 border border-yellow-200 rounded-lg mb-4",
              h3 { class: "text-lg font-medium", "Currently Selected: {user.username}" }
              p { class: "text-sm text-gray-600", "{user.email}" }
              p { class: "mt-1 text-sm", "{user.bio}" }
              p { class: "mt-1 text-xs text-gray-500", "Age: {user.age}" }
              span {
                class: match user.role {
                  Role::Admin => "bg-red-100 text-red-800",
                  Role::Moderator => "bg-yellow-100 text-yellow-800",
                  Role::User => "bg-green-100 text-green-800",
                },
                class: "{class} px-2 py-1 text-xs font-semibold rounded-full",
                "{user.role:?}"
              }
            }
          }
          
          // user list
          ul {
            class: "list-none divide-y divide-gray-200",
            {users().iter().enumerate().map(|(i, user)| {
              // the absolute index for this user in the current page
              let absolute_idx = pagination.page() * pagination.page_size() + i as i32;
              // if this is the currently selected user
              let is_selected = absolute_idx == pagination.idx();
              
              rsx! {
                li {
                  key: "{user.username}",
                  class: tw_join!(
                    "py-4"
                    if is_selected { "bg-blue-50 border-l-4 border-blue-500 pl-2" } else { "" }
                  ),
                  div {
                    class: "flex justify-between",
                    div {
                      p { class: "font-medium", "{user.username}" }
                      p { class: "text-sm text-gray-600", "{user.email}" }
                    }
                    div {
                      span {
                        class: match user.role {
                          Role::Admin => "bg-red-100 text-red-800",
                          Role::Moderator => "bg-yellow-100 text-yellow-800",
                          Role::User => "bg-green-100 text-green-800",
                        },
                        class: "{class} px-2 py-1 text-xs font-semibold rounded-full",
                        "{user.role:?}"
                      }
                    }
                  }
                  p { class: "mt-1 text-sm", "{user.bio}" }
                  p { class: "mt-1 text-xs text-gray-500", "Age: {user.age}" }
                  
                  // button to select this specific item
                  button {
                    class: "mt-2 px-2 py-1 text-xs text-blue-600 hover:text-blue-800",
                    onclick: move |_| {
                      pagination.idx.set(absolute_idx);
                    },
                    "Select"
                  }
                }
              }
            })}
          }
          
          // page navigation controls
          div {
            class: "flex justify-between items-center mt-4",
            
            // prev
            button {
              class: "px-4 py-2 border rounded {if pagination.prev_page_disabled() { 'opacity-50 cursor-not-allowed' } else { 'hover:bg-gray-100' }}",
              disabled: pagination.prev_page_disabled(),
              onclick: move |_| prev_page(),
              "Previous Page"
            }
            
            // page indicator
            span { 
              class: "text-sm font-medium", 
              format!("Page {}", pagination.page() + 1) 
            }
            
            // next
            button {
              class: "px-4 py-2 border rounded {if pagination.next_page_disabled() { 'opacity-50 cursor-not-allowed' } else { 'hover:bg-gray-100' }}",
              disabled: pagination.next_page_disabled(),
              onclick: move |_| next_page(),
              "Next Page"
            }
          }
        }
      }
    }
  }
}

// server function to handle async pagination
#[server]
async fn fetch_users_paginated(page: i32, page_size: i32) -> Result<PaginatedResultDTO<User>, ServerFnError> {
  // the pool from the Dioxus context
  let pool = extract_diesel_pool().await?;
  
  // pagination request with validation
  let pagination_request = PaginationRequestDTO {
    page,
    page_size,
    query: (), // no additional query
  };
  
  // the diesel query builder with pagination
  let query = users::table.into_boxed();
  
  // applying pagination using the PaginateAsync trait
  let paginated_results = query
    .paginate(pagination_request.page, pagination_request.page_size)
    .aload_paginated::<UserRecord>(pool)
    .await
    .map_err(|e| ServerFnError::ServerError(format!("Database error: {}", e)))?;
  
  // UserRecord to User model
  let users = paginated_results.records
    .into_iter()
    .map(|record| &User {
      username: record.username,
      email: record.email,
      bio: record.bio,
      age: record.age,
      role: Role::from_str(&record.role).unwrap_or(Role::User),
    })
    .collect::<Vec<User>>();
  
  Ok(PaginatedResultDTO {
    records: users,
    total_pages: paginated_results.total_pages,
    has_more: paginated_results.has_more,
    current_page: paginated_results.current_page,
  })
}

// sync version for comparison
fn fetch_users_sync(page: i32, page_size: i32) -> Result<PaginatedResultDTO<User>, anyhow::Error> {
  // connection from a synchronous pool
  let pool = create_db_pool_diesel("postgres://username:password@localhost/database");
  let mut conn = pool.get()?;
  
  // the sync pagination extension
  use maestro_diesel::extensions::pagination::paginate_sync::Paginate;
  
  let paginated_results = users::table
    .paginate::<UserRecord>(page, page_size, &mut conn)?;
  
  // UserRecord to User model
  let users = paginated_results.records
    .into_iter()
    .map(|record| &User {
        username: record.username,
        email: record.email,
        bio: record.bio,
        age: record.age,
        role: Role::from_str(&record.role).unwrap_or(Role::User),
    })
    .collect::<Vec<User>>();
  
  Ok(PaginatedResultDTO {
    records: users,
    total_pages: paginated_results.total_pages,
    has_more: paginated_results.has_more,
    current_page: paginated_results.current_page,
  })
}

#[server]
async fn create_user_in_transaction(new_user: User) -> Result<(), ServerFnError> {
  let pool = extract_diesel_pool().await?;
  let mut conn = pool.get().await.map_err(|e| ServerFnError::ServerError(e.to_string()))?;

  let transaction_result = conn
    .transaction(|tx| async move {
      diesel::insert_into(users::table)
        .values((
          users::username.eq(&new_user.username),
          users::email.eq(&new_user.email),
          users::bio.eq(&new_user.bio),
          users::age.eq(&new_user.age),
          users::role.eq(&format!("{:?}", new_user.role).to_lowercase()),
        ))
        .execute(tx)
        .await?;

      // additional operations within the same transaction

      Ok::<_, diesel::result::Error>(())
    })
    .await;

  transaction_result.map_err(|e| ServerFnError::ServerError(format!("Transaction failed: {}", e)))
}
