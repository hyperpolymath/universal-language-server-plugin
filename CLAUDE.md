# CLAUDE.md - Universal Language Connector

## Project Overview

The **Universal Language Connector** is an LSP-based universal plugin architecture that enables **one server to power plugins across all major editors** (VS Code, Neovim, Emacs, JetBrains, Sublime, Zed, Helix). Thin editor clients (<100 LOC each) delegate all logic to a centralized Rust server via Language Server Protocol, with HTTP/WebSocket APIs for web UI integration.

### Core Value Proposition

**"Write once, run everywhere"** - By strictly adhering to the Language Server Protocol (LSP 3.17), we achieve true editor-agnostic functionality. The server handles all business logic, ensuring consistency across editors while keeping clients trivially simple.

### Purpose

This project provides:
- **Universal document conversion** (markdown ↔ HTML ↔ JSON ↔ custom formats)
- **Real-time collaboration** via WebSocket
- **Multi-editor support** with minimal per-editor code
- **High performance** (<100ms responses, <50MB memory, <500ms startup)
- **Web-based management UI** for non-editor workflows

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Editor Clients (<100 LOC)                 │
│  VS Code │ Neovim │ Emacs │ JetBrains │ Sublime │ Zed │ Helix│
└───────────────────────────┬─────────────────────────────────┘
                            │ LSP over stdio
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                  Universal Connector Server (Rust)           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ LSP Handler  │  │  HTTP API    │  │  WebSocket   │      │
│  │ (tower-lsp)  │  │  (axum)      │  │  (real-time) │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         └──────────────────┴──────────────────┘              │
│                            │                                 │
│                  ┌─────────▼─────────┐                       │
│                  │  Conversion Core   │                      │
│                  │  (markdown/html/   │                      │
│                  │   json/custom)     │                      │
│                  └────────────────────┘                      │
│                            │                                 │
│                  ┌─────────▼─────────┐                       │
│                  │  Document Store    │                      │
│                  │  (dashmap - lock-  │                      │
│                  │   free concurrent) │                      │
│                  └────────────────────┘                      │
└─────────────────────────────────────────────────────────────┘
                            │ HTTP/WebSocket
                            ↓
                   ┌────────────────┐
                   │   Web UI       │
                   │ (Single-page   │
                   │  HTML app)     │
                   └────────────────┘
