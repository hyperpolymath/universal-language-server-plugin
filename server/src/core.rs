//! Document conversion core engine
//!
//! Provides bidirectional conversion between formats:
//! - Markdown ↔ HTML ↔ JSON

use anyhow::{anyhow, Result};
use pulldown_cmark::{html, Parser};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported conversion formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Markdown,
    Html,
    Json,
}

impl Format {
    /// Parse format from string
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "markdown" | "md" => Ok(Self::Markdown),
            "html" | "htm" => Ok(Self::Html),
            "json" => Ok(Self::Json),
            _ => Err(anyhow!("Unsupported format: {}", s)),
        }
    }

    /// Get file extension for format
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Markdown => "md",
            Self::Html => "html",
            Self::Json => "json",
        }
    }
}

/// Conversion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRequest {
    pub content: String,
    pub from: Format,
    pub to: Format,
}

/// Conversion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResponse {
    pub content: String,
    pub from: Format,
    pub to: Format,
    pub warnings: Vec<String>,
}

/// Document conversion engine
pub struct ConversionCore;

impl ConversionCore {
    /// Convert document between formats
    pub fn convert(request: ConversionRequest) -> Result<ConversionResponse> {
        let mut warnings = Vec::new();

        let content = match (request.from, request.to) {
            // Markdown → HTML
            (Format::Markdown, Format::Html) => Self::markdown_to_html(&request.content),

            // Markdown → JSON
            (Format::Markdown, Format::Json) => Self::markdown_to_json(&request.content)?,

            // HTML → Markdown
            (Format::Html, Format::Markdown) => {
                warnings.push("HTML to Markdown conversion may lose some formatting".to_string());
                Self::html_to_markdown(&request.content)?
            }

            // HTML → JSON
            (Format::Html, Format::Json) => Self::html_to_json(&request.content)?,

            // JSON → Markdown
            (Format::Json, Format::Markdown) => Self::json_to_markdown(&request.content)?,

            // JSON → HTML
            (Format::Json, Format::Html) => Self::json_to_html(&request.content)?,

            // Same format - no conversion needed
            (Format::Markdown, Format::Markdown) |
            (Format::Html, Format::Html) |
            (Format::Json, Format::Json) => request.content,
        };

        Ok(ConversionResponse {
            content,
            from: request.from,
            to: request.to,
            warnings,
        })
    }

    /// Convert Markdown to HTML using pulldown-cmark
    fn markdown_to_html(markdown: &str) -> String {
        let parser = Parser::new(markdown);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }

    /// Convert Markdown to JSON (structured representation)
    fn markdown_to_json(markdown: &str) -> Result<String> {
        let html = Self::markdown_to_html(markdown);
        Self::html_to_json(&html)
    }

    /// Convert HTML to Markdown (lossy conversion)
    fn html_to_markdown(html_content: &str) -> Result<String> {
        let document = Html::parse_document(html_content);

        // Extract text content and attempt basic structure preservation
        let mut markdown = String::new();

        // Extract headings
        for level in 1..=6 {
            let selector = Selector::parse(&format!("h{level}")).unwrap();
            for element in document.select(&selector) {
                let text = element.text().collect::<String>();
                markdown.push_str(&format!("{} {}\n\n", "#".repeat(level), text.trim()));
            }
        }

        // Extract paragraphs
        if let Ok(selector) = Selector::parse("p") {
            for element in document.select(&selector) {
                let text = element.text().collect::<String>();
                markdown.push_str(&format!("{}\n\n", text.trim()));
            }
        }

        // Extract lists
        if let Ok(li_selector) = Selector::parse("li") {
            for element in document.select(&li_selector) {
                let text = element.text().collect::<String>();
                markdown.push_str(&format!("- {}\n", text.trim()));
            }
        }

        // Extract code blocks
        if let Ok(code_selector) = Selector::parse("pre code, code") {
            for element in document.select(&code_selector) {
                let text = element.text().collect::<String>();
                if element.value().name() == "code"
                    && element.parent().map_or(false, |p| {
                        p.value().as_element().map_or(false, |e| e.name() == "pre")
                    })
                {
                    markdown.push_str(&format!("```\n{}\n```\n\n", text.trim()));
                } else {
                    markdown.push_str(&format!("`{}`", text.trim()));
                }
            }
        }

        // If we got nothing, just extract all text
        if markdown.trim().is_empty() {
            let body_selector = Selector::parse("body").unwrap();
            if let Some(body) = document.select(&body_selector).next() {
                markdown = body.text().collect::<String>();
            } else {
                markdown = document.root_element().text().collect::<String>();
            }
        }

        Ok(markdown.trim().to_string())
    }

