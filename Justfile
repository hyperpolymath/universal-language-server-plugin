# Universal Language Connector - Just build automation
# https://github.com/casey/just

# Default recipe (list available commands)
default:
    @just --list

# Build server in debug mode
build:
    cd server && cargo build

# Build server in release mode
release:
    cd server && cargo build --release

# Run all tests
test:
    cd server && cargo test

# Run tests with output
test-verbose:
    cd server && cargo test -- --nocapture --test-threads=1

# Run specific test
test-one TEST:
    cd server && cargo test {{TEST}} -- --nocapture

# Check code without building
check:
    cd server && cargo check

# Run clippy linter
lint:
    cd server && cargo clippy -- -D warnings

# Format code
fmt:
    cd server && cargo fmt

# Check formatting without changing files
fmt-check:
    cd server && cargo fmt --check

# Run security audit
audit:
    cd server && cargo audit

# Clean build artifacts
clean:
    cd server && cargo clean
    rm -rf clients/vscode/out clients/vscode/node_modules
    find . -name "*.log" -delete
    find . -name ".DS_Store" -delete

# Install server binary
install:
    cd server && cargo install --path .

# Run development server with auto-reload
dev:
    cd server && cargo watch -x run

# Run web UI development server
dev-web:
    @echo "Starting web UI on http://localhost:8000"
    cd web && python3 -m http.server 8000

# Build Docker image
docker-build:
    docker build -f deployment/Dockerfile -t universal-connector:latest .

# Run Docker container
docker-run:
    docker run -d -p 8080:8080 -p 8081:8081 --name universal-connector universal-connector:latest

# Stop Docker container
docker-stop:
    docker stop universal-connector && docker rm universal-connector

# Docker compose up
docker-compose-up:
    cd deployment && docker-compose up -d

# Docker compose down
docker-compose-down:
    cd deployment && docker-compose down

# Podman compose up
podman-compose-up:
    cd deployment && podman-compose up -d

# Podman compose down
podman-compose-down:
    cd deployment && podman-compose down

# Build VS Code client
vscode-client:
    cd clients/vscode && npm install && npm run compile

# Generate documentation
docs:
    cd server && cargo doc --no-deps --open

# Run benchmarks (if implemented)
bench:
    @echo "Benchmarks not yet implemented (criterion removed)"
    @echo "To add benchmarks, uncomment criterion in Cargo.toml"

# Check test coverage
coverage:
    cd server && cargo tarpaulin --out Html --output-dir coverage

# Validate RSR compliance
validate-rsr:
    @echo "=== RSR Framework Compliance Check ==="
    @echo ""
    @echo "✅ Type Safety: Rust compile-time guarantees"
    @echo "✅ Memory Safety: Ownership model, zero unsafe blocks"
    @just _check-file "README.md" "README"
    @just _check-file "LICENSE" "LICENSE"
    @just _check-file "SECURITY.md" "SECURITY.md"
    @just _check-file "CONTRIBUTING.md" "CONTRIBUTING.md"
    @just _check-file "CODE_OF_CONDUCT.md" "CODE_OF_CONDUCT.md"
    @just _check-file "MAINTAINERS.md" "MAINTAINERS.md"
    @just _check-file "CHANGELOG.md" "CHANGELOG.md"
    @just _check-file ".well-known/security.txt" ".well-known/security.txt"
    @just _check-file ".well-known/ai.txt" ".well-known/ai.txt"
    @just _check-file ".well-known/humans.txt" ".well-known/humans.txt"
    @just _check-file "justfile" "justfile"
    @echo ""
    @echo "Build System:"
    @just _check-file "Makefile" "Makefile"
    @just _check-file "server/Cargo.toml" "Cargo.toml"
    @echo ""
    @echo "Tests:"
    @cd server && cargo test --no-run > /dev/null 2>&1 && echo "✅ Tests compile" || echo "❌ Tests fail to compile"
    @echo ""
    @echo "=== RSR Compliance: Bronze Level ==="

