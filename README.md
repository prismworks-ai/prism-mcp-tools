# Prism MCP Tools

Examples, diagnostics and interoperability fixtures for Rust MCP implementations. This repository supports the Prism MCP Rust SDK; it is not a catalogue of validated production applications.

## Current state

The server/client examples and test utilities exist in source. The reviewed database example cannot resolve its local SDK dependency: its manifest requests `^0.1.0`, while the sibling SDK is `3.0.1`. API compatibility must be reviewed after dependency repair. The inspector's SDK dependency is commented out. No fresh-clone working suite or Databridle adapter is established by this review.

## Source inventory

- `prism-mcp-servers/`: `database_server`, `http_server`, `http2_server`, `websocket_server`, `enhanced_echo_server`, `plugin_server`.
- `prism-mcp-clients/`: `http_client`, `advanced_http_client`, `websocket_client`, `conservative_http_demo`.
- `prism-test-utils/`: test utility source.
- `transport_benchmark/` and `mcp-inspector/`: diagnostic source requiring build/behavior validation.

## Direction and implementation

Keep a small supported example set, repair dependencies and APIs, then publish reproducible standalone and optional protected-action fixtures. Demonstrate denied actions, authentication/audience handling, replay and unknown outcomes. Do not make Databridle a requirement for general MCP use.

See [product strategy](docs/product-strategy.md) and [implementation plan](docs/implementation-plan.md). Existing license terms remain unchanged. The previous README is retained in the dated archive as historical material.
