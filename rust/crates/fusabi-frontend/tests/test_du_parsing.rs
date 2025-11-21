//! Integration tests for Discriminated Union parsing (Issue #29 Layer 2)
//!
//! Tests cover:
//! - DU type definitions
//! - Variant construction expressions
//! - Variant patterns in match expressions
//! - Integration with other language features
//! - Error cases

use fusabi_frontend::ast::{DuTypeDef, Expr};
use fusabi_frontend::lexer::Lexer;
use fusabi_frontend::parser::Parser;

// Helper function to parse DU type definition
fn parse_du_typedef(input: &str) -> Result<DuTypeDef, fusabi_frontend::parser::ParseError> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    parser.parse_du_type_def()
}

// Helper function to parse expression
fn parse_expr(input: &str) -> Result<Expr, fusabi_frontend::parser::ParseError> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    parser.parse()
}

// ============================================================================
// DU Type Definition Tests - Simple Enums
// ============================================================================

#[test]
fn test_parse_du_simple_enum_two_variants() {
    let du = parse_du_typedef("type Bool = True | False").unwrap();
    assert_eq!(du.name, "Bool");
    assert_eq!(du.variant_count(), 2);
    assert!(du.is_simple_enum());
    assert_eq!(du.variant_names(), vec!["True", "False"]);
}

#[test]
fn test_parse_du_simple_enum_four_variants() {
    let du = parse_du_typedef("type Direction = Left | Right | Up | Down").unwrap();
    assert_eq!(du.name, "Direction");
    assert_eq!(du.variant_count(), 4);
    assert!(du.is_simple_enum());
}

#[test]
fn test_parse_du_single_variant() {
    let du = parse_du_typedef("type Unit = Unit").unwrap();
    assert_eq!(du.name, "Unit");
    assert_eq!(du.variant_count(), 1);
    assert!(du.is_simple_enum());
}

// ============================================================================
// DU Type Definition Tests - Variants with Fields
// ============================================================================

#[test]
fn test_parse_du_option_single_field() {
    let du = parse_du_typedef("type Option = Some of int | None").unwrap();
    assert_eq!(du.name, "Option");
    assert_eq!(du.variant_count(), 2);
    assert!(!du.is_simple_enum()); // Has one variant with fields

    let some_variant = du.find_variant("Some").unwrap();
    assert_eq!(some_variant.field_count(), 1);
    assert!(!some_variant.is_simple());

    let none_variant = du.find_variant("None").unwrap();
    assert_eq!(none_variant.field_count(), 0);
    assert!(none_variant.is_simple());
}

#[test]
fn test_parse_du_shape_multiple_variants_with_fields() {
    let du = parse_du_typedef("type Shape = Circle of float | Rectangle of float * float | Point")
        .unwrap();
    assert_eq!(du.name, "Shape");
    assert_eq!(du.variant_count(), 3);
    assert!(!du.is_simple_enum());

    let circle = du.find_variant("Circle").unwrap();
    assert_eq!(circle.field_count(), 1);

    let rectangle = du.find_variant("Rectangle").unwrap();
    assert_eq!(rectangle.field_count(), 2);

    let point = du.find_variant("Point").unwrap();
    assert_eq!(point.field_count(), 0);
}

#[test]
fn test_parse_du_result() {
    let du = parse_du_typedef("type Result = Ok of int | Error of string").unwrap();
    assert_eq!(du.name, "Result");
    assert_eq!(du.variant_count(), 2);

    let ok = du.find_variant("Ok").unwrap();
    assert_eq!(ok.field_count(), 1);

    let error = du.find_variant("Error").unwrap();
    assert_eq!(error.field_count(), 1);
}

// ============================================================================
// DU Type Definition Tests - Complex Field Types
// ============================================================================

#[test]
fn test_parse_du_tuple_field() {
    let du = parse_du_typedef("type Point = Point of int * int").unwrap();
    assert_eq!(du.name, "Point");
    assert_eq!(du.variant_count(), 1);

    let point = du.find_variant("Point").unwrap();
    assert_eq!(point.field_count(), 2);
}

#[test]
fn test_parse_du_three_field_tuple() {
    let du = parse_du_typedef("type Color = RGB of int * int * int").unwrap();
    assert_eq!(du.name, "Color");

    let rgb = du.find_variant("RGB").unwrap();
    assert_eq!(rgb.field_count(), 3);
}

