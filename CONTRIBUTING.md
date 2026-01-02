# Contributing to TeleSpot

Thank you for considering contributing to TeleSpot! This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- Git

### Getting Started

1. **Fork the repository**

2. **Clone your fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/Telespot.git
   cd Telespot/rust
   ```

3. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Build and test**
   ```bash
   cargo build
   cargo test
   ```

## Code Style

We use standard Rust formatting:

```bash
# Format your code
cargo fmt

# Run the linter
cargo clippy
```

## Testing

Please add tests for new functionality:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Pull Request Process

1. **Update documentation** - Add/update docs for any new features
2. **Add tests** - Include tests for new functionality
3. **Run formatting** - `cargo fmt` before committing
4. **Run clippy** - `cargo clippy` and address any warnings
5. **Update CHANGELOG** - Add your changes to the unreleased section
6. **Write clear commits** - Use descriptive commit messages

### Commit Message Format

```
type(scope): brief description

Longer description if needed.

Fixes #issue_number
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

### Example

```
feat(search): add Yahoo search engine support

Adds Yahoo as a fourth search engine option alongside
Google, Bing, and DuckDuckGo.

Fixes #42
```

## Project Structure

```
telespot-rust/
├── src/
│   ├── main.rs         # CLI and main execution
│   ├── phone.rs        # Phone number formatting
│   ├── search.rs       # Search engine integration
│   ├── analysis.rs     # Pattern analysis
│   └── us_states.rs    # US states data
├── Cargo.toml          # Dependencies
├── README.md           # Documentation
└── tests/              # Integration tests
```

## Adding New Features

### Adding a New Search Engine

1. Add the search function to `src/search.rs`
2. Follow the existing pattern (return `Vec<SearchResult>`)
3. Add error handling
4. Update `main.rs` to call the new engine
5. Add tests
6. Update README

### Adding New Pattern Extraction

1. Add extraction function to `src/analysis.rs`
2. Update `PatternAnalysis` struct if needed
3. Update `print_summary` to display new patterns
4. Add tests
5. Update README

## Bug Reports

When filing a bug report, please include:

- OS and version
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior
- Any error messages or logs

Use the bug report template in GitHub Issues.

## Feature Requests

We love feature ideas! Please:

- Check existing issues first
- Describe the use case
- Explain why it would be useful
- Consider implementation complexity

## Questions?

- Open a GitHub Discussion
- Check existing issues and PRs
- Read the README thoroughly

## Code of Conduct

- Be respectful and professional
- Welcome newcomers
- Focus on constructive feedback
- Remember we're all volunteers

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

Thank you for contributing to TeleSpot! 🚀
