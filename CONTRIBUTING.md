# Contributing to Expy

Thank you for your interest in contributing to Expy. This document provides guidelines and information for contributors.

## Table of Contents

- [Contributing to Expy](#contributing-to-expy)
  - [Table of Contents](#table-of-contents)
  - [Code of Conduct](#code-of-conduct)
  - [Getting Started](#getting-started)
  - [How to Contribute](#how-to-contribute)
    - [Reporting Bugs](#reporting-bugs)
    - [Suggesting Features](#suggesting-features)
    - [Submitting Changes](#submitting-changes)
  - [Development Workflow](#development-workflow)
    - [Branch Naming](#branch-naming)
  - [Style Guidelines](#style-guidelines)
    - [Rust Code Style](#rust-code-style)
    - [Commit Messages](#commit-messages)
  - [Testing](#testing)
    - [Running Tests](#running-tests)
    - [Writing Tests](#writing-tests)
    - [Test Coverage](#test-coverage)
  - [Review Process](#review-process)
  - [Questions](#questions)

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone. Please be considerate and constructive in all interactions.

## Getting Started

See the [Development section](README.md#Development) for setup instructions.

## How to Contribute

### Reporting Bugs

Before submitting a bug report:

1. Check the [existing issues](https://github.com/nandium/expy/issues) to avoid duplicates
2. Ensure you are using the latest version

When submitting a bug report, include:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- The Excel formula that caused the issue (if applicable)
- Your environment (browser, OS, Rust version)

Use this template:

```markdown
**Description**
A clear description of the bug.

**Steps to Reproduce**
1. Step one
2. Step two
3. Step three

**Expected Behavior**
What you expected to happen.

**Actual Behavior**
What actually happened.

**Input Formula**
`=YOUR_FORMULA_HERE`

**Environment**
- OS: [e.g., macOS 14.0]
- Browser: [e.g., Firefox 120]
- Rust: [output of `rustc --version`]
```

### Suggesting Features

Feature requests are welcome. Please provide:

- A clear description of the feature
- The problem it solves or the use case it addresses
- Examples of Excel formulas that would benefit from this feature
- Any implementation ideas you may have

### Submitting Changes

1. Create a feature branch from `main`:

   ```bash
   git checkout main
   git pull upstream main
   git checkout -b feature/your-feature-name
   ```

2. Make your changes in logical, atomic commits

3. Ensure all checks pass:

   ```bash
   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   wasm-pack build --target web
   ```

4. Push your branch and open a Pull Request:

   ```bash
   git push origin feature/your-feature-name
   ```

## Development Workflow

### Branch Naming

Use descriptive branch names with a prefix:

| Prefix      | Purpose                          |
|-------------|----------------------------------|
| `feature/`  | New features                     |
| `fix/`      | Bug fixes                        |
| `docs/`     | Documentation changes            |
| `refactor/` | Code refactoring                 |
| `test/`     | Adding or updating tests         |
| `chore/`    | Build process or tooling changes |

Examples:

- `feature/support-sumif-function`
- `fix/nested-parentheses-parsing`
- `docs/update-installation-guide`

## Style Guidelines

### Rust Code Style

This project follows the official Rust style guidelines enforced by `rustfmt`.

1. **Format all code before committing:**

   ```bash
   cargo fmt
   ```

2. **Ensure no Clippy warnings:**

   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

### Commit Messages

**Guidelines:**

- Use the imperative mood ("add feature" not "added feature")
- Do not capitalize the first letter of the description
- Do not end the description with a period
- Limit the first line to 72 characters

## Testing

All contributions must include appropriate tests.

### Running Tests

```bash
# Run all native tests
cargo test

# Run WebAssembly tests using node
wasm-pack test --node

# Run WebAssembly tests in the browser
wasm-pack test --headless --firefox
```

### Writing Tests

1. **Unit tests** go in the same file as the code, in a `tests` module:

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_parse_simple_formula() {
           let result = parse("=A1+B1");
           assert!(result.is_ok());
       }

       #[test]
       fn test_parse_invalid_formula() {
           let result = parse("=A1+");
           assert!(result.is_err());
       }
   }
   ```

2. **Integration tests** go in the `tests/` directory

3. **WebAssembly tests** should be marked with `wasm_bindgen_test`:

   ```rust
   #![cfg(target_arch = "wasm32")]

   use wasm_bindgen_test::*;

   wasm_bindgen_test_configure!(run_in_browser);

   #[wasm_bindgen_test]
   fn test_transpile_in_browser() {
       let result = expy::transpile("=SUM(A1:A10)");
       assert!(result.is_ok());
   }
   ```

### Test Coverage

Aim to cover:

- Happy path (valid input)
- Edge cases (empty input, maximum values)
- Error cases (invalid syntax, unsupported functions)
- Boundary conditions

## Review Process

1. All submissions require review before merging
2. Maintainers may request changes or ask questions
3. Address feedback by pushing additional commits to your branch
4. Once approved, a maintainer will merge your Pull Request

**Review Checklist:**

- [ ] Code follows the project style guidelines
- [ ] All tests pass
- [ ] New code has appropriate test coverage
- [ ] Documentation is updated if needed
- [ ] No unrelated changes are included

## Questions

If you have questions about contributing, feel free to:

- Ask in an existing issue
- Reach out to the maintainers

Thank you for contributing to Expy.