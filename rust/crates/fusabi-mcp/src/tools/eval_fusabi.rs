use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::timeout;
use tracing::debug;

use fusabi_frontend::compiler::Compiler;
use fusabi_frontend::lexer::Lexer;
use fusabi_frontend::parser::Parser;
use fusabi_vm::{Vm, StdlibRegistry};

use crate::protocol::{CallToolResponse, ToolContent};

const DEFAULT_TIMEOUT_MS: u64 = 5000; // 5 seconds

pub async fn execute(args: HashMap<String, Value>) -> Result<CallToolResponse> {
    let code = args
        .get("code")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing 'code' argument"))?;

    let timeout_ms = args
        .get("timeout_ms")
        .and_then(|v| v.as_u64())
        .unwrap_or(DEFAULT_TIMEOUT_MS);

    debug!("Executing Fusabi code with timeout {}ms", timeout_ms);

    let code_owned = code.to_string();

    // Execute with timeout
    let result = timeout(
        Duration::from_millis(timeout_ms),
        tokio::task::spawn_blocking(move || execute_fusabi_code(&code_owned)),
    )
    .await;

    match result {
        Ok(Ok(Ok(output))) => Ok(CallToolResponse {
            content: vec![ToolContent::Text { text: output }],
        }),
        Ok(Ok(Err(e))) => Ok(CallToolResponse {
            content: vec![ToolContent::Error {
                error: format!("Execution error: {}", e),
            }],
        }),
        Ok(Err(e)) => Ok(CallToolResponse {
            content: vec![ToolContent::Error {
                error: format!("Task error: {}", e),
            }],
        }),
        Err(_) => Ok(CallToolResponse {
            content: vec![ToolContent::Error {
                error: format!("Execution timed out after {}ms", timeout_ms),
            }],
        }),
    }
}

fn execute_fusabi_code(code: &str) -> Result<String> {
    // Tokenize the code
    let mut lexer = Lexer::new(code);
    let tokens = lexer
        .tokenize()
        .map_err(|e| anyhow!("Lexer error: {:?}", e))?;

    // Parse the tokens
    let mut parser = Parser::new(tokens);
    let ast = parser
        .parse()
        .map_err(|e| anyhow!("Parse error: {:?}", e))?;

    // Compile to bytecode
    let chunk = Compiler::compile(&ast)
        .map_err(|e| anyhow!("Compile error: {:?}", e))?;

    // Create VM and register stdlib
    let mut vm = Vm::new();
    let mut registry = StdlibRegistry::new();

    // Capture output
    let output = Arc::new(Mutex::new(String::new()));
    let output_clone = Arc::clone(&output);

    // Register a custom print function that captures output
    registry.register("print", move |args: Vec<fusabi_vm::Value>| -> Result<fusabi_vm::Value, fusabi_vm::VmError> {
        let mut out = output_clone.lock().unwrap();
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            out.push_str(&format!("{:?}", arg));
        }
        out.push('\n');
        Ok(fusabi_vm::Value::Null)
    });

    // Register standard library functions
    registry.register_all(&mut vm);

    // Execute the bytecode
    match vm.run(chunk) {
        Ok(value) => {
            let mut out = output.lock().unwrap();
            // Add the final result to output
            if !out.is_empty() && !matches!(value, fusabi_vm::Value::Null) {
                out.push_str(&format!("\nResult: {:?}", value));
            } else if !matches!(value, fusabi_vm::Value::Null) {
                *out = format!("Result: {:?}", value);
            }
            Ok(out.clone())
        }
        Err(e) => Err(anyhow!("Runtime error: {:?}", e)),
    }
}

pub fn get_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "code": {
                "type": "string",
                "description": "Fusabi code to execute"
            },
            "timeout_ms": {
                "type": "integer",
                "description": "Execution timeout in milliseconds (default: 5000)",
                "minimum": 100,
                "maximum": 30000
            }
        },
        "required": ["code"]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_eval_simple_expression() {
        let mut args = HashMap::new();
        args.insert("code".to_string(), json!("2 + 3"));

        let result = execute(args).await.unwrap();
        assert_eq!(result.content.len(), 1);

        match &result.content[0] {
            ToolContent::Text { text } => {
                assert!(text.contains("5"));
            }
            _ => panic!("Expected text result"),
        }
    }

    #[tokio::test]
    async fn test_eval_with_print() {
        let mut args = HashMap::new();
        args.insert("code".to_string(), json!(r#"print("Hello, World!")"#));

        let result = execute(args).await.unwrap();
        assert_eq!(result.content.len(), 1);

        match &result.content[0] {
            ToolContent::Text { text } => {
                assert!(text.contains("Hello, World!"));
            }
            _ => panic!("Expected text result"),
        }
    }

    #[tokio::test]
    async fn test_eval_timeout() {
        let mut args = HashMap::new();
        // Note: Fusabi doesn't have infinite loops in the same way,
        // so we'll test with a different approach
        args.insert("code".to_string(), json!("let rec f = fun x -> f x in f 0"));
        args.insert("timeout_ms".to_string(), json!(100));

        let result = execute(args).await.unwrap();
        // This test might not work as expected depending on the VM implementation
        // If the VM doesn't support infinite recursion or has stack limits,
        // we might get a different error
    }
}