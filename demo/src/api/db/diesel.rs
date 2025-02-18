use {
  diesel::prelude::*,
  diesel_async::RunQueryDsl,
  serde::{Deserialize, Serialize},
  std::str::FromStr,
  dioxus::prelude::*,
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

// in a schema.rs file (db schema)
table! {
  users (id) {
    id -> Integer,
    username -> Text,
    email -> Text,
    bio -> Text,
    age -> Integer,
    role -> Text,
  }
}

#[allow(non_snake_case)]
#[component]
fn DieselDemo() -> Element {
    let users = use_signal( || Vec<User>);
    let loading = use_signal( || true);
    let error = use_signal( || None::<String>);

    let page_size = 10;
    let (pagination, (mut next_idx, mut prev_idx, mut next_page, mut prev_page)) =
      use_pagination(use_memo(move || total_items()), page_size);
    
    use_effect( (page, page_size), |(current_page, current_page_size)| {
      to_owned![users, loading, error, total_pages];
      async move {
        loading.set(true);
        
        // paginated users using the async API
        match fetch_users_paginated(*current_page, *current_page_size).await {
          Ok(paginated_result) => {
            users.set(paginated_result.records);
            total_pages.set(paginated_result.total_pages as i32);
            loading.set(false);
          }
          Err(err) => {
            error.set(Some(format!("Error: {}", err)));
            loading.set(false);
          }
        }
        
        // the sync API for comparison (logs only)
        if let Err(e) = std::thread::spawn(move || {
          match fetch_users_sync(*current_page, *current_page_size) {
            Ok(sync_result) => {
              println!("Sync fetch returned {} users", sync_result.records.len());
            }
            Err(err) => {
              println!("Sync fetch error: {}", err);
            }
          }
        }).join() {
          println!("Failed to join thread: {:?}", e);
        }
      }
    });

    // page navigation
    let go_to_page = move |new_page: i32| {
      if new_page >= 1 && new_page <= *total_pages.get() {
        page.set(new_page);
      }
    };

    rsx! {
        div {
            class: "container mx-auto p-4",
            h1 { class: "text-2xl font-bold mb-4", "Maestro-Diesel Demo with Pagination" }
            
            if *loading.get() {
              div { class: "text-blue-500", "Loading users..." }
            } else if let Some(err) = error.get() {
              div { class: "text-red-500", "Error: {err}" }
            } else {
              div {
                h2 { class: "text-xl font-semibold mb-2", "Users (Page {page} of {total_pages})" }
                
                // user list
                ul {
                  class: "list-none divide-y divide-gray-200",
                  users.get().iter().map(|user| {
                    rsx! {
                      li {
                        key: "{user.username}",
                        class: "py-4",
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
                      }
                    }
                  })
                }
                
                // pagination controls
                div {
                  class: "flex justify-between items-center mt-4",
                  button {
                    class: "px-4 py-2 border rounded {if *page.get() == 1 { 'opacity-50 cursor-not-allowed' } else { 'hover:bg-gray-100' }}",
                    disabled: *page.get() == 1,
                    onclick: move |_| go_to_page(*page.get() - 1),
                    "Previous"
                  }
                  
                  div {
                    class: "flex space-x-2",
                    // first page
                    if *page.get() > 3 {
                      button {
                        class: "w-10 h-10 rounded-full border flex items-center justify-center hover:bg-gray-100",
                        onclick: move |_| go_to_page(1),
                        "1"
                      }
                    }
                    
                    // ellipsis if needed
                    if *page.get() > 4 {
                      span { class: "w-10 h-10 flex items-center justify-center", "..." }
                    }
                    
                    // page numbers around current page
                    ((*page.get() - 2).max(1)..=(*page.get() + 2).min(*total_pages.get())).map(|p| {
                      rsx! {
                        button {
                          key: "{p}",
                          class: "w-10 h-10 rounded-full border flex items-center justify-center {if p == *page.get() { 'bg-blue-500 text-white' } else { 'hover:bg-gray-100' }}",
                          onclick: move |_| go_to_page(p),
                          "{p}"
                        }
                      }
                    })
                    
                    // ellipsis if needed
                    if *page.get() < *total_pages.get() - 3 {
                      span { class: "w-10 h-10 flex items-center justify-center", "..." }
                    }
                    
                    // last page
                    if *page.get() < *total_pages.get() - 2 && *total_pages.get() > 3 {
                      button {
                        class: "w-10 h-10 rounded-full border flex items-center justify-center hover:bg-gray-100",
                        onclick: move |_| go_to_page(*total_pages.get()),
                        "{total_pages}"
                      }
                    }
                  }
                  
                  button {
                    class: "px-4 py-2 border rounded {if *page.get() == *total_pages.get() { 'opacity-50 cursor-not-allowed' } else { 'hover:bg-gray-100' }}",
                    disabled: *page.get() == *total_pages.get(),
                    onclick: move |_| go_to_page(*page.get() + 1),
                    "Next"
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
        .map(|record| User {
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
        .map(|record| User {
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
    
    let transaction_result = conn.transaction(|tx| {
        Box::pin(async move {
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
    }).await;
    
    transaction_result.map_err(|e| ServerFnError::ServerError(format!("Transaction failed: {}", e)))
}
