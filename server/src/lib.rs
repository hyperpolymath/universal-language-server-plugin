//! Universal Language Connector Server Library
//!
//! This library provides the core functionality for the Universal Language Connector,
//! an LSP-based universal plugin architecture enabling one server to power plugins
//! across all major editors.

#![deny(clippy::all)]
#![warn(clippy::pedantic)]

#![forbid(unsafe_code)]
pub mod auth;
pub mod core;
pub mod document_store;
pub mod formats;
pub mod http;
pub mod lsp;
pub mod monitoring;
pub mod websocket;

use std::sync::Arc;

pub use crate::auth::{AuthConfig, AuthService};
pub use crate::document_store::DocumentStore;
pub use crate::monitoring::{HealthChecker, Metrics};

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
    /// JWT secret for authentication (Platinum RSR)
    pub jwt_secret: String,
    /// Enable authentication (Platinum RSR)
    pub enable_auth: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            http_addr: "0.0.0.0:8080".to_string(),
            ws_addr: "0.0.0.0:8081".to_string(),
            enable_lsp: true,
            enable_http: true,
            enable_websocket: true,
            jwt_secret: "dev-secret-change-in-production".to_string(),
            enable_auth: false, // Disabled by default for development
        }
    }
}

/// Shared server state
pub struct ServerState {
    /// Document store (thread-safe, lock-free)
    pub documents: Arc<DocumentStore>,
    /// Server configuration
    pub config: ServerConfig,
    /// Metrics collector (Platinum RSR)
    pub metrics: Arc<Metrics>,
    /// Health checker (Platinum RSR)
    pub health_checker: Arc<HealthChecker>,
    /// Authentication service (Platinum RSR)
    pub auth_service: Option<Arc<AuthService>>,
}

impl ServerState {
    /// Create new server state
    pub fn new(config: ServerConfig) -> Self {
        // Create auth service if enabled
        let auth_service = if config.enable_auth {
            let auth_config = AuthConfig {
                secret: config.jwt_secret.clone(),
                expiration_secs: 86400, // 24 hours
                required_scopes: std::collections::HashMap::new(),
                enabled: true,
            };
            Some(Arc::new(AuthService::new(auth_config)))
        } else {
            None
        };

        Self {
            documents: Arc::new(DocumentStore::new()),
            metrics: Arc::new(Metrics::new()),
            health_checker: Arc::new(HealthChecker::new()),
            auth_service,
            config,
        }
    }
}
