# Prism Test Utils

Testing utilities for MCP Rust SDK development.

## Installation

Add this as a dev-dependency in your `Cargo.toml`:

```toml
[dev-dependencies]
prism-test-utils = "0.1"
```

## Features

- **Mock Server**: Simulate MCP server responses for client testing
- **Mock Client**: Queue and send requests for server testing
- **Assertions**: Specialized assertion helpers for MCP types
- **Test Harness** (Coming Soon): End-to-end testing framework

## Usage

### Mock Server

```rust
use prism_test_utils::MockServer;
use prism_mcp_rs::protocol::{JsonRpcRequest, JsonRpcResponse};
use serde_json::json;

#[test]
fn test_client_request() {
    let mut mock_server = MockServer::new();
    
    // Set up expectation
    mock_server.expect_request(
        "tools/list",
        JsonRpcResponse::success_value(json!(1), json!({"tools": []}))
    );
    
    // Your client code would interact with the mock server here
}
```

### Mock Client

```rust
use prism_test_utils::MockClient;
use serde_json::json;

#[test]
fn test_server_response() {
    let mut client = MockClient::new();
    
    // Queue requests
    client.queue_request(MockClient::create_initialize_request());
    client.queue_request(MockClient::create_tool_call_request(
        "calculator",
        json!({"expression": "2+2"})
    ));
    
    // Your server would process these requests
}
```

### Assertions

```rust
use prism_test_utils::{
    assert_tool_success,
    assert_response_success,
    assert_error_code
};

#[test]
fn test_tool_result() {
    let result = // ... your tool result
    assert_tool_success(&result);
    
    let response = // ... your response
    assert_response_success(&response);
}
```

## License

MIT License
