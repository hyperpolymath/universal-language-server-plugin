//! Universal Language Connector Server
//!
//! LSP-based universal plugin architecture enabling one server to power plugins
//! across all major editors (VS Code, Neovim, Emacs, JetBrains, Sublime, Zed, Helix).
//!
//! # Architecture
//!
//! - LSP Server (stdio) - Main editor integration via Language Server Protocol
//! - HTTP API (port 8080) - REST endpoints for web integration
//! - WebSocket (port 8081) - Real-time document updates
//!
//! # Performance Targets
//!
//! - Response time: <100ms
//! - Memory usage: <50MB
//! - Startup time: <500ms

#![deny(clippy::all)]
#![warn(clippy::pedantic)]

mod core;
mod document_store;
mod http;
mod lsp;
mod websocket;

use anyhow::Result;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::document_store::DocumentStore;

/// Main server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// HTTP server bind address
    pub http_addr: String,
    /// WebSocket server bind address
    pub ws_addr: String,
    /// Enable LSP server (stdio)
    pub enable_lsp: bool,
    /// Enable HTTP server
    pub enable_http: bool,
    /// Enable WebSocket server
    pub enable_websocket: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            http_addr: "0.0.0.0:8080".to_string(),
            ws_addr: "0.0.0.0:8081".to_string(),
            enable_lsp: true,
            enable_http: true,
            enable_websocket: true,
        }
    }
}

/// Shared server state
pub struct ServerState {
    /// Document store (thread-safe, lock-free)
    pub documents: Arc<DocumentStore>,
    /// Server configuration
    pub config: ServerConfig,
}

impl ServerState {
    /// Create new server state
    fn new(config: ServerConfig) -> Self {
        Self {
            documents: Arc::new(DocumentStore::new()),
            config,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing/logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .compact()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("🚀 Universal Language Connector Server starting...");

    // Parse configuration from environment
    let config = ServerConfig {
        http_addr: std::env::var("HTTP_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
        ws_addr: std::env::var("WS_ADDR").unwrap_or_else(|_| "0.0.0.0:8081".to_string()),
        enable_lsp: std::env::var("ENABLE_LSP").unwrap_or_else(|_| "true".to_string()) == "true",
        enable_http: std::env::var("ENABLE_HTTP").unwrap_or_else(|_| "true".to_string()) == "true",
        enable_websocket: std::env::var("ENABLE_WS").unwrap_or_else(|_| "true".to_string()) == "true",
    };

    info!("📋 Configuration: {:?}", config);

    let state = Arc::new(ServerState::new(config.clone()));

    // Spawn all server components concurrently
    let mut tasks = JoinSet::new();

    // LSP server (stdio) - primary editor integration
    if config.enable_lsp {
        info!("📝 Starting LSP server (stdio)...");
        let lsp_state = Arc::clone(&state);
        tasks.spawn(async move {
            lsp::run_lsp_server(lsp_state).await
        });
    }

    // HTTP REST API server
    if config.enable_http {
        info!("🌐 Starting HTTP API server on {}...", config.http_addr);
        let http_state = Arc::clone(&state);
        let http_addr = config.http_addr.clone();
        tasks.spawn(async move {
            http::run_http_server(http_state, &http_addr).await
        });
    }

    // WebSocket server for real-time updates
    if config.enable_websocket {
        info!("🔌 Starting WebSocket server on {}...", config.ws_addr);
        let ws_state = Arc::clone(&state);
        let ws_addr = config.ws_addr.clone();
        tasks.spawn(async move {
            websocket::run_websocket_server(ws_state, &ws_addr).await
        });
    }

    info!("✅ All servers started successfully");
    info!("📡 Ready to accept connections");

    // Wait for any task to complete (error case) or Ctrl+C
    tokio::select! {
        result = tasks.join_next() => {
            match result {
                Some(Ok(Ok(()))) => {
                    info!("Server task completed normally");
                }
                Some(Ok(Err(e))) => {
                    eprintln!("❌ Server error: {}", e);
                    std::process::exit(1);
                }
                Some(Err(e)) => {
                    eprintln!("❌ Task join error: {}", e);
                    std::process::exit(1);
                }
                None => {
                    info!("All tasks completed");
                }
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("🛑 Received Ctrl+C, shutting down...");
        }
    }

    Ok(())
}
