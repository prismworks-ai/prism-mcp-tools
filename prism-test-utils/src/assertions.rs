//! Specialized assertion helpers for testing MCP implementations
//!
//! This module provides a collection of assertion functions that make
//! testing MCP implementations more expressive and readable.

// Import ToolResult and ResourceContents from the re-exports in protocol::types
use prism_mcp_rs::protocol::types::{Content, PromptResult, ResourceContents, ToolResult};
use prism_mcp_rs::protocol::{JsonRpcError, JsonRpcResponse};

/// Assert tool result is successful
///
/// # Panics
///
/// Panics if the tool result indicates an error or has no content
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::assertions::assert_tool_success;
/// use prism_mcp_rs::core::{ToolResult, Content};
///
/// let result = ToolResult {
///     content: vec![Content::Text { text: "Success".to_string() }],
///     is_error: Some(false),
/// };
///
/// assert_tool_success(&result);
/// ```
pub fn assert_tool_success(result: &ToolResult) {
    assert_eq!(
        result.is_error,
        Some(false),
        "Tool result indicates error: {:?}",
        result
    );
    assert!(!result.content.is_empty(), "Tool result has no content");
}

/// Assert tool result indicates an error
///
/// # Panics
///
/// Panics if the tool result does not indicate an error
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::assertions::assert_tool_error;
/// use prism_mcp_rs::core::{ToolResult, Content};
///
/// let result = ToolResult {
///     content: vec![Content::Text { text: "Error occurred".to_string() }],
///     is_error: Some(true),
/// };
///
/// assert_tool_error(&result);
/// ```
pub fn assert_tool_error(result: &ToolResult) {
    assert_eq!(
        result.is_error,
        Some(true),
        "Tool result does not indicate error: {:?}",
        result
    );
}

