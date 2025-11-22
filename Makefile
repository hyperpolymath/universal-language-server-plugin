# Universal Language Connector - Makefile
# Build system for server and clients

.PHONY: help build release test bench clean install docker-build docker-run clients docs all

# Default target
help:
	@echo "Universal Language Connector - Build System"
	@echo ""
	@echo "Targets:"
	@echo "  build         - Build server in debug mode"
	@echo "  release       - Build server in release mode"
	@echo "  test          - Run all tests"
	@echo "  bench         - Run benchmarks"
	@echo "  clean         - Clean build artifacts"
	@echo "  install       - Install server binary"
	@echo "  docker-build  - Build Docker image"
	@echo "  docker-run    - Run Docker container"
	@echo "  clients       - Build all editor clients"
	@echo "  docs          - Generate documentation"
	@echo "  lint          - Run linters"
	@echo "  fmt           - Format code"
	@echo "  check         - Check code without building"
	@echo "  all           - Build everything"

# Server targets
build:
	@echo "Building server (debug mode)..."
	cd server && cargo build

release:
	@echo "Building server (release mode)..."
	cd server && cargo build --release

test:
	@echo "Running tests..."
	cd server && cargo test

bench:
	@echo "Running benchmarks..."
	cd server && cargo bench

check:
	@echo "Checking code..."
	cd server && cargo check

lint:
	@echo "Running linters..."
	cd server && cargo clippy -- -D warnings

fmt:
	@echo "Formatting code..."
	cd server && cargo fmt

# Installation
install: release
	@echo "Installing server binary..."
	cd server && cargo install --path .

# Docker targets
docker-build:
	@echo "Building Docker image..."
	docker build -f deployment/Dockerfile -t universal-connector:latest .

docker-run:
	@echo "Running Docker container..."
	docker run -d -p 8080:8080 -p 8081:8081 --name universal-connector universal-connector:latest

docker-compose-up:
	@echo "Starting services with docker-compose..."
	cd deployment && docker-compose up -d

docker-compose-down:
	@echo "Stopping services..."
	cd deployment && docker-compose down

podman-compose-up:
	@echo "Starting services with podman-compose..."
	cd deployment && podman-compose up -d

podman-compose-down:
	@echo "Stopping services..."
	cd deployment && podman-compose down

# Client targets
clients: vscode-client neovim-client

vscode-client:
	@echo "Building VS Code client..."
	cd clients/vscode && npm install && npm run compile

neovim-client:
	@echo "Neovim client ready (Lua, no build needed)"

# Documentation
docs:
	@echo "Generating documentation..."
	cd server && cargo doc --no-deps --open

# Cleaning
clean:
	@echo "Cleaning build artifacts..."
	cd server && cargo clean
	rm -rf clients/vscode/out
	rm -rf clients/vscode/node_modules

# Development
dev:
	@echo "Starting development server with auto-reload..."
	cd server && cargo watch -x run

dev-web:
	@echo "Starting web UI development server..."
	@echo "Serving web UI at http://localhost:8000"
	cd web && python3 -m http.server 8000

# Testing and validation
test-integration:
	@echo "Running integration tests..."
	cd server && cargo test --test '*'

test-coverage:
	@echo "Generating test coverage..."
	cd server && cargo tarpaulin --out Html --output-dir coverage

# All-in-one targets
all: release clients docs

setup:
	@echo "Setting up development environment..."
	@echo "Installing Rust toolchain..."
	rustup update stable
	@echo "Installing cargo tools..."
	cargo install cargo-watch cargo-tarpaulin
	@echo "Installing Node.js dependencies for VS Code client..."
	cd clients/vscode && npm install
	@echo "Setup complete!"

# Deployment
deploy-local: release
	@echo "Deploying locally..."
	mkdir -p ~/.local/bin
	cp server/target/release/universal-connector-server ~/.local/bin/
	@echo "Deployed to ~/.local/bin/universal-connector-server"

# Version info
version:
	@echo "Universal Language Connector v0.1.0"
	@cd server && cargo --version
	@rustc --version

# Quick start
quickstart: build
	@echo "Starting Universal Connector..."
	cd server && cargo run

# Production build with optimizations
production:
	@echo "Building optimized production binary..."
	cd server && RUSTFLAGS="-C target-cpu=native" cargo build --release
	@echo "Production binary: server/target/release/universal-connector-server"

# Security audit
audit:
	@echo "Running security audit..."
	cd server && cargo audit

# Update dependencies
update:
	@echo "Updating dependencies..."
	cd server && cargo update
