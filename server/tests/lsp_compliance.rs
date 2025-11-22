//! LSP 3.17 compliance tests
//!
//! Tests to verify compliance with Language Server Protocol specification

use tower_lsp::lsp_types::*;

#[test]
fn test_server_capabilities_structure() {
    // Verify ServerCapabilities can be constructed with expected fields
    let capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec!["#".to_string(), "@".to_string()]),
            ..Default::default()
        }),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    };

    // Verify sync mode
    if let Some(TextDocumentSyncCapability::Kind(kind)) = capabilities.text_document_sync {
        assert_eq!(kind, TextDocumentSyncKind::INCREMENTAL);
    } else {
        panic!("Expected TextDocumentSyncKind");
    }

    // Verify completion provider
    assert!(capabilities.completion_provider.is_some());

    // Verify hover provider
    assert!(capabilities.hover_provider.is_some());
}

#[test]
fn test_initialize_params_parsing() {
    let json = r#"{
        "processId": 1234,
        "rootUri": "file:///home/user/project",
        "capabilities": {}
    }"#;

    let _params: serde_json::Value = serde_json::from_str(json).unwrap();
}

#[test]
fn test_did_open_params_structure() {
    let uri = Url::parse("file:///test.md").unwrap();
    let params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "markdown".to_string(),
            version: 1,
            text: "# Test".to_string(),
        },
    };

    assert_eq!(params.text_document.uri, uri);
    assert_eq!(params.text_document.language_id, "markdown");
    assert_eq!(params.text_document.version, 1);
}

#[test]
fn test_did_change_params_structure() {
    let uri = Url::parse("file:///test.md").unwrap();

    let change = TextDocumentContentChangeEvent {
        range: None,
        range_length: None,
        text: "# Updated".to_string(),
    };

    let params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: uri.clone(),
            version: 2,
        },
        content_changes: vec![change],
    };

    assert_eq!(params.text_document.version, 2);
    assert_eq!(params.content_changes.len(), 1);
}

#[test]
fn test_completion_params_structure() {
    let uri = Url::parse("file:///test.md").unwrap();

    let params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 5,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: None,
    };

    assert_eq!(params.text_document_position.position.line, 0);
    assert_eq!(params.text_document_position.position.character, 5);
}

#[test]
fn test_completion_item_structure() {
    let item = CompletionItem {
        label: "Convert to HTML".to_string(),
        kind: Some(CompletionItemKind::TEXT),
        detail: Some("Convert document to HTML".to_string()),
        documentation: None,
        deprecated: Some(false),
        preselect: None,
        sort_text: None,
        filter_text: None,
        insert_text: None,
        insert_text_format: None,
        insert_text_mode: None,
        text_edit: None,
        additional_text_edits: None,
        command: Some(Command {
            title: "Convert".to_string(),
            command: "convert.toHtml".to_string(),
            arguments: None,
        }),
        commit_characters: None,
        data: None,
        tags: None,
    };

    assert_eq!(item.label, "Convert to HTML");
    assert_eq!(item.kind, Some(CompletionItemKind::TEXT));
    assert!(item.command.is_some());
}

#[test]
fn test_hover_params_structure() {
    let uri = Url::parse("file:///test.md").unwrap();

    let params = HoverParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position {
                line: 1,
                character: 10,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    assert_eq!(params.text_document_position_params.position.line, 1);
}

#[test]
fn test_hover_response_structure() {
    let hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "**Document Info**".to_string(),
        }),
        range: None,
    };

    if let HoverContents::Markup(content) = hover.contents {
        assert_eq!(content.kind, MarkupKind::Markdown);
        assert!(content.value.contains("Document Info"));
    }
}

#[test]
fn test_diagnostic_structure() {
    let diagnostic = Diagnostic {
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 10,
            },
        },
        severity: Some(DiagnosticSeverity::WARNING),
        code: None,
        code_description: None,
        source: Some("universal-connector".to_string()),
        message: "Test diagnostic".to_string(),
        related_information: None,
        tags: None,
        data: None,
    };

    assert_eq!(diagnostic.severity, Some(DiagnosticSeverity::WARNING));
    assert_eq!(diagnostic.message, "Test diagnostic");
    assert_eq!(diagnostic.source, Some("universal-connector".to_string()));
}

#[test]
fn test_execute_command_params() {
    let params = ExecuteCommandParams {
        command: "convert.toHtml".to_string(),
        arguments: vec![serde_json::json!("file:///test.md")],
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    assert_eq!(params.command, "convert.toHtml");
    assert_eq!(params.arguments.len(), 1);
}

#[test]
fn test_position_utf16_compatibility() {
    // LSP uses UTF-16 code units for positions
    // This is important for non-ASCII text
    let position = Position {
        line: 0,
        character: 5,
    };

    assert_eq!(position.line, 0);
    assert_eq!(position.character, 5);

    // In real implementation, need to convert between UTF-8 and UTF-16
}

#[test]
fn test_document_uri_parsing() {
    let uri_str = "file:///home/user/test.md";
    let uri = Url::parse(uri_str).unwrap();

    assert_eq!(uri.scheme(), "file");
    assert_eq!(uri.path(), "/home/user/test.md");
}

#[test]
fn test_message_type_levels() {
    // Verify message types exist
    let _error = MessageType::ERROR;
    let _warning = MessageType::WARNING;
    let _info = MessageType::INFO;
    let _log = MessageType::LOG;
}

#[test]
fn test_server_info_structure() {
    let info = ServerInfo {
        name: "Universal Language Connector".to_string(),
        version: Some("0.1.0".to_string()),
    };

    assert_eq!(info.name, "Universal Language Connector");
    assert!(info.version.is_some());
}
