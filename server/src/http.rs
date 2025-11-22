//! HTTP REST API server using axum
//!
//! Provides HTTP endpoints for web integration and non-LSP clients.

use crate::core::{ConversionCore, ConversionRequest, Format};
use crate::document_store::Document;
use crate::ServerState;
use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

/// HTTP API error response
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

/// API error types
#[derive(Debug)]
enum ApiError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

/// Convert document request
#[derive(Debug, Deserialize)]
struct ConvertRequest {
    content: String,
    from: String,
    to: String,
}

/// Document list response
#[derive(Debug, Serialize)]
struct DocumentListResponse {
    documents: Vec<Document>,
    count: usize,
}

/// Server statistics
#[derive(Debug, Serialize)]
struct ServerStats {
    document_count: usize,
    uptime_seconds: u64,
    version: String,
}

/// Health check response
#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

/// Convert document handler
async fn convert_document(
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<ConvertRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    info!("Converting document: {} → {}", payload.from, payload.to);

    let from_format = Format::from_str(&payload.from)
        .map_err(|e| ApiError::BadRequest(format!("Invalid 'from' format: {}", e)))?;

    let to_format = Format::from_str(&payload.to)
        .map_err(|e| ApiError::BadRequest(format!("Invalid 'to' format: {}", e)))?;

    let request = ConversionRequest {
        content: payload.content,
        from: from_format,
        to: to_format,
    };

    match ConversionCore::convert(request) {
        Ok(response) => Ok(Json(serde_json::json!({
            "content": response.content,
            "from": response.from,
            "to": response.to,
            "warnings": response.warnings,
        }))),
        Err(e) => {
            error!("Conversion failed: {}", e);
            Err(ApiError::Internal(format!("Conversion failed: {}", e)))
        }
    }
}

/// List all documents handler
async fn list_documents(
    State(state): State<Arc<ServerState>>,
) -> Json<DocumentListResponse> {
    let documents = state.documents.list();
    let count = documents.len();

    Json(DocumentListResponse { documents, count })
}

/// Get document by ID handler
async fn get_document(
    State(state): State<Arc<ServerState>>,
    Path(id): Path<String>,
) -> Result<Json<Document>, ApiError> {
    state
        .documents
        .get_by_id(&id)
        .map(Json)
        .ok_or_else(|| ApiError::NotFound(format!("Document not found: {}", id)))
}

/// Delete document handler
async fn delete_document(
    State(state): State<Arc<ServerState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Find document by ID
    let doc = state
        .documents
        .get_by_id(&id)
        .ok_or_else(|| ApiError::NotFound(format!("Document not found: {}", id)))?;

    // Remove by URI
    state.documents.remove(&doc.uri);

    Ok(StatusCode::NO_CONTENT)
}

/// Validate document handler
async fn validate_document(
    State(_state): State<Arc<ServerState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ApiError::BadRequest("Missing 'content' field".to_string()))?;

    let format_str = payload
        .get("format")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ApiError::BadRequest("Missing 'format' field".to_string()))?;

    let format = Format::from_str(format_str)
        .map_err(|e| ApiError::BadRequest(format!("Invalid format: {}", e)))?;

    match ConversionCore::validate(content, format) {
        Ok(diagnostics) => Ok(Json(serde_json::json!({
            "valid": diagnostics.is_empty(),
            "diagnostics": diagnostics,
        }))),
        Err(e) => Err(ApiError::Internal(format!("Validation failed: {}", e))),
    }
}

/// Server statistics handler
async fn get_stats(State(state): State<Arc<ServerState>>) -> Json<ServerStats> {
    Json(ServerStats {
        document_count: state.documents.count(),
        uptime_seconds: 0, // TODO: Track actual uptime
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Health check handler
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Create HTTP router
fn create_router(state: Arc<ServerState>) -> Router {
    Router::new()
        .route("/api/convert", post(convert_document))
        .route("/api/documents", get(list_documents))
        .route("/api/documents/:id", get(get_document))
        .route("/api/documents/:id", delete(delete_document))
        .route("/api/validate", post(validate_document))
        .route("/api/stats", get(get_stats))
        .route("/api/health", get(health_check))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state)
}

/// Run HTTP server
pub async fn run_http_server(state: Arc<ServerState>, addr: &str) -> Result<()> {
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("HTTP server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ServerConfig;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    fn create_test_state() -> Arc<ServerState> {
        Arc::new(ServerState::new(ServerConfig::default()))
    }

    #[tokio::test]
    async fn test_health_check() {
        let state = create_test_state();
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_convert_document() {
        let state = create_test_state();
        let app = create_router(state);

        let payload = serde_json::json!({
            "content": "# Hello World",
            "from": "markdown",
            "to": "html"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/convert")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_documents() {
        let state = create_test_state();

        // Add a test document
        state.documents.upsert(
            "file:///test.md".to_string(),
            "# Test".to_string(),
            "markdown".to_string(),
        );

        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/documents")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
