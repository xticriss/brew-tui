# Contributing to brew-tui

Thank you for considering contributing to brew-tui! This document provides guidelines and instructions for contributing.

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Homebrew installed
- Git

### Setup

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/brew-tui.git
   cd brew-tui
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/xticriss/brew-tui.git
   ```
4. Build and test:
   ```bash
   cargo build
   cargo test
   ```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

Use descriptive branch names:
- `feature/tap-management` for new features
- `fix/utf8-truncation` for bug fixes
- `docs/installation-guide` for documentation
- `perf/filter-optimization` for performance improvements

### 2. Make Your Changes

- Write clear, readable code
- Follow Rust conventions and idioms
- Add comments for complex logic
- Update documentation if needed

### 3. Test Your Changes

```bash
# Run unit tests
cargo test

# Run clippy for lints
cargo clippy -- -D warnings

# Format code
cargo fmt

# Build release binary
cargo build --release

# Manual testing
./target/release/brew-tui
```

See [TESTING.md](TESTING.md) for comprehensive manual testing guidelines.

### 4. Commit Your Changes

Write clear commit messages:

```bash
git commit -m "feat: add tap management functionality"
git commit -m "fix: prevent panic on emoji in package names"
git commit -m "perf: optimize filter caching for large package lists"
git commit -m "docs: update installation instructions"
```

**Commit message format**:
- `feat:` for new features
- `fix:` for bug fixes
- `perf:` for performance improvements
- `docs:` for documentation changes
- `test:` for test additions/changes
- `refactor:` for code refactoring
- `chore:` for maintenance tasks

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:
- Clear title describing the change
- Description of what changed and why
- Reference any related issues (`Fixes #123`)
- Screenshots/GIFs for UI changes

## Code Guidelines

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Pass `cargo clippy` without warnings
- Prefer explicit over implicit
- Use descriptive variable names

### Performance

- Avoid unnecessary allocations
- Use async/await for I/O operations
- Cache computed results when appropriate
- Benchmark performance-critical code

### Safety

- Validate all user input
- Check UTF-8 boundaries when slicing strings
- Handle errors explicitly (avoid `.unwrap()` in production code)
- Use `.expect()` with descriptive messages for invariants

### UI/UX

- Keep the interface minimal and focused
- Maintain consistent keybindings
- Provide clear feedback for operations
- Use loading indicators for async operations
- Confirm destructive actions

## Project Structure

```
brew-tui/
├── src/
│   ├── app.rs              # Core application state
│   ├── brew/               # Homebrew integration
│   │   ├── commands.rs     # Async brew operations
│   │   └── types.rs        # Data structures
│   ├── events/             # Event handling
│   │   ├── handler.rs      # Input handling
│   │   └── mod.rs
│   ├── ui/                 # Terminal UI
│   │   ├── constants.rs    # UI constants
│   │   ├── utils.rs        # Shared utilities
│   │   ├── colors.rs       # Theme
│   │   ├── package_list.rs # Main package table
│   │   ├── details.rs      # Details panel
│   │   ├── dependency_tree.rs
│   │   └── ...
│   └── main.rs             # Entry point
├── tests/                  # Integration tests
├── TESTING.md             # Manual testing guide
├── IMPROVEMENTS.md        # Optimization history
└── README.md
```

## Testing

### Unit Tests

Add tests for new functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_function() {
        assert_eq!(your_function(input), expected);
    }
}
```

### Integration Tests

Add tests in `tests/` directory for end-to-end scenarios.

### Manual Testing

Follow the checklist in [TESTING.md](TESTING.md):
- UTF-8 safety (emoji, CJK characters)
- Navigation edge cases
- Performance with large package lists
- All operations and confirmations

## Pull Request Process

1. **Update documentation** if you've changed APIs or added features
2. **Add tests** for new functionality
3. **Update TESTING.md** if you've added testable features
4. **Run the full test suite** before submitting
5. **Squash commits** if you have many small commits
6. **Respond to feedback** promptly

### Pull Request Checklist

- [ ] Code follows project style guidelines
- [ ] Tests pass: `cargo test`
- [ ] Clippy passes: `cargo clippy`
- [ ] Code formatted: `cargo fmt`
- [ ] Documentation updated if needed
- [ ] Manual testing completed (from TESTING.md)
- [ ] Commit messages are clear
- [ ] PR description explains what and why

## Reporting Bugs

When filing an issue, include:

1. **Environment**:
   - Rust version (`rustc --version`)
   - OS and version
   - Homebrew version (`brew --version`)
   - Terminal emulator

2. **Steps to reproduce**:
   - What you did
   - What you expected
   - What actually happened

3. **Logs/Output**:
   - Error messages
   - Stack traces
   - Screenshots for UI issues

4. **Additional context**:
   - Number of packages installed
   - Relevant system specs

## Feature Requests

For feature requests, describe:

- **Problem**: What pain point does this solve?
- **Proposed solution**: How should it work?
- **Alternatives considered**: Other approaches?
- **UI mockups**: Screenshots or sketches if applicable

## Code Review

All submissions require review. We look for:

- **Correctness**: Does it work as intended?
- **Safety**: No panics, proper error handling
- **Performance**: No unnecessary slowdowns
- **Maintainability**: Clear, readable code
- **Testing**: Adequate test coverage

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

- Open an issue with the `question` label
- Check existing issues and pull requests
- Review the README and documentation

Thank you for contributing to brew-tui! 🎉
