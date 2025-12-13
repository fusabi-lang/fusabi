//! Environment/Config Type Provider
//!
//! Generates Fusabi types from environment variable definitions.
//! Prevents typos and ensures type safety for configuration access.
//!
//! # Usage
//!
//! ```text
//! // From .env or .env.example file
//! type Env = EnvProvider<"file://.env.example">
//!
//! // From a JSON schema
//! type Config = EnvProvider<"file://config.schema.json">
//!
//! // Typed, safe access - compile errors for typos!
//! let dbUrl: string = Env.DATABASE_URL
//! let port: int = Env.PORT
//! let debug: bool = Env.DEBUG
//! ```
//!
//! # Supported Formats
//!
//! ## .env files
//! ```
//! # Database configuration
//! DATABASE_URL=postgres://localhost/mydb
//! DATABASE_POOL_SIZE=10
//!
//! # App settings
//! PORT=3000
//! DEBUG=true
//! API_KEY=           # Required (empty = must be provided)
//! ```
//!
//! ## JSON Schema
//! ```json
//! {
//!   "properties": {
//!     "DATABASE_URL": { "type": "string", "format": "uri" },
//!     "PORT": { "type": "integer", "default": 3000 }
//!   },
//!   "required": ["DATABASE_URL"]
//! }
//! ```

pub mod parser;

use crate::ast::{RecordTypeDef, TypeExpr, TypeDefinition};
use crate::type_provider::{
    TypeProvider, ProviderParams, Schema,
    GeneratedTypes, GeneratedModule, TypeGenerator, NamingStrategy,
    error::{ProviderError, ProviderResult},
};
use parser::{ParsedEnvConfig, EnvVar, EnvVarType};

/// Environment/Config type provider
pub struct EnvConfigProvider {
    generator: TypeGenerator,
}

impl EnvConfigProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PreserveOriginal),
        }
    }

    /// Generate types from parsed config
    fn generate_from_config(
        &self,
        config: &ParsedEnvConfig,
        namespace: &str,
    ) -> ProviderResult<GeneratedTypes> {
        // Create a single record with all env vars as fields
        let fields: Vec<(String, TypeExpr)> = config.vars.iter()
            .map(|var| {
                let base_type = var.var_type.to_fusabi_type();
                let field_type = if var.required {
                    TypeExpr::Named(base_type)
                } else {
                    // Optional vars with defaults are still the base type
                    // but we include the default in comments
                    TypeExpr::Named(base_type)
                };
                (var.name.clone(), field_type)
            })
            .collect();

        let root_record = TypeDefinition::Record(RecordTypeDef {
            name: "Config".to_string(),
            fields,
        });

        // Also create grouped records for each prefix
        let mut group_types = Vec::new();
        for (prefix, vars) in &config.groups {
            if vars.len() > 1 {
                let group_name = to_pascal_case(prefix);
                let group_fields: Vec<(String, TypeExpr)> = vars.iter()
                    .map(|var| {
                        // Remove prefix from field name
                        let field_name = var.name
                            .strip_prefix(&format!("{}_", prefix.to_uppercase()))
                            .unwrap_or(&var.name)
                            .to_string();
                        let field_name = to_camel_case(&field_name);
                        let base_type = var.var_type.to_fusabi_type();
                        (field_name, TypeExpr::Named(base_type))
                    })
                    .collect();

                group_types.push(TypeDefinition::Record(RecordTypeDef {
                    name: group_name,
                    fields: group_fields,
                }));
            }
        }

        let mut modules = vec![
            GeneratedModule {
                path: vec![namespace.to_string()],
                types: vec![root_record],
            },
        ];

        if !group_types.is_empty() {
            modules.push(GeneratedModule {
                path: vec![namespace.to_string(), "Groups".to_string()],
                types: group_types,
            });
        }

        Ok(GeneratedTypes {
            modules,
            root_types: vec![],
        })
    }
}

impl Default for EnvConfigProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for EnvConfigProvider {
    fn name(&self) -> &str {
        "EnvConfigProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        let content = if source.starts_with("file://") {
            let path = source.strip_prefix("file://").unwrap();
            std::fs::read_to_string(path)
                .map_err(|e| ProviderError::IoError(format!("Failed to read {}: {}", path, e)))?
        } else {
            // Assume it's a file path
            std::fs::read_to_string(source)
                .map_err(|e| ProviderError::IoError(format!("Failed to read {}: {}", source, e)))?
        };

        Ok(Schema::Custom(content))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        let content = match schema {
            Schema::Custom(s) => s,
            _ => return Err(ProviderError::ParseError("Expected Custom schema".to_string())),
        };

        // Try to parse as JSON schema first, then as .env file
        let config = if content.trim().starts_with('{') {
            ParsedEnvConfig::parse_json_schema(content)
                .map_err(|e| ProviderError::ParseError(e))?
        } else {
            ParsedEnvConfig::parse(content)
        };

        self.generate_from_config(&config, namespace)
    }

    fn get_documentation(&self, _type_path: &str) -> Option<String> {
        None
    }
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().chain(chars.flat_map(|c| c.to_lowercase())).collect()
                }
            }
        })
        .collect()
}

fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut chars = pascal.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().chain(chars).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_name() {
        let provider = EnvConfigProvider::new();
        assert_eq!(provider.name(), "EnvConfigProvider");
    }

    #[test]
    fn test_generate_from_env_content() {
        let env_content = r#"
# Database
DATABASE_URL=postgres://localhost/test
DATABASE_POOL_SIZE=10

# Server
PORT=3000
HOST=localhost
DEBUG=false

# Required secrets
API_KEY=
SECRET_TOKEN=
"#;

        let provider = EnvConfigProvider::new();
        let schema = Schema::Custom(env_content.to_string());
        let types = provider.generate_types(&schema, "Env").unwrap();

        // Should have root module
        assert!(!types.modules.is_empty());

        let root_module = types.modules.iter()
            .find(|m| m.path == vec!["Env".to_string()]);
        assert!(root_module.is_some());

        let root_module = root_module.unwrap();
        assert_eq!(root_module.types.len(), 1);

        if let TypeDefinition::Record(config) = &root_module.types[0] {
            assert_eq!(config.name, "Config");

            let field_names: Vec<&str> = config.fields.iter()
                .map(|(n, _)| n.as_str())
                .collect();

            assert!(field_names.contains(&"DATABASE_URL"));
            assert!(field_names.contains(&"PORT"));
            assert!(field_names.contains(&"API_KEY"));
        } else {
            panic!("Expected Record type");
        }
    }

    #[test]
    fn test_generate_from_json_schema() {
        let schema_json = r#"
{
    "type": "object",
    "properties": {
        "DATABASE_URL": {
            "type": "string",
            "format": "uri",
            "description": "PostgreSQL connection string"
        },
        "PORT": {
            "type": "integer",
            "default": 3000
        },
        "DEBUG": {
            "type": "boolean",
            "default": false
        },
        "ALLOWED_ORIGINS": {
            "type": "array",
            "items": { "type": "string" }
        }
    },
    "required": ["DATABASE_URL"]
}
"#;

        let provider = EnvConfigProvider::new();
        let schema = Schema::Custom(schema_json.to_string());
        let types = provider.generate_types(&schema, "Config").unwrap();

        assert!(!types.modules.is_empty());
    }

    #[test]
    fn test_case_conversion() {
        assert_eq!(to_pascal_case("database_url"), "DatabaseUrl");
        assert_eq!(to_camel_case("DATABASE_URL"), "databaseUrl");
    }
}
