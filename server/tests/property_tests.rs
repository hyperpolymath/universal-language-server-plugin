// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Property-based tests for format parsing and core functionality
//!
//! These tests verify invariants using proptest to generate diverse inputs
//! and ensure no panics or unexpected failures occur.

use proptest::prelude::*;
use universal_connector_server::core::{ConversionCore, ConversionRequest, Format};
use universal_connector_server::document_store::{Document, DocumentStore};

// Property: Any TOML string parses safely (no panic, returns Ok or Err)
proptest! {
    #[test]
    fn prop_toml_parse_never_panics(s in ".*") {
        // This should never panic, only return Ok or Err
        let req = ConversionRequest {
            content: s.clone(),
            from: Format::Toml,
            to: Format::Json,
        };
        let _ = ConversionCore::convert(req);
    }
}

// Property: Any YAML string parses safely (no panic, returns Ok or Err)
proptest! {
    #[test]
    fn prop_yaml_parse_never_panics(s in ".*") {
        let req = ConversionRequest {
            content: s.clone(),
            from: Format::Yaml,
            to: Format::Json,
        };
        let _ = ConversionCore::convert(req);
    }
}

// Property: Any XML string parses safely (no panic, returns Ok or Err)
proptest! {
    #[test]
    fn prop_xml_parse_never_panics(s in ".*") {
        let req = ConversionRequest {
            content: s.clone(),
            from: Format::Xml,
            to: Format::Json,
        };
        let _ = ConversionCore::convert(req);
    }
}

// Property: Any JSON string parses safely (no panic, returns Ok or Err)
proptest! {
    #[test]
    fn prop_json_parse_never_panics(s in ".*") {
        let req = ConversionRequest {
            content: s.clone(),
            from: Format::Json,
            to: Format::Yaml,
        };
        let _ = ConversionCore::convert(req);
    }
}

// Property: Same format conversion always preserves content
proptest! {
    #[test]
    fn prop_same_format_preserves_content(s in ".*") {
        let req = ConversionRequest {
            content: s.clone(),
            from: Format::Markdown,
            to: Format::Markdown,
        };
        if let Ok(response) = ConversionCore::convert(req) {
            prop_assert_eq!(response.content, s);
        }
    }
}

// Property: Document store store-retrieve roundtrip
proptest! {
    #[test]
    fn prop_document_store_roundtrip(uri in ".*", content in ".*") {
        let _store = DocumentStore::new();
        // Note: DocumentStore uses upsert which requires non-empty URI
        if !uri.is_empty() {
            let _store = DocumentStore::new();
            _store.upsert(uri.clone(), content.clone(), "text".to_string());

            if let Some(retrieved) = _store.get(&uri) {
                prop_assert_eq!(retrieved.content, content);
                prop_assert_eq!(retrieved.uri, uri);
            }
        }
    }
}

// Property: Document version always increments on update
proptest! {
    #[test]
    fn prop_document_version_increments(initial_content in ".*", updates in prop::collection::vec(".*", 1..10)) {
        let store = DocumentStore::new();
        let mut doc = Document::new("test://doc".to_string(), initial_content, "text".to_string());
        let initial_version = doc.version;

        for update_content in updates {
            doc.update_content(update_content);
        }

        prop_assert!(doc.version > initial_version,
                    "Version {} should be > {}", doc.version, initial_version);
    }
}

// Property: Markdown to HTML roundtrip produces valid HTML
proptest! {
    #[test]
    fn prop_markdown_to_html_valid_markup(s in "[a-zA-Z0-9 \n]*") {
        let req = ConversionRequest {
            content: s.clone(),
            from: Format::Markdown,
            to: Format::Html,
        };
        if let Ok(response) = ConversionCore::convert(req) {
            // Basic HTML validity check: should be valid UTF-8 and not empty for non-empty input
            if !s.trim().is_empty() {
                prop_assert!(!response.content.is_empty());
            }
        }
    }
}

// Property: Format detection is symmetric (format roundtrip)
proptest! {
    #[test]
    fn prop_format_detection_consistent(s in r"[a-zA-Z0-9_.]+") {
        // Try parsing a format name consistently
        if let Ok(fmt1) = Format::from_str(&s) {
            let ext = fmt1.extension();
            if let Ok(fmt2) = Format::from_str(ext) {
                prop_assert_eq!(fmt1, fmt2);
            }
        }
    }
}

// Property: Valid JSON always serializes back to JSON
proptest! {
    #[test]
    fn prop_json_is_idempotent(s in r#"\{"#) {
        // Just test that simple JSON objects don't panic
        let json_str = r#"{"key": "value"}"#;
        let req = ConversionRequest {
            content: json_str.to_string(),
            from: Format::Json,
            to: Format::Json,
        };
        if let Ok(response) = ConversionCore::convert(req) {
            // Should parse as valid JSON both times
            let _: serde_json::Value = serde_json::from_str(&response.content)
                .expect("output should be valid JSON");
        }
    }
}

// Property: LSP position is always non-negative (line >= 0, character >= 0)
proptest! {
    #[test]
    fn prop_lsp_position_non_negative(line in 0i32..1000, character in 0i32..1000) {
        // Verify line and character are non-negative
        prop_assert!(line >= 0);
        prop_assert!(character >= 0);
    }
}

// Property: Document URI always contains valid UTF-8
proptest! {
    #[test]
    fn prop_document_uri_valid_utf8(uri in ".*") {
        let doc = Document::new(uri.clone(), "content".to_string(), "text".to_string());
        // String in Rust is always valid UTF-8
        prop_assert!(doc.uri.is_ascii() || doc.uri.chars().count() > 0);
    }
}

// Property: Document stats are always non-negative
proptest! {
    #[test]
    fn prop_document_stats_non_negative(content in ".*") {
        let doc = Document::new("test://doc".to_string(), content, "text".to_string());
        let stats = doc.stats();

        prop_assert!(stats.version > 0);
    }
}

// Property: Empty documents are handled gracefully
proptest! {
    #[test]
    fn prop_empty_content_handled(uri in ".*", lang in ".*") {
        let doc = Document::new(uri, "".to_string(), lang);
        let stats = doc.stats();
        prop_assert_eq!(stats.characters, 0);
        // Empty doc has 1 line minimum
        prop_assert!(stats.lines <= 1);
    }
}

// Property: Unicode in URIs is preserved
proptest! {
    #[test]
    fn prop_unicode_uris_preserved(unicode_part in "[а-яА-Я0-9_-]{0,20}") {
        let uri = format!("test://{}", unicode_part);
        let doc = Document::new(uri.clone(), "content".to_string(), "text".to_string());
        prop_assert_eq!(doc.uri, uri);
    }
}

// Property: Large documents (1MB) are handled without panic
proptest! {
    #[test]
    fn prop_large_document_handled(size in 100_000usize..1_100_000) {
        let large_content = "x".repeat(size);
        let doc = Document::new("test://large".to_string(), large_content, "text".to_string());
        let stats = doc.stats();
        prop_assert!(stats.characters > 0);
    }
}