```

## Project Structure

```
universal-language-server-plugin/
├── server/                          # Rust server implementation
│   ├── src/
│   │   ├── main.rs                 # Entry point, server orchestration
│   │   ├── lsp.rs                  # LSP handler (tower-lsp)
│   │   ├── http.rs                 # HTTP REST API (axum)
│   │   ├── websocket.rs            # WebSocket server
│   │   ├── core.rs                 # Conversion engine core
│   │   └── document_store.rs       # Concurrent document storage (dashmap)
│   ├── tests/
│   │   ├── lsp_compliance.rs       # LSP 3.17 compliance tests
│   │   ├── core_tests.rs           # Conversion engine tests
│   │   └── http_api_tests.rs       # HTTP API tests
│   ├── Cargo.toml                  # Dependencies
│   └── Cargo.lock
├── clients/                         # Editor clients (<100 LOC each)
│   ├── vscode/                     # VS Code extension (~70 LOC)
│   │   ├── extension.ts
│   │   └── package.json
│   ├── neovim/                     # Neovim plugin (~65 LOC)
│   │   └── init.lua
│   ├── emacs/                      # Emacs package (~75 LOC)
│   │   └── universal-connector.el
│   ├── jetbrains/                  # JetBrains plugin (~55 LOC)
│   │   └── UniversalConnector.kt
│   ├── sublime/                    # Sublime Text plugin (~60 LOC)
│   │   └── UniversalConnector.py
│   ├── zed/                        # Zed configuration
│   │   └── settings.json
│   └── helix/                      # Helix configuration
│       └── languages.toml
├── web/                            # Web UI
│   ├── index.html                  # Single-page app
│   ├── styles.css
│   └── app.js
├── deployment/                     # Container deployment
│   ├── Dockerfile
│   ├── podman-compose.yml
│   └── docker-compose.yml
├── docs/                           # Documentation
│   ├── API.md                      # Complete API specification
│   ├── LSP_COMPLIANCE.md           # LSP 3.17 implementation notes
│   ├── CLIENT_DEVELOPMENT.md       # Guide for adding new editors
│   └── ARCHITECTURE.md             # Deep dive into design decisions
├── examples/                       # Usage examples
│   ├── configs/                    # Example configurations
│   └── conversions/                # Sample conversion workflows
├── Makefile                        # Build system
├── CONTRIBUTING.md                 # Development guide
├── CLAUDE.md                       # This file
└── README.md                       # User-facing documentation
```

## Technology Stack

### Server (Rust)
- **tower-lsp** - LSP 3.17 implementation framework
- **tokio** - Async runtime for high-performance I/O
- **axum** - HTTP REST API framework
- **tokio-tungstenite** - WebSocket support
- **dashmap** - Lock-free concurrent HashMap for document storage
- **serde** - JSON serialization/deserialization
- **chrono** - Timestamp handling for WebSocket events

### Clients
- **VS Code**: TypeScript with vscode-languageclient
- **Neovim**: Lua with built-in LSP client
- **Emacs**: Emacs Lisp with lsp-mode
- **JetBrains**: Kotlin with IntelliJ Platform SDK
- **Sublime**: Python with LSP package
- **Zed/Helix**: TOML/JSON configuration (native LSP support)

### Web UI
- Vanilla JavaScript (no frameworks)
- WebSocket for real-time updates
- Fetch API for HTTP requests

## Key Design Principles

### 1. All Logic in Server, Clients Are Thin Wrappers

**Rationale**: Ensures consistency, single source of truth, and maintainability.

Each editor client MUST:
- Be <100 LOC (proves architecture works)
- Only handle editor-specific initialization
- Delegate ALL business logic to server via LSP
- Never implement conversion logic locally

### 2. LSP 3.17 Strict Compliance

**Rationale**: Guarantees editor compatibility and leverages battle-tested protocol.

- Follow spec exactly: https://microsoft.github.io/language-server-protocol/
- No custom extensions unless absolutely necessary
- Use stdio for communication (LSP standard, no network complexity)
- Implement all core LSP methods

### 3. Performance Targets

**Non-negotiable constraints**:
- Response time: <100ms for all operations
- Memory usage: <50MB steady state
- Startup time: <500ms
- Concurrent connections: Support 100+ simultaneous clients

### 4. "Write Once, Run Everywhere" via LSP

**Critical insight**: The "universal" part works because LSP abstracts editor differences.

- Focus on LSP compliance, not editor-specific features
- When in doubt, delegate to server
- Don't fight the protocol - embrace it

## Development Guidelines

### Code Style

**Rust Server**:
- Use `rustfmt` for formatting
- Enable clippy lints (`#![deny(clippy::all)]`)
- Prefer async/await over callbacks
- Use `Result<T, E>` for all fallible operations
- Document public APIs with `///` comments
- Keep functions under 50 lines

**Editor Clients**:
- Follow language-specific conventions (rustfmt, prettier, etc.)
- Minimize dependencies
- Keep under 100 LOC (hard constraint)
- Clear comments explaining LSP integration points

### Testing

**Server**:
- Unit tests for conversion core
- Integration tests for LSP compliance
- HTTP API contract tests
- Performance benchmarks

**Clients**:
- Manual testing (automated testing adds too much complexity for <100 LOC)
- Test against real editors
- Verify all LSP methods work

**Coverage target**: >80% for server code

### Performance Optimization

- Use `dashmap` for concurrent document storage (lock-free)
- Profile with `cargo flamegraph`
- Benchmark critical paths with `criterion`
- Lazy initialization where possible
- Consider WASM for portable, sandboxed core module (future)

## LSP Implementation Details

### Supported LSP Methods

**Document Synchronization**:
- `textDocument/didOpen`
- `textDocument/didChange`
- `textDocument/didSave`
- `textDocument/didClose`

**Language Features**:
- `textDocument/completion` - Format-aware completions
- `textDocument/hover` - Document statistics and preview
- `textDocument/definition` - Navigate to conversion definitions
- `textDocument/diagnostic` - Validate document format

