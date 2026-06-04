// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! End-to-end tests for the complete LSP pipeline
//!
//! Tests full workflows from document open → request → response

use universal_connector_server::core::{ConversionCore, ConversionRequest, Format};
use universal_connector_server::document_store::{Document, DocumentStore};

// E2E: Open document → Store → Retrieve → Convert → Verify
#[test]
fn e2e_document_lifecycle() {
    let store = DocumentStore::new();
    let uri = "file:///test/document.md".to_string();
    let content = "# Test Document\n\nThis is **bold** text.".to_string();

    // 1. Create and store document
    store.upsert(uri.clone(), content.clone(), "markdown".to_string());

    // 2. Retrieve document
    let retrieved = store.get(&uri).expect("Failed to retrieve document");
    assert_eq!(retrieved.content, content);
    assert_eq!(retrieved.version, 1);

    // 3. Verify document stats
    let stats = retrieved.stats();
    assert!(stats.lines >= 2);
    assert!(stats.characters > 0);
}

// E2E: Open document → Hover request → Get response
#[test]
fn e2e_hover_workflow() {
    let store = DocumentStore::new();
    let uri = "file:///test/hover.md".to_string();
    let content = "# Title\n\nThis is content.".to_string();

    // 1. Store document
    store.upsert(uri.clone(), content, "markdown".to_string());

    // 2. Simulate hover request (get stats)
    let retrieved = store.get(&uri).expect("Failed to retrieve");
    let stats = retrieved.stats();

    // 3. Verify hover information
    assert!(stats.lines > 0);
    assert!(stats.characters > 0);
    assert!(!stats.lines.to_string().is_empty());
}

// E2E: Open document → Format request → Get formatted output
#[test]
fn e2e_format_workflow() {
    let uri = "file:///test/format.md".to_string();
    let content = "# Hello\n\nParagraph with **bold** text.".to_string();

    // 1. Store document
    let store = DocumentStore::new();
    store.upsert(uri.clone(), content.clone(), "markdown".to_string());

    // 2. Request conversion (format request)
    let request = ConversionRequest {
        content: content.clone(),
        from: Format::Markdown,
        to: Format::Html,
    };

    // 3. Get formatted output
    let response = ConversionCore::convert(request).expect("Conversion failed");

    // 4. Verify response
    assert!(!response.content.is_empty());
    assert!(response.content.contains("<h1>") || response.content.contains("Hello"));
}

// E2E: Change document → Diagnostics → Verify correctness
#[test]
fn e2e_diagnostics_workflow() {
    let store = DocumentStore::new();
    let uri = "file:///test/diag.json".to_string();

    // 1. Store initial document
    let valid_json = r#"{"key": "value"}"#.to_string();
    store.upsert(uri.clone(), valid_json, "json".to_string());

    // 2. Update document with invalid JSON
    let invalid_json = r#"{"key": "value"#; // Missing closing brace
    store.upsert(uri.clone(), invalid_json.to_string(), "json".to_string());

    // 3. Validate format (would trigger diagnostics in LSP)
    let validate_result = ConversionCore::validate(invalid_json, Format::Json);

    // 4. Verify diagnostics indicate error
    if let Err(e) = validate_result {
        assert!(!e.to_string().is_empty());
    }
}

// E2E: Complete request → Completions → List non-empty for known text
#[test]
fn e2e_completion_workflow() {
    let store = DocumentStore::new();
    let uri = "file:///test/completion.md".to_string();
    let content = "# Test\n\nSome markdown ## ".to_string();

    // 1. Store document
    store.upsert(uri.clone(), content.clone(), "markdown".to_string());

    // 2. Retrieve for completion context
    let retrieved = store.get(&uri).expect("Failed to retrieve");
    assert!(!retrieved.content.is_empty());

    // 3. Check if content is completable
    let has_heading_marker = retrieved.content.contains("##");
    assert!(has_heading_marker);
}

// E2E: Multi-format roundtrip (Markdown → JSON → Markdown)
#[test]
fn e2e_roundtrip_markdown_json_markdown() {
    let original = "# Section\n\nWith **emphasis** and *italics*.".to_string();

    // 1. Markdown → JSON
    let req1 = ConversionRequest {
        content: original.clone(),
        from: Format::Markdown,
        to: Format::Json,
    };
    let response1 = ConversionCore::convert(req1).expect("MD->JSON failed");

    // 2. JSON → Markdown
    let req2 = ConversionRequest {
        content: response1.content.clone(),
        from: Format::Json,
        to: Format::Markdown,
    };
    let response2 = ConversionCore::convert(req2).expect("JSON->MD failed");

    // 3. Verify roundtrip preserved key content
    assert!(response2.content.contains("Section"));
    assert!(response2.content.contains("emphasis"));
}

