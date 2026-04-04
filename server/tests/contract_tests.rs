// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Contract and reflexive tests
//!
//! Tests verify pre/post-conditions and type system contracts

use universal_connector_server::core::{ConversionCore, ConversionRequest, Format};
use universal_connector_server::document_store::{Document, DocumentStore};

// Contract: Format::from_str roundtrip
#[test]
fn contract_format_string_roundtrip() {
    for format in &[
        Format::Markdown,
        Format::Html,
        Format::Json,
        Format::Yaml,
        Format::Xml,
        Format::Toml,
    ] {
        let ext = format.extension();
        let parsed = Format::from_str(ext).expect(&format!("Failed to parse {}", ext));
        assert_eq!(parsed, *format);
    }
}

// Contract: ConversionRequest invariants
#[test]
fn contract_conversion_request_invariants() {
    let request = ConversionRequest {
        content: "test".to_string(),
        from: Format::Markdown,
        to: Format::Html,
    };

    // Invariant: from and to are both valid formats
    assert_ne!(request.from, request.to);
    assert!(!request.content.is_empty());
}

// Contract: ConversionResponse format consistency
#[test]
fn contract_conversion_response_consistency() {
    let request = ConversionRequest {
        content: "# Test".to_string(),
        from: Format::Markdown,
        to: Format::Html,
    };

    let response = ConversionCore::convert(request).expect("Conversion failed");

    // Invariant: response matches request formats
    assert_eq!(response.from, Format::Markdown);
    assert_eq!(response.to, Format::Html);
    assert!(!response.content.is_empty());
}

// Contract: Document version is always positive
#[test]
fn contract_document_version_always_positive() {
    let doc = Document::new("file:///test".to_string(), "content".to_string(), "text".to_string());
    assert!(doc.version > 0);

    let stats = doc.stats();
    assert!(stats.version > 0);
}

// Contract: Document timestamps are ordered
#[test]
fn contract_document_timestamps_ordered() {
    let mut doc = Document::new("file:///test".to_string(), "v1".to_string(), "text".to_string());
    let created = doc.created_at;

    // Sleep minimal amount to ensure time difference
    std::thread::sleep(std::time::Duration::from_millis(1));

    doc.update_content("v2".to_string());
    let modified = doc.modified_at;

    assert!(modified >= created, "Modified time should be >= created time");
}

// Contract: Document stats are valid
#[test]
fn contract_document_stats_valid() {
    let content = "Line 1\nLine 2\nLine 3".to_string();
    let doc = Document::new("file:///test".to_string(), content, "text".to_string());

    let stats = doc.stats();
    assert!(stats.lines > 0);
    assert!(stats.characters > 0);
    assert!(stats.words > 0);
}

// Contract: DocumentStore insert-retrieve consistency
#[test]
fn contract_document_store_consistency() {
    let store = DocumentStore::new();
    let uri = "file:///test".to_string();
    let content = "test content".to_string();

    store.upsert(uri.clone(), content.clone(), "text".to_string());
    let retrieved = store.get(&uri).expect("Get failed");

    assert_eq!(retrieved.content, content);
    assert_eq!(retrieved.uri, uri);
}

// Contract: DocumentStore length is accurate
#[test]
fn contract_document_store_length() {
    let store = DocumentStore::new();

    for i in 0..10 {
        let uri = format!("file:///doc{}", i);
        store.upsert(uri, format!("content{}", i), "text".to_string());
    }

    assert_eq!(store.count(), 10);
}

// Contract: Format::from_str accepts various input formats
#[test]
fn contract_format_parsing_variations() {
    // Lowercase should work
    assert!(Format::from_str("markdown").is_ok());
    assert!(Format::from_str("json").is_ok());

    // Uppercase should work
    assert!(Format::from_str("MARKDOWN").is_ok());
    assert!(Format::from_str("JSON").is_ok());

    // Mixed case should work
    assert!(Format::from_str("Markdown").is_ok());
}

// Contract: Conversion idempotence for same format
#[test]
fn contract_same_format_idempotent() {
    let content = "# Test Content".to_string();

    let req1 = ConversionRequest {
        content: content.clone(),
        from: Format::Markdown,
        to: Format::Markdown,
    };

    let resp1 = ConversionCore::convert(req1).expect("First conversion failed");

    let req2 = ConversionRequest {
        content: resp1.content.clone(),
        from: Format::Markdown,
        to: Format::Markdown,
    };

    let resp2 = ConversionCore::convert(req2).expect("Second conversion failed");

    assert_eq!(resp1.content, resp2.content);
}