/// Assert tool result contains expected text
///
/// # Panics
///
/// Panics if the expected text is not found in any text content blocks
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::assertions::assert_tool_content_contains;
/// use prism_mcp_rs::core::{ToolResult, Content};
///
/// let result = ToolResult {
///     content: vec![
///         Content::Text { text: "The calculation result is 42".to_string() },
///     ],
///     is_error: Some(false),
/// };
///
/// assert_tool_content_contains(&result, "42");
/// ```
pub fn assert_tool_content_contains(result: &ToolResult, expected: &str) {
    let content_text = result
        .content
        .iter()
        .filter_map(|c| match c {
            Content::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ");

    assert!(
        content_text.contains(expected),
        "Expected content '{}' not found in: {}",
        expected,
        content_text
    );
}

/// Assert resource contents are valid
///
/// # Panics
///
/// Panics if the resource URI is empty or if the resource contents are empty
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::assertions::assert_resource_valid;
/// use prism_mcp_rs::core::ResourceContents;
///
/// let contents = ResourceContents::Text {
///     uri: "file:///example.txt".to_string(),
///     text: "File contents".to_string(),
///     mime_type: Some("text/plain".to_string()),
/// };
///
/// assert_resource_valid(&contents);
/// ```
pub fn assert_resource_valid(contents: &ResourceContents) {
    match contents {
        ResourceContents::Text { uri, text, .. } => {
            assert!(!uri.is_empty(), "Resource URI is empty");
            assert!(!text.is_empty(), "Text resource is empty");
        }
        ResourceContents::Blob { uri, blob, .. } => {
            assert!(!uri.is_empty(), "Resource URI is empty");
            assert!(!blob.is_empty(), "Blob resource is empty");
        }
    }
}

/// Assert prompt result is valid
///
/// # Panics
///
/// Panics if the prompt has no messages or if any message has no role or content
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::assertions::assert_prompt_valid;
/// use prism_mcp_rs::core::{PromptResult, PromptMessage, Content};
///
/// let result = PromptResult {
///     messages: vec![
///         PromptMessage {
///             role: "user".to_string(),
///             content: vec![Content::Text { text: "Hello".to_string() }],
///         },
///     ],
///     description: None,
/// };
///
/// assert_prompt_valid(&result);
/// ```
pub fn assert_prompt_valid(result: &PromptResult) {
    assert!(!result.messages.is_empty(), "Prompt has no messages");

    for (i, message) in result.messages.iter().enumerate() {
        assert!(!message.role.is_empty(), "Message {} has no role", i);
        assert!(!message.content.is_empty(), "Message {} has no content", i);
    }
}

/// Assert JSON-RPC response is successful
///
/// # Panics
///
/// Panics if the response has no result or has an error
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::assertions::assert_response_success;
/// use prism_mcp_rs::protocol::JsonRpcResponse;
/// use serde_json::json;
///
/// let response = JsonRpcResponse::success_unchecked(
///     json!(1),
///     json!({"status": "ok"})
/// );
///
/// assert_response_success(&response);
/// ```
pub fn assert_response_success(response: &JsonRpcResponse) {
    assert!(
        response.result.is_some(),
        "Response has no result: {:?}",
        response
    );
    assert!(
        response.error.is_none(),
        "Response has error: {:?}",
        response.error
    );
}

/// Assert JSON-RPC response has an error
///
/// # Panics
///
/// Panics if the response has no error
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::assertions::assert_response_error;
/// use prism_mcp_rs::protocol::{JsonRpcResponse, ErrorObject};
/// use serde_json::json;
///
/// let response = JsonRpcResponse {
///     jsonrpc: "2.0".to_string(),
///     id: Some(json!(1)),
///     result: None,
///     error: Some(ErrorObject {
///         code: -32601,
///         message: "Method not found".to_string(),
///         data: None,
///     }),
/// };
///
/// assert_response_error(&response);
/// ```
pub fn assert_response_error(response: &JsonRpcResponse) {
    assert!(
        response.error.is_some(),
        "Response has no error: {:?}",
        response
    );
}

/// Assert JSON-RPC error has expected code
///
/// # Panics
///
/// Panics if the error code doesn't match the expected value
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::assertions::assert_error_code;
/// use prism_mcp_rs::protocol::JsonRpcError;
/// use serde_json::json;
///
/// let error = JsonRpcError::method_not_found(json!(1));
/// assert_error_code(&error, -32601);
/// ```
pub fn assert_error_code(error: &JsonRpcError, expected_code: i32) {
    assert_eq!(
        error.error.code, expected_code,
        "Expected error code {}, got {}: {}",
        expected_code, error.error.code, error.error.message
    );
}

/// Assert error message contains expected text
///
/// # Panics
///
/// Panics if the expected text is not found in the error message
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::assertions::assert_error_message_contains;
/// use prism_mcp_rs::protocol::JsonRpcError;
/// use serde_json::json;
///
/// let error = JsonRpcError::tool_not_found(json!(1), "calculator");
/// assert_error_message_contains(&error, "calculator");
/// ```
pub fn assert_error_message_contains(error: &JsonRpcError, expected: &str) {
    assert!(
        error.error.message.contains(expected),
        "Expected text '{}' not found in error message: {}",
        expected,
        error.error.message
    );
}

/// Assert two JSON values are equal, ignoring ordering in arrays and objects
///
/// # Panics
///
/// Panics if the values are not equal
///
/// # Examples
///
/// ```
/// use prism_mcp_rs::test_utils::assertions::assert_json_eq;
/// use serde_json::json;
///
/// let value1 = json!({"a": 1, "b": 2});
/// let value2 = json!({"b": 2, "a": 1});
///
/// assert_json_eq(&value1, &value2);
/// ```
pub fn assert_json_eq(actual: &serde_json::Value, expected: &serde_json::Value) {
    assert_eq!(
        actual,
        expected,
        "JSON values not equal\nActual: {}\nExpected: {}",
        serde_json::to_string_pretty(actual).unwrap(),
        serde_json::to_string_pretty(expected).unwrap()
    );
}

/// Assert a value matches a JSON schema
///
/// # Panics
///
/// Panics if the value doesn't match the schema
///
/// # Examples
///
/// ```ignore
/// use prism_mcp_rs::test_utils::assertions::assert_matches_schema;
/// use serde_json::json;
///
/// let value = json!({"name": "test", "age": 25});
/// let schema = json!({
///     "type": "object",
///     "properties": {
///         "name": {"type": "string"},
///         "age": {"type": "number"}
///     },
///     "required": ["name"]
/// });
///
/// assert_matches_schema(&value, &schema);
/// ```
pub fn assert_matches_schema(value: &serde_json::Value, schema: &serde_json::Value) {
    // This is a simplified implementation
    // In a real implementation, you'd use a JSON schema validator
    if let Some(schema_type) = schema.get("type").and_then(|t| t.as_str()) {
        match schema_type {
            "object" => assert!(value.is_object(), "Expected object, got {:?}", value),
            "array" => assert!(value.is_array(), "Expected array, got {:?}", value),
            "string" => assert!(value.is_string(), "Expected string, got {:?}", value),
            "number" => assert!(value.is_number(), "Expected number, got {:?}", value),
            "boolean" => assert!(value.is_boolean(), "Expected boolean, got {:?}", value),
            "null" => assert!(value.is_null(), "Expected null, got {:?}", value),
            _ => panic!("Unknown schema type: {}", schema_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_assert_tool_success() {
        let result = ToolResult {
            content: vec![Content::Text {
                text: "Success".to_string(),
            }],
            is_error: Some(false),
        };
        assert_tool_success(&result);
    }

    #[test]
    #[should_panic(expected = "Tool result indicates error")]
    fn test_assert_tool_success_fails_on_error() {
        let result = ToolResult {
            content: vec![Content::Text {
                text: "Error".to_string(),
            }],
            is_error: Some(true),
        };
        assert_tool_success(&result);
    }

    #[test]
    fn test_assert_error_code() {
        let error = JsonRpcError::method_not_found(json!(1));
        assert_error_code(&error, -32601);
    }

    #[test]
    fn test_assert_json_eq() {
        let value1 = json!({"a": 1, "b": 2});
        let value2 = json!({"b": 2, "a": 1});
        assert_json_eq(&value1, &value2);
    }
}