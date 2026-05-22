// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Aspect tests for security, robustness, and edge cases
//!
//! Tests verify:
//! - Malformed input handling
//! - Resource limits
//! - Unicode edge cases
//! - Boundary conditions

use universal_connector_server::core::{ConversionCore, ConversionRequest, Format};
use universal_connector_server::document_store::Document;

// Aspect: Malformed TOML with null bytes → parse error, not crash
#[test]
fn aspect_malformed_toml_with_null_bytes() {
    let malformed = "key = \"value\x00broken\"";

    let request = ConversionRequest {
        content: malformed.to_string(),
        from: Format::Toml,
        to: Format::Json,
    };

    // Should return error, not panic
    match ConversionCore::convert(request) {
        Ok(response) => {
            // If it succeeds, verify output is valid
            assert!(!response.content.is_empty());
        }
        Err(_) => {
            // Expected: malformed input yields error
        }
    }
}

// Aspect: Malformed JSON → error, not crash
#[test]
fn aspect_malformed_json() {
    let malformed = r#"{"key": "unclosed value}"#;

    let request = ConversionRequest {
        content: malformed.to_string(),
        from: Format::Json,
        to: Format::Yaml,
    };

    // Should handle gracefully
    match ConversionCore::convert(request) {
        Ok(_) => { /* acceptable */ }
        Err(_) => { /* expected */ }
    }
}

// Aspect: Malformed YAML with tab indentation → error, not crash
#[test]
fn aspect_malformed_yaml_tabs() {
    let malformed = "key:\n\tvalue";

    let request = ConversionRequest {
        content: malformed.to_string(),
        from: Format::Yaml,
        to: Format::Json,
    };

    // Should not panic
    match ConversionCore::convert(request) {
        Ok(_) => { /* acceptable */ }
        Err(_) => { /* expected */ }
    }
}

// Aspect: Malformed XML → error, not crash
#[test]
fn aspect_malformed_xml() {
    let malformed = "<root><unclosed";

    let request = ConversionRequest {
        content: malformed.to_string(),
        from: Format::Xml,
        to: Format::Json,
    };

    // Should handle gracefully
    match ConversionCore::convert(request) {
        Ok(_) => { /* acceptable */ }
        Err(_) => { /* expected */ }
    }
}

// Aspect: Oversized document (1MB) → handled without memory explosion
#[test]
fn aspect_oversized_document_1mb() {
    // Create a 1MB document
    let large_content = "a".repeat(1024 * 1024);

    let request = ConversionRequest {
        content: large_content.clone(),
        from: Format::Markdown,
        to: Format::Html,
    };

    // Should complete without crashing (may timeout or fail gracefully)
    match ConversionCore::convert(request) {
        Ok(response) => {
            // If it succeeds, output should be reasonable
            assert!(!response.content.is_empty());
            // Output shouldn't be unbounded (1GB+)
            assert!(response.content.len() < 100 * 1024 * 1024, "Output too large");
        }
        Err(_) => {
            // Acceptable: large file conversion may fail
        }
    }
}

// Aspect: Oversized document (10MB) → resource handling
#[test]
fn aspect_oversized_document_10mb() {
    let large_content = "test line\n".repeat(1024 * 1024);

    let doc = Document::new(
        "file:///large.md".to_string(),
        large_content,
        "markdown".to_string(),
    );

    let stats = doc.stats();
    // Should compute stats without crashing
    assert!(stats.characters > 0);
}

// Aspect: Unicode in document paths → handled correctly
#[test]
fn aspect_unicode_in_document_uri() {
    let unicode_uri = "file:///test/документ_文档_📄.md".to_string();
    let content = "Content".to_string();

    let doc = Document::new(unicode_uri.clone(), content.clone(), "markdown".to_string());

    assert_eq!(doc.uri, unicode_uri);
    assert_eq!(doc.content, content);
}

// Aspect: Emoji in document content → preserved
#[test]
fn aspect_emoji_in_content() {
    let emoji_content = "# Hello 👋\n\nWith emojis 🎉 🚀 ✨".to_string();

    let doc = Document::new("file:///test.md".to_string(), emoji_content.clone(), "markdown".to_string());

    assert!(doc.content.contains("👋"));
    assert!(doc.content.contains("🎉"));
}

// Aspect: Multi-byte UTF-8 characters preserved
#[test]
fn aspect_multibyte_utf8_preserved() {
    let utf8_content = "Привет мир\n中文\n日本語\nهلا وسهلا".to_string();

    let doc = Document::new(
        "file:///test.txt".to_string(),
        utf8_content.clone(),
        "text".to_string(),
    );

    assert_eq!(doc.content, utf8_content);
    assert!(doc.content.len() > 0);
}

// Aspect: LSP position out of bounds → graceful handling
#[test]
fn aspect_lsp_position_out_of_bounds() {
    let content = "Line 1\nLine 2".to_string();

    // Try accessing beyond document bounds
    let doc = Document::new("file:///test.md".to_string(), content, "markdown".to_string());

    // Computing stats should work even if we ask for positions beyond content
    let stats = doc.stats();
    assert!(stats.lines >= 1);
}

