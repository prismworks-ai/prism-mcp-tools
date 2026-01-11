//! Mock server for testing MCP clients
//!
//! This module provides a mock MCP server that can be used to test client implementations.
//! It allows setting up expected requests and responses for controlled testing scenarios.

use prism_mcp_rs::protocol::*;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::VecDeque;

/// Mock server for testing MCP clients
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::mock_server::MockServer;
/// use prism_mcp_rs::protocol::{JsonRpcRequest, JsonRpcResponse};
/// use serde_json::json;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut mock_server = MockServer::new();
///
/// // Set up an expectation
/// mock_server.expect_request(
///     "tools/list",
///     JsonRpcResponse::success_value(json!(1), json!({"tools": []}))
/// );
///
/// // Simulate receiving a request
/// let request = JsonRpcRequest::new(json!(1), "tools/list".to_string(), None::<()>)?;
/// let response = mock_server.handle(request).await;
///
/// // Verify all expectations were met
/// mock_server.verify()?;
/// # Ok(())
/// # }
/// ```
pub struct MockServer {
    /// Expected requests and their responses
    expectations: HashMap<String, VecDeque<JsonRpcResponse>>,
    /// Record of received requests
    received_requests: Vec<JsonRpcRequest>,
    /// Whether to track order of requests
    ordered: bool,
    /// Optional default response for unexpected requests
    default_response: Option<Box<dyn Fn(&JsonRpcRequest) -> JsonRpcResponse + Send + Sync>>,
}

impl MockServer {
    /// Create a new mock server
    pub fn new() -> Self {
        Self {
            expectations: HashMap::new(),
            received_requests: Vec::new(),
            ordered: false,
            default_response: None,
        }
    }

    /// Create a new mock server that enforces request order
    pub fn new_ordered() -> Self {
        Self {
            expectations: HashMap::new(),
            received_requests: Vec::new(),
            ordered: true,
            default_response: None,
        }
    }

    /// Set up an expectation for a method
    ///
    /// # Examples
    ///
    /// ```
    /// # use prism_mcp_rs::test_utils::mock_server::MockServer;
    /// # use prism_mcp_rs::protocol::JsonRpcResponse;
    /// # use serde_json::json;
    /// let mut server = MockServer::new();
    /// server.expect_request(
    ///     "initialize",
    ///     JsonRpcResponse::success_value(json!(1), json!({"capabilities": {}}))
    /// );
    /// ```
    pub fn expect_request(&mut self, method: &str, response: JsonRpcResponse) -> &mut Self {
        self.expectations
            .entry(method.to_string())
            .or_insert_with(VecDeque::new)
            .push_back(response);
        self
    }

    /// Set up multiple expectations for the same method
    pub fn expect_requests(&mut self, method: &str, responses: Vec<JsonRpcResponse>) -> &mut Self {
        let queue = self
            .expectations
            .entry(method.to_string())
            .or_insert_with(VecDeque::new);
        for response in responses {
            queue.push_back(response);
        }
        self
    }

    /// Set a default response for unexpected requests
    pub fn with_default_response<F>(mut self, handler: F) -> Self
    where
        F: Fn(&JsonRpcRequest) -> JsonRpcResponse + Send + Sync + 'static,
    {
        self.default_response = Some(Box::new(handler));
        self
    }

