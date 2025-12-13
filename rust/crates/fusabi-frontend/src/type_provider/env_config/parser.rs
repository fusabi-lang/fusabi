//! Environment/Config File Parser
//!
//! Parses .env and similar configuration files to infer types.

use std::collections::HashMap;

/// A parsed environment variable definition
#[derive(Debug, Clone)]
pub struct EnvVar {
    /// Variable name (e.g., "DATABASE_URL")
    pub name: String,
    /// Inferred type
    pub var_type: EnvVarType,
    /// Whether this variable is required (has no default or empty default)
    pub required: bool,
    /// Default value if any
    pub default: Option<String>,
    /// Comment/description
    pub description: Option<String>,
}

/// Inferred type of an environment variable
#[derive(Debug, Clone, PartialEq)]
pub enum EnvVarType {
    String,
    Int,
    Float,
    Bool,
    Url,
    Path,
    /// Comma-separated list
    List(Box<EnvVarType>),
}

impl EnvVarType {
    /// Convert to Fusabi type name
    pub fn to_fusabi_type(&self) -> String {
        match self {
            EnvVarType::String => "string".to_string(),
            EnvVarType::Int => "int".to_string(),
            EnvVarType::Float => "float".to_string(),
            EnvVarType::Bool => "bool".to_string(),
            EnvVarType::Url => "string".to_string(),  // URLs are strings
            EnvVarType::Path => "string".to_string(), // Paths are strings
            EnvVarType::List(inner) => format!("{} list", inner.to_fusabi_type()),
        }
    }

    /// Infer type from a value
    pub fn infer(value: &str) -> Self {
        let value = value.trim();

        // Empty or placeholder
        if value.is_empty() || value.starts_with("${") || value.starts_with("$") {
            return EnvVarType::String;
        }

        // Boolean
        if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("false") {
            return EnvVarType::Bool;
        }

        // Integer
        if value.parse::<i64>().is_ok() {
            return EnvVarType::Int;
        }

        // Float
        if value.parse::<f64>().is_ok() && value.contains('.') {
            return EnvVarType::Float;
        }

        // URL patterns
        if value.starts_with("http://") || value.starts_with("https://")
            || value.starts_with("postgres://") || value.starts_with("mysql://")
            || value.starts_with("redis://") || value.starts_with("mongodb://")
            || value.starts_with("amqp://") || value.starts_with("nats://") {
            return EnvVarType::Url;
        }

        // Path patterns
        if value.starts_with("/") || value.starts_with("./") || value.starts_with("../")
            || value.contains("/") && !value.contains("://") {
            return EnvVarType::Path;
        }

        // Comma-separated list
        if value.contains(',') {
            let parts: Vec<&str> = value.split(',').collect();
            if parts.len() > 1 {
                // Infer type of first non-empty element
                let inner_type = parts.iter()
                    .find(|p| !p.trim().is_empty())
                    .map(|p| EnvVarType::infer(p.trim()))
                    .unwrap_or(EnvVarType::String);
                return EnvVarType::List(Box::new(inner_type));
            }
        }

        EnvVarType::String
    }
}

/// Parsed environment configuration
#[derive(Debug, Clone, Default)]
pub struct ParsedEnvConfig {
    pub vars: Vec<EnvVar>,
    /// Grouped by prefix (e.g., "DATABASE_" -> database group)
    pub groups: HashMap<String, Vec<EnvVar>>,
}

impl ParsedEnvConfig {
    /// Parse a .env file content
    pub fn parse(content: &str) -> Self {
        let mut vars = Vec::new();
        let mut current_comment: Option<String> = None;

        for line in content.lines() {
            let line = line.trim();

            // Skip empty lines
            if line.is_empty() {
                current_comment = None;
                continue;
            }

            // Collect comments
            if line.starts_with('#') {
                let comment = line.trim_start_matches('#').trim();
                current_comment = Some(comment.to_string());
                continue;
            }

            // Parse KEY=VALUE
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                // Remove quotes from value
                let value = value.trim_matches('"').trim_matches('\'');

                let var_type = EnvVarType::infer(value);
                let required = value.is_empty()
                    || value.starts_with("${")
                    || value.starts_with("$");

                vars.push(EnvVar {
                    name: key.to_string(),
                    var_type,
                    required,
                    default: if required { None } else { Some(value.to_string()) },
                    description: current_comment.take(),
                });
            }
        }

        // Group by prefix
        let mut groups: HashMap<String, Vec<EnvVar>> = HashMap::new();
        for var in &vars {
            let prefix = var.name.split('_').next()
                .map(|s| s.to_lowercase())
                .unwrap_or_else(|| "general".to_string());
            groups.entry(prefix).or_default().push(var.clone());
        }

