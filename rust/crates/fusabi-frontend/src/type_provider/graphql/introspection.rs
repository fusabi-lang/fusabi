//! GraphQL Introspection Types
//!
//! Models the GraphQL introspection schema response.
//! See: https://spec.graphql.org/October2021/#sec-Introspection

use serde::Deserialize;

/// Root introspection response
#[derive(Debug, Clone, Deserialize)]
pub struct IntrospectionResponse {
    pub data: Option<IntrospectionData>,
}

/// The __schema field
#[derive(Debug, Clone, Deserialize)]
pub struct IntrospectionData {
    #[serde(rename = "__schema")]
    pub schema: IntrospectionSchema,
}

/// Full schema introspection result
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntrospectionSchema {
    pub query_type: Option<TypeRef>,
    pub mutation_type: Option<TypeRef>,
    pub subscription_type: Option<TypeRef>,
    pub types: Vec<FullType>,
    pub directives: Option<Vec<Directive>>,
}

/// Reference to a type by name
#[derive(Debug, Clone, Deserialize)]
pub struct TypeRef {
    pub name: Option<String>,
    pub kind: Option<TypeKind>,
    #[serde(rename = "ofType")]
    pub of_type: Option<Box<TypeRef>>,
}

impl TypeRef {
    /// Get the base type name, unwrapping NON_NULL and LIST wrappers
    pub fn base_name(&self) -> Option<&str> {
        match self.kind {
            Some(TypeKind::NonNull) | Some(TypeKind::List) => {
                self.of_type.as_ref().and_then(|t| t.base_name())
            }
            _ => self.name.as_deref(),
        }
    }

    /// Check if this type is non-null (required)
    pub fn is_non_null(&self) -> bool {
        matches!(self.kind, Some(TypeKind::NonNull))
    }

    /// Check if this type is a list
    pub fn is_list(&self) -> bool {
        match self.kind {
            Some(TypeKind::List) => true,
            Some(TypeKind::NonNull) => {
                self.of_type.as_ref().map(|t| t.is_list()).unwrap_or(false)
            }
            _ => false,
        }
    }

    /// Convert to Fusabi type string
    pub fn to_fusabi_type(&self) -> String {
        match self.kind {
            Some(TypeKind::NonNull) => {
                self.of_type.as_ref()
                    .map(|t| t.to_fusabi_type_inner())
                    .unwrap_or_else(|| "any".to_string())
            }
            _ => {
                let inner = self.to_fusabi_type_inner();
                format!("{} option", inner)
            }
        }
    }

    fn to_fusabi_type_inner(&self) -> String {
        match self.kind {
            Some(TypeKind::List) => {
                let item = self.of_type.as_ref()
                    .map(|t| t.to_fusabi_type())
                    .unwrap_or_else(|| "any".to_string());
                format!("{} list", item)
            }
            Some(TypeKind::NonNull) => {
                self.of_type.as_ref()
                    .map(|t| t.to_fusabi_type_inner())
                    .unwrap_or_else(|| "any".to_string())
            }
            _ => {
                let name = self.name.as_deref().unwrap_or("any");
                graphql_scalar_to_fusabi(name)
            }
        }
    }
}

/// GraphQL type kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TypeKind {
    Scalar,
    Object,
    Interface,
    Union,
    Enum,
    InputObject,
    List,
    NonNull,
}

/// Full type definition from introspection
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullType {
    pub kind: TypeKind,
    pub name: Option<String>,
    pub description: Option<String>,
    pub fields: Option<Vec<Field>>,
    pub input_fields: Option<Vec<InputValue>>,
    pub interfaces: Option<Vec<TypeRef>>,
    pub enum_values: Option<Vec<EnumValue>>,
    pub possible_types: Option<Vec<TypeRef>>,
}