#[test]
fn test_parse_du_mixed_variants() {
    let du = parse_du_typedef("type Mixed = Simple | Single of int | Multiple of string * bool")
        .unwrap();
    assert_eq!(du.name, "Mixed");
    assert_eq!(du.variant_count(), 3);
    assert!(!du.is_simple_enum());
}

#[test]
fn test_parse_du_multiple_variants_mixed_fields() {
    let du = parse_du_typedef("type Tree = Leaf | Node of int | Branch of int * int").unwrap();
    assert_eq!(du.name, "Tree");
    assert_eq!(du.variant_count(), 3);

    let leaf = du.find_variant("Leaf").unwrap();
    assert!(leaf.is_simple());

    let node = du.find_variant("Node").unwrap();
    assert_eq!(node.field_count(), 1);

    let branch = du.find_variant("Branch").unwrap();
    assert_eq!(branch.field_count(), 2);
}

// ============================================================================
// Variant Construction Expression Tests - Simple Variants
// ============================================================================

#[test]
fn test_parse_variant_construct_simple() {
    let expr = parse_expr("Left").unwrap();
    assert!(expr.is_variant_construct());

    match expr {
        Expr::VariantConstruct {
            variant, fields, ..
        } => {
            assert_eq!(variant, "Left");
            assert_eq!(fields.len(), 0);
        }
        _ => panic!("Expected VariantConstruct"),
    }
}

#[test]
fn test_parse_variant_construct_none() {
    let expr = parse_expr("None").unwrap();
    assert!(expr.is_variant_construct());

    match expr {
        Expr::VariantConstruct {
            variant, fields, ..
        } => {
            assert_eq!(variant, "None");
            assert_eq!(fields.len(), 0);
        }
        _ => panic!("Expected VariantConstruct"),
    }
}

// ============================================================================
// Variant Construction Expression Tests - With Arguments
// ============================================================================