        ParsedEnvConfig { vars, groups }
    }

    /// Parse a JSON schema for environment variables
    pub fn parse_json_schema(json: &str) -> Result<Self, String> {
        let value: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| format!("Invalid JSON: {}", e))?;

        let mut vars = Vec::new();

        if let Some(properties) = value.get("properties").and_then(|p| p.as_object()) {
            let required: Vec<String> = value.get("required")
                .and_then(|r| r.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default();

            for (name, prop) in properties {
                let var_type = match prop.get("type").and_then(|t| t.as_str()) {
                    Some("string") => {
                        if prop.get("format").and_then(|f| f.as_str()) == Some("uri") {
                            EnvVarType::Url
                        } else {
                            EnvVarType::String
                        }
                    }
                    Some("integer") => EnvVarType::Int,
                    Some("number") => EnvVarType::Float,
                    Some("boolean") => EnvVarType::Bool,
                    Some("array") => {
                        let inner = prop.get("items")
                            .and_then(|i| i.get("type"))
                            .and_then(|t| t.as_str())
                            .map(|t| match t {
                                "integer" => EnvVarType::Int,
                                "number" => EnvVarType::Float,
                                "boolean" => EnvVarType::Bool,
                                _ => EnvVarType::String,
                            })
                            .unwrap_or(EnvVarType::String);
                        EnvVarType::List(Box::new(inner))
                    }
                    _ => EnvVarType::String,
                };

                let default = prop.get("default")
                    .map(|d| d.to_string().trim_matches('"').to_string());

                let description = prop.get("description")
                    .and_then(|d| d.as_str())
                    .map(String::from);

                vars.push(EnvVar {
                    name: name.clone(),
                    var_type,
                    required: required.contains(name),
                    default,
                    description,
                });
            }
        }

        let mut groups: HashMap<String, Vec<EnvVar>> = HashMap::new();
        for var in &vars {
            let prefix = var.name.split('_').next()
                .map(|s| s.to_lowercase())
                .unwrap_or_else(|| "general".to_string());
            groups.entry(prefix).or_default().push(var.clone());
        }

        Ok(ParsedEnvConfig { vars, groups })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_types() {
        assert_eq!(EnvVarType::infer("hello"), EnvVarType::String);
        assert_eq!(EnvVarType::infer("42"), EnvVarType::Int);
        assert_eq!(EnvVarType::infer("3.14"), EnvVarType::Float);
        assert_eq!(EnvVarType::infer("true"), EnvVarType::Bool);
        assert_eq!(EnvVarType::infer("false"), EnvVarType::Bool);
        assert_eq!(EnvVarType::infer("https://example.com"), EnvVarType::Url);
        assert_eq!(EnvVarType::infer("postgres://localhost/db"), EnvVarType::Url);
        assert_eq!(EnvVarType::infer("/var/log/app.log"), EnvVarType::Path);
    }

    #[test]
    fn test_infer_list() {
        assert_eq!(
            EnvVarType::infer("a,b,c"),
            EnvVarType::List(Box::new(EnvVarType::String))
        );
        assert_eq!(
            EnvVarType::infer("1,2,3"),
            EnvVarType::List(Box::new(EnvVarType::Int))
        );
    }

    #[test]
    fn test_parse_env_file() {
        let content = r#"
# Database configuration
DATABASE_URL=postgres://localhost/mydb
DATABASE_POOL_SIZE=10

# App settings
PORT=3000
DEBUG=true
API_KEY=
LOG_LEVEL=info
"#;

        let config = ParsedEnvConfig::parse(content);
        assert_eq!(config.vars.len(), 6);

        let db_url = config.vars.iter().find(|v| v.name == "DATABASE_URL").unwrap();
        assert_eq!(db_url.var_type, EnvVarType::Url);
        assert!(!db_url.required);
        assert_eq!(db_url.description, Some("Database configuration".to_string()));

        let pool_size = config.vars.iter().find(|v| v.name == "DATABASE_POOL_SIZE").unwrap();
        assert_eq!(pool_size.var_type, EnvVarType::Int);

        let api_key = config.vars.iter().find(|v| v.name == "API_KEY").unwrap();
        assert!(api_key.required); // Empty value means required

        let debug = config.vars.iter().find(|v| v.name == "DEBUG").unwrap();
        assert_eq!(debug.var_type, EnvVarType::Bool);
    }

    #[test]
    fn test_type_to_fusabi() {
        assert_eq!(EnvVarType::String.to_fusabi_type(), "string");
        assert_eq!(EnvVarType::Int.to_fusabi_type(), "int");
        assert_eq!(EnvVarType::Bool.to_fusabi_type(), "bool");
        assert_eq!(
            EnvVarType::List(Box::new(EnvVarType::String)).to_fusabi_type(),
            "string list"
        );
    }
}
