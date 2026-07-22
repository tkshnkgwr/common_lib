# AI Coding Instructions (INSTRUCTIONS.md) - common_lib

**English** | [日本語版](../ja/INSTRUCTIONS.md)

Guidelines, conventions, and rules for AI agents and developers modifying or extending the `common_lib` project.

---

## 1. Naming Conventions

Follow idiomatic Rust naming conventions:

- **Variables, Functions, Macros**: Snake case (`snake_case`)
  - Examples: `check_single_instance`, `suggest_tags`, `raw_bytes`
- **Types, Structs, Enums, Traits**: Pascal case (`PascalCase`)
  - Examples: `SingleInstanceGuard`, `Error`, `DiffType`, `DiffPart`
- **Constants, Statics**: Screaming snake case (`SCREAMING_SNAKE_CASE`)
  - Example: `MAX_BUFFER_SIZE`
- **Files & Directories**:
  - Source code in snake case (`snake_case.rs`)
  - Documentation files in uppercase snake case (`DOC_NAME.md`)
    - Examples: `ARCHITECTURE.md`, `SPEC.md`
  - Multilingual documentation structure: Place under `docs/en/` and `docs/ja/` without language suffixes (`_JA`, etc.)
- **Test Functions**:
  - Snake case starting with `test_` describing target and behavior.
  - Examples: `test_acquire_single_instance`, `test_count_occurrences`

---

## 2. Error Handling Policy

Prioritize avoiding panics (`panic!`, `unwrap`). Always return errors via `Result` to allow caller handling.

- **Common Error Type**:
  - Use `crate::Error` enum and `crate::Result<T>` alias.
  - When introducing new error categories, add new variants to `Error` in `src/error.rs` and implement `Display` with descriptive error messages.
- **Platform Agnostic Errors**:
  - Handle Win32 API errors cleanly and convert them into common `Error` variants (e.g. `Error::MutexCreation`).

---

## 3. Component & Module Division Rules

Organize code based on cohesion and platform dependencies:

- **OS / Platform-dependent Logic**:
  - Place platform-specific code in `src/desktop.rs`.
  - Use `#[cfg(target_os = "windows")]` and feature flags like `#[cfg(feature = "windows_desktop")]`.
  - Provide fallback dummy structs and functions for non-Windows platforms or disabled feature builds.
- **Platform-agnostic Logic**:
  - Place text processing, string searching, and diff computation in `src/text.rs`.
- **Adding New Modules**:
  - When introducing new feature domains, create new module files under `src/` and re-export them in `src/lib.rs`.
- **Refactoring Threshold**:
  - If a source file (`.rs`) exceeds **1,000 lines**, AI agents must propose refactoring the code into smaller, focused submodules.

---

## 4. AI Response Formatting Rules

When presenting answers or proposals to the user, AI agents (Gem) must strictly follow these rules:

- **Concise Explanations**: Keep rationale brief and focus on code snippets, diffs, and precise steps.
- **Clickable File Links**:
  - Always use standard markdown links with `file://` scheme without backticks around link text.
  - Good: `[lib.rs](file:///c:/Users/632792/Documents/自作/common_lib/src/lib.rs)`
  - Bad: ``[`lib.rs`](file://...)`` (backticks break link rendering)
- **Structured Implementation Plans**: Group modified files logically by component.
