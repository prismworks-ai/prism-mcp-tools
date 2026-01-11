//! Test harness for end-to-end testing of MCP implementations
//!
//! This module provides a complete test environment for testing MCP server and client
//! interactions in an isolated, controlled manner.

use prism_mcp_rs::core::error::{McpError, McpResult};
use prism_mcp_rs::core::*;
use prism_mcp_rs::protocol::*;
use prism_mcp_rs::server::{McpServer, ServerConfig};
use crate::mock_client::MockClient;
// TODO: Implement MemoryTransport
// use prism_mcp_rs::transport::memory::MemoryTransport;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Test harness for end-to-end testing
///
/// Provides a complete test environment with server, client, and transport
/// for comprehensive MCP testing.
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::harness::TestHarness;
/// use prism_mcp_rs::core::{Tool, ToolHandler, ToolResult, Content};
/// use serde_json::json;
/// use std::collections::HashMap;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Set up test environment
/// let mut harness = TestHarness::setup().await;
///
/// // Add a test tool
/// struct TestTool;
/// #[async_trait::async_trait]
/// impl ToolHandler for TestTool {
///     async fn call(&self, _args: HashMap<String, serde_json::Value>) -> Result<ToolResult, Box<dyn std::error::Error + Send + Sync>> {
///         Ok(ToolResult {
///             content: vec![Content::Text { text: "Test result".to_string() }],
///             is_error: Some(false),
///         })
///     }
/// }
///
/// harness.add_tool("test_tool", TestTool).await?;
///
/// // Initialize and test
/// let init_result = harness.initialize().await?;
/// harness.assert_server_ready();
///
/// // Call the tool
/// let result = harness.call_tool("test_tool", json!({})).await?;
/// assert_eq!(result.is_error, Some(false));
/// # Ok(())
/// # }
/// ```
pub struct TestHarness {
    /// The MCP server instance
    pub server: Arc<Mutex<McpServer>>,
    /// Mock client for sending requests
    pub client: MockClient,
    /// Memory transport for communication
    pub transport: Arc<MemoryTransport>,
    /// Server configuration
    pub config: ServerConfig,
    /// Whether the server has been initialized
    initialized: bool,
}

impl TestHarness {
    /// Set up a complete test environment
    pub async fn setup() -> Self {
        let server = McpServer::new("test-server".to_string(), "1.0.0".to_string());
        let client = MockClient::new();
        let transport = Arc::new(MemoryTransport::new());
        let config = ServerConfig::default();

        Self {
            server: Arc::new(Mutex::new(server)),
            client,
            transport,
            config,
            initialized: false,
        }
    }

    /// Set up with custom server configuration
    pub async fn setup_with_config(config: ServerConfig) -> Self {
        let server = McpServer::with_config(
            "test-server".to_string(),
            "1.0.0".to_string(),
            config.clone(),
        );
        let client = MockClient::new();
        let transport = Arc::new(MemoryTransport::new());

        Self {
            server: Arc::new(Mutex::new(server)),
            client,
            transport,
            config,
            initialized: false,
        }
    }

    /// Add a tool to the test server
    pub async fn add_tool<H>(&mut self, name: &str, handler: H) -> McpResult<()>
    where
        H: ToolHandler + 'static,
    {
        let tool = Tool::new(
            name.to_string(),
            Some(format!("Test tool: {}", name)),
            json!({
                "type": "object",
                "properties": {},
            }),
            handler,
        );

        let mut server = self.server.lock().await;
        server.add_tool(tool)
    }

    /// Add a resource to the test server
    pub async fn add_resource<H>(&mut self, uri: &str, handler: H) -> McpResult<()>
    where
        H: ResourceHandler + 'static,
    {
        let mut server = self.server.lock().await;
        server
            .add_resource(
                uri.to_string(), // name
                uri.to_string(), // uri
                handler,
            )
            .await
    }

    /// Add a prompt to the test server
    pub async fn add_prompt<H>(&mut self, name: &str, handler: H) -> McpResult<()>
    where
        H: PromptHandler + 'static,
    {
        let info = PromptInfo {
            name: name.to_string(),
            description: Some(format!("Test prompt: {}", name)),
            arguments: None,
            title: None,
            meta: None,
        };

        let mut server = self.server.lock().await;
        server.add_prompt(info, handler).await
    }