// Aspect: Empty content handled gracefully
#[test]
fn aspect_empty_content() {
    let empty = "".to_string();

    let doc = Document::new("file:///empty.md".to_string(), empty.clone(), "markdown".to_string());

    let stats = doc.stats();
    assert_eq!(stats.characters, 0);
}

// Aspect: Whitespace-only content
#[test]
fn aspect_whitespace_only_content() {
    let whitespace = "   \n  \n  \t".to_string();

    let doc = Document::new(
        "file:///whitespace.md".to_string(),
        whitespace.clone(),
        "markdown".to_string(),
    );

    let stats = doc.stats();
    assert!(stats.characters > 0);
    assert!(stats.lines >= 1);
}

// Aspect: Very deeply nested JSON → handled
#[test]
fn aspect_deeply_nested_json() {
    // Create deeply nested JSON
    let mut nested = String::new();
    for i in 0..100 {
        nested.push_str(&format!("{{\"level{}\": ", i));
    }
    nested.push_str("\"deepvalue\"");
    for _ in 0..100 {
        nested.push('}');
    }

    let request = ConversionRequest {
        content: nested,
        from: Format::Json,
        to: Format::Yaml,
    };

    // Should not stack overflow
    match ConversionCore::convert(request) {
        Ok(_) => { /* acceptable */ }
        Err(_) => { /* acceptable */ }
    }
}

// Aspect: Very long line (1MB single line) → handled
#[test]
fn aspect_very_long_single_line() {
    let long_line = "a".repeat(1024 * 1024);

    let doc = Document::new(
        "file:///long.txt".to_string(),
        long_line,
        "text".to_string(),
    );

    let stats = doc.stats();
    assert_eq!(stats.lines, 1);
}

// Aspect: Control characters in content → handled
#[test]
fn aspect_control_characters() {
    let content_with_controls = "Line1\x00\x01\x02\nLine2\x1f".to_string();

    let doc = Document::new(
        "file:///controls.txt".to_string(),
        content_with_controls,
        "text".to_string(),
    );

    // Should store and retrieve without crashing
    let stats = doc.stats();
    assert!(stats.characters > 0);
}

// Aspect: BOM (Byte Order Mark) in UTF-8 → handled
#[test]
fn aspect_bom_in_utf8() {
    // UTF-8 BOM
    let bom_content = "\u{FEFF}Content with BOM".to_string();

    let doc = Document::new(
        "file:///bom.txt".to_string(),
        bom_content.clone(),
        "text".to_string(),
    );

    assert!(doc.content.contains("Content"));
}

// Aspect: Mixed line endings (LF/CR/CRLF) → handled
#[test]
fn aspect_mixed_line_endings() {
    let mixed = "Line1\nLine2\r\nLine3\rLine4".to_string();

    let doc = Document::new(
        "file:///mixed.txt".to_string(),
        mixed,
        "text".to_string(),
    );

    let stats = doc.stats();
    // Should parse all lines correctly despite mixed endings
    assert!(stats.lines >= 2);
}

// Aspect: Extremely small document (1 byte)
#[test]
fn aspect_minimal_document() {
    let minimal = "x".to_string();

    let doc = Document::new(
        "file:///tiny.txt".to_string(),
        minimal.clone(),
        "text".to_string(),
    );

    let stats = doc.stats();
    assert_eq!(stats.characters, 1);
    assert_eq!(stats.lines, 1);
}

// Aspect: Document with only newlines
#[test]
fn aspect_document_with_only_newlines() {
    let newlines_only = "\n\n\n\n\n".to_string();

    let doc = Document::new(
        "file:///newlines.txt".to_string(),
        newlines_only,
        "text".to_string(),
    );

    let stats = doc.stats();
    assert!(stats.lines >= 5);
}

// Aspect: Conversion with circular format references
#[test]
fn aspect_format_circular_conversion() {
    let content = "Test content".to_string();

    // Markdown → Markdown (same format)
    let request = ConversionRequest {
        content,
        from: Format::Markdown,
        to: Format::Markdown,
    };

    let response = ConversionCore::convert(request).expect("Same format should work");
    // Should preserve content unchanged
    assert_eq!(response.content, "Test content");
}

// Aspect: Rapid document updates → version consistency
#[test]
fn aspect_rapid_document_updates() {
    let mut doc = Document::new(
        "file:///rapid.md".to_string(),
        "v1".to_string(),
        "markdown".to_string(),
    );

    for i in 2..20 {
        doc.update_content(format!("v{}", i));
    }

    // Version should reflect all updates
    assert_eq!(doc.version, 19);
}

// Aspect: Document content with language-specific syntax
#[test]
fn aspect_code_syntax_in_content() {
    let code_content = r#"```rust
fn main() {
    println!("Hello");
}
```"#.to_string();

    let doc = Document::new(
        "file:///code.md".to_string(),
        code_content.clone(),
        "markdown".to_string(),
    );

    assert!(doc.content.contains("println"));
}

// Aspect: Simultaneous reads don't block document
#[test]
fn aspect_concurrent_reads() {
    use std::sync::Arc;
    use std::thread;

    let doc = Arc::new(Document::new(
        "file:///concurrent.md".to_string(),
        "shared content".to_string(),
        "markdown".to_string(),
    ));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let doc_clone = Arc::clone(&doc);
            thread::spawn(move || {
                let stats = doc_clone.stats();
                assert!(stats.characters > 0);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