    /// Convert HTML to JSON (DOM structure)
    fn html_to_json(html_content: &str) -> Result<String> {
        let document = Html::parse_document(html_content);

        let mut data = HashMap::new();
        data.insert("type", "html".to_string());

        // Extract title
        if let Ok(title_selector) = Selector::parse("title") {
            if let Some(title) = document.select(&title_selector).next() {
                data.insert("title", title.text().collect::<String>());
            }
        }

        // Extract all text content
        let text: String = document.root_element().text().collect();
        data.insert("content", text.trim().to_string());

        // Extract headings
        let mut headings = Vec::new();
        for level in 1..=6 {
            let selector = Selector::parse(&format!("h{level}")).unwrap();
            for element in document.select(&selector) {
                headings.push(format!("H{}: {}", level, element.text().collect::<String>()));
            }
        }
        if !headings.is_empty() {
            data.insert("headings", headings.join("; "));
        }

        serde_json::to_string_pretty(&data)
            .map_err(|e| anyhow!("Failed to serialize to JSON: {}", e))
    }

    /// Convert JSON to Markdown
    fn json_to_markdown(json_content: &str) -> Result<String> {
        let data: HashMap<String, serde_json::Value> = serde_json::from_str(json_content)
            .map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;

        let mut markdown = String::new();

        // Add title if present
        if let Some(title) = data.get("title") {
            if let Some(title_str) = title.as_str() {
                markdown.push_str(&format!("# {}\n\n", title_str));
            }
        }

        // Add content
        if let Some(content) = data.get("content") {
            if let Some(content_str) = content.as_str() {
                markdown.push_str(&format!("{}\n\n", content_str));
            }
        }

        // Add other fields as key-value pairs
        for (key, value) in &data {
            if key != "title" && key != "content" && key != "type" {
                markdown.push_str(&format!("**{}**: {}\n\n", key, value));
            }
        }

        Ok(markdown.trim().to_string())
    }

    /// Convert JSON to HTML
    fn json_to_html(json_content: &str) -> Result<String> {
        let markdown = Self::json_to_markdown(json_content)?;
        Ok(Self::markdown_to_html(&markdown))
    }

    /// Validate document format
    pub fn validate(content: &str, format: Format) -> Result<Vec<String>> {
        let mut diagnostics = Vec::new();

        match format {
            Format::Markdown => {
                // Basic Markdown validation
                if content.is_empty() {
                    diagnostics.push("Document is empty".to_string());
                }
            }
            Format::Html => {
                // HTML validation - check if it parses
                let document = Html::parse_document(content);
                if document.errors.is_empty() {
                    // No parse errors
                } else {
                    for error in &document.errors {
                        diagnostics.push(format!("HTML parse error: {:?}", error));
                    }
                }
            }
            Format::Json => {
                // JSON validation
                if let Err(e) = serde_json::from_str::<serde_json::Value>(content) {
                    diagnostics.push(format!("Invalid JSON: {}", e));
                }
            }
        }

        Ok(diagnostics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html() {
        let markdown = "# Hello World\n\nThis is a **test**.";
        let html = ConversionCore::markdown_to_html(markdown);
        assert!(html.contains("<h1>"));
        assert!(html.contains("Hello World"));
        assert!(html.contains("<strong>"));
    }

    #[test]
    fn test_html_to_markdown() {
        let html = "<h1>Hello World</h1><p>This is a test.</p>";
        let markdown = ConversionCore::html_to_markdown(html).unwrap();
        assert!(markdown.contains("# Hello World"));
        assert!(markdown.contains("This is a test"));
    }

    #[test]
    fn test_conversion_request() {
        let request = ConversionRequest {
            content: "# Test".to_string(),
            from: Format::Markdown,
            to: Format::Html,
        };
        let response = ConversionCore::convert(request).unwrap();
        assert!(response.content.contains("<h1>"));
    }

    #[test]
    fn test_same_format_conversion() {
        let request = ConversionRequest {
            content: "# Test".to_string(),
            from: Format::Markdown,
            to: Format::Markdown,
        };
        let response = ConversionCore::convert(request).unwrap();
        assert_eq!(response.content, "# Test");
    }

    #[test]
    fn test_json_roundtrip() {
        let markdown = "# Title\n\nContent here";
        let request = ConversionRequest {
            content: markdown.to_string(),
            from: Format::Markdown,
            to: Format::Json,
        };
        let json_response = ConversionCore::convert(request).unwrap();

        let request = ConversionRequest {
            content: json_response.content,
            from: Format::Json,
            to: Format::Markdown,
        };
        let md_response = ConversionCore::convert(request).unwrap();
        assert!(md_response.content.contains("Title"));
    }

    #[test]
    fn test_validate_json() {
        let valid = r#"{"key": "value"}"#;
        let diagnostics = ConversionCore::validate(valid, Format::Json).unwrap();
        assert!(diagnostics.is_empty());

        let invalid = r#"{"key": invalid}"#;
        let diagnostics = ConversionCore::validate(invalid, Format::Json).unwrap();
        assert!(!diagnostics.is_empty());
    }
}
