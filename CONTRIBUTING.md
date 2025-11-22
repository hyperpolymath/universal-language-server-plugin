# Contributing to Universal Language Connector

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Architecture Overview](#architecture-overview)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Code Style](#code-style)
- [Adding New Features](#adding-new-features)

## Code of Conduct

- Be respectful and constructive
- Welcome newcomers and help them get started
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/universal-language-connector.git`
3. Add upstream remote: `git remote add upstream https://github.com/universal-connector/universal-language-connector.git`
4. Create a branch: `git checkout -b feature/your-feature-name`

## Development Setup

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Node.js 18+ (for VS Code client development)
- Docker/Podman (optional, for container testing)

### Initial Setup

```bash
# Install development tools
make setup

# Build the project
make build

# Run tests
make test

# Start development server
make dev
```

### Project Structure

```
universal-language-server-plugin/
├── server/           # Rust server implementation
├── clients/          # Editor clients (<100 LOC each)
├── web/              # Web UI
├── deployment/       # Docker and deployment configs
├── docs/             # Documentation
└── examples/         # Usage examples
```

## Architecture Overview

### Core Principles

1. **All logic in server** - Clients are thin wrappers (<100 LOC)
2. **LSP 3.17 strict compliance** - No custom extensions without justification
3. **Performance targets** - <100ms response, <50MB memory, <500ms startup
4. **Server-first thinking** - When in doubt, implement in the server

### Technology Stack

**Server:**
- Rust with tokio (async runtime)
- tower-lsp (LSP implementation)
- axum (HTTP API)
- dashmap (concurrent document storage)

**Clients:**
- Language-specific LSP client libraries
- Minimal code, maximum delegation to server

### Key Components

1. **LSP Handler (`server/src/lsp.rs`)** - Language Server Protocol implementation
2. **HTTP API (`server/src/http.rs`)** - REST endpoints
3. **WebSocket (`server/src/websocket.rs`)** - Real-time updates
4. **Conversion Core (`server/src/core.rs`)** - Document format conversion
5. **Document Store (`server/src/document_store.rs`)** - Concurrent document management

## Making Changes

### Workflow

1. **Update your fork:**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Make your changes:**
   - Write code following style guidelines
   - Add tests for new functionality
   - Update documentation

3. **Test your changes:**
   ```bash
   make test
   make lint
   make fmt
   ```

4. **Commit your changes:**
   ```bash
   git add .
   git commit -m "feat: add new conversion format"
   ```

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(core): add YAML conversion support
fix(lsp): handle UTF-16 position correctly
docs(api): update WebSocket examples
```

## Testing

### Running Tests

```bash
# All tests
make test

# Specific test file
cd server && cargo test --test core_tests

# With output
cd server && cargo test -- --nocapture

# Integration tests
make test-integration
```

### Writing Tests

**Unit tests** (same file as code):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        // Test code here
    }
}
```

**Integration tests** (`server/tests/`):
```rust
#[tokio::test]
async fn test_http_endpoint() {
    // Test code here
}
```

### Test Coverage

Aim for >80% code coverage:

```bash
make test-coverage
```

## Submitting Changes

### Pull Request Process

1. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create Pull Request:**
   - Go to GitHub and create a PR from your fork
   - Fill in the PR template
   - Link related issues

3. **PR Requirements:**
   - All tests must pass
   - Code must be formatted (`make fmt`)
   - Linter must pass (`make lint`)
   - Documentation updated if needed
   - At least one review approval

4. **After Approval:**
   - Squash commits if requested
   - Rebase on main if needed
   - Maintainers will merge

### PR Title Format

Use conventional commit format:

```
feat: add support for YAML conversion
fix: resolve UTF-16 position handling bug
docs: improve API documentation
```

## Code Style

### Rust Code

Follow standard Rust conventions:

```rust
// Use rustfmt
cargo fmt

// Pass clippy
cargo clippy -- -D warnings

// Naming
const MAX_SIZE: usize = 100;
fn convert_document() {}
struct DocumentStore {}

// Error handling
fn process() -> Result<Output, Error> {
    // Use ? operator
    let data = fetch_data()?;
    Ok(process_data(data))
}

// Documentation
/// Converts a document between formats.
///
/// # Arguments
/// * `request` - Conversion request with content and formats
///
/// # Returns
/// Converted content or error
pub fn convert(request: ConversionRequest) -> Result<ConversionResponse> {
    // Implementation
}
```

### TypeScript/JavaScript

For client code:

```typescript
// Use consistent formatting
// Prefer async/await over callbacks
// Document public APIs

/**
 * Converts the current document to HTML
 */
async function convertToHtml(): Promise<void> {
  // Implementation
}
```

### Keep Clients Under 100 LOC

This is a hard constraint. If a client exceeds 100 lines:
- Move logic to the server
- Simplify the implementation
- Remove unnecessary code

## Adding New Features

### Adding a New Conversion Format

1. **Add parser dependency** to `server/Cargo.toml`
2. **Implement converter** in `server/src/core.rs`
3. **Add tests** in `server/tests/core_tests.rs`
4. **Update documentation** in `docs/API.md`
5. **Add examples** to `examples/conversions/`

Example:
```rust
// In core.rs
pub enum Format {
    Markdown,
    Html,
    Json,
    Yaml, // New format
}

impl ConversionCore {
    fn markdown_to_yaml(markdown: &str) -> Result<String> {
        // Implementation
    }
}
```

### Adding a New Editor Client

1. **Create directory**: `clients/<editor-name>/`
2. **Implement LSP client** using editor's native API
3. **Keep under 100 LOC**
4. **Add README** with installation instructions
5. **Test end-to-end** with real editor

Template structure:
```
clients/myeditor/
├── plugin.ext           # Main plugin file (<100 LOC)
├── README.md            # Installation guide
└── package.json/config  # Package metadata (if applicable)
```

### Adding HTTP Endpoints

1. **Define route** in `server/src/http.rs`
2. **Add handler function**
3. **Add tests** in `server/tests/http_api_tests.rs`
4. **Update API docs** in `docs/API.md`

Example:
```rust
async fn new_endpoint(
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, ApiError> {
    // Implementation
}

// In create_router()
.route("/api/new-endpoint", post(new_endpoint))
```

### Adding LSP Methods

1. **Implement method** in `server/src/lsp.rs`
2. **Update capabilities** in `initialize()`
3. **Add tests** in `server/tests/lsp_compliance.rs`
4. **Document in** `docs/API.md`

Example:
```rust
#[tower_lsp::async_trait]
impl LanguageServer for UniversalConnectorBackend {
    async fn new_method(&self, params: Params) -> LspResult<Response> {
        // Implementation
    }
}
```

## Performance Guidelines

Always consider performance:

- Use async/await for I/O operations
- Avoid blocking operations on main thread
- Use `dashmap` for concurrent access
- Profile changes with `cargo bench`
- Target <100ms response times

## Documentation

Update documentation for:

- New features
- API changes
- Configuration options
- Breaking changes

Documentation locations:
- API: `docs/API.md`
- Architecture: `docs/ARCHITECTURE.md`
- User guide: `README.md`
- Code comments: Inline documentation

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/universal-connector/universal-language-connector/issues)
- **Discussions**: [GitHub Discussions](https://github.com/universal-connector/universal-language-connector/discussions)
- **Documentation**: See `docs/` directory

## Recognition

Contributors are recognized in:
- README.md contributors section
- Release notes
- Git commit history

Thank you for contributing to Universal Language Connector!
