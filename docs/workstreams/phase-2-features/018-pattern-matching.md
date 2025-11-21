# Issue #018: Pattern Matching AST and Parser

## Overview
Implement pattern matching expressions in AST and parser, supporting patterns over literals, tuples, lists, and variables.

## Labels
- `feature`, `phase-2: features`, `priority: critical`, `requires-coordination`, `component: frontend`, `effort: l` (4-5 days)

## Milestone
Phase 2.3: Pattern Matching (Week 6)

## Track
Frontend (Developer 1)

## Dependencies
- #015 (Tuples) - HARD
- #016 (Lists) - HARD
- #017 (Arrays) - SOFT

## Blocks
- #019 (Pattern Compiler) - HARD
- #020 (Type Inference) - SOFT

## Parallel-Safe
⚠️ **REQUIRES COORDINATION** - Must align interface with #019 on Day 1

## Acceptance Criteria
- [ ] `Match` expression in AST
- [ ] Parser for `match expr with | pat -> expr`
- [ ] Pattern types: literals, variables, wildcards, tuples, lists
- [ ] Pattern guards (optional): `| pat when cond -> expr`
- [ ] Exhaustiveness checking (basic)
- [ ] 40+ pattern tests

## Technical Specification

### AST Extension
```rust
// fusabi-frontend/src/ast.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Match {
        scrutinee: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    // ...
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Box<Expr>>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Wildcard: _
    Wildcard,

    /// Variable binding: x
    Var(String),

    /// Literal: 42, true, "hello"
    Lit(Literal),

    /// Tuple: (pat1, pat2, ...)
    Tuple(Vec<Pattern>),

    /// List: [pat1; pat2; ...]
    List(Vec<Pattern>),

    /// Cons: pat :: pats
    Cons { head: Box<Pattern>, tail: Box<Pattern> },

    /// Nil: []
    Nil,

    /// Or pattern: pat1 | pat2
    Or(Vec<Pattern>),
}
```

### Parser
```rust
// fusabi-frontend/src/parser.rs

impl Parser {
    fn parse_match(&mut self) -> Result<Expr, ParseError> {
        self.expect_keyword("match")?;
        let scrutinee = self.parse_expr()?;
        self.expect_keyword("with")?;

        let mut arms = Vec::new();
        loop {
            if !self.check_token(Token::Pipe) {
                break;
            }
            self.advance(); // consume '|'

            let pattern = self.parse_pattern()?;

            let guard = if self.check_keyword("when") {
                self.advance();
                Some(Box::new(self.parse_expr()?))
            } else {
                None
            };

            self.expect_token(Token::Arrow)?;
            let body = self.parse_expr()?;

            arms.push(MatchArm { pattern, guard, body });
        }

        Ok(Expr::Match {
            scrutinee: Box::new(scrutinee),
            arms,
        })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        match self.peek()? {
            Token::Underscore => {
                self.advance();
                Ok(Pattern::Wildcard)
            }
            Token::Identifier(name) => {
                self.advance();
                Ok(Pattern::Var(name))
            }
            Token::Int(n) => {
                self.advance();
                Ok(Pattern::Lit(Literal::Int(n)))
            }
            Token::LParen => self.parse_tuple_pattern(),
            Token::LBracket => self.parse_list_pattern(),
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn parse_tuple_pattern(&mut self) -> Result<Pattern, ParseError> {
        self.expect_token(Token::LParen)?;
        let mut patterns = vec![self.parse_pattern()?];

        while self.check_token(Token::Comma) {
            self.advance();
            patterns.push(self.parse_pattern()?);
        }

        self.expect_token(Token::RParen)?;
        Ok(Pattern::Tuple(patterns))
    }

    fn parse_list_pattern(&mut self) -> Result<Pattern, ParseError> {
        self.expect_token(Token::LBracket)?;

        if self.check_token(Token::RBracket) {
            self.advance();
            return Ok(Pattern::Nil);
        }

        let mut patterns = vec![self.parse_pattern()?];

        while self.check_token(Token::Semicolon) {
            self.advance();
            patterns.push(self.parse_pattern()?);
        }

        self.expect_token(Token::RBracket)?;
        Ok(Pattern::List(patterns))
    }
}
```

## Testing Requirements

```rust
#[test]
fn test_parse_simple_match() {
    let code = r#"
        match x with
        | 0 -> "zero"
        | 1 -> "one"
        | _ -> "other"
    "#;
    let expr = parse_expr(code).unwrap();
    assert!(matches!(expr, Expr::Match { .. }));
}

#[test]
fn test_parse_list_pattern() {
    let code = r#"
        match list with
        | [] -> 0
        | x :: xs -> 1 + length xs
    "#;
    let expr = parse_expr(code).unwrap();
    // Verify pattern structure
}

#[test]
fn test_parse_tuple_pattern() {
    let code = r#"
        match pair with
        | (x, y) -> x + y
    "#;
    let expr = parse_expr(code).unwrap();
}

#[test]
fn test_parse_pattern_guard() {
    let code = r#"
        match x with
        | n when n > 0 -> "positive"
        | n when n < 0 -> "negative"
        | _ -> "zero"
    "#;
    let expr = parse_expr(code).unwrap();
}
```

## Implementation Steps
1. **Day 1 MORNING**: Design pattern AST with Dev 2, define interface
2. **Day 1 AFTERNOON**: Implement pattern types
3. **Day 2**: Implement match expression parsing
4. **Day 3**: Pattern parsing (tuples, lists, cons)
5. **Day 4**: Pattern guards and exhaustiveness
6. **Day 5**: Testing and examples

## Estimated Effort
**4-5 days** (Large)

## Coordination Protocol
**Day 1 MANDATORY**: All-hands design session
- Define Pattern AST structure
- Define pattern compilation interface
- Share header files with Dev 2
- Agree on pattern representation

## Notes
- Coordinate with #019 on pattern IR
- Exhaustiveness checking is basic (warn on non-exhaustive)
- Pattern guards optional for Phase 2
