//! Tests for Discriminated Union AST structures (Layer 1).
//!
//! Tests cover:
//! - VariantDef creation and helper methods
//! - DuTypeDef creation and helper methods
//! - TypeDefinition enum
//! - Pattern::Variant creation and helper methods
//! - Expr::VariantConstruct creation and helper methods
//! - Token::Of lexer support

use fusabi_frontend::ast::{DuTypeDef, Expr, Pattern, TypeDefinition, TypeExpr, VariantDef};
use fusabi_frontend::lexer::{Lexer, Token};

// ============================================================================
// VariantDef Tests
// ============================================================================

#[test]
fn test_variant_def_simple() {
    let variant = VariantDef::new_simple("None".to_string());

    assert_eq!(variant.name, "None");
    assert!(variant.fields.is_empty());
    assert!(variant.is_simple());
    assert_eq!(variant.field_count(), 0);
}

#[test]
fn test_variant_def_with_single_field() {
    let variant = VariantDef::new("Some".to_string(), vec![TypeExpr::Named("int".to_string())]);

    assert_eq!(variant.name, "Some");
    assert_eq!(variant.fields.len(), 1);
    assert!(!variant.is_simple());
    assert_eq!(variant.field_count(), 1);
}

#[test]
fn test_variant_def_with_multiple_fields() {
    let variant = VariantDef::new(
        "Rectangle".to_string(),
        vec![
            TypeExpr::Named("float".to_string()),
            TypeExpr::Named("float".to_string()),
        ],
    );

    assert_eq!(variant.name, "Rectangle");
    assert_eq!(variant.fields.len(), 2);
    assert!(!variant.is_simple());
    assert_eq!(variant.field_count(), 2);
}

#[test]
fn test_variant_def_with_tuple_type() {
    let variant = VariantDef::new(
        "Point".to_string(),
        vec![TypeExpr::Tuple(vec![
            TypeExpr::Named("float".to_string()),
            TypeExpr::Named("float".to_string()),
        ])],
    );

    assert_eq!(variant.name, "Point");
    assert_eq!(variant.fields.len(), 1);
    assert!(!variant.is_simple());
}

#[test]
fn test_variant_def_with_function_type() {
    let variant = VariantDef::new(
        "Handler".to_string(),
        vec![TypeExpr::Function(
            Box::new(TypeExpr::Named("int".to_string())),
            Box::new(TypeExpr::Named("string".to_string())),
        )],
    );

    assert_eq!(variant.name, "Handler");
    assert_eq!(variant.fields.len(), 1);
    assert!(!variant.is_simple());
}

#[test]
fn test_variant_def_display_simple() {
    let variant = VariantDef::new_simple("Left".to_string());
    assert_eq!(format!("{}", variant), "Left");
}

#[test]
fn test_variant_def_display_with_field() {
    let variant = VariantDef::new("Some".to_string(), vec![TypeExpr::Named("int".to_string())]);
    assert_eq!(format!("{}", variant), "Some of int");
}

#[test]
fn test_variant_def_display_with_multiple_fields() {
    let variant = VariantDef::new(
        "Rectangle".to_string(),
        vec![
            TypeExpr::Named("float".to_string()),
            TypeExpr::Named("float".to_string()),
        ],
    );
    assert_eq!(format!("{}", variant), "Rectangle of float * float");
}

// ============================================================================
// DuTypeDef Tests
// ============================================================================

#[test]
fn test_du_typedef_simple_enum() {
    let du = DuTypeDef {
        name: "Direction".to_string(),
        variants: vec![
            VariantDef::new_simple("Left".to_string()),
            VariantDef::new_simple("Right".to_string()),
            VariantDef::new_simple("Up".to_string()),
            VariantDef::new_simple("Down".to_string()),
        ],
    };

    assert_eq!(du.name, "Direction");
    assert_eq!(du.variant_count(), 4);
    assert!(du.is_simple_enum());

    let names = du.variant_names();
    assert_eq!(names, vec!["Left", "Right", "Up", "Down"]);
}

#[test]
fn test_du_typedef_option() {
    let du = DuTypeDef {
        name: "Option".to_string(),
        variants: vec![
            VariantDef::new("Some".to_string(), vec![TypeExpr::Named("int".to_string())]),
            VariantDef::new_simple("None".to_string()),
        ],
    };

    assert_eq!(du.name, "Option");
    assert_eq!(du.variant_count(), 2);
    assert!(!du.is_simple_enum()); // Has one variant with fields

    let names = du.variant_names();
    assert_eq!(names, vec!["Some", "None"]);
}

#[test]
fn test_du_typedef_find_variant() {
    let du = DuTypeDef {
        name: "Shape".to_string(),
        variants: vec![
            VariantDef::new(
                "Circle".to_string(),
                vec![TypeExpr::Named("float".to_string())],
            ),
            VariantDef::new(
                "Rectangle".to_string(),
                vec![
                    TypeExpr::Named("float".to_string()),
                    TypeExpr::Named("float".to_string()),
                ],
            ),
        ],
    };

    let circle = du.find_variant("Circle");
    assert!(circle.is_some());
    assert_eq!(circle.unwrap().name, "Circle");

    let triangle = du.find_variant("Triangle");
    assert!(triangle.is_none());
}

