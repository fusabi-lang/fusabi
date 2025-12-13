//! GraphQL Type Provider
//!
//! Generates Fusabi types from any GraphQL API via introspection.
//! Every GraphQL server exposes its complete schema, making this
//! provider work with ANY compliant GraphQL endpoint.
//!
//! # Usage
//!
//! ```fsharp
//! // From a GraphQL endpoint (introspection query sent automatically)
//! type GitHub = GraphQLProvider<"https://api.github.com/graphql">
//!
//! // From a local schema file
//! type API = GraphQLProvider<"file://./schema.json">
//!
//! // Use fully typed queries
//! let repo: GitHub.Repository = query {
//!     repository(owner: "anthropics", name: "claude-code") {
//!         name
//!         stargazerCount
//!         issues(first: 10) {
//!             nodes { title }
//!         }
//!     }
//! }
//! ```
//!
//! # Generated Types
//!
//! | GraphQL | Fusabi |
//! |---------|--------|
//! | `type User { ... }` | `record User = { ... }` |
//! | `enum Status { ... }` | `type Status = Active \| Pending \| ...` |
//! | `union Result = A \| B` | `type Result = A of A \| B of B` |
//! | `input CreateUser { ... }` | `record CreateUserInput = { ... }` |
//! | `String!` | `string` (required) |
//! | `String` | `string option` (nullable) |
//! | `[String!]!` | `string list` |

pub mod introspection;

use crate::ast::{RecordTypeDef, DuTypeDef, VariantDef, TypeExpr, TypeDefinition};
use crate::type_provider::{
    TypeProvider, ProviderParams, Schema,
    GeneratedTypes, GeneratedModule, TypeGenerator, NamingStrategy,
    error::{ProviderError, ProviderResult},
};
use introspection::{
    IntrospectionResponse, IntrospectionSchema, FullType, TypeKind,
    Field, InputValue, EnumValue,
};

/// GraphQL type provider
pub struct GraphQLProvider {
    generator: TypeGenerator,
}

impl GraphQLProvider {
    pub fn new() -> Self {
        Self {
            generator: TypeGenerator::new(NamingStrategy::PascalCase),
        }
    }

    /// Generate types from introspection schema
    fn generate_from_schema(
        &self,
        schema: &IntrospectionSchema,
        namespace: &str,
    ) -> ProviderResult<GeneratedTypes> {
        let mut types_module = Vec::new();
        let mut inputs_module = Vec::new();
        let mut enums_module = Vec::new();

        for full_type in &schema.types {
            // Skip built-in types
            if full_type.is_builtin() {
                continue;
            }

            // Skip scalar types (handled specially)
            if full_type.is_scalar() {
                continue;
            }

            let type_name = match &full_type.name {
                Some(n) => n,
                None => continue,
            };

            match full_type.kind {
                TypeKind::Object | TypeKind::Interface => {
                    if let Some(type_def) = self.object_to_record(type_name, full_type)? {
                        types_module.push(type_def);
                    }
                }
                TypeKind::InputObject => {
                    if let Some(type_def) = self.input_to_record(type_name, full_type)? {
                        inputs_module.push(type_def);
                    }
                }
                TypeKind::Enum => {
                    if let Some(type_def) = self.enum_to_du(type_name, full_type)? {
                        enums_module.push(type_def);
                    }
                }
                TypeKind::Union => {
                    if let Some(type_def) = self.union_to_du(type_name, full_type)? {
                        types_module.push(type_def);
                    }
                }
                _ => {}
            }
        }

        let mut modules = Vec::new();

        if !types_module.is_empty() {
            modules.push(GeneratedModule {
                path: vec![namespace.to_string(), "Types".to_string()],
                types: types_module,
            });
        }

        if !inputs_module.is_empty() {
            modules.push(GeneratedModule {
                path: vec![namespace.to_string(), "Inputs".to_string()],
                types: inputs_module,
            });
        }

        if !enums_module.is_empty() {
            modules.push(GeneratedModule {
                path: vec![namespace.to_string(), "Enums".to_string()],
                types: enums_module,
            });
        }

        Ok(GeneratedTypes {
            modules,
            root_types: vec![],
        })
    }

    /// Convert GraphQL object type to Fusabi record
    fn object_to_record(
        &self,
        name: &str,
        full_type: &FullType,
    ) -> ProviderResult<Option<TypeDefinition>> {
        let fields = match &full_type.fields {
            Some(f) => f,
            None => return Ok(None),
        };

        if fields.is_empty() {
            return Ok(None);
        }

        let record_fields: Vec<(String, TypeExpr)> = fields.iter()
            .map(|field| {
                let field_name = to_camel_case(&field.name);
                let field_type = TypeExpr::Named(field.field_type.to_fusabi_type());
                (field_name, field_type)
            })
            .collect();

        Ok(Some(TypeDefinition::Record(RecordTypeDef {
            name: name.to_string(),
            fields: record_fields,
        })))
    }

