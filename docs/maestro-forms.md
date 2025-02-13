# Maestro Forms

A powerful, type-safe form management solution for Dioxus applications that brings the best of Formik's paradigms to Rust.

## Features

- **🦀 Type-safe Form Handling**: Fully type-safe form state management with Rust's powerful type system
- **⚡ Performance Optimized**: Built-in debounced inputs prevent unnecessary re-renders
- **✅ Validation Integration**: Seamless integration with the `validator` crate for declarative validation
- **🔄 Smart Field Tracking**: Automatic touched state management and error tracking
- **🎯 Context-Based Architecture**: Efficient form state management using Dioxus context
- **🎨 UI Agnostic**: Flexible design that separates form logic from presentation

## Why Maestro Forms?

Unlike vanilla Dioxus form handling, Maestro Forms provides a comprehensive solution that:

1.**Eliminates Boilerplate**

```rust
// Instead of manual state management:
let username = use_signal(|| String::new());
let email = use_signal(|| String::new());
let errors = use_signal(|| Vec::new());

// Use type-safe, structured form state:
#[derive(Validate, Serialize, Deserialize)]
struct UserForm {
  username: String,
  email: String
}

Form {
  initial_value: UserForm::default(),
  onsubmit: handle_submit,
  inner: form_content
}
```

2.**Built-in Validation**

```rust
#[derive(Validate, Serialize, Deserialize)]
pub struct User {
  #[validate(length(min = 3, max = 20))]
  username: String,
  #[validate(email)]
  email: String
}
```

3.**Smart Field Components**

```rust
// Automatic debouncing, error handling, and state management
TextFormInput::<User> {
  name: "username",
  class: "input-class"
}
```

## Quick Start

1. Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-maestro = ""
validator = ""
serde = { version = "", features = ["derive"] }
```

2.Define your form structure:

```rust
use validator::Validate;
use serde::{Serialize, Deserialize};

#[derive(Validate, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserForm {
  #[validate(length(min = 3))]
  username: String,
  #[validate(email)]
  email: String,
}
```

3.Create your form:

```rust
use maestro_forms::fields::form::Form;

#[component]
pub fn UserFormComponent() -> Element {
  rsx! {
    Form {
      initial_value: UserForm::default(),
      onsubmit: move |event: FormEvent| {
        // Handle submission
      },
      inner: form_content
    }
  }
}
```

## Advanced Features

### Form State Debugging

Maestro Forms includes a built-in form state debugger that helps during development:

```rust
FormStateDebugger { form: props.form }
```

### Field Wrappers

Create consistent field layouts with reusable wrappers:

```rust
FormFieldWrapper {
  label: "Username",
  field: form.get_form_field("username"),
  children: // Your input component
}
```

### Select Fields with Custom Labels

```rust
SelectFormField::<User, String> {
  name: "role",
  values: roles,
  labels: Some(role_labels),
}
```

## Comparison with Alternatives

### vs. Raw Dioxus Forms

- ✅ No manual state management needed
- ✅ Built-in validation
- ✅ Automatic error tracking
- ✅ Performance optimizations
- ✅ Type-safe field access
- ✅ Form-wide state management

### vs. Other Form Libraries

- ✅ Native Dioxus integration
- ✅ Rust-first approach
- ✅ Full type safety
- ✅ Zero runtime overhead
- ✅ Minimal dependencies

## Performance

Maestro Forms includes several optimizations:

- Debounced inputs prevent excessive re-renders
- Context-based state management reduces prop drilling
- Smart validation that only runs when needed
- Efficient field tracking with minimal overhead

## Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

## License

MIT or Apache-2.0, at your option.
