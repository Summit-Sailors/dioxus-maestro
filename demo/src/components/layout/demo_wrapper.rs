use {
  crate::components::editor::editor::CodeEditor, 
  dioxus::prelude::*
};

#[derive(Props, Clone, PartialEq)]
pub struct DemoWrapperProps {
  title: String,
  source_code: String,
  #[props(into)]
  children: Element,
}

#[component]
pub fn DemoWrapper(props: DemoWrapperProps) -> Element {
  rsx! {
    div {
      class: "space-y-8 pb-8",
      // title
      div {
        class: "mb-6",
        h1 {
          class: "text-3xl font-bold text-gray-900",
          "{props.title}"
        }
      }
      
      // demo
      CodeEditor {
        code: props.source_code.to_string(),
        title: props.title.to_string(),
        demo: props.children.clone()
      }
    }
  }
}