#[test]
fn test_parse_variant_construct_some_int() {
    let expr = parse_expr("Some(42)").unwrap();
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
fn test_parse_variant_construct_circle() {
    let expr = parse_expr("Circle(5.0)").unwrap();
    assert!(expr.is_variant_construct());

    match expr {
        Expr::VariantConstruct {
            variant, fields, ..
        } => {
            assert_eq!(variant, "Circle");
            assert_eq!(fields.len(), 1);
        }
        _ => panic!("Expected VariantConstruct"),
    }
}

#[test]
fn test_parse_variant_construct_rectangle_two_args() {
    let expr = parse_expr("Rectangle(10.0, 20.0)").unwrap();
    assert!(expr.is_variant_construct());

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
fn test_parse_variant_construct_three_args() {
    let expr = parse_expr("RGB(255, 128, 0)").unwrap();
    assert!(expr.is_variant_construct());

    match expr {
        Expr::VariantConstruct {
            variant, fields, ..
        } => {
            assert_eq!(variant, "RGB");
            assert_eq!(fields.len(), 3);
        }
        _ => panic!("Expected VariantConstruct"),
    }
}

// ============================================================================
// Variant Construction Expression Tests - Complex Arguments
// ============================================================================

#[test]
fn test_parse_variant_construct_with_expression() {
    let expr = parse_expr("Some(1 + 2)").unwrap();
    assert!(expr.is_variant_construct());

    match expr {
        Expr::VariantConstruct {
            variant, fields, ..
        } => {
            assert_eq!(variant, "Some");
            assert_eq!(fields.len(), 1);
            assert!(fields[0].is_binop());
        }
        _ => panic!("Expected VariantConstruct"),
    }
}

#[test]
fn test_parse_variant_construct_with_variable() {
    let expr = parse_expr("Some(x)").unwrap();
    assert!(expr.is_variant_construct());

    match expr {
        Expr::VariantConstruct {
            variant, fields, ..
        } => {
            assert_eq!(variant, "Some");
            assert_eq!(fields.len(), 1);
            assert!(fields[0].is_var());
        }
        _ => panic!("Expected VariantConstruct"),
    }
}

#[test]
fn test_parse_variant_construct_nested() {
    let expr = parse_expr("Some(Some(42))").unwrap();
    assert!(expr.is_variant_construct());

    match expr {
        Expr::VariantConstruct {
            variant, fields, ..
        } => {
            assert_eq!(variant, "Some");
            assert_eq!(fields.len(), 1);
            assert!(fields[0].is_variant_construct());
        }
        _ => panic!("Expected VariantConstruct"),
    }
}

#[test]
fn test_parse_variant_construct_empty_args() {
    let expr = parse_expr("Unit()").unwrap();
    assert!(expr.is_variant_construct());

    match expr {
        Expr::VariantConstruct {
            variant, fields, ..
        } => {
            assert_eq!(variant, "Unit");
            assert_eq!(fields.len(), 0);
        }
        _ => panic!("Expected VariantConstruct"),
    }
}

#[test]
fn test_parse_variant_construct_trailing_comma() {
    let expr = parse_expr("RGB(255, 128, 0,)").unwrap();
    assert!(expr.is_variant_construct());

    match expr {
        Expr::VariantConstruct {
            variant, fields, ..
        } => {
            assert_eq!(variant, "RGB");
            assert_eq!(fields.len(), 3);
        }
        _ => panic!("Expected VariantConstruct"),
    }
}

// ============================================================================
// Variant Pattern Tests
// ============================================================================

#[test]
fn test_parse_variant_pattern_simple_in_match() {
    let expr = parse_expr("match x with | Left -> 1 | Right -> 2").unwrap();
    assert!(expr.is_match());

    let (_, arms) = expr.as_match().unwrap();
    assert_eq!(arms.len(), 2);
}

#[test]
fn test_parse_variant_pattern_with_args() {
    let expr = parse_expr("match opt with | Some(x) -> x | None -> 0").unwrap();
    assert!(expr.is_match());

    let (_, arms) = expr.as_match().unwrap();
    assert_eq!(arms.len(), 2);

    // First arm should be variant pattern with one sub-pattern
    assert!(arms[0].pattern.is_variant());
    let (variant, patterns) = arms[0].pattern.as_variant().unwrap();
    assert_eq!(variant, "Some");
    assert_eq!(patterns.len(), 1);
}

#[test]
fn test_parse_variant_pattern_rectangle() {
    let expr = parse_expr("match shape with | Rectangle(w, h) -> w * h").unwrap();
    assert!(expr.is_match());

    let (_, arms) = expr.as_match().unwrap();
    assert_eq!(arms.len(), 1);

    assert!(arms[0].pattern.is_variant());
    let (variant, patterns) = arms[0].pattern.as_variant().unwrap();
    assert_eq!(variant, "Rectangle");
    assert_eq!(patterns.len(), 2);
}

#[test]
fn test_parse_variant_pattern_nested() {
    let expr = parse_expr("match x with | Some(Some(y)) -> y | _ -> 0").unwrap();
    assert!(expr.is_match());

    let (_, arms) = expr.as_match().unwrap();
    assert_eq!(arms.len(), 2);

    assert!(arms[0].pattern.is_variant());
    let (variant, patterns) = arms[0].pattern.as_variant().unwrap();
    assert_eq!(variant, "Some");
    assert_eq!(patterns.len(), 1);
    assert!(patterns[0].is_variant());
}

#[test]
fn test_parse_variant_pattern_with_wildcard() {
    let expr = parse_expr("match shape with | Rectangle(_, h) -> h").unwrap();
    assert!(expr.is_match());

    let (_, arms) = expr.as_match().unwrap();
    assert_eq!(arms.len(), 1);

    assert!(arms[0].pattern.is_variant());
    let (variant, patterns) = arms[0].pattern.as_variant().unwrap();
    assert_eq!(variant, "Rectangle");
    assert_eq!(patterns.len(), 2);
    assert!(patterns[0].is_wildcard());
    assert!(patterns[1].is_var());
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_parse_variant_in_let() {
    let expr = parse_expr("let x = Some(42) in x").unwrap();
    assert!(expr.is_let());

    if let Expr::Let { value, .. } = &expr {
        assert!(value.is_variant_construct());
    }
}

#[test]
fn test_parse_variant_as_function_argument() {
    let expr = parse_expr("process Some(42)").unwrap();
    assert!(expr.is_app());

    if let Expr::App { arg, .. } = &expr {
        assert!(arg.is_variant_construct());
    }
}

#[test]
fn test_parse_variant_in_tuple() {
    let expr = parse_expr("(Some(1), None)").unwrap();
    assert!(expr.is_tuple());

    if let Expr::Tuple(elements) = &expr {
        assert_eq!(elements.len(), 2);
        assert!(elements[0].is_variant_construct());
        assert!(elements[1].is_variant_construct());
    }
}

#[test]
fn test_parse_variant_in_list() {
    let expr = parse_expr("[Some(1); None; Some(2)]").unwrap();
    assert!(expr.is_list());

    if let Expr::List(elements) = &expr {
        assert_eq!(elements.len(), 3);
        assert!(elements[0].is_variant_construct());
        assert!(elements[1].is_variant_construct());
        assert!(elements[2].is_variant_construct());
    }
}

#[test]
fn test_parse_variant_in_if() {
    let expr = parse_expr("if true then Some(1) else None").unwrap();
    assert!(expr.is_if());

    if let Expr::If {
        then_branch,
        else_branch,
        ..
    } = &expr
    {
        assert!(then_branch.is_variant_construct());
        assert!(else_branch.is_variant_construct());
    }
}

// ============================================================================
// Complex Integration Tests
// ============================================================================

#[test]
fn test_parse_option_map() {
    let expr = parse_expr("match opt with | Some(x) -> Some(x + 1) | None -> None").unwrap();
    assert!(expr.is_match());

    let (_, arms) = expr.as_match().unwrap();
    assert_eq!(arms.len(), 2);
    assert!(arms[0].body.is_variant_construct());
    assert!(arms[1].body.is_variant_construct());
}

#[test]
fn test_parse_tree_match() {
    let expr =
        parse_expr("match tree with | Leaf -> 0 | Node(x) -> x | Branch(x, y) -> x + y").unwrap();
    assert!(expr.is_match());

    let (_, arms) = expr.as_match().unwrap();
    assert_eq!(arms.len(), 3);
}

#[test]
fn test_parse_result_chain() {
    let expr = parse_expr("match res with | Ok(x) -> Ok(x * 2) | Error(e) -> Error(e)").unwrap();
    assert!(expr.is_match());

    let (_, arms) = expr.as_match().unwrap();
    assert_eq!(arms.len(), 2);
    assert!(arms[0].pattern.is_variant());
    assert!(arms[1].pattern.is_variant());
}

#[test]
fn test_parse_nested_match_with_variants() {
    let expr = parse_expr(
        "match outer with | Some(inner) -> (match inner with | Some(x) -> x | None -> 0) | None -> 0"
    ).unwrap();
    assert!(expr.is_match());

    let (_, arms) = expr.as_match().unwrap();
    assert_eq!(arms.len(), 2);
    assert!(arms[0].body.is_match());
}

// ============================================================================
// Error Cases
// ============================================================================

#[test]
fn test_parse_du_missing_eq() {
    let result = parse_du_typedef("type Option Some of int | None");
    assert!(result.is_err());
}

#[test]
fn test_parse_du_missing_variant_name() {
    let result = parse_du_typedef("type Option = | None");
    assert!(result.is_err());
}

#[test]
fn test_parse_du_invalid_field_type() {
    let result = parse_du_typedef("type Option = Some of | None");
    assert!(result.is_err());
}

// ============================================================================
// Display/Formatting Tests
// ============================================================================

#[test]
fn test_du_display_simple_enum() {
    let du = parse_du_typedef("type Bool = True | False").unwrap();
    assert_eq!(format!("{}", du), "type Bool = True | False");
}

#[test]
fn test_du_display_option() {
    let du = parse_du_typedef("type Option = Some of int | None").unwrap();
    assert_eq!(format!("{}", du), "type Option = Some of int | None");
}

#[test]
fn test_du_display_shape() {
    let du = parse_du_typedef("type Shape = Circle of float | Rectangle of float * float").unwrap();
    assert_eq!(
        format!("{}", du),
        "type Shape = Circle of float | Rectangle of float * float"
    );
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_parse_variant_single_letter_names() {
    let du = parse_du_typedef("type T = A | B | C").unwrap();
    assert_eq!(du.variant_count(), 3);
}

#[test]
fn test_parse_variant_long_names() {
    let du = parse_du_typedef("type Status = InitializingConnection | EstablishingHandshake | TransferringData | ClosingConnection").unwrap();
    assert_eq!(du.variant_count(), 4);
}

#[test]
fn test_parse_variant_camel_case() {
    let du = parse_du_typedef("type Event = ButtonClicked | TextChanged | WindowResized").unwrap();
    assert_eq!(du.variant_count(), 3);
}

#[test]
fn test_parse_variant_with_many_fields() {
    let du = parse_du_typedef("type Complex = Many of int * float * string * bool").unwrap();
    let many = du.find_variant("Many").unwrap();
    assert_eq!(many.field_count(), 4);
}