impl FullType {
    /// Check if this is a built-in GraphQL type
    pub fn is_builtin(&self) -> bool {
        self.name.as_ref()
            .map(|n| n.starts_with("__"))
            .unwrap_or(false)
    }

    /// Check if this is a scalar type
    pub fn is_scalar(&self) -> bool {
        matches!(self.kind, TypeKind::Scalar)
    }
}

/// Object/Interface field
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub args: Vec<InputValue>,
    #[serde(rename = "type")]
    pub field_type: TypeRef,
    pub is_deprecated: Option<bool>,
    pub deprecation_reason: Option<String>,
}

/// Input value (argument or input field)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputValue {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub input_type: TypeRef,
    pub default_value: Option<String>,
}

/// Enum value
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub name: String,
    pub description: Option<String>,
    pub is_deprecated: Option<bool>,
    pub deprecation_reason: Option<String>,
}

/// Directive definition
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Directive {
    pub name: String,
    pub description: Option<String>,
    pub locations: Vec<String>,
    pub args: Vec<InputValue>,
}

/// Convert GraphQL scalar to Fusabi type
pub fn graphql_scalar_to_fusabi(scalar: &str) -> String {
    match scalar {
        "String" | "ID" => "string".to_string(),
        "Int" => "int".to_string(),
        "Float" => "float".to_string(),
        "Boolean" => "bool".to_string(),
        // Custom scalars become strings by default
        other => other.to_string(),
    }
}

/// Standard GraphQL introspection query
pub const INTROSPECTION_QUERY: &str = r#"
query IntrospectionQuery {
  __schema {
    queryType { name }
    mutationType { name }
    subscriptionType { name }
    types {
      kind
      name
      description
      fields(includeDeprecated: true) {
        name
        description
        args {
          name
          description
          type {
            kind
            name
            ofType {
              kind
              name
              ofType {
                kind
                name
                ofType {
                  kind
                  name
                }
              }
            }
          }
          defaultValue
        }
        type {
          kind
          name
          ofType {
            kind
            name
            ofType {
              kind
              name
              ofType {
                kind
                name
              }
            }
          }
        }
        isDeprecated
        deprecationReason
      }
      inputFields {
        name
        description
        type {
          kind
          name
          ofType {
            kind
            name
            ofType {
              kind
              name
            }
          }
        }
        defaultValue
      }
      interfaces {
        kind
        name
      }
      enumValues(includeDeprecated: true) {
        name
        description
        isDeprecated
        deprecationReason
      }
      possibleTypes {
        kind
        name
      }
    }
  }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_ref_base_name() {
        let simple = TypeRef {
            name: Some("String".to_string()),
            kind: Some(TypeKind::Scalar),
            of_type: None,
        };
        assert_eq!(simple.base_name(), Some("String"));

        let non_null = TypeRef {
            name: None,
            kind: Some(TypeKind::NonNull),
            of_type: Some(Box::new(simple.clone())),
        };
        assert_eq!(non_null.base_name(), Some("String"));
    }

    #[test]
    fn test_type_ref_to_fusabi() {
        let string_type = TypeRef {
            name: Some("String".to_string()),
            kind: Some(TypeKind::Scalar),
            of_type: None,
        };
        assert_eq!(string_type.to_fusabi_type(), "string option");

        let non_null_string = TypeRef {
            name: None,
            kind: Some(TypeKind::NonNull),
            of_type: Some(Box::new(string_type)),
        };
        assert_eq!(non_null_string.to_fusabi_type(), "string");
    }

    #[test]
    fn test_graphql_scalar_mapping() {
        assert_eq!(graphql_scalar_to_fusabi("String"), "string");
        assert_eq!(graphql_scalar_to_fusabi("Int"), "int");
        assert_eq!(graphql_scalar_to_fusabi("Float"), "float");
        assert_eq!(graphql_scalar_to_fusabi("Boolean"), "bool");
        assert_eq!(graphql_scalar_to_fusabi("ID"), "string");
    }
}
