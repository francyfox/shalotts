# sha-web AGENTS Guide (Dioxus 0.7)

This is a specialized guide for the **sha-web** crate within the Shalotts workspace. For general workspace information, see the root `AGENTS.md`.

## Overview

**sha-web** is a Dioxus 0.7 web application that provides a web-based interface for project generation. It's part of the Shalotts scaffolding tool ecosystem.

## Quick Start

```bash
# Navigate to sha-web directory
cd crates/sha-web

# Development server (auto-reload)
dx serve

# Build for web
dx build

# Desktop platform
dx serve --platform desktop

# Mobile platform
dx serve --platform mobile
```

## Dioxus 0.7 Essentials

**Important**: Dioxus 0.7 has major API changes - `cx`, `Scope`, and `use_state` are removed.

### Core Hooks

```rust
// Local state
let mut count = use_signal(|| 0);

// Memoized value
let doubled = use_memo(move || count() * 2);

// Async resource
let data = use_resource(move || async move {
    // fetch data
});

// Context
let theme = use_context::<Signal<String>>();
```

### Component Definition

```rust
#[component]
fn MyComponent(mut value: Signal<String>) -> Element {
    rsx! {
        div {
            class: "container",
            input {
                value,
                oninput: move |e| *value.write() = e.value(),
            }
        }
    }
}
```

### Routing

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
}

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}
```

## Project Structure

```
sha-web/
├── Cargo.toml          # Dependencies + features (web, desktop, mobile)
├── Dioxus.toml         # Dioxus build configuration
├── src/
│   └── main.rs        # App entry + routes
├── assets/            # Static files (favicon, images)
├── tailwind.css       # Styles (auto-processed by dx)
└── README.md          # Development guide
```

## Key Files

### src/main.rs
- Contains Route enum with `#[derive(Routable)]`
- Defines App component with Router
- Layout components (Navbar, etc.)

### Cargo.toml
- Features: `default = ["web"]`, plus `desktop`, `mobile`
- Dependency: `dioxus = { version = "0.7.1", features = ["router"] }`

### Dioxus.toml
- Platform configuration
- Asset handling
- Tailwind settings

## Common Patterns

### Asset Handling
```rust
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }
}
```

### Event Handling
```rust
button {
    onclick: move |_| *count.write() += 1,
    "Increment"
}

input {
    oninput: move |e| *value.write() = e.value(),
}
```

### Conditional Rendering
```rust
if condition {
    rsx! { div { "Show this" } }
}

{condition.then(|| rsx! { div { "Alternative" } })}
```

### Loops
```rust
for item in items {
    div { "{item}" }
}
```

## Development Tips

1. **Hot Reload**: `dx serve` provides hot reload - changes appear instantly
2. **Tailwind**: Auto-processed - edit `tailwind.css` for styles
3. **Console Logging**: Use `console::log!` for debugging
4. **Component Props**: Must implement `PartialEq` and `Clone`

## Testing

```bash
# Build checks
cargo check

# Run tests
cargo test

# Format
cargo fmt

# Lint
cargo clippy -- -D warnings
```

## Integration with Workspace

sha-web is a standalone Dioxus app but follows the Shalotts pattern:
- Uses TOML configuration (see root AGENTS.md)
- Works with examples/elysia configs
- Part of workspace build system

## Troubleshooting

- **Build fails**: Check `Dioxus.toml` and asset paths
- **Tailwind not working**: Ensure `tailwind.css` exists next to Cargo.toml
- **Routing issues**: Verify `#[derive(Routable)]` and route strings
