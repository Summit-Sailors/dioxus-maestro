
use {
  crate::components::query::{
    basic_query::QueryDemoWrapper, 
    batch::BatchOperationsDemo, 
    cache::CacheDemo, 
    mutation::{ManualMutationDemo, SilentMutationDemo}, 
    parallel_query::ParallelQueriesWrapper,
  }, 
  dioxus::prelude::*
};

#[component]
pub fn CompleteQueryDemo() -> Element {
  rsx! {
    div { 
      class: "min-h-screen w-full",

      div { 
        class: "text-center mb-8",
        h1 { 
          class: "text-3xl font-bold text-center text-gray-800",
          "Maestro Query Demonstrations" 
        }
      }

      div {
        class: "space-y-8",
        
        div {
          QueryDemoWrapper {}
        }
        
        div {
          CacheDemo {}
        }
        
        div {
          SilentMutationDemo {}
        }
        
        div {
          ManualMutationDemo {}
        }

        div {
          ParallelQueriesWrapper {}
        }

        div {
          BatchOperationsDemo {}
        }
      }
      
    }
  }
}
