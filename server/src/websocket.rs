//! WebSocket server for real-time document updates
//!
//! Provides bidirectional communication for live collaboration and updates.

use crate::ServerState;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::{error, info, warn};

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum WsMessage {
    /// Subscribe to document updates
    Subscribe { document_id: String },
    /// Unsubscribe from document updates
    Unsubscribe { document_id: String },
    /// Document updated notification
    DocumentUpdated {
        document_id: String,
        content: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// Error message
    Error { message: String },
    /// Ping/pong for keepalive
    Ping,
    Pong,
}

/// Handle a single WebSocket connection
async fn handle_connection(
    stream: TcpStream,
    state: Arc<ServerState>,
    tx: broadcast::Sender<WsMessage>,
) -> Result<()> {
    let addr = stream.peer_addr()?;
    info!("New WebSocket connection from: {}", addr);

    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // Subscribe to broadcast channel
    let mut rx = tx.subscribe();

    // Spawn task to forward broadcast messages to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if ws_sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Handle incoming messages from this client
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<WsMessage>(&text) {
                        Ok(ws_msg) => {
                            info!("Received WebSocket message: {:?}", ws_msg);

                            match ws_msg {
                                WsMessage::Subscribe { document_id } => {
                                    info!("Client subscribed to document: {}", document_id);
                                    // In a full implementation, track subscriptions per client
                                    // For now, all clients receive all updates
                                }
                                WsMessage::Unsubscribe { document_id } => {
                                    info!("Client unsubscribed from document: {}", document_id);
                                }
                                WsMessage::Ping => {
                                    // Broadcast pong response
                                    let _ = tx.send(WsMessage::Pong);
                                }
                                _ => {
                                    warn!("Unexpected message type from client");
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse WebSocket message: {}", e);
                            let error_msg = WsMessage::Error {
                                message: format!("Invalid message format: {}", e),
                            };
                            let _ = tx.send(error_msg);
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("Client {} disconnected", addr);
                    break;
                }
                Ok(Message::Ping(data)) => {
                    // Handled automatically by tokio-tungstenite
                    info!("Received ping from {}", addr);
                }
                Ok(Message::Pong(_)) => {
                    // Keepalive response
                }
                Err(e) => {
                    error!("WebSocket error from {}: {}", addr, e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }

    info!("WebSocket connection closed: {}", addr);
    Ok(())
}

/// Run WebSocket server
pub async fn run_websocket_server(state: Arc<ServerState>, addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("WebSocket server listening on {}", addr);

    // Create broadcast channel for updates
    let (tx, _rx) = broadcast::channel::<WsMessage>(100);

    // Spawn periodic update broadcast (example)
    let broadcast_tx = tx.clone();
    let broadcast_state = Arc::clone(&state);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
        loop {
            interval.tick().await;

            // Broadcast document updates periodically
            for doc in broadcast_state.documents.list() {
                let update = WsMessage::DocumentUpdated {
                    document_id: doc.id.clone(),
                    content: doc.content.clone(),
                    timestamp: chrono::Utc::now(),
                };

                // Ignore send errors (no subscribers)
                let _ = broadcast_tx.send(update);
            }
        }
    });

    // Accept connections
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let state_clone = Arc::clone(&state);
                let tx_clone = tx.clone();

                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, state_clone, tx_clone).await {
                        error!("WebSocket connection error: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Failed to accept WebSocket connection: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ServerConfig;

    #[test]
    fn test_ws_message_serialization() {
        let msg = WsMessage::Subscribe {
            document_id: "doc-123".to_string(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: WsMessage = serde_json::from_str(&json).unwrap();

        match deserialized {
            WsMessage::Subscribe { document_id } => {
                assert_eq!(document_id, "doc-123");
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_document_updated_message() {
        let msg = WsMessage::DocumentUpdated {
            document_id: "doc-456".to_string(),
            content: "Updated content".to_string(),
            timestamp: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("DocumentUpdated"));
        assert!(json.contains("doc-456"));
    }
}
