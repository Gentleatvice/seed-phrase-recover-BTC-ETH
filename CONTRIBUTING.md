# Contributing to Seed Phrase Auto Recovery

Thank you for your interest in contributing! This document provides guidelines for contributing to this project.

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Follow security best practices

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported
2. Create a detailed issue with:
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - System information
   - Screenshots (if applicable)

### Suggesting Features

1. Check existing feature requests
2. Describe the feature and its benefits
3. Provide use cases
4. Consider implementation details

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Write or update tests
5. Ensure all tests pass: `cargo test`
6. Format your code: `cargo fmt`
7. Run linter: `cargo clippy`
8. Commit with descriptive messages
9. Push to your fork
10. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70+
- Node.js 16+
- Git

### Local Setup

```bash
# Clone your fork
git clone https://github.com/yourusername/seed-phrase-auto-recovery.git
cd seed-phrase-auto-recovery

# Build Rust backend
cargo build

# Run tests
cargo test

# Install web dependencies
cd web
npm install
```

## Code Style

### Rust

- Follow Rust standard formatting (`cargo fmt`)
- Use meaningful variable names
- Add comments for complex logic
- Write unit tests for new functions

### JavaScript

- Use ES6+ features
- Follow consistent naming conventions
- Add JSDoc comments for functions
- Keep functions small and focused

## Testing

### Rust Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### JavaScript Tests

```bash
cd web
npm test
```

## Security Guidelines

⚠️ **CRITICAL**: This tool handles sensitive cryptographic material

- Never log or store seed phrases
- Use secure random number generation
- Validate all inputs
- Follow cryptographic best practices
- Review security-sensitive changes carefully

## Documentation

- Update README.md for user-facing changes
- Add inline documentation for code
- Include examples for new features
- Keep documentation accurate and up-to-date

## Commit Messages

Use clear, descriptive commit messages:

```
feat: add support for Ethereum recovery
fix: correct checksum validation bug
docs: update installation instructions
test: add tests for word suggestion
refactor: optimize recovery algorithm
```

Prefixes:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `test`: Tests
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `chore`: Maintenance

## Questions?

- Open an issue for questions
- Join discussions
- Check existing documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