**Workspace Features**:
- `workspace/didChangeConfiguration`
- `workspace/executeCommand` - Trigger conversions

### LSP Capabilities

```rust
ServerCapabilities {
    text_document_sync: Some(TextDocumentSyncCapability::Kind(
        TextDocumentSyncKind::INCREMENTAL,
    )),
    completion_provider: Some(CompletionOptions {
        trigger_characters: Some(vec!["#".to_string(), "@".to_string()]),
        ..Default::default()
    }),
    hover_provider: Some(HoverProviderCapability::Simple(true)),
    definition_provider: Some(OneOf::Left(true)),
    diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
        DiagnosticOptions::default(),
    )),
    execute_command_provider: Some(ExecuteCommandOptions {
        commands: vec![
            "convert.toMarkdown".to_string(),
            "convert.toHtml".to_string(),
            "convert.toJson".to_string(),
        ],
        ..Default::default()
    }),
    ..Default::default()
}
```

## HTTP/WebSocket API

### HTTP Endpoints

```
POST   /api/convert          # Convert document
GET    /api/documents         # List all documents
GET    /api/documents/:id     # Get document by ID
DELETE /api/documents/:id     # Delete document
POST   /api/validate          # Validate document format
GET    /api/stats             # Server statistics
GET    /api/health            # Health check
```

### WebSocket Events

**Client → Server**:
```json
{
  "type": "subscribe",
  "documentId": "doc-123"
}
```

**Server → Client**:
```json
{
  "type": "documentUpdated",
  "documentId": "doc-123",
  "content": "...",
  "timestamp": "2025-11-22T12:00:00Z"
}
```

## Conversion Core

### Supported Formats

1. **Markdown** → HTML, JSON
2. **HTML** → Markdown, JSON
3. **JSON** → Markdown, HTML
4. **Custom formats** (extensible via plugin system)

### Conversion Pipeline

```
Input → Parse → Validate → Transform → Serialize → Output
         ↓        ↓           ↓          ↓
      AST     Diagnostics   IR      Format-specific
```

### Placeholder Implementation

Current implementation uses placeholder conversions for proof-of-concept. Real conversion logic should:
- Use proper parsers (pulldown-cmark for Markdown, scraper for HTML)
- Generate rich AST representations
- Provide bidirectional conversion with minimal loss
- Validate output format

## Client Development

### Adding a New Editor

1. Create directory in `clients/<editor-name>/`
2. Initialize LSP client using editor's native API
3. Connect to server via stdio
4. Register document types
5. Keep code under 100 LOC
6. Test end-to-end

### Example: VS Code Client

```typescript
// clients/vscode/extension.ts (~70 LOC)
import * as vscode from 'vscode';
import { LanguageClient, ServerOptions, LanguageClientOptions } from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  const serverOptions: ServerOptions = {
    command: 'universal-connector-server',
    args: [],
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: '*' }],
  };

  client = new LanguageClient('universalConnector', 'Universal Connector', serverOptions, clientOptions);
  client.start();
}

export function deactivate() {
  return client?.stop();
}
```

## Deployment

### Docker

```bash
# Build image
make docker-build

# Run server
docker run -p 8080:8080 universal-connector

# Or with compose
docker-compose up
```

### Standalone Binary

```bash
# Build release binary
cargo build --release

# Run server
./target/release/universal-connector-server

# Server listens on:
# - stdio (LSP)
# - 0.0.0.0:8080 (HTTP)
# - 0.0.0.0:8081 (WebSocket)
```

## Build System

### Makefile Targets

```bash
make build          # Build server (debug)
make release        # Build server (release)
make test           # Run all tests
make bench          # Run benchmarks
make docker-build   # Build Docker image
make clean          # Clean build artifacts
make install        # Install server binary
make clients        # Build all editor clients
```

## Common Tasks

### Running the Server

```bash
# Development mode (hot reload)
cargo watch -x run

# Production mode
cargo run --release

# With logging
RUST_LOG=debug cargo run
```

### Testing

