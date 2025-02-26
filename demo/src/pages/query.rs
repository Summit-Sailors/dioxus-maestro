use {
	crate::components::{query::{
		basic_query::QueryDemoWrapper,
		batch::BatchOperationsDemo,
		cache::CacheDemo,
		mutation::{ManualMutationDemo, SilentMutationDemo},
		parallel_query::ParallelQueriesWrapper,
	}, ui::features::Features},
	dioxus::prelude::*, tailwind_fuse::tw_join,
};

#[derive(PartialEq)]
enum QuerySection {
  Default,
  Cache,
  Mutation,
  Parallel,
  Batch
}

#[component]
pub fn CompleteQueryDemo() -> Element {
  let mut active_section = use_signal(|| QuerySection::Default);

	rsx! {
    div { 
      class: "w-full bg-gray-900 p-6 rounded-lg shadow-lg",

      div { class: "mb-8",
        h1 { class: "text-gray-100 text-center text-3xl font-bold mb-2", "Maestro Query" }
        p { class: "text-gray-300 text-center",
          "A powerful and flexible query management system for Dioxus applications that provides advanced caching, synchronization, and state management capabilities."
        }
      }

      div {
        class: "flex space-x-2",  
        Features {
          title: "Form".to_string(),
          features: vec![
            "Automatic Cache Management: Built-in stale-time tracking and cache invalidation".to_string(),
            "Smart Re-rendering: Only updates components when their specific query data changes".to_string(),
            "Type-safe Query Keys: Strongly typed query keys for compile-time safety".to_string(),
            "Built-in Loading States: Sophisticated handling of loading, error, and success states".to_string(),
            "Query Deduplication: Automatically batches and deduplicates identical queries".to_string(),
            "Fine-grained Control: Silent mutations and manual query execution when needed".to_string(),
            "Background Updates: Supports automatic background data refreshing".to_string(),
            "Zero Configuration: Works out of the box with sensible defaults".to_string()
          ]
        }
      }

      div {
        class: "flex flex-wrap sm:flex-nowrap space-x-2 sm:space-x-0 border-b border-gray-700 pb-4",
        span {
          class: tw_join!(
            "py-2 px-2 cursor-pointer rounded text-sm sm:text-xs",
            if *active_section.read() == QuerySection::Default { "text-white border-b-4 border-gray-500" } else { "text-gray-300" }
          ),
          onclick: move |_| active_section.set(QuerySection::Default),
          "Default"
        }
        span {
          class: tw_join!(
            "py-2 px-2 cursor-pointer rounded text-sm sm:text-xs",
            if *active_section.read() == QuerySection::Cache { "text-white border-b-4 border-gray-500" } else { "text-gray-300" }
          ),
          onclick: move |_| active_section.set(QuerySection::Cache),
          "Cache"
        }
        span {
          class: tw_join!(
            "py-2 px-2 cursor-pointer rounded text-sm sm:text-xs",
            if *active_section.read() == QuerySection::Mutation { "text-white border-b-4 border-gray-500" } else { "text-gray-300" }
          ),
          onclick: move |_| active_section.set(QuerySection::Mutation),
          "Mutation"
        }
        span {
          class: tw_join!(
            "py-2 px-2 cursor-pointer rounded text-sm sm:text-xs",
            if *active_section.read() == QuerySection::Parallel { "text-white border-b-4 border-gray-500" } else { "text-gray-300" }
          ),
          onclick: move |_| active_section.set(QuerySection::Parallel),
          "Parallel"
        }
        span {
          class: tw_join!(
            "py-2 px-2 cursor-pointer rounded text-sm sm:text-xs",
            if *active_section.read() == QuerySection::Batch { "text-white border-b-4 border-gray-500" } else { "text-gray-300" }
          ),
          onclick: move |_| active_section.set(QuerySection::Batch),
          "Batch"
        }
      }

      match *active_section.read() {
        QuerySection::Default => rsx! {
          div {  
            QueryDemoWrapper { }
          }

          div {
            class: "flex mt-4",
            Features {
              title: "Deafult Query".to_string(),
              features: vec![
                "Automatic Cache Management: Built-in stale-time tracking and cache invalidation".to_string(),
                "Smart Re-rendering: Only updates components when their specific query data changes".to_string(),
                "Type-safe Query Keys: Strongly typed query keys for compile-time safety".to_string(),
                "Built-in Loading States: Sophisticated handling of loading, error, and success states".to_string(),
                "Query Deduplication: Automatically batches and deduplicates identical queries".to_string(),
                "Fine-grained Control: Silent mutations and manual query execution when needed".to_string(),
                "Background Updates: Supports automatic background data refreshing".to_string(),
                "Zero Configuration: Works out of the box with sensible defaults".to_string()
              ]
            }
          }
        },
        QuerySection::Cache => rsx! {
          div {  
            CacheDemo { }
          }

          div {
            class: "flex mt-4",
            Features {
              title: "Caching".to_string(),
              features: vec![
                "Automatic Cache Management: Built-in stale-time tracking and cache invalidation".to_string(),
                "Smart Re-rendering: Only updates components when their specific query data changes".to_string(),
                "Built-in Loading States: Sophisticated handling of loading, error, and success states".to_string(),
                "Query Deduplication: Automatically batches and deduplicates identical queries".to_string(),
                "Fine-grained Control: Silent mutations and manual query execution when needed".to_string(),
                "Background Updates: Supports automatic background data refreshing".to_string(),
                "Zero Configuration: Works out of the box with sensible defaults".to_string()
              ]
            }
          }
        },
        QuerySection::Mutation => rsx! {
          div {
            class: "space-y-8", 
            div {
              SilentMutationDemo { }
            }

            div {
              ManualMutationDemo { }
            }
          }

          div {
            class: "flex mt-4",
            Features {
              title: "Query Mutation".to_string(),
              features: vec![
                "Smart Re-rendering: Only updates components when their specific query data changes".to_string(),
                "Type-safe Query Keys: Strongly typed query keys for compile-time safety".to_string(),
                "Built-in Loading States: Sophisticated handling of loading, error, and success states".to_string(),
                "Query Deduplication: Automatically batches and deduplicates identical queries".to_string(),
                "Fine-grained Control: Silent mutations and manual query execution when needed".to_string(),
                "Background Updates: Supports automatic background data refreshing".to_string(),
                "Zero Configuration: Works out of the box with sensible defaults".to_string()
              ]
            }
          }
        },
        QuerySection::Parallel => rsx! {
          div {  
            ParallelQueriesWrapper { }
          }

          div {
            class: "flex mt-4",
            Features {
              title: "Parallel Queries".to_string(),
              features: vec![
                "Smart Re-rendering: Only updates components when their specific query data changes".to_string(),
                "Type-safe Query Keys: Strongly typed query keys for compile-time safety".to_string(),
                "Built-in Loading States: Sophisticated handling of loading, error, and success states".to_string(),
                "Query Deduplication: Automatically batches and deduplicates identical queries".to_string(),
                "Fine-grained Control: Silent mutations and manual query execution when needed".to_string(),
                "Background Updates: Supports automatic background data refreshing".to_string(),
                "Zero Configuration: Works out of the box with sensible defaults".to_string()
              ]
            }
          }
        },
        QuerySection::Batch => rsx! {
          div {  
            BatchOperationsDemo { }
          }

          div {
            class: "flex mt-4",
            Features {
              title: "Batch Operations".to_string(),
              features: vec![
                "Smart Re-rendering: Only updates components when their specific query data changes".to_string(),
                "Type-safe Query Keys: Strongly typed query keys for compile-time safety".to_string(),
                "Built-in Loading States: Sophisticated handling of loading, error, and success states".to_string(),
                "Query Deduplication: Automatically batches and deduplicates identical queries".to_string(),
                "Fine-grained Control: Silent mutations and manual query execution when needed".to_string(),
                "Background Updates: Supports automatic background data refreshing".to_string(),
                "Zero Configuration: Works out of the box with sensible defaults".to_string()
              ]
            }
          }
        }
      }
    }
  }
}
