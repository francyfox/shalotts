# AGENTS.md - Shalotts Workspace

This document provides essential information for AI agents working in the **Shalotts** repository - a Rust-based project scaffolding and generation tool inspired by BatiJS.

## 📁 Project Overview

**Shalotts** is a Rust workspace containing three main crates that work together to create scaffolded project templates:

- **sha-cli**: Terminal UI application for interactive project generation
- **sha-validator**: TOML configuration validation library
- **sha-web**: Web UI application (Dioxus-based) for project generation

The project follows a "generation seed" concept where users select features and configurations to generate pre-configured, ready-to-start applications.

## 🚀 Essential Commands

### Workspace Commands

```bash
# Build entire workspace
cargo build

# Build specific package
cargo build --package sha-cli
cargo build --package sha-validator
cargo build --package sha-web

# Run workspace default (sha-cli)
cargo run

# Run specific package
cargo run --package sha-cli
cargo run --package sha-web

# Run tests for workspace
cargo test

# Run tests for specific package
cargo test --package sha-validator

# Check formatting
cargo fmt -- --check

# Run clippy lints
cargo clippy -- -D warnings

# Generate documentation
cargo doc --no-deps --all-features
```

### sha-web (Dioxus) Commands

```bash
# Development server for web
cd crates/sha-web && dx serve

# Desktop platform
cd crates/sha-web && dx serve --platform desktop

# Mobile platform  
cd crates/sha-web && dx serve --platform mobile

# Build for production
cd crates/sha-web && dx build
```

### Makefile Commands

```bash
# Development for sha-app (note: actual path may vary)
make dev

# Run specific binary
make sha-cli  # or any binary name
```

## 🏗️ Architecture & Structure

### Workspace Layout

```
shalotts/
├── Cargo.toml              # Workspace manifest
├── Cargo.lock              # Dependency lockfile
├── Makefile               # Development commands
├── crates/
│   ├── sha-cli/           # TUI application
│   │   ├── Cargo.toml
│   │   ├── src/main.rs    # Ratatui-based app entry
│   │   └── .github/       # CI/CD for this crate
│   ├── sha-validator/     # TOML validation library
│   │   ├── Cargo.toml
│   │   └── src/lib.rs     # Validation logic
│   └── sha-web/           # Dioxus web app
│       ├── Cargo.toml
│       ├── Dioxus.toml    # Dioxus config
│       ├── src/main.rs    # Dioxus app entry
│       ├── assets/        # Static assets
│       ├── tailwind.css   # Styles
│       └── AGENTS.md      # Dioxus-specific docs
├── examples/
│   └── elysia/            # Template examples
│       ├── sha.toml       # Template config
│       └── packages/
│           ├── core/
│           └── html/
└── docs/                  # Project documentation
```

### Dependency Flow

1. **sha-validator** → Standalone library for config validation
2. **sha-cli** → Uses ratatui/crossterm for TUI, can optionally use sha-validator
3. **sha-web** → Dioxus-based web interface for generation
4. **Configuration** → Uses TOML format with `[seed, name, ecosystem, tab]` structure

## 📝 Code Conventions

### Rust Edition & Style

- **Edition**: Mixed (sha-cli: 2024, sha-web: 2021)
- **Formatter**: Standard `cargo fmt` enforced via CI
- **Linter**: `clippy` with strict warnings (`-D warnings`)
- **Output**: Mix of `println!` (sha-cli) and Dioxus RSX (sha-web)

### Key Patterns

#### sha-cli (Ratatui TUI)
```rust
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
```

#### sha-validator (Library)
```rust
use serde::Deserialize;
use toml::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct ShaMainConfig {
    seed: String,
    name: String,
    ecosystem: String,
    tab: HashMap<String, HashMap<String, Value>>,
}

pub fn validate_main_config(file_path: String) {
    // TOML validation logic
}
```

#### sha-web (Dioxus)
```rust
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        Router::<Route> {}
    }
}
```

### Configuration Format

Template configurations use TOML with specific structure:

```toml
seed = ""
name = "Elysia starter"
ecosystem = "elysia"

[tab.back]
server = "Server"
pack = "Packages"

[tab.back.serve]
required = true
packages = ["core"]

[tab.back.pack]
packages = ["html", "static"]
```

## 🧪 Testing Approach