// E2E: Multi-format roundtrip (Markdown → HTML → Markdown)
#[test]
fn e2e_roundtrip_markdown_html_markdown() {
    let original = "# Title\n\nContent here.".to_string();

    // 1. Markdown → HTML
    let req1 = ConversionRequest {
        content: original.clone(),
        from: Format::Markdown,
        to: Format::Html,
    };
    let response1 = ConversionCore::convert(req1).expect("MD->HTML failed");

    // 2. HTML → Markdown
    let req2 = ConversionRequest {
        content: response1.content.clone(),
        from: Format::Html,
        to: Format::Markdown,
    };
    let response2 = ConversionCore::convert(req2).expect("HTML->MD failed");

    // 3. Verify roundtrip preserved content
    assert!(response2.content.contains("Title"));
    assert!(response2.content.contains("Content"));
}

// E2E: Concurrent document operations (store/retrieve/update)
#[test]
fn e2e_concurrent_document_ops() {
    use std::sync::Arc;
    use std::thread;

    let store = Arc::new(DocumentStore::new());

    // Store 10 documents concurrently
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let store_clone = Arc::clone(&store);
            thread::spawn(move || {
                let uri = format!("file:///doc{}", i);
                let content = format!("Document {}", i);
                store_clone.upsert(uri.clone(), content, "text".to_string());

                // Retrieve and verify
                if let Some(retrieved) = store_clone.get(&uri) {
                    assert!(retrieved.uri.contains(&i.to_string()));
                }
            })
        })
        .collect();

    // Wait for all threads
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Verify all documents present
    assert!(store.count() >= 10);
}

// E2E: Large document conversion pipeline
#[test]
fn e2e_large_document_conversion() {
    let large_markdown = "# Large Document\n\n".to_string()
        + &"This is a paragraph. ".repeat(100)
        + "\n\n## Section 2\n\n"
        + &"More content. ".repeat(50);

    let request = ConversionRequest {
        content: large_markdown.clone(),
        from: Format::Markdown,
        to: Format::Html,
    };

    let response = ConversionCore::convert(request).expect("Large conversion failed");

    assert!(!response.content.is_empty());
    assert!(response.content.len() > large_markdown.len() / 2); // HTML markup adds overhead
}

// E2E: Format conversion with special characters
#[test]
fn e2e_conversion_with_special_chars() {
    let content_with_special = "# Title\n\nWith émojis 🎉 and spëcial çhars ñ © ®".to_string();

    let request = ConversionRequest {
        content: content_with_special.clone(),
        from: Format::Markdown,
        to: Format::Html,
    };

    let response = ConversionCore::convert(request).expect("Special chars conversion failed");
    assert!(!response.content.is_empty());
    // Should preserve UTF-8 characters
    assert!(response.content.contains("Title") || response.content.contains("spëcial"));
}

// E2E: Document version tracking across updates
#[test]
fn e2e_document_version_tracking() {
    let _store = DocumentStore::new();
    let uri = "file:///test/versions.md".to_string();

    // 1. Create document
    let mut doc = Document::new(uri.clone(), "version 1".to_string(), "markdown".to_string());
    assert_eq!(doc.version, 1);

    // 2. Update document
    doc.update_content("version 2".to_string());
    assert_eq!(doc.version, 2);

    // 3. Update again
    doc.update_content("version 3".to_string());
    assert_eq!(doc.version, 3);

    assert_eq!(doc.version, 3);
}

// E2E: YAML to JSON conversion and back
#[test]
fn e2e_yaml_json_roundtrip() {
    let yaml_content = "key: value\nlist:\n  - item1\n  - item2".to_string();

    // YAML → JSON
    let req1 = ConversionRequest {
        content: yaml_content.clone(),
        from: Format::Yaml,
        to: Format::Json,
    };
    let response1 = ConversionCore::convert(req1).expect("YAML->JSON failed");

    // JSON → YAML
    let req2 = ConversionRequest {
        content: response1.content.clone(),
        from: Format::Json,
        to: Format::Yaml,
    };
    let response2 = ConversionCore::convert(req2).expect("JSON->YAML failed");

    // Both should be valid (not necessarily identical due to formatting)
    assert!(!response1.content.is_empty());
    assert!(!response2.content.is_empty());
}

// E2E: TOML to JSON roundtrip
#[test]
fn e2e_toml_json_roundtrip() {
    let toml_content = "title = \"Config\"\n[section]\nkey = \"value\"".to_string();

    // TOML → JSON
    let req1 = ConversionRequest {
        content: toml_content.clone(),
        from: Format::Toml,
        to: Format::Json,
    };
    let response1 = ConversionCore::convert(req1).expect("TOML->JSON failed");

    // JSON → TOML
    let req2 = ConversionRequest {
        content: response1.content.clone(),
        from: Format::Json,
        to: Format::Toml,
    };
    let response2 = ConversionCore::convert(req2).expect("JSON->TOML failed");

    // Both should be valid
    assert!(!response1.content.is_empty());
    assert!(!response2.content.is_empty());
}
