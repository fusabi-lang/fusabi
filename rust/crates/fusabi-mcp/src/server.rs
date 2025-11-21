use anyhow::Result;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

use crate::protocol::*;
use crate::tools;
use crate::transport::StdioTransport;

pub struct McpServer {
    initialized: Arc<RwLock<bool>>,
}

impl McpServer {
    pub fn new() -> Self {
        Self {
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn run(self) -> Result<()> {
        info!("Starting Fusabi MCP Server");

        let (transport, mut rx) = StdioTransport::new();
        transport.start_reading();

        info!("Server ready, waiting for messages...");

        while let Some(message) = rx.recv().await {
            match message {
                JsonRpcMessage::Request(request) => {
                    let response = self.handle_request(request.clone()).await;
                    if let Err(e) = StdioTransport::write_response(&response) {
                        error!("Failed to write response: {}", e);
                    }
                }
                JsonRpcMessage::Notification(notification) => {
                    self.handle_notification(notification).await;
                }
                JsonRpcMessage::Response(_) => {
                    // We're a server, we don't expect responses
                    debug!("Received unexpected response");
                }
            }
        }

        info!("Server shutting down");
        Ok(())
    }

    async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        debug!("Handling request: {}", request.method);

        match request.method.as_str() {
            "initialize" => self.handle_initialize(request).await,
            "initialized" => self.handle_initialized(request).await,
            "tools/list" => self.handle_list_tools(request).await,
            "tools/call" => self.handle_call_tool(request).await,
            _ => JsonRpcResponse::error(
                error_codes::METHOD_NOT_FOUND,
                format!("Method not found: {}", request.method),
                None,
                request.id,
            ),
        }
    }

    async fn handle_notification(&self, notification: JsonRpcNotification) {
        debug!("Handling notification: {}", notification.method);

        match notification.method.as_str() {
            "notifications/initialized" => {
                let mut initialized = self.initialized.write().await;
                *initialized = true;
                info!("Server initialized");
            }
            _ => {
                debug!("Unknown notification: {}", notification.method);
            }
        }
    }

    async fn handle_initialize(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let params: InitializeRequest = match request.params {
            Some(params) => match serde_json::from_value(params) {
                Ok(p) => p,
                Err(e) => {
                    return JsonRpcResponse::error(
                        error_codes::INVALID_PARAMS,
                        format!("Invalid params: {}", e),
                        None,
                        request.id,
                    );
                }
            },
            None => {
                return JsonRpcResponse::error(
                    error_codes::INVALID_PARAMS,
                    "Missing params".to_string(),
                    None,
                    request.id,
                );
            }
        };

        info!("Initializing with client: {} {}", params.client_info.name, params.client_info.version);

        let response = InitializeResponse {
            protocol_version: "2024-11-05".to_string(),
            capabilities: ServerCapabilities {
                tools: ToolsCapability { list_tools: true },
            },
            server_info: ServerInfo {
                name: "fusabi-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        JsonRpcResponse::success(serde_json::to_value(response).unwrap(), request.id)
    }

    async fn handle_initialized(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let mut initialized = self.initialized.write().await;
        *initialized = true;
        info!("Server initialized");
        JsonRpcResponse::success(json!({}), request.id)
    }

    async fn handle_list_tools(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        // Check if initialized
        let initialized = self.initialized.read().await;
        if !*initialized {
            return JsonRpcResponse::error(
                error_codes::INVALID_REQUEST,
                "Server not initialized".to_string(),
                None,
                request.id,
            );
        }

        let tools = tools::list_tools();
        let response = ListToolsResponse { tools };

        JsonRpcResponse::success(serde_json::to_value(response).unwrap(), request.id)
    }

    async fn handle_call_tool(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        // Check if initialized
        let initialized = self.initialized.read().await;
        if !*initialized {
            return JsonRpcResponse::error(
                error_codes::INVALID_REQUEST,
                "Server not initialized".to_string(),
                None,
                request.id,
            );
        }

        let call_request: CallToolRequest = match request.params {
            Some(params) => match serde_json::from_value(params) {
                Ok(p) => p,
                Err(e) => {
                    return JsonRpcResponse::error(
                        error_codes::INVALID_PARAMS,
                        format!("Invalid params: {}", e),
                        None,
                        request.id,
                    );
                }
            },
            None => {
                return JsonRpcResponse::error(
                    error_codes::INVALID_PARAMS,
                    "Missing params".to_string(),
                    None,
                    request.id,
                );
            }
        };

        debug!("Calling tool: {}", call_request.name);

        match tools::execute_tool(&call_request.name, call_request.arguments).await {
            Ok(response) => JsonRpcResponse::success(
                serde_json::to_value(response).unwrap(),
                request.id,
            ),
            Err(e) => JsonRpcResponse::error(
                error_codes::INTERNAL_ERROR,
                format!("Tool execution failed: {}", e),
                None,
                request.id,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_creation() {
        let server = McpServer::new();
        let initialized = server.initialized.read().await;
        assert_eq!(*initialized, false);
    }
}