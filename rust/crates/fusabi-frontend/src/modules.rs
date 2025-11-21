//! Module System for Fusabi Mini-F#
//!
//! This module implements the module system for organizing code in Fusabi,
//! supporting module definitions, imports (open statements), and qualified names.
//!
//! # Features
//!
//! - Module definitions with nested modules
//! - Open imports for bringing module bindings into scope
//! - Qualified name resolution (e.g., Math.add)
//! - Name conflict detection
//! - Type environment tracking per module
//!
//! # Example
//!
//! ```fsharp
//! module Math =
//!     let add x y = x + y
//!     let multiply x y = x * y
//!
//! open Math
//!
//! let result = add 5 10  // Uses Math.add via open
//! let result2 = Math.multiply 3 4  // Qualified access
//! ```

use crate::ast::{DuTypeDef, Expr, RecordTypeDef};
use crate::types::TypeEnv;
use std::collections::HashMap;

/// Module registry for name resolution
///
/// Maintains a registry of all modules and their exported bindings,
/// enabling both qualified and unqualified (via open) name lookups.
#[derive(Debug, Clone)]
pub struct ModuleRegistry {
    /// Map from module name to Module
    modules: HashMap<String, Module>,
}

/// A compiled module with its bindings and type environment
#[derive(Debug, Clone)]
pub struct Module {
    /// Module name
    pub name: String,
    /// Value bindings (functions and constants)
    pub bindings: HashMap<String, Expr>,
    /// Type definitions (records and discriminated unions)
    pub types: HashMap<String, TypeDefinition>,
    /// Type environment for this module
    pub type_env: TypeEnv,
}

/// Type definition exported by a module
#[derive(Debug, Clone)]
pub enum TypeDefinition {
    /// Record type definition
    Record(RecordTypeDef),
    /// Discriminated union type definition
    Du(DuTypeDef),
}

impl ModuleRegistry {
    /// Create a new empty module registry
    pub fn new() -> Self {
        ModuleRegistry {
            modules: HashMap::new(),
        }
    }

    /// Register a module with its bindings and types
    ///
    /// # Arguments
    ///
    /// * `name` - Module name
    /// * `bindings` - Map of value bindings (name -> expression)
    /// * `types` - Map of type definitions (type name -> definition)
    pub fn register_module(
        &mut self,
        name: String,
        bindings: HashMap<String, Expr>,
        types: HashMap<String, TypeDefinition>,
    ) {
        let module = Module {
            name: name.clone(),
            bindings,
            types,
            type_env: TypeEnv::new(),
        };
        self.modules.insert(name, module);
    }

    /// Resolve a qualified name (e.g., "Math.add")
    ///
    /// # Returns
    ///
    /// The expression bound to the qualified name, or None if not found.
    pub fn resolve_qualified(&self, module_name: &str, binding_name: &str) -> Option<&Expr> {
        self.modules
            .get(module_name)
            .and_then(|m| m.bindings.get(binding_name))
    }

    /// Get all bindings from a module (for "open" imports)
    ///
    /// # Returns
    ///
    /// A reference to all bindings in the module, or None if module not found.
    pub fn get_module_bindings(&self, module_name: &str) -> Option<&HashMap<String, Expr>> {
        self.modules.get(module_name).map(|m| &m.bindings)
    }

    /// Get all type definitions from a module
    ///
    /// # Returns
    ///
    /// A reference to all type definitions in the module, or None if module not found.
    pub fn get_module_types(&self, module_name: &str) -> Option<&HashMap<String, TypeDefinition>> {
        self.modules.get(module_name).map(|m| &m.types)
    }

    /// Check if a module exists
    pub fn has_module(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }

    /// Get a module by name
    pub fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.get(name)
    }

    /// List all registered module names
    pub fn module_names(&self) -> Vec<&str> {
        self.modules.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Module path for nested modules
///
/// Represents a path like ["Geometry", "Point"] for accessing nested modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulePath {
    /// Path components (e.g., ["Math", "Geometry"])
    pub components: Vec<String>,
}

