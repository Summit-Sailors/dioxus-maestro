use {
  crate::models::user::User, dioxus::prelude::*, maestro_sqlx::{ acreate::acreate_sqlx_pool, create::create_sqlx_pool }, sqlx::{PgPool, Postgres}, std::env
};

#[server]
pub async fn create_user(user: User, pool: PgPool) -> Result<(), ServerFnError>{
  sqlx::query::<Postgres>(
    "INSERT INTO users (username, email, age, bio, role) VALUES ($1, $2, $3, $4, $5)
    ")
    .bind(user.username)
    .bind(user.email)
    .bind(user.age)
    .bind(user.bio)
    .bind(user.role)
    .execute(&pool);
		Ok(())
}

#[server]
async fn fetch_users_async() -> Result<Vec<User>, ServerFnError> {
  // creating a pool asynchronously
  let pool = acreate_sqlx_pool(env::var("DATABASE_URL")?.as_str()).await;
  let rows: Vec<User> = sqlx::query_as(
    r#"
      SELECT username, email, bio, age, role
      FROM users
      ORDER by age
      LIMIT 10 
    "#
  ).fetch_all(&pool)
  .await?;

  let users = rows.iter().map(|row| User {
    username: row.username,
    email: row.email,
    bio: row.bio,
    age: row.age,
    role: row.role.to_string(),
    id: todo!(),
  }).collect();

  Ok(users)
}



#[server]
pub async fn fetch_users_sync() -> Result<Vec<User>, ServerFnError> {
  use tokio::runtime::Runtime;

  let pool = create_sqlx_pool(env::var("DATABASE_URL")?.as_str());
  
  let rows = {
    async fn execute_query(pool: ) -> Result<Vec<dyn sqlx::Row>, sqlx::Error> {
      sqlx::query(
        r#"
          SELECT username, email, bio, age, role
          FROM users
          ORDER by age
          LIMIT 10 
        "#,
      )
      .fetch_all(pool);
    }

    match Handle::try_current() {
      Ok(handle) => handle.block_on(execute_query(&pool)).map_err(ServerFnError::from)?,
      Err(_) => {
        let rt = Runtime::new().map_err(ServerFnError::from)?;
        rt.block_on(execute_query(&pool)).map_err(ServerFnError::from)?
      }
    }
  };

  let users = rows.(&mut pool).map(|row: User| &User {username:row.username,email:row.email,bio:row.bio,age:row.age,role:row.role, id: todo!() }).collect();

  Ok(users)
}



#[derive(Debug, Clone, PartialEq)]
struct User {
  id: i64,
  username: String,
  email: String,
  bio: String,
  age: i32,
  role: String,
}

#[component]
fn SqlxDemo() -> Element {
    let users = use_signal(|| Vec<User>);
    let loading = use_signal(|| true);
    let error = use_signal(|| None::<String>);

    // users onmount - shows both sync and async methods
    use_effect(|_| {
      to_owned![users, loading, error];
      async move {
        // 1. using async pool creation
        // preferred method when in an async context
        let result = fetch_users_async().await;
        
        match result {
          Ok(fetched_users) => {
            users.set(fetched_users);
            loading.set(false);
          }
          Err(err) => {
            error.set(Some(format!("Async fetch error: {}", err)));
            loading.set(false);
          }
        }
        
        // the sync approach
        // in a real application, you'd use one or the other
        match fetch_users_sync() {
          Ok(sync_users) => {
            println!("Sync fetch returned {} users", sync_users.len());
          }
          Err(err) => {
            println!("Sync fetch resulted in error: {}", err);
          }
        }
      }
    });

    rsx! {
        div {
            class: "container mx-auto p-4",
            h1 { class: "text-2xl font-bold mb-4", "Maestro-SQLx Demo" }
            
            if *loading() {
                div { class: "text-blue-500", "Loading users..." }
            } else if let Some(err) = error.read() {
                div { class: "text-red-500", "Error: {err}" }
            } else {
                div {
                    h2 { class: "text-xl font-semibold mb-2", "Users" }
                    ul {
                        class: "list-disc pl-6",
                        users.read().iter().map(|user| {
                            rsx! {
                                li {
                                    key: "{user.id}",
                                    p { "Username: {user.username}" }
                                    p { "Email: {user.email}" }
                                    p { "Role: {user.role}" }
                                }
                            }
                        })
                    }
                }
            }
        }
    }
}

// asynchronous user fetching with async pool creation
#[server]
async fn fetch_users_async() -> Result<Vec<User>, ServerFnError> {
    let pool = acreate_sqlx_pool("postgres://username:password@localhost/database").await;
    
    let rows = sqlx::query!(
        r#"
        SELECT id, username, email, bio, age, role
        FROM users
        ORDER BY id
        LIMIT 10
        "#
    )
    .fetch_all(&pool)
    .await?;
    
    // Convert rows to User structs
    let users = rows.into_iter()
        .map(|row| User {
            id: row.id,
            username: row.username,
            email: row.email,
            bio: row.bio,
            age: row.age,
            role: row.role,
        })
        .collect();
    
    Ok(users)
}

// synchronous user fetching with sync pool creation
fn fetch_users_sync() -> Result<Vec<User>, anyhow::Error> {
    // creating a pool synchronously - automatically handles runtime management
    let pool = create_sqlx_pool("postgres://username:password@localhost/database");
    
    let mut rt = tokio::runtime::Runtime::new()?;
    
    // query execution within the runtime
    let users = rt.block_on(async {
        let rows = sqlx::query!(
            r#"
            SELECT id, username, email, bio, age, role
            FROM users
            ORDER BY id
            LIMIT 10
            "#
        )
        .fetch_all(&pool)
        .await?;
        
        // rows to User structs
        let users = rows.into_iter()
            .map(|row| &User {
                id: row.,
                username: row.username,
                email: row.email,
                bio: row.bio,
                age: row.age,
                role: row.role,
            })
            .collect();
        
        Ok::<Vec<User>, sqlx::Error>(users)
    })?;
    
    Ok(users)
}

// demonstrates real-time user monitoring with SQL LISTEN/NOTIFY
#[server]
async fn listen_for_user_changes() -> Result<Vec<User>, ServerFnError> {
    let pool = acreate_sqlx_pool("postgres://username:password@localhost/database").await;
    
    let mut listener = sqlx::postgres::PgListener::connect_with(&pool).await?;
    
    // listen for the 'user_changed' notification
    listener.listen("user_changed").await?;
    
    // process notifications - in a real app you'd handle this as a stream
    let mut users = Vec::new();
    
    // get first 5 notifications (or timeout after 10 seconds)
    let mut stream = listener.into_stream();
    let timeout = std::time::Duration::from_secs(10);
    
    for _ in 0..5 {
        if let Some(notification) = tokio::time::timeout(timeout, stream.next()).await.ok().flatten() {
            // parse user ID from notification payload
            let user_id: i64 = notification.payload().parse()?;
            
            // fetch updated user data
            let user = fetch_user_by_id(pool.clone(), user_id).await?;
            users.push(user);
        } else {
            break;
        }
    }
    
    Ok(users)
}


#[server]
async fn fetch_user_by_id(pool: SqlxPgPool, id: i64) -> Result<User, ServerFnError> {
    let row = sqlx::query!(
        r#"
        SELECT id, username, email, bio, age, role
        FROM users
        WHERE id = $1
        "#
    )
    .fetch_one(&pool)
    .await?;
    
    Ok(User {
        id: row.id,
        username: row.username,
        email: row.email,
        bio: row.bio,
        age: row.age,
        role: row.role,
    })
}