    /// Convert GraphQL input type to Fusabi record
    fn input_to_record(
        &self,
        name: &str,
        full_type: &FullType,
    ) -> ProviderResult<Option<TypeDefinition>> {
        let input_fields = match &full_type.input_fields {
            Some(f) => f,
            None => return Ok(None),
        };

        if input_fields.is_empty() {
            return Ok(None);
        }

        let record_fields: Vec<(String, TypeExpr)> = input_fields.iter()
            .map(|field| {
                let field_name = to_camel_case(&field.name);
                let field_type = TypeExpr::Named(field.input_type.to_fusabi_type());
                (field_name, field_type)
            })
            .collect();

        Ok(Some(TypeDefinition::Record(RecordTypeDef {
            name: format!("{}Input", name.trim_end_matches("Input")),
            fields: record_fields,
        })))
    }

    /// Convert GraphQL enum to Fusabi discriminated union
    fn enum_to_du(
        &self,
        name: &str,
        full_type: &FullType,
    ) -> ProviderResult<Option<TypeDefinition>> {
        let enum_values = match &full_type.enum_values {
            Some(v) => v,
            None => return Ok(None),
        };

        if enum_values.is_empty() {
            return Ok(None);
        }

        let variants: Vec<VariantDef> = enum_values.iter()
            .filter(|v| !v.is_deprecated.unwrap_or(false))
            .map(|v| VariantDef {
                name: to_pascal_case(&v.name),
                fields: vec![],
            })
            .collect();

        Ok(Some(TypeDefinition::Du(DuTypeDef {
            name: name.to_string(),
            variants,
        })))
    }

    /// Convert GraphQL union to Fusabi discriminated union
    fn union_to_du(
        &self,
        name: &str,
        full_type: &FullType,
    ) -> ProviderResult<Option<TypeDefinition>> {
        let possible_types = match &full_type.possible_types {
            Some(t) => t,
            None => return Ok(None),
        };

        if possible_types.is_empty() {
            return Ok(None);
        }

        let variants: Vec<VariantDef> = possible_types.iter()
            .filter_map(|t| t.name.as_ref())
            .map(|type_name| VariantDef {
                name: type_name.clone(),
                fields: vec![TypeExpr::Named(type_name.clone())],
            })
            .collect();

        Ok(Some(TypeDefinition::Du(DuTypeDef {
            name: name.to_string(),
            variants,
        })))
    }
}

impl Default for GraphQLProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeProvider for GraphQLProvider {
    fn name(&self) -> &str {
        "GraphQLProvider"
    }

    fn resolve_schema(&self, source: &str, _params: &ProviderParams) -> ProviderResult<Schema> {
        let json_content = if source.starts_with("file://") {
            // Load from local schema file
            let path = source.strip_prefix("file://").unwrap();
            std::fs::read_to_string(path)
                .map_err(|e| ProviderError::IoError(format!("Failed to read {}: {}", path, e)))?
        } else if source.starts_with("http://") || source.starts_with("https://") {
            // TODO: Send introspection query to endpoint
            return Err(ProviderError::FetchError(
                "HTTP introspection not yet implemented. Save introspection result to file and use file://".to_string()
            ));
        } else {
            // Assume it's a file path
            std::fs::read_to_string(source)
                .map_err(|e| ProviderError::IoError(format!("Failed to read {}: {}", source, e)))?
        };

        Ok(Schema::Custom(json_content))
    }

    fn generate_types(&self, schema: &Schema, namespace: &str) -> ProviderResult<GeneratedTypes> {
        let content = match schema {
            Schema::Custom(s) => s,
            _ => return Err(ProviderError::ParseError("Expected Custom schema".to_string())),
        };

        // Parse introspection response
        let response: IntrospectionResponse = serde_json::from_str(content)
            .map_err(|e| ProviderError::ParseError(format!("Invalid introspection JSON: {}", e)))?;

        let schema_data = response.data
            .ok_or_else(|| ProviderError::ParseError("Missing 'data' in introspection response".to_string()))?;

        self.generate_from_schema(&schema_data.schema, namespace)
    }

    fn get_documentation(&self, _type_path: &str) -> Option<String> {
        // Could return GraphQL descriptions
        None
    }
}

fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for (i, c) in s.chars().enumerate() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap_or(c));
            capitalize_next = false;
        } else if i == 0 {
            result.push(c.to_lowercase().next().unwrap_or(c));
        } else {
            result.push(c);
        }
    }
    result
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

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INTROSPECTION: &str = r#"
{
  "data": {
    "__schema": {
      "queryType": { "name": "Query" },
      "mutationType": { "name": "Mutation" },
      "subscriptionType": null,
      "types": [
        {
          "kind": "OBJECT",
          "name": "User",
          "description": "A user in the system",
          "fields": [
            {
              "name": "id",
              "description": "Unique identifier",
              "args": [],
              "type": {
                "kind": "NON_NULL",
                "name": null,
                "ofType": { "kind": "SCALAR", "name": "ID", "ofType": null }
              },
              "isDeprecated": false,
              "deprecationReason": null
            },
            {
              "name": "name",
              "description": "User's display name",
              "args": [],
              "type": { "kind": "SCALAR", "name": "String", "ofType": null },
              "isDeprecated": false,
              "deprecationReason": null
            },
            {
              "name": "email",
              "description": "User's email",
              "args": [],
              "type": {
                "kind": "NON_NULL",
                "name": null,
                "ofType": { "kind": "SCALAR", "name": "String", "ofType": null }
              },
              "isDeprecated": false,
              "deprecationReason": null
            },
            {
              "name": "posts",
              "description": "User's posts",
              "args": [],
              "type": {
                "kind": "NON_NULL",
                "name": null,
                "ofType": {
                  "kind": "LIST",
                  "name": null,
                  "ofType": { "kind": "OBJECT", "name": "Post", "ofType": null }
                }
              },
              "isDeprecated": false,
              "deprecationReason": null
            }
          ],
          "inputFields": null,
          "interfaces": [],
          "enumValues": null,
          "possibleTypes": null
        },
        {
          "kind": "OBJECT",
          "name": "Post",
          "description": "A blog post",
          "fields": [
            {
              "name": "id",
              "args": [],
              "type": {
                "kind": "NON_NULL",
                "name": null,
                "ofType": { "kind": "SCALAR", "name": "ID", "ofType": null }
              },
              "isDeprecated": false
            },
            {
              "name": "title",
              "args": [],
              "type": {
                "kind": "NON_NULL",
                "name": null,
                "ofType": { "kind": "SCALAR", "name": "String", "ofType": null }
              },
              "isDeprecated": false
            }
          ],
          "inputFields": null,
          "interfaces": [],
          "enumValues": null,
          "possibleTypes": null
        },
        {
          "kind": "ENUM",
          "name": "Status",
          "description": "User status",
          "fields": null,
          "inputFields": null,
          "interfaces": null,
          "enumValues": [
            { "name": "ACTIVE", "description": null, "isDeprecated": false },
            { "name": "INACTIVE", "description": null, "isDeprecated": false },
            { "name": "PENDING", "description": null, "isDeprecated": false }
          ],
          "possibleTypes": null
        },
        {
          "kind": "INPUT_OBJECT",
          "name": "CreateUserInput",
          "description": "Input for creating a user",
          "fields": null,
          "inputFields": [
            {
              "name": "name",
              "type": {
                "kind": "NON_NULL",
                "name": null,
                "ofType": { "kind": "SCALAR", "name": "String", "ofType": null }
              },
              "defaultValue": null
            },
            {
              "name": "email",
              "type": {
                "kind": "NON_NULL",
                "name": null,
                "ofType": { "kind": "SCALAR", "name": "String", "ofType": null }
              },
              "defaultValue": null
            }
          ],
          "interfaces": null,
          "enumValues": null,
          "possibleTypes": null
        }
      ]
    }
  }
}
"#;

    #[test]
    fn test_provider_name() {
        let provider = GraphQLProvider::new();
        assert_eq!(provider.name(), "GraphQLProvider");
    }

    #[test]
    fn test_generate_types_from_introspection() {
        let provider = GraphQLProvider::new();
        let schema = Schema::Custom(SAMPLE_INTROSPECTION.to_string());

        let types = provider.generate_types(&schema, "API").unwrap();

        // Should have Types, Inputs, and Enums modules
        assert!(!types.modules.is_empty());

        // Find types module
        let types_module = types.modules.iter()
            .find(|m| m.path.contains(&"Types".to_string()));
        assert!(types_module.is_some(), "Should have Types module");

        let types_module = types_module.unwrap();

        // Should have User and Post types
        let type_names: Vec<&str> = types_module.types.iter()
            .filter_map(|t| match t {
                TypeDefinition::Record(r) => Some(r.name.as_str()),
                TypeDefinition::Du(d) => Some(d.name.as_str()),
            })
            .collect();

        assert!(type_names.contains(&"User"), "Should have User type");
        assert!(type_names.contains(&"Post"), "Should have Post type");
    }

    #[test]
    fn test_enum_generation() {
        let provider = GraphQLProvider::new();
        let schema = Schema::Custom(SAMPLE_INTROSPECTION.to_string());

        let types = provider.generate_types(&schema, "API").unwrap();

        let enums_module = types.modules.iter()
            .find(|m| m.path.contains(&"Enums".to_string()));
        assert!(enums_module.is_some(), "Should have Enums module");

        let status = enums_module.unwrap().types.iter()
            .find(|t| match t {
                TypeDefinition::Du(d) => d.name == "Status",
                _ => false,
            });
        assert!(status.is_some(), "Should have Status enum");

        if let Some(TypeDefinition::Du(du)) = status {
            let variant_names: Vec<&str> = du.variants.iter()
                .map(|v| v.name.as_str())
                .collect();
            assert!(variant_names.contains(&"Active"));
            assert!(variant_names.contains(&"Inactive"));
            assert!(variant_names.contains(&"Pending"));
        }
    }

    #[test]
    fn test_case_conversion() {
        assert_eq!(to_camel_case("user_name"), "userName");
        assert_eq!(to_camel_case("id"), "id");
        assert_eq!(to_pascal_case("ACTIVE_STATUS"), "ActiveStatus");
        assert_eq!(to_pascal_case("active"), "Active");
    }
}
