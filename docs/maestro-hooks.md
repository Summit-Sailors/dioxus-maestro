# 🎭 Maestro-Hooks

Enhanced hooks collection for Dioxus applications that provides type safety, cross-platform compatibility, and optimized performance.

## Features

### 📋 Cross-Platform Clipboard Hook (`use_clipboard`)

A unified clipboard interface that works seamlessly across desktop and web platforms with comprehensive error handling.

```rust
let clipboard = use_clipboard();
```

**Advantages over standard clipboard implementations:**

- Feature-flagged platform-specific optimizations (`desktop` and `web` features)
- Structured error handling with custom `ClipboardError` enum
- Automatic context management and cleanup
- First-class support for async operations
- Type-safe clipboard operations with Result types

### 🎯 Explicit Memo Hook (`use_explicit_memo`)

An enhanced memoization hook that provides stronger guarantees against unnecessary rerenders.

```rust
let memoized_value = use_explicit_memo(deps, init_fn);
```

**Key improvements over `use_memo`:**

- Zero unnecessary rerenders guaranteed by explicit dependency tracking
- More predictable and efficient memory usage
- Clear separation between dependencies and computation logic
- Perfect for expensive computations where performance is critical

### 📑 Advanced Pagination Hook (`use_pagination`)

A comprehensive pagination solution that handles complex pagination state management.

```rust
let (pagination, (next_idx, prev_idx, next_page, prev_page)) = use_pagination(total_items);
```

**Features beyond basic pagination:**

- Automatic page size calculations
- Bidirectional navigation (both by page and by individual items)
- Real-time counter labels and disabled states
- Optimized state updates with Signal integration
- Zero-based index support with automatic boundary handling

## Installation

Add dioxus-maestro to your `Cargo.toml`:

```toml
[dependencies]
dioxus-maestro = { version = "", features = ["desktop", "web"] }
```

## Usage Examples

### Clipboard Hook

```rust
let clipboard = use_clipboard();

// Setting clipboard content
if let Ok(()) = clipboard.set("Hello, World!".to_string()).await {
  println!("Content copied successfully!");
}

// Getting clipboard content
match clipboard.get().await {
  Ok(content) => println!("Clipboard content: {}", content),
  Err(ClipboardError::NotAvailable) => println!("Clipboard not available"),
  Err(_) => println!("Failed to read clipboard"),
}
```

### Explicit Memo Hook

```rust
let expensive_result = use_explicit_memo(
  (dependency1, dependency2),
  || perform_expensive_calculation(dependency1, dependency2)
);
```

### Pagination Hook

```rust
let (pagination, (next_idx, prev_idx, next_page, prev_page)) = use_pagination(total_items);

// Access pagination state
println!("Current page: {}", pagination.page());
println!("Items per page: {}", pagination.page_size());

// Navigate through pages
next_page();  // Go to next page
prev_idx();   // Go to previous item
```

## Why Maestro-Hooks?

1. **Type Safety**: Enhanced error handling and type-safe operations compared to standard Dioxus hooks
2. **Cross-Platform**: Seamless operation across desktop and web platforms with optimized implementations
3. **Performance**: Optimized state management and preventing unnecessary rerenders
4. **Developer Experience**: Intuitive APIs with clear separation of concerns
5. **Maintainability**: Well-structured code with comprehensive error handling

## Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

## License

MIT or Apache-2.0, at your option.