### Unit Tests
- sha-validator has test scaffolding (currently commented out)
- sha-cli uses ratatui testing utilities
- sha-web uses Dioxus testing patterns

### Integration Tests
- Run `cargo test --workspace` to test all crates
- CI runs tests on multiple OS (macOS, Windows via GitHub Actions)

### Configuration Tests
- Configuration validation via sha-validator
- TOML parsing errors are handled with `expect()` (consider using `Result` for production)

## ⚠️ Important Gotchas

### 1. **Edition Mismatch**
- sha-cli uses Rust 2024 edition
- sha-web uses Rust 2021 edition
- Ensure compatibility when sharing code between crates

### 2. **Dioxus Version Specificity**
- sha-web uses Dioxus 0.7.1 specifically
- Dioxus 0.7 changed APIs significantly (no `cx`, `Scope`, or `use_state`)
- Use `use_signal`, `use_memo`, `use_resource` hooks
- See `crates/sha-web/AGENTS.md` for Dioxus 0.7 patterns

### 3. **Platform-Specific Code**
- sha-cli is terminal-only (ratatui/crossterm)
- sha-web has platform features: `web`, `desktop`, `mobile`
- Default feature is `web`

### 4. **Release Optimization**
- sha-cli has specific release profile optimizations in Cargo.toml
- Uses LTO, reduced codegen units, and stripping for binary size

### 5. **CI/CD Considerations**
- GitHub Actions only configured for sha-cli crate
- No CI for workspace root or other crates
- Dependabot configured for Cargo and GitHub Actions

### 6. **Missing Components**
- No workspace-level tests
- Limited error handling (uses `expect()` frequently)
- Validator library has incomplete implementation
- No shared utilities between crates

## 🔧 Development Workflow

### Adding New Features
1. Check `sha.toml` structure for template configurations
2. Update validator if adding new config fields
3. Update TUI prompts in sha-cli
4. Update web UI in sha-web if needed

### Debugging
- Use `color_eyre` for error reporting in sha-cli
- Dioxus provides hot-reload via `dx serve`
- Check `target/debug/` for build artifacts

### Releasing
1. Update version numbers in individual Cargo.toml files
2. Run `cargo test` to verify all crates
3. Check `cargo fmt` and `cargo clippy`
4. Build with `cargo build --release`

## 📚 External References

### BatiJS (Inspiration)
Shalotts is inspired by [BatiJS](https://github.com/vikejs/bati), a next-generation scaffolding tool. Key concepts borrowed:
- Dynamic boilerplate loading
- Feature selection with dependencies
- Rules engine for validation
- Meta-driven generation
- Hooks system for post-processing

### Dioxus 0.7
sha-web uses Dioxus 0.7 which introduced major API changes:
- No more `cx` or `Scope`
- New reactive primitives: `Signal`, `Memo`, `Resource`
- Component props must implement `PartialEq` and `Clone`
- Router uses derive macros on enums

### Ratatui
sha-cli uses Ratatui for terminal UI:
- Event-driven architecture
- Widget-based rendering
- Crossterm backend for cross-platform terminal

## 🎯 Current Development Focus

Based on commit history and file states:
- **Recent**: CLI + Web crate initialization
- **Examples**: Elysia template scaffolding
- **Validator**: Basic TOML parsing (incomplete)
- **Migration**: Moving from JSON to TOML configs (evident in examples/)

## 🔍 Agent Commands Summary

```bash
# Check project state
git status && git log -n 3

# Build and test everything
cargo build --workspace && cargo test --workspace

# Check specific crates
cargo check --package sha-cli
cargo check --package sha-validator  
cargo check --package sha-web

# Run linting
cargo fmt -- --check
cargo clippy --workspace -- -D warnings

# Generate docs and open
cargo doc --open --no-deps

# Run main CLI
cargo run --package sha-cli

# Run web app
cd crates/sha-web && dx serve
```

## 🚨 Critical Issues to Watch For

1. **TOML Migration**: Examples show JSON→TOML transition; ensure consistency
2. **Incomplete Implementation**: sha-validator has skeleton code
3. **Missing CI**: Only sha-cli has GitHub workflows
4. **Platform Detection**: Web vs Desktop vs Mobile detection needed
5. **Error Handling**: Replace `expect()` with proper `Result` propagation
