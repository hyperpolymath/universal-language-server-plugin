//! LSP server implementation using tower-lsp
//!
//! Provides Language Server Protocol 3.17 compliant server for editor integration.

use crate::core::{ConversionCore, ConversionRequest, Format};
use crate::ServerState;
use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tracing::{error, info};

/// Universal Language Connector LSP backend
pub struct UniversalConnectorBackend {
    /// LSP client handle
    client: Client,
    /// Shared server state
    state: Arc<ServerState>,
}

impl UniversalConnectorBackend {
    /// Create a new LSP backend
    fn new(client: Client, state: Arc<ServerState>) -> Self {
        Self { client, state }
    }

    /// Convert URI to format
    fn uri_to_format(uri: &Url) -> Format {
        let path = uri.path();
        if path.ends_with(".md") || path.ends_with(".markdown") {
            Format::Markdown
        } else if path.ends_with(".html") || path.ends_with(".htm") {
            Format::Html
        } else if path.ends_with(".json") {
            Format::Json
        } else {
            Format::Markdown // Default
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for UniversalConnectorBackend {
    async fn initialize(&self, _params: InitializeParams) -> LspResult<InitializeResult> {
        info!("LSP client initializing...");

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec!["#".to_string(), "@".to_string(), "[".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec![
                        "convert.toMarkdown".to_string(),
                        "convert.toHtml".to_string(),
                        "convert.toJson".to_string(),
                    ],
                    ..Default::default()
                }),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: Some("universal-connector".to_string()),
                        inter_file_dependencies: false,
                        workspace_diagnostics: false,
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "Universal Language Connector".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        info!("LSP server initialized successfully");
        self.client
            .log_message(MessageType::INFO, "Universal Language Connector ready")
            .await;
    }

    async fn shutdown(&self) -> LspResult<()> {
        info!("LSP server shutting down");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let content = params.text_document.text;
        let language = params.text_document.language_id;

        info!("Document opened: {}", uri);

        self.state
            .documents
            .upsert(uri.clone(), content.clone(), language);

        // Send diagnostics
        self.send_diagnostics(&params.text_document.uri, &content)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        for change in params.content_changes {
            let text = change.text;
            let format = Self::uri_to_format(&params.text_document.uri);
            self.state.documents.upsert(
                uri.clone(),
                text.clone(),
                format.extension().to_string(),
            );

            // Send updated diagnostics
            self.send_diagnostics(&params.text_document.uri, &text)
                .await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        info!("Document saved: {}", params.text_document.uri);
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        info!("Document closed: {}", uri);
        // Note: We keep documents in store for potential HTTP/WS access
    }

    async fn completion(&self, params: CompletionParams) -> LspResult<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.to_string();

        // Provide format-specific completions
        let completions = vec![
            CompletionItem {
                label: "Convert to HTML".to_string(),
                kind: Some(CompletionItemKind::TEXT),
                detail: Some("Convert current document to HTML".to_string()),
                command: Some(Command {
                    title: "Convert to HTML".to_string(),
                    command: "convert.toHtml".to_string(),
                    arguments: Some(vec![serde_json::json!(uri)]),
                }),
                ..Default::default()
            },
            CompletionItem {
                label: "Convert to Markdown".to_string(),
                kind: Some(CompletionItemKind::TEXT),
                detail: Some("Convert current document to Markdown".to_string()),
                command: Some(Command {
                    title: "Convert to Markdown".to_string(),
                    command: "convert.toMarkdown".to_string(),
                    arguments: Some(vec![serde_json::json!(uri)]),
                }),
                ..Default::default()
            },
            CompletionItem {
                label: "Convert to JSON".to_string(),
                kind: Some(CompletionItemKind::TEXT),
                detail: Some("Convert current document to JSON".to_string()),
                command: Some(Command {
                    title: "Convert to JSON".to_string(),
                    command: "convert.toJson".to_string(),
                    arguments: Some(vec![serde_json::json!(uri)]),
                }),
                ..Default::default()
            },
        ];

        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn hover(&self, params: HoverParams) -> LspResult<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();

        if let Some(doc) = self.state.documents.get(&uri) {
            let stats = doc.stats();
            let content = format!(
                "**Document Statistics**\n\n\
                - Lines: {}\n\
                - Words: {}\n\
                - Characters: {}\n\
                - Version: {}\n\
                - Format: {}",
                stats.lines, stats.words, stats.characters, stats.version, doc.language
            );

            Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: content,
                }),
                range: None,
            }))
        } else {
            Ok(None)
        }
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> LspResult<Option<Value>> {
        info!("Executing command: {}", params.command);

        let uri = params
            .arguments
            .get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| tower_lsp::jsonrpc::Error::invalid_params("Missing URI argument"))?;

        let doc = self
            .state
            .documents
            .get(uri)
            .ok_or_else(|| tower_lsp::jsonrpc::Error::invalid_params("Document not found"))?;

        let from_format = Format::from_str(&doc.language).unwrap_or(Format::Markdown);

        let to_format = match params.command.as_str() {
            "convert.toMarkdown" => Format::Markdown,
            "convert.toHtml" => Format::Html,
            "convert.toJson" => Format::Json,
            _ => {
                return Err(tower_lsp::jsonrpc::Error::method_not_found());
            }
        };

        let request = ConversionRequest {
            content: doc.content.clone(),
            from: from_format,
            to: to_format,
        };

        match ConversionCore::convert(request) {
            Ok(response) => {
                // Show result to user
                self.client
                    .show_message(
                        MessageType::INFO,
                        format!("Converted {} → {}", from_format.extension(), to_format.extension()),
                    )
                    .await;

                Ok(Some(serde_json::json!({
                    "content": response.content,
                    "format": to_format,
                    "warnings": response.warnings,
                })))
            }
            Err(e) => {
                error!("Conversion failed: {}", e);
                self.client
                    .show_message(MessageType::ERROR, format!("Conversion failed: {}", e))
                    .await;
                Err(tower_lsp::jsonrpc::Error::internal_error())
            }
        }
    }

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> LspResult<DocumentDiagnosticReportResult> {
        let uri = params.text_document.uri.to_string();

        if let Some(doc) = self.state.documents.get(&uri) {
            let format = Format::from_str(&doc.language).unwrap_or(Format::Markdown);
            let diagnostics = match ConversionCore::validate(&doc.content, format) {
                Ok(issues) => issues
                    .into_iter()
                    .map(|message| Diagnostic {
                        range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                        severity: Some(DiagnosticSeverity::WARNING),
                        message,
                        source: Some("universal-connector".to_string()),
                        ..Default::default()
                    })
                    .collect(),
                Err(_) => vec![],
            };

            Ok(DocumentDiagnosticReportResult::Report(
                DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                    related_documents: None,
                    full_document_diagnostic_report: FullDocumentDiagnosticReport {
                        result_id: None,
                        items: diagnostics,
                    },
                }),
            ))
        } else {
            Ok(DocumentDiagnosticReportResult::Report(
                DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                    related_documents: None,
                    full_document_diagnostic_report: FullDocumentDiagnosticReport {
                        result_id: None,
                        items: vec![],
                    },
                }),
            ))
        }
    }
}

impl UniversalConnectorBackend {
    /// Send diagnostics for a document
    async fn send_diagnostics(&self, uri: &Url, content: &str) {
        let format = Self::uri_to_format(uri);
        if let Ok(issues) = ConversionCore::validate(content, format) {
            let diagnostics: Vec<Diagnostic> = issues
                .into_iter()
                .map(|message| Diagnostic {
                    range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                    severity: Some(DiagnosticSeverity::WARNING),
                    message,
                    source: Some("universal-connector".to_string()),
                    ..Default::default()
                })
                .collect();

            self.client
                .publish_diagnostics(uri.clone(), diagnostics, None)
                .await;
        }
    }
}

/// Run the LSP server on stdio
pub async fn run_lsp_server(state: Arc<ServerState>) -> Result<()> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| UniversalConnectorBackend::new(client, state))
        .finish();

    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