    /// Run initialization sequence
    pub async fn initialize(&mut self) -> McpResult<InitializeResult> {
        // Send initialize request
        let init_request = MockClient::create_initialize_request();
        let response = self.send_request(init_request).await?;

        // Extract initialize result
        let result = response
            .result
            .ok_or_else(|| McpError::protocol("Initialize response has no result"))?;

        let init_result: InitializeResult = serde_json::from_value(result)
            .map_err(|e| McpError::protocol(format!("Failed to parse initialize result: {}", e)))?;

        // Send initialized notification
        let notification = MockClient::create_initialized_notification();
        self.send_notification(notification).await?;

        self.initialized = true;
        Ok(init_result)
    }

    /// Send a request to the server and get response
    pub async fn send_request(&mut self, request: JsonRpcRequest) -> McpResult<JsonRpcResponse> {
        // Write request to transport
        self.transport
            .write(JsonRpcMessage::Request(request.clone()))
            .await
            .map_err(|e| McpError::transport(format!("Failed to write request: {}", e)))?;

        // Process on server side
        let mut server = self.server.lock().await;
        if let Some(message) = self
            .transport
            .read()
            .await
            .map_err(|e| McpError::transport(format!("Failed to read request: {}", e)))?
        {
            match message {
                JsonRpcMessage::Request(req) => {
                    let response = server.handle_request(req).await?;

                    // Write response back
                    self.transport
                        .write(JsonRpcMessage::Response(response.clone()))
                        .await
                        .map_err(|e| {
                            McpError::transport(format!("Failed to write response: {}", e))
                        })?;

                    Ok(response)
                }
                _ => Err(McpError::protocol("Expected request message")),
            }
        } else {
            Err(McpError::transport("No message available"))
        }
    }

    /// Send a notification to the server
    pub async fn send_notification(&mut self, notification: JsonRpcNotification) -> McpResult<()> {
        // Write notification to transport
        self.transport
            .write(JsonRpcMessage::Notification(notification.clone()))
            .await
            .map_err(|e| McpError::transport(format!("Failed to write notification: {}", e)))?;

        // Process on server side
        let mut server = self.server.lock().await;
        if let Some(message) = self
            .transport
            .read()
            .await
            .map_err(|e| McpError::transport(format!("Failed to read notification: {}", e)))?
        {
            match message {
                JsonRpcMessage::Notification(notif) => {
                    server.handle_notification(notif).await;
                    Ok(())
                }
                _ => Err(McpError::protocol("Expected notification message")),
            }
        } else {
            Err(McpError::transport("No message available"))
        }
    }

    /// Call a tool and get result
    pub async fn call_tool(&mut self, name: &str, args: Value) -> McpResult<ToolResult> {
        if !self.initialized {
            return Err(McpError::protocol("Server not initialized"));
        }

        let request = MockClient::create_tool_call_request(name, args);
        let response = self.send_request(request).await?;

        // Check if response has a result
        let result = response
            .result
            .ok_or_else(|| McpError::protocol("Tool response has no result"))?;

        serde_json::from_value(result)
            .map_err(|e| McpError::protocol(format!("Failed to parse tool result: {}", e)))
    }

    /// Read a resource and get contents
    pub async fn read_resource(&mut self, uri: &str) -> McpResult<ResourceContents> {
        if !self.initialized {
            return Err(McpError::protocol("Server not initialized"));
        }

        let request = MockClient::create_resource_read_request(uri);
        let response = self.send_request(request).await?;

        let result = response
            .result
            .ok_or_else(|| McpError::protocol("Resource response has no result"))?;

        serde_json::from_value(result)
            .map_err(|e| McpError::protocol(format!("Failed to parse resource contents: {}", e)))
    }

    /// Get a prompt result
    pub async fn get_prompt(&mut self, name: &str, args: Value) -> McpResult<PromptResult> {
        if !self.initialized {
            return Err(McpError::protocol("Server not initialized"));
        }

        let request = MockClient::create_prompt_get_request(name, args);
        let response = self.send_request(request).await?;

        let result = response
            .result
            .ok_or_else(|| McpError::protocol("Prompt response has no result"))?;

        serde_json::from_value(result)
            .map_err(|e| McpError::protocol(format!("Failed to parse prompt result: {}", e)))
    }

