//! XML format support for document conversion

use anyhow::{Result, Context};
use serde_json::Value;
use quick_xml::de::from_str as xml_from_str;
use quick_xml::se::to_string as xml_to_string;

/// Convert XML to JSON
pub fn xml_to_json(xml: &str) -> Result<String> {
    let value: Value = xml_from_str(xml)
        .context("Failed to parse XML")?;
    Ok(serde_json::to_string_pretty(&value)?)
}

/// Convert JSON to XML
pub fn json_to_xml(json: &str) -> Result<String> {
    let value: Value = serde_json::from_str(json)?;
    let xml = xml_to_string(&value)
        .context("Failed to serialize to XML")?;
    Ok(format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", xml))
}

/// Validate XML syntax
pub fn validate_xml(xml: &str) -> Result<Vec<String>> {
    let mut diagnostics = Vec::new();

    if xml.trim().is_empty() {
        diagnostics.push("XML document is empty".to_string());
    }

    // Check for XML declaration
    if !xml.starts_with("<?xml") {
        diagnostics.push("Missing XML declaration".to_string());
    }

    // Basic tag matching
    let open_tags = xml.matches('<').count();
    let close_tags = xml.matches('>').count();
    if open_tags != close_tags {
        diagnostics.push("Mismatched XML tags".to_string());
    }

    Ok(diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_xml_empty() {
        let diagnostics = validate_xml("").unwrap();
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_validate_xml_no_declaration() {
        let xml = "<root></root>";
        let diagnostics = validate_xml(xml).unwrap();
        assert!(diagnostics.iter().any(|d| d.contains("declaration")));
    }
}