// Contract: Document update increments version consistently
#[test]
fn contract_document_update_versioning() {
    let mut doc = Document::new("file:///test".to_string(), "v1".to_string(), "text".to_string());
    let v1 = doc.version;

    doc.update_content("v2".to_string());
    let v2 = doc.version;

    doc.update_content("v3".to_string());
    let v3 = doc.version;

    assert!(v2 > v1);
    assert!(v3 > v2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(v3, 3);
}

// Contract: DocumentStore.contains matches get
#[test]
fn contract_document_store_contains_matches_get() {
    let store = DocumentStore::new();
    let uri = "file:///test".to_string();

    store.upsert(uri.clone(), "content".to_string(), "text".to_string());

    // If contains returns true, get should succeed
    if store.contains(&uri) {
        assert!(store.get(&uri).is_some());
    }
}

// Contract: Format extensions are unique
#[test]
fn contract_format_extensions_unique() {
    let formats = vec![
        Format::Markdown,
        Format::Html,
        Format::Json,
        Format::Yaml,
        Format::Xml,
        Format::Toml,
    ];

    let extensions: Vec<_> = formats.iter().map(|f| f.extension()).collect();

    // Check uniqueness: no duplicates
    for i in 0..extensions.len() {
        for j in (i + 1)..extensions.len() {
            assert_ne!(extensions[i], extensions[j]);
        }
    }
}

// Contract: Reflexive test for Format Clone
#[test]
fn contract_format_clone_identity() {
    let format = Format::Markdown;
    let cloned = format;

    assert_eq!(format, cloned);
}

// Contract: Reflexive test for Document Clone
#[test]
fn contract_document_clone_identity() {
    let doc = Document::new("file:///test".to_string(), "content".to_string(), "text".to_string());
    let cloned = doc.clone();

    assert_eq!(doc.id, cloned.id);
    assert_eq!(doc.content, cloned.content);
    assert_eq!(doc.uri, cloned.uri);
    assert_eq!(doc.version, cloned.version);
}

// Contract: ConversionRequest serialization roundtrip
#[test]
fn contract_conversion_request_serialization() {
    let request = ConversionRequest {
        content: "test".to_string(),
        from: Format::Markdown,
        to: Format::Html,
    };

    let json = serde_json::to_string(&request).expect("Serialization failed");
    let deserialized: ConversionRequest = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(request.content, deserialized.content);
    assert_eq!(request.from, deserialized.from);
    assert_eq!(request.to, deserialized.to);
}

// Contract: Document serialization roundtrip
#[test]
fn contract_document_serialization() {
    let doc = Document::new("file:///test".to_string(), "content".to_string(), "text".to_string());

    let json = serde_json::to_string(&doc).expect("Serialization failed");
    let deserialized: Document = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(doc.id, deserialized.id);
    assert_eq!(doc.content, deserialized.content);
    assert_eq!(doc.uri, deserialized.uri);
    assert_eq!(doc.version, deserialized.version);
}

// Contract: Document stats reflect content accurately
#[test]
fn contract_document_stats_accuracy() {
    let content = "Line 1\nLine 2\nLine 3 with more words".to_string();
    let doc = Document::new("file:///test".to_string(), content.clone(), "text".to_string());

    let stats = doc.stats();

    // Count manually
    let manual_lines = content.lines().count();
    let manual_chars = content.len();
    let manual_words = content.split_whitespace().count();

    assert_eq!(stats.lines, manual_lines);
    assert_eq!(stats.characters, manual_chars);
    assert_eq!(stats.words, manual_words);
}

// Contract: Validation returns consistent diagnostics
#[test]
fn contract_validation_consistency() {
    let content = "# Valid Markdown";

    let diag1 = ConversionCore::validate(content, Format::Markdown);
    let diag2 = ConversionCore::validate(content, Format::Markdown);

    // Same input should yield same validation result
    match (diag1, diag2) {
        (Ok(d1), Ok(d2)) => {
            assert_eq!(d1.len(), d2.len());
        }
        (Err(e1), Err(e2)) => {
            assert_eq!(e1.to_string(), e2.to_string());
        }
        _ => panic!("Validation consistency violated"),
    }
}

// Contract: Empty format string raises error
#[test]
fn contract_empty_format_string_fails() {
    let result = Format::from_str("");
    assert!(result.is_err());
}

// Contract: Document URI is immutable after creation
#[test]
fn contract_document_uri_immutable() {
    let uri = "file:///test".to_string();
    let doc = Document::new(uri.clone(), "content".to_string(), "text".to_string());

    // URI should not change
    assert_eq!(doc.uri, uri);
}

// Contract: Document content is mutable
#[test]
fn contract_document_content_mutable() {
    let mut doc = Document::new("file:///test".to_string(), "original".to_string(), "text".to_string());
    doc.update_content("updated".to_string());

    assert_eq!(doc.content, "updated");
}