impl ModulePath {
    /// Create a new module path from components
    pub fn new(components: Vec<String>) -> Self {
        ModulePath { components }
    }

    /// Create a single-component path
    pub fn single(name: String) -> Self {
        ModulePath {
            components: vec![name],
        }
    }

    /// Get the full qualified name (e.g., "Math.Geometry")
    pub fn qualified_name(&self) -> String {
        self.components.join(".")
    }

    /// Get the last component (e.g., "Geometry" from "Math.Geometry")
    pub fn last(&self) -> Option<&str> {
        self.components.last().map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Literal, TypeExpr};

    #[test]
    fn test_module_registry_new() {
        let registry = ModuleRegistry::new();
        assert_eq!(registry.module_names().len(), 0);
    }

    #[test]
    fn test_register_and_resolve_module() {
        let mut registry = ModuleRegistry::new();

        // Create a simple Math module
        let mut bindings = HashMap::new();
        bindings.insert(
            "add".to_string(),
            Expr::Lambda {
                param: "x".to_string(),
                body: Box::new(Expr::Lambda {
                    param: "y".to_string(),
                    body: Box::new(Expr::BinOp {
                        op: crate::ast::BinOp::Add,
                        left: Box::new(Expr::Var("x".to_string())),
                        right: Box::new(Expr::Var("y".to_string())),
                    }),
                }),
            },
        );

        registry.register_module("Math".to_string(), bindings, HashMap::new());

        // Verify module exists
        assert!(registry.has_module("Math"));
        assert_eq!(registry.module_names(), vec!["Math"]);

        // Resolve qualified name
        let expr = registry.resolve_qualified("Math", "add");
        assert!(expr.is_some());
        assert!(expr.unwrap().is_lambda());
    }

    #[test]
    fn test_get_module_bindings() {
        let mut registry = ModuleRegistry::new();

        let mut bindings = HashMap::new();
        bindings.insert("x".to_string(), Expr::Lit(Literal::Int(42)));
        bindings.insert("y".to_string(), Expr::Lit(Literal::Int(100)));

        registry.register_module("Test".to_string(), bindings, HashMap::new());

        let module_bindings = registry.get_module_bindings("Test");
        assert!(module_bindings.is_some());
        assert_eq!(module_bindings.unwrap().len(), 2);
    }

    #[test]
    fn test_resolve_nonexistent_module() {
        let registry = ModuleRegistry::new();
        assert!(!registry.has_module("Nonexistent"));
        assert!(registry.resolve_qualified("Nonexistent", "add").is_none());
    }

    #[test]
    fn test_module_path() {
        let path = ModulePath::new(vec!["Math".to_string(), "Geometry".to_string()]);
        assert_eq!(path.qualified_name(), "Math.Geometry");
        assert_eq!(path.last(), Some("Geometry"));

        let single = ModulePath::single("Math".to_string());
        assert_eq!(single.qualified_name(), "Math");
        assert_eq!(single.last(), Some("Math"));
    }

    #[test]
    fn test_module_with_types() {
        let mut registry = ModuleRegistry::new();

        let mut types = HashMap::new();
        types.insert(
            "Person".to_string(),
            TypeDefinition::Record(RecordTypeDef {
                name: "Person".to_string(),
                fields: vec![
                    ("name".to_string(), TypeExpr::Named("string".to_string())),
                    ("age".to_string(), TypeExpr::Named("int".to_string())),
                ],
            }),
        );

        registry.register_module("Data".to_string(), HashMap::new(), types);

        let module_types = registry.get_module_types("Data");
        assert!(module_types.is_some());
        assert_eq!(module_types.unwrap().len(), 1);
        assert!(module_types.unwrap().contains_key("Person"));
    }
}
