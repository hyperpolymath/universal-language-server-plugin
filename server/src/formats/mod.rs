//! Extended format support
//!
//! Provides conversion support for YAML, XML, and TOML formats.

pub mod yaml;
pub mod xml;
pub mod toml;

use anyhow::Result;

/// Extended format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtendedFormat {
    Yaml,
    Xml,
    Toml,
}

impl ExtendedFormat {
    /// Parse format from string
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "yaml" | "yml" => Ok(Self::Yaml),
            "xml" => Ok(Self::Xml),
            "toml" => Ok(Self::Toml),
            _ => Err(anyhow::anyhow!("Unsupported extended format: {}", s)),
        }
    }

    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Yaml => "yaml",
            Self::Xml => "xml",
            Self::Toml => "toml",
        }
    }

    /// Validate format
    pub fn validate(&self, content: &str) -> Result<Vec<String>> {
        match self {
            Self::Yaml => yaml::validate_yaml(content),
            Self::Xml => xml::validate_xml(content),
            Self::Toml => toml::validate_toml(content),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_from_str() {
        assert_eq!(ExtendedFormat::from_str("yaml").unwrap(), ExtendedFormat::Yaml);
        assert_eq!(ExtendedFormat::from_str("yml").unwrap(), ExtendedFormat::Yaml);
        assert_eq!(ExtendedFormat::from_str("xml").unwrap(), ExtendedFormat::Xml);
        assert_eq!(ExtendedFormat::from_str("toml").unwrap(), ExtendedFormat::Toml);
    }

    #[test]
    fn test_format_extension() {
        assert_eq!(ExtendedFormat::Yaml.extension(), "yaml");
        assert_eq!(ExtendedFormat::Xml.extension(), "xml");
        assert_eq!(ExtendedFormat::Toml.extension(), "toml");
    }
}
