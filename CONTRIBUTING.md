# Contributing to common_lib

Thank you for your interest in contributing to `common_lib`! We welcome bug reports, feature requests, and code contributions.

To ensure a smooth collaboration, please follow the guidelines below.

## 1. Local Development Flow

### Prerequisites
Make sure you have Rust (stable, 2024 edition or newer) installed.

### Pre-commit Validation Process
Before submitting a pull request or committing changes, you must run the following validation pipeline locally and ensure all checks pass with zero errors and warnings:

1. **Unit Tests**:
   ```bash
   cargo test
   ```
2. **Clippy (Linter)**:
   Ensure code conforms to Rust idiomatic styles and has no warnings:
   ```bash
   cargo clippy --all-targets -- -D warnings
   ```
3. **Rustfmt (Formatter)**:
   Ensure code formatting conforms strictly to standard Rust guidelines:
   ```bash
   cargo fmt --check
   ```

---

## 2. Coding Guidelines
- **Unit Testing**: If you add a new feature or modify core logic, you should add or expand unit tests (usually placed in the `tests` module at the bottom of the file).
- **Documentation**: All public APIs (`pub fn`, `pub struct`, `pub enum`) must be documented with doc comments (`///`). If API specifications change, remember to update `docs/SPEC.md` and `docs/DIAGRAM.md`.
- **Config & Workflow Protection**: Do not modify files under `.github/workflows/`, `.github/dependabot.yml`, or `.editorconfig` without explicit approval and recording the changes in `CHANGELOG.md`.

---

## 3. How to Contribute

### Step 1: Open an Issue / Propose Changes
If you find a bug or want to propose a feature, please open an issue first to discuss it.

### Step 2: Create a Branch
Work on a separate branch for your changes:
```bash
git checkout -b feature/your-feature-name
```

### Step 3: Implement and Document
Write your code, add unit tests, and update documentation as needed.

### Step 4: Record Changes in CHANGELOG.md
Every contribution must add a record of changes in `CHANGELOG.md`. Follow the [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format:
- `Added`: for new features.
- `Fixed`: for bug fixes.
- `Optimized`: for performance improvements.
- `Removed`: for deprecated features.

### Step 5: Submit a Pull Request
Push your branch to your repository and open a Pull Request. Once CI passes and code review is complete, your changes will be merged.

---

Thank you for helping us keep `common_lib` reliable and robust!