    /// List available tools
    pub async fn list_tools(&mut self) -> McpResult<Vec<ToolInfo>> {
        if !self.initialized {
            return Err(McpError::protocol("Server not initialized"));
        }

        let request = MockClient::create_list_tools_request();
        let response = self.send_request(request).await?;

        let result = response
            .result
            .ok_or_else(|| McpError::protocol("List tools response has no result"))?;

        let tools_response: ListToolsResponse = serde_json::from_value(result)
            .map_err(|e| McpError::protocol(format!("Failed to parse tools list: {}", e)))?;

        Ok(tools_response.tools)
    }

    /// Assert server is ready (initialized and running)
    pub fn assert_server_ready(&self) {
        assert!(self.initialized, "Server is not initialized");
        // Additional checks could be added here
    }

    /// Assert server has specific capabilities
    pub async fn assert_has_capability(&self, capability: &str) {
        let server = self.server.lock().await;
        let capabilities = server.capabilities();

        match capability {
            "tools" => assert!(
                capabilities.tools.is_some(),
                "Server missing tools capability"
            ),
            "resources" => assert!(
                capabilities.resources.is_some(),
                "Server missing resources capability"
            ),
            "prompts" => assert!(
                capabilities.prompts.is_some(),
                "Server missing prompts capability"
            ),
            _ => panic!("Unknown capability: {}", capability),
        }
    }

    /// Get server info
    pub async fn server_info(&self) -> (String, String) {
        let server = self.server.lock().await;
        ("test-server".to_string(), "1.0.0".to_string())
    }

    /// Reset the harness (clear transport, reset client)
    pub async fn reset(&mut self) {
        self.transport.clear().await;
        self.client = MockClient::new();
        self.initialized = false;
    }
}

// Helper struct for list tools response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ListToolsResponse {
    tools: Vec<ToolInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::collections::HashMap;

    struct TestTool;

    #[async_trait]
    impl ToolHandler for TestTool {
        async fn call(&self, _args: HashMap<String, Value>) -> McpResult<ToolResult> {
            Ok(ToolResult {
                content: vec![ContentBlock::Text {
                    text: "Test result".to_string(),
                    annotations: None,
                    meta: None,
                }],
                is_error: Some(false),
                structured_content: None,
                meta: None,
            })
        }
    }

    #[tokio::test]
    async fn test_harness_setup_and_initialize() {
        let mut harness = TestHarness::setup().await;

        // Add a test tool
        harness.add_tool("test_tool", TestTool).await.unwrap();

        // Initialize
        let init_result = harness.initialize().await.unwrap();
        assert_eq!(init_result.protocol_version, LATEST_PROTOCOL_VERSION);

        // Assert ready
        harness.assert_server_ready();
    }

    #[tokio::test]
    async fn test_harness_tool_call() {
        let mut harness = TestHarness::setup().await;

        harness.add_tool("test_tool", TestTool).await.unwrap();
        harness.initialize().await.unwrap();

        // Call tool
        let result = harness.call_tool("test_tool", json!({})).await.unwrap();
        assert_eq!(result.is_error, Some(false));
        assert!(!result.content.is_empty());
    }

    #[tokio::test]
    async fn test_harness_list_tools() {
        let mut harness = TestHarness::setup().await;

        harness.add_tool("tool1", TestTool).await.unwrap();
        harness.add_tool("tool2", TestTool).await.unwrap();
        harness.initialize().await.unwrap();

        let tools = harness.list_tools().await.unwrap();
        assert_eq!(tools.len(), 2);

        let tool_names: Vec<String> = tools.iter().map(|t| t.name.clone()).collect();
        assert!(tool_names.contains(&"tool1".to_string()));
        assert!(tool_names.contains(&"tool2".to_string()));
    }

    #[tokio::test]
    async fn test_harness_capabilities() {
        let config = ServerConfig {
            enable_tools: true,
            enable_resources: true,
            enable_prompts: false,
            ..Default::default()
        };

        let mut harness = TestHarness::setup_with_config(config).await;
        harness.add_tool("test", TestTool).await.unwrap();
        harness.initialize().await.unwrap();

        harness.assert_has_capability("tools").await;
        harness.assert_has_capability("resources").await;
    }

    #[tokio::test]
    async fn test_harness_reset() {
        let mut harness = TestHarness::setup().await;

        harness.add_tool("test", TestTool).await.unwrap();
        harness.initialize().await.unwrap();

        harness.assert_server_ready();

        // Reset
        harness.reset().await;

        // Should panic because not initialized
        let result = std::panic::catch_unwind(|| {
            harness.assert_server_ready();
        });
        assert!(result.is_err());
    }
}