#[test]
fn test_du_typedef_display() {
    let du = DuTypeDef {
        name: "Option".to_string(),
        variants: vec![
            VariantDef::new("Some".to_string(), vec![TypeExpr::Named("int".to_string())]),
            VariantDef::new_simple("None".to_string()),
        ],
    };

    assert_eq!(format!("{}", du), "type Option = Some of int | None");
}

#[test]
fn test_du_typedef_display_simple_enum() {
    let du = DuTypeDef {
        name: "Direction".to_string(),
        variants: vec![
            VariantDef::new_simple("Left".to_string()),
            VariantDef::new_simple("Right".to_string()),
        ],
    };

    assert_eq!(format!("{}", du), "type Direction = Left | Right");
}

// ============================================================================
// TypeDefinition Tests
// ============================================================================

#[test]
fn test_type_definition_du() {
    let du = DuTypeDef {
        name: "Option".to_string(),
        variants: vec![
            VariantDef::new("Some".to_string(), vec![TypeExpr::Named("int".to_string())]),
            VariantDef::new_simple("None".to_string()),
        ],
    };

    let typedef = TypeDefinition::Du(du);

    match typedef {
        TypeDefinition::Du(du) => {
            assert_eq!(du.name, "Option");
            assert_eq!(du.variant_count(), 2);
        }
        _ => panic!("Expected TypeDefinition::Du"),
    }
}

#[test]
fn test_type_definition_display_du() {
    let du = DuTypeDef {
        name: "Bool".to_string(),
        variants: vec![
            VariantDef::new_simple("True".to_string()),
            VariantDef::new_simple("False".to_string()),
        ],
    };

    let typedef = TypeDefinition::Du(du);
    assert_eq!(format!("{}", typedef), "type Bool = True | False");
}

// ============================================================================
// Pattern::Variant Tests
// ============================================================================

#[test]
fn test_pattern_variant_simple() {
    let pattern = Pattern::Variant {
        variant: "None".to_string(),
        patterns: vec![],
    };

    assert!(pattern.is_variant());
    assert!(!pattern.is_wildcard());
    assert!(!pattern.is_var());

    match pattern.as_variant() {
        Some((name, patterns)) => {
            assert_eq!(name, "None");
            assert!(patterns.is_empty());
        }
        None => panic!("Expected variant pattern"),
    }
}

#[test]
fn test_pattern_variant_with_nested() {
    let pattern = Pattern::Variant {
        variant: "Some".to_string(),
        patterns: vec![Pattern::Var("x".to_string())],
    };

    assert!(pattern.is_variant());

    match pattern.as_variant() {
        Some((name, patterns)) => {
            assert_eq!(name, "Some");
            assert_eq!(patterns.len(), 1);
            assert!(patterns[0].is_var());
        }
        None => panic!("Expected variant pattern"),
    }
}

#[test]
fn test_pattern_variant_with_multiple_nested() {
    let pattern = Pattern::Variant {
        variant: "Rectangle".to_string(),
        patterns: vec![
            Pattern::Var("width".to_string()),
            Pattern::Var("height".to_string()),
        ],
    };

    match pattern.as_variant() {
        Some((name, patterns)) => {
            assert_eq!(name, "Rectangle");
            assert_eq!(patterns.len(), 2);
        }
        None => panic!("Expected variant pattern"),
    }
}

#[test]
fn test_pattern_variant_display_simple() {
    let pattern = Pattern::Variant {
        variant: "Left".to_string(),
        patterns: vec![],
    };

    assert_eq!(format!("{}", pattern), "Left");
}

#[test]
fn test_pattern_variant_display_with_nested() {
    let pattern = Pattern::Variant {
        variant: "Some".to_string(),
        patterns: vec![Pattern::Var("x".to_string())],
    };

    assert_eq!(format!("{}", pattern), "Some(x)");
}

#[test]
fn test_pattern_variant_display_with_multiple_nested() {
    let pattern = Pattern::Variant {
        variant: "Rectangle".to_string(),
        patterns: vec![Pattern::Var("w".to_string()), Pattern::Var("h".to_string())],
    };

    assert_eq!(format!("{}", pattern), "Rectangle(w, h)");
}

// ============================================================================
// Expr::VariantConstruct Tests
// ============================================================================

#[test]
fn test_expr_variant_construct_simple() {
    let expr = Expr::VariantConstruct {
        type_name: String::new(), // Filled by typechecker
        variant: "None".to_string(),
        fields: vec![],
    };

    assert!(expr.is_variant_construct());
    assert!(!expr.is_literal());
    assert!(!expr.is_var());
}

