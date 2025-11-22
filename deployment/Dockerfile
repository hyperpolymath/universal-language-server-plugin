# Universal Language Connector - Dockerfile
# Multi-stage build for optimal image size

# Build stage
FROM rust:1.75-slim as builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY server/Cargo.toml server/Cargo.lock ./

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY server/src ./src

# Build for release
RUN cargo build --release --bin universal-connector-server

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/target/release/universal-connector-server /usr/local/bin/

# Copy web UI (optional, for serving static files)
COPY web /app/web

# Create non-root user
RUN useradd -m -u 1000 connector && \
    chown -R connector:connector /app

USER connector

# Expose ports
EXPOSE 8080 8081

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["sh", "-c", "curl -f http://localhost:8080/api/health || exit 1"]

# Run server
CMD ["universal-connector-server"]
