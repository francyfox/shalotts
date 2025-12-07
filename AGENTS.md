# AGENT Guidelines for genoplate

This document outlines essential information for AI agents working in this repository.

## Commands

### Build
To build the main project:
```bash
cargo build
```
To build a specific package (e.g., `grrs`):
```bash
cargo build --package grrs
```
To run the main project:
```bash
cargo run
```
To run a specific package:
```bash
cargo run --package grrs
```

### Test
To run tests for the entire workspace:
```bash
cargo test
```
To run tests for a specific package:
```bash
cargo test --package grrs
```

### Run
To run the main executable:
```bash
cargo run
```

## Project Structure

This repository contains a Rust project structured as a workspace.

-   `Cargo.toml`: The main project manifest.
-   `src/main.rs`: The main entry point for the `genoplate` crate.
-   `grrs/`: A sub-crate within the workspace.
    -   `Cargo.toml`: The manifest for the `grrs` crate.
    -   `src/main.rs`: The main entry point for the `grrs` crate.

## Code Conventions

-   **Language**: Rust
-   **Edition**: 2024
-   Standard Rust formatting (enforced by `cargo fmt`) should be followed.
-   Current code uses `println!` for output.

## External Project References

### BatiJS (from vikejs/bati)

**Purpose:** A next-generation scaffolding tool (`genoplate` takes inspiration from it) for quickly creating fully-functional, highly customizable applications. It streamlines project setup by allowing developers to select features/libraries and generate pre-configured, ready-to-start applications.

**Key Concepts/Features from CLI (`packages/cli/index.ts`):**
*   **Dynamic Boilerplate Loading:** Reads `boilerplates.json` and dynamically imports `bati.config.ts` for each boilerplate, allowing for live code configuration and conditional logic.
*   **Feature Selection & Dependencies:** Processes user flags, resolves `dependsOn` relationships between features, and interactively prompts for choices if needed.
*   **Rules Engine:** Validates feature combinations using `@batijs/features/rules` to ensure compatibility and prevent broken generated projects, providing info, warnings, or errors.
*   **`VikeMeta` Object:** A central `meta` object constructed from selected features and environment info, passed throughout the generation process to enable dynamic behavior based on user choices.
*   **Boilerplate Filtering & Sorting:** Boilerplates are filtered by `bl.config.if(meta, pm.name)` conditions and sorted by an `enforce` property (`pre`, `default`, `post`) to control the order of application.
*   **Hooks System:** Supports `after` hooks defined in boilerplate `hooks/` directories for post-processing or additional setup (e.g., `npm install`).
*   **Next Steps Guidance:** Collects and displays `nextSteps` defined in boilerplate configs to guide the user on how to proceed with the generated project.

**Key Concepts/Features from Build Engine (`packages/build/src/index.ts` - the core `exec` function):**
*   **`walk()` Function:** Recursively finds all template files within `boilerplates/<feature>/files/` directories.
*   **`toDist()` Path Mapping:** Determines destination paths, handling special file naming conventions like `$$remove-prefix.tsx?` or `!important-file`, which implies templating for filenames.
*   **Operations Rearrangement:** Collects file operations and sorts them (`OperationsRearranger`) to ensure correct application order (e.g., copy before transform).
*   **Content Transformation Logic:** Differentiates between `kind: "file"` (direct copy/merge) and `kind: "transform"` (code modification/injection for `$` or `!` prefixed files) operations.
*   **Meta-driven Operations:** The `meta: VikeMeta` object is passed to `executeOperationFile` and `executeOperationTransform`, enabling dynamic content generation or modification based on selected features.
*   **Smart File Cleanup:** Uses `RelationImport` to track imports and can automatically remove unimported files, ensuring a lean generated project.
*   **Package.json Handling:** Safely reads, transforms, and writes `package.json` incrementally.

**Relevance to Genoplate:** BatiJS demonstrates an advanced approach to project scaffolding, emphasizing flexibility, customization, and ease of use. `Genoplate`'s "generation seed" concept aligns with Bati's ability for tailored project configurations and its focus on replacing traditional Git templates for project setup. The detailed mechanisms discovered in its CLI and build engine provide concrete patterns for implementing a robust `Genoplate` system. Genoplate's "generation seed" concept aligns with Bati's ability for tailored project configurations and its focus on replacing traditional Git templates for project setup.
