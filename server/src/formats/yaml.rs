//! YAML format support for document conversion
//!
//! Provides bidirectional conversion between YAML and other formats.

use anyhow::Result;
use serde_json::Value;

/// Convert YAML to JSON
pub fn yaml_to_json(yaml: &str) -> Result<String> {
    let value: Value = serde_yaml::from_str(yaml)?;
    Ok(serde_json::to_string_pretty(&value)?)
}

/// Convert JSON to YAML
pub fn json_to_yaml(json: &str) -> Result<String> {
    let value: Value = serde_json::from_str(json)?;
    Ok(serde_yaml::to_string(&value)?)
}

/// Convert YAML to Markdown
pub fn yaml_to_markdown(yaml: &str) -> Result<String> {
    let json = yaml_to_json(yaml)?;
    crate::core::ConversionCore::json_to_markdown(&json)
}

/// Convert Markdown to YAML
pub fn markdown_to_yaml(markdown: &str) -> Result<String> {
    let json = crate::core::ConversionRequest {
        content: markdown.to_string(),
        from: crate::core::Format::Markdown,
        to: crate::core::Format::Json,
    };
    let response = crate::core::ConversionCore::convert(json)?;
    json_to_yaml(&response.content)
}

/// Validate YAML syntax
pub fn validate_yaml(yaml: &str) -> Result<Vec<String>> {
    let mut diagnostics = Vec::new();

    // Basic validation
    if yaml.trim().is_empty() {
        diagnostics.push("YAML document is empty".to_string());
    }

    // Check for common YAML issues
    if yaml.contains('\t') {
        diagnostics.push("YAML should use spaces, not tabs for indentation".to_string());
    }

    Ok(diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_to_json() {
        let yaml = "key: value";
        let result = yaml_to_json(yaml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_yaml_tabs() {
        let yaml = "key:\tvalue"; // Has tab
        let diagnostics = validate_yaml(yaml).unwrap();
        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].contains("tabs"));
    }

    #[test]
    fn test_validate_yaml_empty() {
        let diagnostics = validate_yaml("   ").unwrap();
        assert!(!diagnostics.is_empty());
    }
}
