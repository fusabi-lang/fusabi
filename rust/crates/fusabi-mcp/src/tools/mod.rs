pub mod eval_fusabi;
pub mod get_context;

use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use crate::protocol::{CallToolResponse, Tool};

pub async fn execute_tool(
    name: &str,
    arguments: HashMap<String, Value>,
) -> Result<CallToolResponse> {
    match name {
        "eval_fusabi" => eval_fusabi::execute(arguments).await,
        "get_context" => get_context::execute(arguments).await,
        _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
    }
}

pub fn list_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "eval_fusabi".to_string(),
            description: "Execute Fusabi code and return the result".to_string(),
            input_schema: Some(eval_fusabi::get_schema()),
        },
        Tool {
            name: "get_context".to_string(),
            description: "Get context about Fusabi language (syntax, examples, stdlib)"
                .to_string(),
            input_schema: Some(get_context::get_schema()),
        },
    ]
}