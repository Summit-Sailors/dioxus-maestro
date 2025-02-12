
use {
  crate::components::query::{
    basic_query::BasicQueryDemo, 
    batch::BatchOperationsDemo, 
    cache::CacheDemo, 
    mutation::{ManualMutationDemo, SilentMutationDemo}, 
    parallel_query::ParallelQueriesDemo,
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
          BasicQueryDemo {}
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
          ParallelQueriesDemo {}
        }

        div {
          BatchOperationsDemo {}
        }
      }
      
    }
  }
}
