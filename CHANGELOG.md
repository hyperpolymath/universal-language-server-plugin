# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- JWT-based authentication for HTTP API
- Rate limiting for all endpoints
- TLS support for encrypted connections
- WASM core module for sandboxed conversion execution
- Advanced format support (YAML, XML, TOML)
- Plugin system for custom converters
- LSP 3.18 feature support
- Performance optimizations

## [0.1.0] - 2025-11-22

### Added
- **LSP Server**: Complete LSP 3.17 implementation with tower-lsp
  - Document synchronization (didOpen, didChange, didSave, didClose)
  - Code completion with format-aware suggestions
  - Hover information showing document statistics
  - Execute commands for document conversion
  - Diagnostic validation for document formats
- **HTTP REST API**: Full REST API with axum
  - POST /api/convert - Convert documents between formats
  - GET /api/documents - List all documents
  - GET /api/documents/:id - Get specific document
  - DELETE /api/documents/:id - Delete document
  - POST /api/validate - Validate document format
  - GET /api/stats - Server statistics
  - GET /api/health - Health check endpoint
- **WebSocket Server**: Real-time document updates
  - Subscribe/unsubscribe to document changes
  - Document update notifications
  - Ping/pong keepalive
  - Broadcast messaging
- **Conversion Engine**: Bidirectional format conversion
  - Markdown ↔ HTML (full support via pulldown-cmark)
  - Markdown ↔ JSON (structured representation)
  - HTML ↔ JSON (DOM structure extraction)
  - Format validation and diagnostics
- **Document Store**: Lock-free concurrent storage
  - DashMap for thread-safe operations
  - Document versioning
  - Metadata tracking (creation time, modification time)
  - UUID-based identification
- **Editor Clients**: 7 editor integrations (all <100 LOC)
  - VS Code extension (~70 LOC)
  - Neovim plugin (~65 LOC)
  - Emacs package (~75 LOC)
  - JetBrains plugin (~55 LOC)
  - Sublime Text plugin (~60 LOC)
  - Zed configuration
  - Helix configuration
- **Web UI**: Single-page application
  - Live document converter
  - Real-time dashboard with WebSocket updates
  - Document manager interface
  - Server statistics display
  - Responsive design for mobile/desktop
- **Infrastructure**:
  - Dockerfile with multi-stage builds
  - docker-compose.yml for orchestration
  - podman-compose.yml for Podman support
  - Comprehensive Makefile with 20+ targets
  - Example configurations and conversions
- **Documentation**:
  - Complete README with quick start
  - API documentation with examples
  - CONTRIBUTING.md for developers
  - SECURITY.md for security policies
  - CODE_OF_CONDUCT.md for community
  - MAINTAINERS.md for governance
- **Testing**:
  - LSP compliance test suite
  - HTTP API integration tests
  - Core conversion engine tests
  - Document store concurrency tests
- **RSR Compliance**:
  - .well-known/security.txt (RFC 9116)
  - .well-known/ai.txt (AI training policies)
  - .well-known/humans.txt (attribution)
  - justfile with build recipes
  - flake.nix for Nix reproducible builds
  - .gitlab-ci.yml for CI/CD

### Changed
- N/A (initial release)

### Deprecated
- N/A (initial release)

### Removed
- N/A (initial release)

### Fixed
- N/A (initial release)

### Security
- Memory safety guaranteed by Rust ownership system
- No unsafe code blocks in implementation
- Input validation for all HTTP endpoints
- LSP message validation
- CORS enabled for web UI access

## Version History

- [0.1.0] - 2025-11-22: Initial release with core functionality

## Migration Guides

### From Non-LSP Solutions

If migrating from traditional editor-specific plugins:

1. **Install Server**: Build and install the Rust server
2. **Install Client**: Install editor-specific client for your editor
3. **Configure**: Set server path in client configuration
4. **Test**: Open a Markdown/HTML/JSON file and test conversion commands

### Future Migrations

When upgrading to future versions, consult version-specific migration guides.

## Deprecation Policy

- **Minor versions**: Features deprecated with 6-month warning
- **Major versions**: Breaking changes allowed with migration guide
- **Security**: Immediate deprecation if security risk identified

## Support

- **Current Version**: 0.1.0 (active development)
- **LTS**: Not yet designated
- **EOL**: None yet

## Links

- [Repository](https://github.com/universal-connector/universal-language-connector)
- [Issue Tracker](https://github.com/universal-connector/universal-language-connector/issues)
- [Documentation](https://universal-connector.org/docs)
- [Changelog](https://github.com/universal-connector/universal-language-connector/blob/main/CHANGELOG.md)

---

**Changelog Format**: [Keep a Changelog](https://keepachangelog.com/)
**Versioning**: [Semantic Versioning](https://semver.org/)