    /// Handle a request
    ///
    /// Returns the expected response or an error if no expectation was set
    pub async fn handle(&mut self, request: JsonRpcRequest) -> JsonRpcResponse {
        self.received_requests.push(request.clone());

        if let Some(queue) = self.expectations.get_mut(&request.method) {
            if let Some(response) = queue.pop_front() {
                // Update response ID to match request
                let mut response = response;
                response.id = Some(request.id.clone());
                return response;
            }
        }

        // Use default response if available
        if let Some(ref default_handler) = self.default_response {
            return default_handler(&request);
        }

        // Return method not found error
        JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: Some(request.id.clone()),
            result: None,
            error: Some(ErrorObject {
                code: -32601,
                message: format!("Unexpected method: {}", request.method),
                data: None,
            }),
        }
    }

    /// Handle a notification (no response expected)
    pub async fn handle_notification(&mut self, notification: JsonRpcNotification) {
        // Convert notification to request for tracking
        let request = JsonRpcRequest {
            jsonrpc: notification.jsonrpc,
            id: serde_json::json!(null),
            method: notification.method,
            params: notification.params,
        };
        self.received_requests.push(request);
    }

    /// Assert all expectations were met
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all expectations were met, or an error describing what wasn't met
    pub fn verify(&self) -> Result<(), String> {
        // Check for unmet expectations
        let mut unmet = Vec::new();
        for (method, queue) in &self.expectations {
            if !queue.is_empty() {
                unmet.push(format!("{} ({} remaining)", method, queue.len()));
            }
        }

        if !unmet.is_empty() {
            return Err(format!(
                "Unmet expectations for methods: {}",
                unmet.join(", ")
            ));
        }

        Ok(())
    }

    /// Get received requests for assertions
    pub fn received_requests(&self) -> &[JsonRpcRequest] {
        &self.received_requests
    }

    /// Clear all expectations and received requests
    pub fn reset(&mut self) {
        self.expectations.clear();
        self.received_requests.clear();
    }

    /// Assert a specific request was received
    pub fn assert_request_received(&self, method: &str) -> Result<(), String> {
        if self.received_requests.iter().any(|r| r.method == method) {
            Ok(())
        } else {
            Err(format!("Expected request '{}' was not received", method))
        }
    }

    /// Assert a request was received with specific parameters
    pub fn assert_request_with_params(&self, method: &str, params: Value) -> Result<(), String> {
        for request in &self.received_requests {
            if request.method == method {
                if request.params == Some(params.clone()) {
                    return Ok(());
                }
            }
        }
        Err(format!(
            "Request '{}' with expected params was not received",
            method
        ))
    }

    /// Get the count of received requests for a method
    pub fn request_count(&self, method: &str) -> usize {
        self.received_requests
            .iter()
            .filter(|r| r.method == method)
            .count()
    }
}

impl Default for MockServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_mock_server_basic() {
        let mut server = MockServer::new();

        // Set up expectation
        server.expect_request(
            "test_method",
            JsonRpcResponse::success_value(json!(1), json!({"result": "ok"})),
        );

        // Handle request
        let request = JsonRpcRequest::new(json!(1), "test_method".to_string(), None::<()>).unwrap();

        let response = server.handle(request).await;
        assert!(response.result.is_some());
        assert_eq!(response.result.unwrap()["result"], "ok");

        // Verify expectations met
        server.verify().unwrap();
    }

    #[tokio::test]
    async fn test_mock_server_unexpected_request() {
        let mut server = MockServer::new();

        let request = JsonRpcRequest::new(json!(1), "unexpected".to_string(), None::<()>).unwrap();

        let response = server.handle(request).await;
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, -32601);
    }

    #[tokio::test]
    async fn test_mock_server_multiple_expectations() {
        let mut server = MockServer::new();

        server.expect_requests(
            "multi",
            vec![
                JsonRpcResponse::success_value(json!(1), json!({"call": 1})),
                JsonRpcResponse::success_value(json!(2), json!({"call": 2})),
            ],
        );

        // First call
        let request1 = JsonRpcRequest::new(json!(1), "multi".to_string(), None::<()>).unwrap();
        let response1 = server.handle(request1).await;
        assert_eq!(response1.result.unwrap()["call"], 1);

        // Second call
        let request2 = JsonRpcRequest::new(json!(2), "multi".to_string(), None::<()>).unwrap();
        let response2 = server.handle(request2).await;
        assert_eq!(response2.result.unwrap()["call"], 2);

        server.verify().unwrap();
    }

    #[tokio::test]
    async fn test_mock_server_with_default() {
        let mut server = MockServer::new().with_default_response(|req| {
            JsonRpcResponse::success_value(req.id.clone(), json!({"method": req.method}))
        });

        let request = JsonRpcRequest::new(json!(1), "any_method".to_string(), None::<()>).unwrap();

        let response = server.handle(request).await;
        assert!(response.result.is_some());
        assert_eq!(response.result.unwrap()["method"], "any_method");
    }

    #[test]
    fn test_verify_unmet_expectations() {
        let mut server = MockServer::new();
        server.expect_request(
            "never_called",
            JsonRpcResponse::success_value(json!(1), json!({})),
        );

        let result = server.verify();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("never_called"));
    }
}