#[test]
fn test_expr_variant_construct_with_field() {
    let expr = Expr::VariantConstruct {
        type_name: String::new(),
        variant: "Some".to_string(),
        fields: vec![Box::new(Expr::Lit(fusabi_frontend::ast::Literal::Int(42)))],
    };

    assert!(expr.is_variant_construct());

    match expr {
        Expr::VariantConstruct {
            variant, fields, ..
        } => {
            assert_eq!(variant, "Some");
            assert_eq!(fields.len(), 1);
        }
        _ => panic!("Expected VariantConstruct"),
    }
}

#[test]
fn test_expr_variant_construct_with_multiple_fields() {
    let expr = Expr::VariantConstruct {
        type_name: String::new(),
        variant: "Rectangle".to_string(),
        fields: vec![
            Box::new(Expr::Lit(fusabi_frontend::ast::Literal::Float(10.0))),
            Box::new(Expr::Lit(fusabi_frontend::ast::Literal::Float(20.0))),
        ],
    };

    match expr {
        Expr::VariantConstruct {
            variant, fields, ..
        } => {
            assert_eq!(variant, "Rectangle");
            assert_eq!(fields.len(), 2);
        }
        _ => panic!("Expected VariantConstruct"),
    }
}

#[test]
fn test_expr_variant_construct_display_simple() {
    let expr = Expr::VariantConstruct {
        type_name: String::new(),
        variant: "Left".to_string(),
        fields: vec![],
    };

    assert_eq!(format!("{}", expr), "Left");
}

#[test]
fn test_expr_variant_construct_display_with_field() {
    let expr = Expr::VariantConstruct {
        type_name: String::new(),
        variant: "Some".to_string(),
        fields: vec![Box::new(Expr::Lit(fusabi_frontend::ast::Literal::Int(42)))],
    };

    assert_eq!(format!("{}", expr), "Some(42)");
}

#[test]
fn test_expr_variant_construct_display_with_multiple_fields() {
    let expr = Expr::VariantConstruct {
        type_name: String::new(),
        variant: "Rectangle".to_string(),
        fields: vec![
            Box::new(Expr::Lit(fusabi_frontend::ast::Literal::Float(10.0))),
            Box::new(Expr::Lit(fusabi_frontend::ast::Literal::Float(20.0))),
        ],
    };

    assert_eq!(format!("{}", expr), "Rectangle(10, 20)");
}

// ============================================================================
// Token::Of Lexer Tests
// ============================================================================

#[test]
fn test_lexer_of_keyword() {
    let mut lexer = Lexer::new("of");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 2); // of + EOF
    assert_eq!(tokens[0].token, Token::Of);
}

#[test]
fn test_lexer_of_in_variant() {
    let mut lexer = Lexer::new("Some of int");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 4); // Some + of + int + EOF
    assert_eq!(tokens[0].token, Token::Ident("Some".to_string()));
    assert_eq!(tokens[1].token, Token::Of);
    assert_eq!(tokens[2].token, Token::Ident("int".to_string()));
}

#[test]
fn test_lexer_of_in_du_definition() {
    let mut lexer = Lexer::new("type Option = Some of int | None");
    let tokens = lexer.tokenize().unwrap();

    let of_token = tokens.iter().find(|t| t.token == Token::Of);
    assert!(of_token.is_some());
}

#[test]
fn test_token_of_display() {
    let token = Token::Of;
    assert_eq!(format!("{}", token), "of");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_du_with_all_variant_types() {
    let du = DuTypeDef {
        name: "Mixed".to_string(),
        variants: vec![
            VariantDef::new_simple("Simple".to_string()),
            VariantDef::new(
                "Single".to_string(),
                vec![TypeExpr::Named("int".to_string())],
            ),
            VariantDef::new(
                "Multiple".to_string(),
                vec![
                    TypeExpr::Named("string".to_string()),
                    TypeExpr::Named("bool".to_string()),
                ],
            ),
        ],
    };

    assert_eq!(du.variant_count(), 3);
    assert!(!du.is_simple_enum());
    assert_eq!(
        format!("{}", du),
        "type Mixed = Simple | Single of int | Multiple of string * bool"
    );
}

#[test]
fn test_pattern_variant_nested_tuple() {
    let pattern = Pattern::Variant {
        variant: "Point".to_string(),
        patterns: vec![Pattern::Tuple(vec![
            Pattern::Var("x".to_string()),
            Pattern::Var("y".to_string()),
        ])],
    };

    assert_eq!(format!("{}", pattern), "Point((x, y))");
}

#[test]
fn test_expr_variant_nested_tuple() {
    let expr = Expr::VariantConstruct {
        type_name: String::new(),
        variant: "Point".to_string(),
        fields: vec![Box::new(Expr::Tuple(vec![
            Expr::Lit(fusabi_frontend::ast::Literal::Float(1.0)),
            Expr::Lit(fusabi_frontend::ast::Literal::Float(2.0)),
        ]))],
    };

    assert_eq!(format!("{}", expr), "Point((1, 2))");
}
