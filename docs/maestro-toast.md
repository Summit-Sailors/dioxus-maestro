# Maestro Toast

## Overview

`maestro-toast` is a powerful, flexible, and intuitive toast notification system designed specifically for Dioxus applications. While Dioxus provides basic UI capabilities, our toast component takes notification management to the next level with its robust feature set and elegant design.

## 🚀 Key Differentiators

### 1. Intelligent State Management

- **Seamless Context Integration**: Leverages Dioxus's signal and context providers for reactive, performance-optimized toast management
- **Automatic State Cleanup**: Built-in mechanism to automatically remove expired toasts, preventing memory bloat

### 2. Comprehensive Customization

- **Multiple Toast Types**:
  - Success
  - Warning
  - Error
  - Info
  - Fully Custom Configurations

- **Positioning Flexibility**:

  ```rust
  // Four predefined positions, easily configurable
  EToastPosition {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight
  }
  ```

### 3. Advanced Configuration Options

```rust
ToastInfo {
  heading: Option<String>,         // Optional title
  context: String,                 // Main message
  allow_toast_close: bool,         // User dismissal control
  position: EToastPosition,        // Placement strategy
  icon: Option<EToastCode>,        // Dynamic icon selection
  hide_after: usize                // Configurable duration
}
```

### 4. Smart Toast Management

- **Maximum Toast Limit**: Configurable maximum number of simultaneous toasts
- **Intelligent Removal**: Automatically removes oldest toast when limit is reached
- **Unique ID Generation**: Robust ID management to prevent conflicts

## 💡 Usage Example

```rust
// Initialize toast context
use_init_toast_ctx()

// Create and display a toast
let info = ToastInfo::builder()
  .heading("Success!")
  .context("Operation completed")
  .icon(EToastCode::Success)
  .position(EToastPosition::TopRight)
  .build();

toast.write().popup(info);
```

## 🔍 Performance Characteristics

- **Low Overhead**: Minimal performance impact
- **Reactive Updates**: Leverages Dioxus signals for efficient rendering
- **Cross-Platform**: Supports both web and desktop environments

## 🏆 Advantages Over Vanilla Dioxus

1. **No Manual State Management**: Automatic toast tracking and removal
2. **Rich Styling**: Pre-configured color schemes and icon mappings
3. **Flexible Configuration**: Granular control over toast behavior
4. **Built-in Best Practices**: Sensible defaults with full customization

## 🛠 Installation

```toml
[dependencies]
dioxus-maestro = { version = "0.1.0", features = ["toast"] }
```

## 🌟 Pro Tips

- Use `clear()` to remove all toasts
- Customize `maximum_toast` during initialization
- Leverage builder pattern for concise toast creation

## Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

## License

MIT or Apache-2.0, at your option.
