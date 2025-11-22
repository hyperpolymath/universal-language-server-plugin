//! Core conversion engine tests

use universal_connector_server::core::{ConversionCore, ConversionRequest, Format};

#[test]
fn test_markdown_to_html_conversion() {
    let request = ConversionRequest {
        content: "# Hello World\n\nThis is **bold**.".to_string(),
        from: Format::Markdown,
        to: Format::Html,
    };

    let response = ConversionCore::convert(request).unwrap();
    assert!(response.content.contains("<h1>"));
    assert!(response.content.contains("Hello World"));
    assert!(response.content.contains("<strong>"));
    assert!(response.content.contains("bold"));
}

#[test]
fn test_html_to_markdown_conversion() {
    let request = ConversionRequest {
        content: "<h1>Title</h1><p>Paragraph text.</p>".to_string(),
        from: Format::Html,
        to: Format::Markdown,
    };

    let response = ConversionCore::convert(request).unwrap();
    assert!(response.content.contains("# Title"));
    assert!(response.content.contains("Paragraph text"));
}

#[test]
fn test_markdown_to_json_conversion() {
    let request = ConversionRequest {
        content: "# Test Document\n\nContent here.".to_string(),
        from: Format::Markdown,
        to: Format::Json,
    };

    let response = ConversionCore::convert(request).unwrap();
    assert!(response.content.contains("content"));
    assert!(response.content.contains("Test Document"));

    // Verify it's valid JSON
    let _: serde_json::Value = serde_json::from_str(&response.content).unwrap();
}

#[test]
fn test_json_to_markdown_conversion() {
    let json = r#"{
        "title": "Test",
        "content": "This is test content"
    }"#;

    let request = ConversionRequest {
        content: json.to_string(),
        from: Format::Json,
        to: Format::Markdown,
    };

    let response = ConversionCore::convert(request).unwrap();
    assert!(response.content.contains("Test"));
    assert!(response.content.contains("test content"));
}

#[test]
fn test_same_format_no_conversion() {
    let original = "# Same Format";
    let request = ConversionRequest {
        content: original.to_string(),
        from: Format::Markdown,
        to: Format::Markdown,
    };

    let response = ConversionCore::convert(request).unwrap();
    assert_eq!(response.content, original);
    assert!(response.warnings.is_empty());
}

#[test]
fn test_html_to_markdown_with_warnings() {
    let request = ConversionRequest {
        content: "<div><p>Test</p></div>".to_string(),
        from: Format::Html,
        to: Format::Markdown,
    };

    let response = ConversionCore::convert(request).unwrap();
    assert!(!response.warnings.is_empty());
}

#[test]
fn test_json_roundtrip() {
    let original = "# Title\n\nContent paragraph.";

    // Markdown -> JSON
    let to_json = ConversionRequest {
        content: original.to_string(),
        from: Format::Markdown,
        to: Format::Json,
    };
    let json_result = ConversionCore::convert(to_json).unwrap();

    // JSON -> Markdown
    let to_md = ConversionRequest {
        content: json_result.content,
        from: Format::Json,
        to: Format::Markdown,
    };
    let md_result = ConversionCore::convert(to_md).unwrap();

    assert!(md_result.content.contains("Title"));
    assert!(md_result.content.contains("Content paragraph"));
}

#[test]
fn test_validate_markdown() {
    let diagnostics = ConversionCore::validate("# Valid Markdown", Format::Markdown).unwrap();
    assert!(diagnostics.is_empty());
}

#[test]
fn test_validate_empty_document() {
    let diagnostics = ConversionCore::validate("", Format::Markdown).unwrap();
    assert!(!diagnostics.is_empty());
}

#[test]
fn test_validate_invalid_json() {
    let diagnostics = ConversionCore::validate("{ invalid json }", Format::Json).unwrap();
    assert!(!diagnostics.is_empty());
}

#[test]
fn test_validate_valid_json() {
    let diagnostics = ConversionCore::validate(r#"{"key": "value"}"#, Format::Json).unwrap();
    assert!(diagnostics.is_empty());
}

#[test]
fn test_format_from_string() {
    assert_eq!(Format::from_str("markdown").unwrap(), Format::Markdown);
    assert_eq!(Format::from_str("md").unwrap(), Format::Markdown);
    assert_eq!(Format::from_str("html").unwrap(), Format::Html);
    assert_eq!(Format::from_str("htm").unwrap(), Format::Html);
    assert_eq!(Format::from_str("json").unwrap(), Format::Json);
    assert!(Format::from_str("invalid").is_err());
}

#[test]
fn test_format_extension() {
    assert_eq!(Format::Markdown.extension(), "md");
    assert_eq!(Format::Html.extension(), "html");
    assert_eq!(Format::Json.extension(), "json");
}

#[test]
fn test_complex_markdown_features() {
    let markdown = r#"
# Heading 1
## Heading 2

**Bold** and *italic* text.

- List item 1
- List item 2

```rust
fn main() {
    println!("Code block");
}
```

[Link](https://example.com)
"#;

    let request = ConversionRequest {
        content: markdown.to_string(),
        from: Format::Markdown,
        to: Format::Html,
    };

    let response = ConversionCore::convert(request).unwrap();
    assert!(response.content.contains("<h1>"));
    assert!(response.content.contains("<h2>"));
    assert!(response.content.contains("<strong>"));
    assert!(response.content.contains("<em>"));
    assert!(response.content.contains("<li>"));
    assert!(response.content.contains("<code>"));
    assert!(response.content.contains("<a"));
}

#[test]
fn test_unicode_content() {
    let content = "# 你好世界 🌍\n\nКириллица";

    let request = ConversionRequest {
        content: content.to_string(),
        from: Format::Markdown,
        to: Format::Html,
    };

    let response = ConversionCore::convert(request).unwrap();
    assert!(response.content.contains("你好世界"));
    assert!(response.content.contains("🌍"));
    assert!(response.content.contains("Кириллица"));
}
