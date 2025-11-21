//! Integration tests for tuple functionality
//! Tests the complete pipeline: AST -> Compiler -> VM execution

use fusabi_frontend::ast::{BinOp, Expr, Literal};
use fusabi_frontend::compiler::Compiler;
use fusabi_vm::value::Value;
use fusabi_vm::vm::Vm;

#[test]
fn test_tuple_empty() {
    // Test: ()
    let expr = Expr::Tuple(vec![]);
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    assert_eq!(result, Value::Tuple(vec![]));
}

#[test]
fn test_tuple_pair() {
    // Test: (1, 2)
    let expr = Expr::Tuple(vec![Expr::Lit(Literal::Int(1)), Expr::Lit(Literal::Int(2))]);
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    assert_eq!(result, Value::Tuple(vec![Value::Int(1), Value::Int(2)]));
}

#[test]
fn test_tuple_mixed_types() {
    // Test: (42, "hello", true)
    let expr = Expr::Tuple(vec![
        Expr::Lit(Literal::Int(42)),
        Expr::Lit(Literal::Str("hello".to_string())),
        Expr::Lit(Literal::Bool(true)),
    ]);
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    assert_eq!(
        result,
        Value::Tuple(vec![
            Value::Int(42),
            Value::Str("hello".to_string()),
            Value::Bool(true)
        ])
    );
}

#[test]
fn test_tuple_in_let_binding() {
    // Test: let pair = (1, 2) in pair
    let expr = Expr::Let {
        name: "pair".to_string(),
        value: Box::new(Expr::Tuple(vec![
            Expr::Lit(Literal::Int(1)),
            Expr::Lit(Literal::Int(2)),
        ])),
        body: Box::new(Expr::Var("pair".to_string())),
    };
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    assert_eq!(result, Value::Tuple(vec![Value::Int(1), Value::Int(2)]));
}

#[test]
fn test_tuple_with_expressions() {
    // Test: (1 + 2, 3 * 4)
    let expr = Expr::Tuple(vec![
        Expr::BinOp {
            op: BinOp::Add,
            left: Box::new(Expr::Lit(Literal::Int(1))),
            right: Box::new(Expr::Lit(Literal::Int(2))),
        },
        Expr::BinOp {
            op: BinOp::Mul,
            left: Box::new(Expr::Lit(Literal::Int(3))),
            right: Box::new(Expr::Lit(Literal::Int(4))),
        },
    ]);
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    assert_eq!(result, Value::Tuple(vec![Value::Int(3), Value::Int(12)]));
}

#[test]
fn test_tuple_nested() {
    // Test: (1, (2, 3))
    let expr = Expr::Tuple(vec![
        Expr::Lit(Literal::Int(1)),
        Expr::Tuple(vec![Expr::Lit(Literal::Int(2)), Expr::Lit(Literal::Int(3))]),
    ]);
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    assert_eq!(
        result,
        Value::Tuple(vec![
            Value::Int(1),
            Value::Tuple(vec![Value::Int(2), Value::Int(3)])
        ])
    );
}

#[test]
fn test_tuple_deeply_nested() {
    // Test: ((1, 2), (3, 4))
    let expr = Expr::Tuple(vec![
        Expr::Tuple(vec![Expr::Lit(Literal::Int(1)), Expr::Lit(Literal::Int(2))]),
        Expr::Tuple(vec![Expr::Lit(Literal::Int(3)), Expr::Lit(Literal::Int(4))]),
    ]);
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    assert_eq!(
        result,
        Value::Tuple(vec![
            Value::Tuple(vec![Value::Int(1), Value::Int(2)]),
            Value::Tuple(vec![Value::Int(3), Value::Int(4)])
        ])
    );
}

#[test]
fn test_tuple_with_variables() {
    // Test: let x = 1 in let y = 2 in (x, y)
    let expr = Expr::Let {
        name: "x".to_string(),
        value: Box::new(Expr::Lit(Literal::Int(1))),
        body: Box::new(Expr::Let {
            name: "y".to_string(),
            value: Box::new(Expr::Lit(Literal::Int(2))),
            body: Box::new(Expr::Tuple(vec![
                Expr::Var("x".to_string()),
                Expr::Var("y".to_string()),
            ])),
        }),
    };
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    assert_eq!(result, Value::Tuple(vec![Value::Int(1), Value::Int(2)]));
}

#[test]
fn test_tuple_equality() {
    // Test: (1, 2) == (1, 2)
    let expr = Expr::BinOp {
        op: BinOp::Eq,
        left: Box::new(Expr::Tuple(vec![
            Expr::Lit(Literal::Int(1)),
            Expr::Lit(Literal::Int(2)),
        ])),
        right: Box::new(Expr::Tuple(vec![
            Expr::Lit(Literal::Int(1)),
            Expr::Lit(Literal::Int(2)),
        ])),
    };
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    assert_eq!(result, Value::Bool(true));
}

#[test]
fn test_tuple_inequality() {
    // Test: (1, 2) != (1, 3)
    let expr = Expr::BinOp {
        op: BinOp::Neq,
        left: Box::new(Expr::Tuple(vec![
            Expr::Lit(Literal::Int(1)),
            Expr::Lit(Literal::Int(2)),
        ])),
        right: Box::new(Expr::Tuple(vec![
            Expr::Lit(Literal::Int(1)),
            Expr::Lit(Literal::Int(3)),
        ])),
    };
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    assert_eq!(result, Value::Bool(true));
}

#[test]
fn test_tuple_large() {
    // Test: (1, 2, 3, 4, 5, 6, 7, 8)
    let expr = Expr::Tuple(
        (1..=8)
            .map(|i| Expr::Lit(Literal::Int(i)))
            .collect::<Vec<_>>(),
    );
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    let expected = Value::Tuple((1..=8).map(Value::Int).collect::<Vec<_>>());
    assert_eq!(result, expected);
}

#[test]
fn test_tuple_in_conditional() {
    // Test: if true then (1, 2) else (3, 4)
    let expr = Expr::If {
        cond: Box::new(Expr::Lit(Literal::Bool(true))),
        then_branch: Box::new(Expr::Tuple(vec![
            Expr::Lit(Literal::Int(1)),
            Expr::Lit(Literal::Int(2)),
        ])),
        else_branch: Box::new(Expr::Tuple(vec![
            Expr::Lit(Literal::Int(3)),
            Expr::Lit(Literal::Int(4)),
        ])),
    };
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    assert_eq!(result, Value::Tuple(vec![Value::Int(1), Value::Int(2)]));
}

#[test]
fn test_tuple_complex_expression() {
    // Test: let x = 10 in let y = 20 in (x + y, x * y)
    let expr = Expr::Let {
        name: "x".to_string(),
        value: Box::new(Expr::Lit(Literal::Int(10))),
        body: Box::new(Expr::Let {
            name: "y".to_string(),
            value: Box::new(Expr::Lit(Literal::Int(20))),
            body: Box::new(Expr::Tuple(vec![
                Expr::BinOp {
                    op: BinOp::Add,
                    left: Box::new(Expr::Var("x".to_string())),
                    right: Box::new(Expr::Var("y".to_string())),
                },
                Expr::BinOp {
                    op: BinOp::Mul,
                    left: Box::new(Expr::Var("x".to_string())),
                    right: Box::new(Expr::Var("y".to_string())),
                },
            ])),
        }),
    };
    let chunk = Compiler::compile(&expr).unwrap();
    let mut vm = Vm::new();
    let result = vm.execute(chunk).unwrap();
    assert_eq!(result, Value::Tuple(vec![Value::Int(30), Value::Int(200)]));
}