```bash
# All tests
cargo test

# LSP compliance only
cargo test lsp_compliance

# With coverage
cargo tarpaulin --out Html

# Performance benchmarks
cargo bench
```

### Adding a Conversion Format

1. Add parser dependency to `Cargo.toml`
2. Implement converter in `src/core.rs`
3. Add tests in `tests/core_tests.rs`
4. Update API documentation
5. Add example to `examples/conversions/`

## Critical Architecture Insights

### Why This Works

The "universal" approach succeeds because:
1. **LSP abstracts editor differences** - We don't need editor-specific code
2. **Stdio is universal** - Every editor can spawn processes
3. **Server holds state** - Clients are stateless, making them trivial
4. **Protocol is well-defined** - LSP 3.17 is battle-tested

### Gotchas

1. **Position-to-offset conversion**: LSP uses UTF-16 positions, Rust uses UTF-8
   - Use `tower-lsp::lsp_types::Position` carefully
   - Test with non-ASCII characters

2. **Client LOC constraint**: If a client exceeds 100 LOC:
   - You're doing too much in the client
   - Move logic to the server
   - Simplify initialization

3. **WebSocket timestamps**: Need `chrono` dependency
   - Already added to `Cargo.toml`
   - Use UTC for consistency

4. **VS Code packaging**: Needs `package.json` with proper `engines.vscode` field
   - Minimum version: "^1.80.0"

## Next Steps for Development

1. ✅ Complete Rust server implementation
2. ✅ Implement all 7 editor clients
3. ✅ Build web UI
4. ⏳ Replace placeholder conversions with real parsers
5. ⏳ Add WASM core module for sandboxed execution
6. ⏳ End-to-end testing with real editors
7. ⏳ Performance profiling and optimization
8. ⏳ LSP 3.17 compliance testing with official suite
9. ⏳ Container deployment testing
10. ⏳ Production readiness review

## Design Philosophy

### Principles

1. **Write once, run everywhere** - LSP makes this possible
2. **When in doubt, delegate to server** - Keeps clients simple
3. **Performance is a feature** - Sub-100ms responses required
4. **Strict compliance** - Follow LSP 3.17 exactly
5. **Web UI is secondary** - LSP is the core value proposition

### Anti-patterns to Avoid

❌ Implementing business logic in clients
❌ Custom LSP extensions without strong justification
❌ Blocking operations on main thread
❌ Client-side state management
❌ Editor-specific features that break universality

### Patterns to Embrace

✅ Thin clients that delegate everything
✅ Server-side caching and optimization
✅ Async/await for all I/O
✅ Lock-free data structures (dashmap)
✅ Incremental document synchronization

## Resources

### LSP
- [LSP Specification 3.17](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
- [tower-lsp Documentation](https://docs.rs/tower-lsp/)
- [LSP Inspector](https://microsoft.github.io/language-server-protocol/inspector/)

### Rust
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Axum Documentation](https://docs.rs/axum/)
- [DashMap Documentation](https://docs.rs/dashmap/)

### Editor Integration
- [VS Code LSP Client](https://code.visualstudio.com/api/language-extensions/language-server-extension-guide)
- [Neovim LSP](https://neovim.io/doc/user/lsp.html)
- [Emacs LSP Mode](https://emacs-lsp.github.io/lsp-mode/)

## Notes for Claude

- **Performance is critical** - Always profile changes
- **LSP compliance** - Never deviate from spec without strong reason
- **Client simplicity** - If adding lines to a client, reconsider the approach
- **Server-first thinking** - Default to implementing features in the server
- **Test with real editors** - Manual testing is essential
- **Document design decisions** - Future maintainers will thank you
- **Consider WASM** - For portable, sandboxed core module (not yet implemented)

## Current Status

- ✅ Server: Complete Rust implementation (1,500+ LOC)
- ✅ Clients: All 7 editor clients implemented
- ✅ Web UI: Single-page HTML app with dashboard
- ✅ Deployment: Dockerfile, podman-compose.yml, Makefile
- ✅ Tests: LSP compliance, core engine, HTTP API suites
- ✅ Documentation: API spec, READMEs, CONTRIBUTING.md
- ⏳ Production: Needs real conversion logic and performance validation
