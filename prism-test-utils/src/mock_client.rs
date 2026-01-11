//! Mock client for testing MCP servers
//!
//! This module provides a mock MCP client that can be used to test server implementations.
//! It allows queuing requests and capturing responses for validation.

use prism_mcp_rs::protocol::*;
use serde_json::{Value, json};
use std::collections::VecDeque;

/// Mock client for testing MCP servers
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::mock_client::MockClient;
/// use serde_json::json;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut client = MockClient::new();
///
/// // Queue requests to send
/// client.queue_request(MockClient::create_initialize_request());
/// client.queue_request(MockClient::create_tool_call_request(
///     "calculator",
///     json!({"expression": "2+2"})
/// ));
///
/// // Send all requests (would connect to a server in real scenario)
/// // let responses = client.send_all(&mut server).await;
/// # Ok(())
/// # }
/// ```
pub struct MockClient {
    /// Queue of requests to send
    request_queue: VecDeque<JsonRpcRequest>,
    /// Responses received
    responses: Vec<JsonRpcResponse>,
    /// ID counter for generating unique request IDs
    id_counter: u64,
    /// Client info for initialization
    client_info: ClientInfo,
}

/// Client information for initialization
#[derive(Clone, Debug)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

impl Default for ClientInfo {
    fn default() -> Self {
        Self {
            name: "mock-client".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

impl MockClient {
    /// Create a new mock client
    pub fn new() -> Self {
        Self {
            request_queue: VecDeque::new(),
            responses: Vec::new(),
            id_counter: 1,
            client_info: ClientInfo::default(),
        }
    }

    /// Create a new mock client with custom info
    pub fn with_info(name: String, version: String) -> Self {
        Self {
            request_queue: VecDeque::new(),
            responses: Vec::new(),
            id_counter: 1,
            client_info: ClientInfo { name, version },
        }
    }

    /// Get the next request ID
    fn next_id(&mut self) -> RequestId {
        let id = json!(self.id_counter);
        self.id_counter += 1;
        id
    }

    /// Queue a request to be sent
    pub fn queue_request(&mut self, mut request: JsonRpcRequest) -> &mut Self {
        // Update request ID if it's a placeholder
        if request.id == json!("test-123") || request.id == json!(null) {
            request.id = self.next_id();
        }
        self.request_queue.push_back(request);
        self
    }

    /// Queue a custom request
    pub fn queue_custom<T: serde::Serialize>(
        &mut self,
        method: &str,
        params: T,
    ) -> Result<&mut Self, serde_json::Error> {
        let request = JsonRpcRequest::new(self.next_id(), method.to_string(), Some(params))?;
        self.request_queue.push_back(request);
        Ok(self)
    }

    /// Get the next queued request
    pub fn next_request(&mut self) -> Option<JsonRpcRequest> {
        self.request_queue.pop_front()
    }

    /// Send next queued request (in a real scenario, this would send to a server)
    ///
    /// For testing, this simulates sending and receiving
    pub async fn send_next(&mut self) -> Option<JsonRpcResponse> {
        if let Some(request) = self.request_queue.pop_front() {
            // In a real implementation, this would send to server and await response
            // For testing, we create a mock response
            let response = self.create_mock_response(&request);
            self.responses.push(response.clone());
            Some(response)
        } else {
            None
        }
    }

    /// Send all queued requests
    pub async fn send_all(&mut self) -> Vec<JsonRpcResponse> {
        let mut responses = Vec::new();
        while let Some(response) = self.send_next().await {
            responses.push(response);
        }
        responses
    }

    /// Get all received responses
    pub fn responses(&self) -> &[JsonRpcResponse] {
        &self.responses
    }

    /// Clear responses
    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }

    /// Create a mock response for testing (simulates server response)
    fn create_mock_response(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            "initialize" => JsonRpcResponse::success_value(
                request.id.clone(),
                json!({
                    "protocolVersion": LATEST_PROTOCOL_VERSION,
                    "capabilities": {},
                    "serverInfo": {
                        "name": "mock-server",
                        "version": "1.0.0"
                    }
                }),
            ),
            "tools/list" => {
                JsonRpcResponse::success_value(request.id.clone(), json!({"tools": []}))
            }
            "resources/list" => {
                JsonRpcResponse::success_value(request.id.clone(), json!({"resources": []}))
            }
            "prompts/list" => {
                JsonRpcResponse::success_value(request.id.clone(), json!({"prompts": []}))
            }
            _ => JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id: Some(request.id.clone()),
                result: None,
                error: Some(ErrorObject {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            },
        }
    }

    // Standard request creation helpers

    /// Create standard initialize request
    pub fn create_initialize_request() -> JsonRpcRequest {
        JsonRpcRequest::with_params(
            json!("init-1"),
            "initialize".to_string(),
            json!({
                "protocolVersion": LATEST_PROTOCOL_VERSION,
                "capabilities": {},
                "clientInfo": {
                    "name": "mock-client",
                    "version": "1.0.0"
                }
            }),
        )
        .expect("Failed to create initialize request")
    }

    /// Create initialized notification
    pub fn create_initialized_notification() -> JsonRpcNotification {
        JsonRpcNotification {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: "notifications/initialized".to_string(),
            params: Some(json!({})),
        }
    }

    /// Create tool call request
    pub fn create_tool_call_request(name: &str, args: Value) -> JsonRpcRequest {
        JsonRpcRequest::with_params(
            json!("tool-1"),
            "tools/call".to_string(),
            json!({
                "name": name,
                "arguments": args
            }),
        )
        .expect("Failed to create tool call request")
    }

    /// Create resource read request
    pub fn create_resource_read_request(uri: &str) -> JsonRpcRequest {
        JsonRpcRequest::with_params(
            json!("resource-1"),
            "resources/read".to_string(),
            json!({
                "uri": uri
            }),
        )
        .expect("Failed to create resource read request")
    }

    /// Create prompt get request
    pub fn create_prompt_get_request(name: &str, args: Value) -> JsonRpcRequest {
        JsonRpcRequest::with_params(
            json!("prompt-1"),
            "prompts/get".to_string(),
            json!({
                "name": name,
                "arguments": args
            }),
        )
        .expect("Failed to create prompt get request")
    }

    /// Create list tools request
    pub fn create_list_tools_request() -> JsonRpcRequest {
        JsonRpcRequest::without_params(json!("list-tools-1"), "tools/list".to_string())
    }

    /// Create list resources request
    pub fn create_list_resources_request() -> JsonRpcRequest {
        JsonRpcRequest::without_params(json!("list-resources-1"), "resources/list".to_string())
    }

    /// Create list prompts request
    pub fn create_list_prompts_request() -> JsonRpcRequest {
        JsonRpcRequest::without_params(json!("list-prompts-1"), "prompts/list".to_string())
    }

    /// Assert response is successful
    pub fn assert_response_success(&self, index: usize) -> Result<(), String> {
        if index >= self.responses.len() {
            return Err(format!("Response {} not found", index));
        }

        let response = &self.responses[index];
        if response.error.is_some() {
            return Err(format!(
                "Response {} has error: {:?}",
                index, response.error
            ));
        }
        if response.result.is_none() {
            return Err(format!("Response {} has no result", index));
        }

        Ok(())
    }

    /// Assert all responses are successful
    pub fn assert_all_responses_success(&self) -> Result<(), String> {
        for (i, response) in self.responses.iter().enumerate() {
            if response.error.is_some() {
                return Err(format!("Response {} has error: {:?}", i, response.error));
            }
            if response.result.is_none() {
                return Err(format!("Response {} has no result", i));
            }
        }
        Ok(())
    }
}

impl Default for MockClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_requests() {
        let init_req = MockClient::create_initialize_request();
        assert_eq!(init_req.method, "initialize");

        let tool_req = MockClient::create_tool_call_request("calc", json!({"x": 1}));
        assert_eq!(tool_req.method, "tools/call");

        let resource_req = MockClient::create_resource_read_request("file:///test.txt");
        assert_eq!(resource_req.method, "resources/read");
    }

    #[tokio::test]
    async fn test_queue_and_send() {
        let mut client = MockClient::new();

        client.queue_request(MockClient::create_initialize_request());
        client.queue_request(MockClient::create_list_tools_request());

        assert_eq!(client.request_queue.len(), 2);

        let responses = client.send_all().await;
        assert_eq!(responses.len(), 2);
        assert_eq!(client.responses.len(), 2);

        client.assert_all_responses_success().unwrap();
    }

    #[test]
    fn test_custom_request() {
        let mut client = MockClient::new();

        client
            .queue_custom("custom/method", json!({"param": "value"}))
            .unwrap();

        let request = client.next_request().unwrap();
        assert_eq!(request.method, "custom/method");
        assert_eq!(request.params, Some(json!({"param": "value"})));
    }

    #[test]
    fn test_id_generation() {
        let mut client = MockClient::new();

        let id1 = client.next_id();
        let id2 = client.next_id();
        let id3 = client.next_id();

        assert_eq!(id1, json!(1));
        assert_eq!(id2, json!(2));
        assert_eq!(id3, json!(3));
    }
}