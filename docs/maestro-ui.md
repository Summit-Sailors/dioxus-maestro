# Maestro UI

## 🌟 Overview

 Maestro UI is a comprehensive, type-safe, and highly customizable UI component library for Dioxus, designed to provide developers with powerful, flexible, and elegant UI building blocks.

## 📦 Components

### 1. 🔘 Button Component

```rust
#[component]
pub fn Button(props: ButtonProps) -> Element
```

#### Button Component Key Features

- Multiple variants: Default, Destructive, Outline, Secondary, Ghost, Link, Icon, Rounded
- Flexible sizing options
- Built-in event handling
- Tailwind CSS styling
- Supports custom classes and styles

### 2. 📝 Input Component

```rust
#[component]
pub fn Input(props: InputProps) -> Element
```

#### Input Component Key Features

- Support for different input types (Text, Search, Password, Email)
- Multiple input variants (Default, Underlined)
- Error handling
- Event callbacks for change and enter key
- Responsive design

### 3. 🏷️ Label Component

```rust
#[component]
pub fn Label(props: LabelProps) -> Element
```

#### Label Component Key Features

- Flexible labeling
- Custom styling support
- Easy composition with children elements

### 4. 📋 Multi-Select Component

```rust
#[component]
pub fn MultiSelect<T: Clone + PartialEq + Display + 'static>(
    values: Vec<T>, 
    current_value: Vec<T>, 
    callback: EventHandler<T>
) -> Element
```

#### Multi-Select Component Key Features

- Generic type support
- Dropdown selection
- Multiple item selection
- Customizable placeholder
- Tailwind CSS styling

### 5. 🔘 Radio Component

```rust
#[component]
pub fn Radio(props: RadioProps) -> Element
```

#### Radio Component Key Features

- Flexible labeling
- Disabled state support
- Grouped radio button functionality
- Responsive design

### 6. 📊 Range Component

```rust
#[component]
pub fn Range(
  current_value: i32, 
  callback: EventHandler<i32>, 
  min_value: i32, 
  max_value: i32
) -> Element
```

#### Range Component Key Features

- Value range selection
- Customizable min and max values
- Step configuration
- Dynamic value display

### 7. 🔽 Select Component

```rust
#[component]
pub fn Select<T: Clone + PartialEq + Display + 'static>(
    values: Vec<T>,
    current_value: T,
    callback: EventHandler<T>
) -> Element
```

#### Select Component Key Features

- Generic type support
- Dropdown selection
- Single item selection
- Customizable label

### 8. 🌀 Spinner Component

```rust
#[component]
pub fn FreeIconSpinner(size: u32) -> Element
```

#### Spinner Key Features

- Configurable size
- Animated spinning effect

### 9. 📝 Textarea Component

```rust
#[component]
pub fn Textarea(props: TextareaProps) -> Element
```

#### TextArea Key Features

- Supports different variants
- Error handling
- Event callbacks
- Responsive design

### 10. 🔛 Toggle Switch Component

```rust
#[component]
pub fn ToggleSwitch(
    state: Signal<bool>, 
    label_states: Option<ToggleSwitchLabelStatesProp>
) -> Element
```

#### Toggle Switch Key Features

- State management
- Configurable label states
- Flexible label placement

### 11. Calendar Component

The maestro-ui Calendar Component a type-safe, and highly customizable calendar implementation that goes beyond traditional calendar components. Designed with Rust's type system and Dioxus's reactive paradigms, it offers unparalleled flexibility and developer experience.

## Calendar Key Features

### 1. Type-Safe Enum-Driven Design

```rust
pub enum ECalendarMonth {
  January = 1,
  February = 2,
  // ...
}

pub enum ECalendarDay {
  Sun = 0,
  Mon = 1,
  // ...
}
```

- **Compile-Time Safety**: Leverages Rust's enum system to prevent invalid calendar operations
- **Built-in Methods**: Rich set of methods like `is_weekend()`, `next()`, `prev()`
- **Serialization Support**: Easy conversion and storage with `serde` derive macros

### 2. Reactive State Management

```rust
let selected_date = use_memo(move || 
  NaiveDate::from_ymd_opt(selected_year(), selected_month() as u32, selected_day() as u32)
);
```

- **Signal-Based Reactivity**: Seamless state updates with Dioxus signals
- **Memoized Computations**: Efficient date calculations
- **Dynamic UI Updates**: Instant reflection of state changes

### 3. Flexible Rendering Modes

```rust
pub fn CalendarMaybeWrapper(props: CalendarMaybeWrapperProps) -> Element {
  // Supports both full and compact/modal views
  if is_full {
    // Render full calendar
  } else {
    // Render compact calendar button
  }
}
```

- **Full Calendar View**: Comprehensive month display
- **Compact Modal View**: Minimalist, space-efficient design
- **Configurable Rendering**: Easy customization through props

### 4. Advanced Date Constraints

```rust
#[derive(PartialEq, bon::Builder)]
pub struct CalendarSelectProps {
  min_date: Signal<Option<NaiveDate>>,
  max_date: Signal<Option<NaiveDate>>,
}
```

- **Date Range Limitations**: Easily set minimum and maximum selectable dates
- **Dynamic Constraints**: Update constraints programmatically
- **Prevents Invalid Selections**: Automatically disables out-of-range dates

### 5. Rich Styling and Interaction

- **Tailwind CSS Integration**: Responsive, modern design
- **State-Aware Styling**:
  - Today's date
  - Selected date
  - Disabled dates
  - Hover states
- **Navigation Icons**: Intuitive month/year navigation
- **Accessibility**: Semantic HTML and ARIA attributes

## 🛠️ Basic Usage

```rust
#[component]
fn App() -> Element {
  let display_props = CalendarDisplayProps::builder().build();
  let select_props = CalendarSelectProps::builder().build();
  
  rsx! {
    Calendar {
      display_props,
      select_props
    }
  }
}
```

## 🌈 Customization Example

```rust
let min_date = use_signal(|| NaiveDate::from_ymd_opt(2024, 1, 1));
let max_date = use_signal(|| NaiveDate::from_ymd_opt(2025, 12, 31));

let select_props = CalendarSelectProps::builder()
  .min_date(min_date)
  .max_date(max_date)
  .build();
```

## 🔍 Why Maestro Calendar?

- **Type Safety**: Rust's enum system prevents runtime errors
- **Reactivity**: Seamless state management with Dioxus
- **Flexibility**: Adaptable to various use cases
- **Performance**: Efficient computations and rendering
- **Modern Design**: Tailwind CSS for responsive styling

## 📦 Installation

```toml
[dependencies]
dioxus-maestro = { git = "" }
```

## 🛠️ Usage Example

```rust
use dioxus_maestro::prelude::*;

#[component]
fn App() -> Element {
  rsx! {
    Button { 
      variant: ButtonVariant::Primary,
      "Click me" 
    }
    Input { 
      input_type: InputType::Text,
      placeholder: "Enter your name" 
    }
  }
}
```

## 🌈 Why Maestro UI?

- **Type Safety**: Leverages Rust's type system
- **Reactive Design**: Built for Dioxus's reactive paradigm
- **Flexible Styling**: Tailwind CSS integration
- **Comprehensive Components**: Wide range of UI elements
- **Performance**: Efficient and lightweight

## Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

## License

MIT or Apache-2.0, at your option.
