use {
	async_std::task::sleep,
	dioxus::prelude::*,
	maestro_query::prelude::*,
	maestro_ui::button::Button,
	serde::{Deserialize, Serialize},
	std::{collections::HashMap, fmt::Error, time::Duration},
};

// Base trait for displayable data
trait DisplayData: Serialize + PartialEq + for<'de> Deserialize<'de> {
	fn get_title(&self) -> String;
	fn get_content(&self) -> String;
	fn get_metadata(&self) -> Vec<(String, String)>;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct User {
	id: String,
	name: String,
	email: String,
	role: String,
}

impl DisplayData for User {
	fn get_title(&self) -> String {
		self.name.clone()
	}

	fn get_content(&self) -> String {
		self.email.clone()
	}

	fn get_metadata(&self) -> Vec<(String, String)> {
		vec![("role".into(), self.role.clone())]
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Post {
	id: String,
	title: String,
	author_id: String,
	content: String,
	status: String,
	views: i32,
}

impl DisplayData for Post {
	fn get_title(&self) -> String {
		self.title.clone()
	}

	fn get_content(&self) -> String {
		self.content.clone()
	}

	fn get_metadata(&self) -> Vec<(String, String)> {
		vec![("status".into(), self.status.clone()), ("views".into(), format!("👁 {}", self.views))]
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Comment {
	id: String,
	post_id: String,
	author_id: String,
	content: String,
	likes: i32,
}

impl DisplayData for Comment {
	fn get_title(&self) -> String {
		format!("Comment on {}", self.post_id)
	}

	fn get_content(&self) -> String {
		self.content.clone()
	}

	fn get_metadata(&self) -> Vec<(String, String)> {
		vec![("likes".into(), format!("❤️ {}", self.likes))]
	}
}

async fn simulate_db_delay() {
	sleep(Duration::from_millis(800)).await;
}

#[component]
fn DataCard<T: DisplayData + 'static>(data: T) -> Element {
	rsx! {
		div {
			class: "bg-white p-4 rounded-lg shadow",
			div {
				class: "flex justify-between items-start mb-2",
				h4 {
					class: "font-semibold text-lg text-gray-800",
					"{data.get_title()}"
				}
			}
			p {
				class: "text-gray-600 text-sm mb-2 line-clamp-2",
				"{data.get_content()}"
			}
			div {
				class: "flex justify-between text-sm text-gray-500",
				for (_, value) in data.get_metadata() {
					span { "{value}" }
				}
			}
		}
	}
}

fn render_query_state(query_result: &QueryResult<HashMap<String, String>, Error>) -> Element {
	match query_result {
		QueryResult::Ok(data) => {
			rsx! {
				div {
					class: "space-y-4",
					for (key, value) in data.iter() {
						div {
							match key.as_str() {
								"users" => {
									let items: Vec<User> = serde_json::from_str(value).unwrap_or_default();
									rsx! {
										div {
											class: "space-y-1",
											h3 { class: "text-lg text-center font-semibold mb-2 text-gray-800", "Users ({items.len()})" }
											div {
												class: "grid grid-cols-1 gap-2",
												for item in items {
													DataCard { data: item }
												}
											}
										}
									}
								}
								"posts" => {
									let items: Vec<Post> = serde_json::from_str(value).unwrap_or_default();
									rsx! {
										div {
											class: "space-y-4",
											h3 { class: "text-lg text-center font-semibold mb-2 text-gray-800", "Posts ({items.len()})" }
											div {
												class: "grid grid-cols-1 md:grid-cols-2 gap-2",
												for item in items {
													DataCard { data: item }
												}
											}
										}
									}
								}
								"comments" => {
									let items: Vec<Comment> = serde_json::from_str(value).unwrap_or_default();
									rsx! {
										div {
											class: "space-y-4",
											h3 { class: "text-lg text-center font-semibold mb-2 text-gray-800", "Comments ({items.len()})" }
											div {
												class: "grid grid-cols-1 md:grid-cols-2 gap-2",
												for item in items {
													DataCard { data: item }
												}
											}
										}
									}
								}
								_ => rsx! {
									div {
										class: "bg-white p-4 rounded-lg shadow",
										pre {
											class: "text-sm overflow-x-auto",
											"{value}"
										}
									}
								}
							}
						}
					}
				}
			}
		},
		QueryResult::Err(err) => {
			rsx! {
				div {
					class: "bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded relative",
					"Error: {err}"
				}
			}
		},
		QueryResult::Loading(_) => {
			rsx! {
				div {
					class: "flex justify-center items-center p-8",
					div {
						class: "animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"
					}
				}
			}
		},
	}
}

#[component]
pub fn BatchOperationsDemo() -> Element {
	let query_client: UseQueryClient<HashMap<String, String>, Error, String> = use_init_query_client();
	let operation_status = use_signal(|| "Idle");

	let mock_users = vec![
		User { id: "u1".into(), name: "Steve".into(), email: "steve.smith@example.com".into(), role: "Admin".into() },
		User { id: "u2".into(), name: "Sarah".into(), email: "sarah.j@example.com".into(), role: "Editor".into() },
	];

	let mock_posts = vec![
		Post {
			id: "p1".into(),
			title: "Rust Basics".into(),
			author_id: "u1".into(),
			content: "Guide to Rust programming...".into(),
			status: "published".into(),
			views: 1250,
		},
		Post {
			id: "p2".into(),
			title: "Dioxus Patterns".into(),
			author_id: "u2".into(),
			content: "Dioxus reactive patterns...".into(),
			status: "published".into(),
			views: 850,
		},
	];

	let mock_comments = vec![
		Comment { id: "c1".into(), post_id: "p1".into(), author_id: "u2".into(), content: "Great introduction to Rust!".into(), likes: 15 },
		Comment { id: "c2".into(), post_id: "p2".into(), author_id: "u1".into(), content: "Very helpful patterns explanation.".into(), likes: 12 },
	];

	let users_query = use_get_query(["users-batch".to_string()], move |_| {
		let mock_users_clone = mock_users.clone();
		async move {
			simulate_db_delay().await;
			let mut data = HashMap::new();
			data.insert("users".into(), serde_json::to_string(&mock_users_clone).unwrap());
			QueryResult::Ok(data)
		}
	});

	let posts_query = use_get_query(["posts-batch".to_string()], move |_| {
		let mock_posts_clone = mock_posts.clone();
		async move {
			simulate_db_delay().await;
			let mut data = HashMap::new();
			data.insert("posts".into(), serde_json::to_string(&mock_posts_clone).unwrap());
			QueryResult::Ok(data)
		}
	});

	let comments_query = use_get_query(["comments-batch".to_string()], move |_| {
		let mock_comments_clone = mock_comments.clone();
		async move {
			simulate_db_delay().await;
			let mut data = HashMap::new();
			data.insert("comments".into(), serde_json::to_string(&mock_comments_clone).unwrap());
			QueryResult::Ok(data)
		}
	});

	let batch_mutation = use_mutation(|operations: Vec<String>| async move {
		simulate_db_delay().await;
		if operations.is_empty() {
			MutationResult::Err(Error)
		} else {
			MutationResult::Ok(operations.len())
		}
	});

	let handle_batch_invalidate = move |_| {
		let mut status = operation_status.clone();
		let client = query_client.clone();

		spawn(async move {
			status.set("Invalidating queries...");
			client.invalidate_queries(&["users-batch".to_string(), "posts-batch".to_string(), "comments-batch".to_string()]);
			sleep(Duration::from_secs(1)).await;
			status.set("Queries invalidated");
			sleep(Duration::from_secs(1)).await;
			status.set("Idle");
		});
	};

	let handle_batch_mutation = move |_| {
		let mut status = operation_status.clone();
		let mutation = batch_mutation.clone();

		spawn(async move {
			status.set("Starting batch operation...");

			let operations = vec!["Operation 1".to_string(), "Operation 2".to_string(), "Operation 3".to_string()];

			mutation.mutate(operations);

			sleep(Duration::from_secs(1)).await;

			match *mutation.result() {
				MutationResult::Ok(_count) => {
					status.set("Completed 3 tasks");
					sleep(Duration::from_secs(2)).await;
					status.set("Idle");
				},
				MutationResult::Err(_) => {
					status.set("Batch operation failed");
					sleep(Duration::from_secs(2)).await;
					status.set("Idle");
				},
				MutationResult::Loading(_) => {
					status.set("Operation in progress...");
				},
				_ => {
					status.set("No operation performed");
					sleep(Duration::from_secs(2)).await;
					status.set("Idle");
				},
			}
		});
	};

	rsx! {
		div {
			class: "p-6 bg-white rounded-lg shadow-lg",
			h3 { class: "text-2xl text-gray-800 text-center font-bold mb-4", "Batch Operations Demo" }

			div {
				class: "mb-4 p-2 bg-gray-100 rounded text-center text-gray-700",
				"Status: ",
				span {
					class: match operation_status() {
						"Idle" => "text-gray-500",
						s if s.contains("Completed") => "text-green-500",
						s if s.contains("failed") => "text-red-500",
						_ => "text-blue-500"
					},
					"{operation_status}"
				}
			}

			div {
				class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-4",
				div {
					class: "p-4 border rounded shadow-md",
					{render_query_state(users_query.result().value())}
				}
				div {
					class: "p-4 border rounded shadow-md",
					{render_query_state(posts_query.result().value())}
				}
				div {
					class: "p-4 border rounded shadow-md",
					{render_query_state(comments_query.result().value())}
				}
			}

			div {
				class: "grid flex justify-center gap-4 mt-6",
				Button {
					class: "bg-blue-500 text-white rounded hover:bg-blue-600",
					onclick: handle_batch_invalidate,
					"Invalidate All Queries"
				}
				Button {
					class: "bg-green-500 text-white rounded hover:bg-green-600",
					onclick: handle_batch_mutation,
					"Run Batch Mutation"
				}
			}
		}
	}
}