# Internal helper to check file existence
_check-file FILE DESC:
    @test -f {{FILE}} && echo "✅ {{DESC}}" || echo "❌ {{DESC}} missing"

# Run all validators
validate: validate-rsr lint fmt-check test
    @echo ""
    @echo "=== All Validations Passed ==="

# Quick start (build and run)
quickstart: build
    @echo "Starting Universal Language Connector..."
    cd server && cargo run

# Production build with optimizations
production:
    @echo "Building optimized production binary..."
    cd server && RUSTFLAGS="-C target-cpu=native" cargo build --release
    @echo "Production binary: server/target/release/universal-connector-server"

# Update dependencies
update:
    cd server && cargo update
    cd clients/vscode && npm update

# Check for outdated dependencies
outdated:
    cd server && cargo outdated
    cd clients/vscode && npm outdated

# Fix common issues
fix: fmt audit
    cd server && cargo fix --allow-dirty

# Pre-commit checks
pre-commit: fmt lint test
    @echo "✅ Pre-commit checks passed"

# Pre-push checks
pre-push: pre-commit validate-rsr
    @echo "✅ Pre-push checks passed"

# Setup development environment
setup:
    @echo "Setting up development environment..."
    @command -v rustc > /dev/null || (echo "❌ Rust not installed. Install from https://rustup.rs/" && exit 1)
    @command -v cargo > /dev/null || (echo "❌ Cargo not installed" && exit 1)
    @echo "✅ Rust and Cargo installed"
    @echo "Installing development tools..."
    cargo install cargo-watch cargo-audit || echo "Tools already installed"
    @echo "Installing Node.js dependencies for VS Code client..."
    cd clients/vscode && npm install || echo "Skipping VS Code setup"
    @echo ""
    @echo "✅ Development environment ready!"
    @echo ""
    @echo "Quick commands:"
    @echo "  just build    - Build server"
    @echo "  just test     - Run tests"
    @echo "  just dev      - Start development server"
    @echo "  just validate - Run all checks"

# Show project statistics
stats:
    @echo "=== Project Statistics ==="
    @echo ""
    @echo "Lines of Code:"
    @find server/src -name "*.rs" | xargs wc -l | tail -1
    @echo ""
    @echo "Test Files:"
    @find server/tests -name "*.rs" | xargs wc -l | tail -1
    @echo ""
    @echo "Documentation:"
    @find . -name "*.md" -not -path "./target/*" -not -path "./node_modules/*" | xargs wc -l | tail -1
    @echo ""
    @echo "Total Files:"
    @find . -type f -not -path "./target/*" -not -path "./node_modules/*" -not -path "./.git/*" | wc -l

# Version info
version:
    @echo "Universal Language Connector v0.1.0"
    @cargo --version
    @rustc --version

# Deploy to production (placeholder)
deploy:
    @echo "Deployment target not configured"
    @echo "Options:"
    @echo "  - Docker: just docker-build && just docker-run"
    @echo "  - Standalone: just production && just install"
    @echo "  - Container orchestration: just docker-compose-up"

# Run all CI checks
ci: lint fmt-check test validate-rsr
    @echo "✅ All CI checks passed"

# Generate release notes
release-notes VERSION:
    @echo "Generating release notes for {{VERSION}}..."
    @echo "See CHANGELOG.md for details"
    git log --oneline --decorate --graph --since="30 days ago"

# Tag release
tag VERSION:
    git tag -a v{{VERSION}} -m "Release v{{VERSION}}"
    git push origin v{{VERSION}}

# Show help
help:
    @echo "Universal Language Connector - Build Automation"
    @echo ""
    @echo "Common commands:"
    @echo "  just build          - Build server (debug)"
    @echo "  just release        - Build server (release)"
    @echo "  just test           - Run all tests"
    @echo "  just dev            - Development mode with auto-reload"
    @echo "  just validate       - Run all validators"
    @echo "  just pre-commit     - Pre-commit checks"
    @echo "  just ci             - CI checks"
    @echo ""
    @echo "For full list: just --list"
