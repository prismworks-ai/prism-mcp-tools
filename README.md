# Prism MCP Tools

Production-ready implementations, developer tools, and testing utilities for MCP development in Rust.

## 🎯 Repository Purpose

**prism-mcp-tools** contains production-ready applications and developer tools that **use** the [prism-mcp-rs SDK](https://github.com/prismworks-ai/prism-mcp-rs):

- ✅ **Production Applications** - Complete, deployable MCP servers and clients
- ✅ **Testing Utilities** - Mock servers, clients, and assertion helpers for testing MCP apps
- ✅ **Development Tools** - Benchmarking, debugging, and performance analysis tools
- ✅ **Templates** - Starting points for building your own MCP applications

⚠️ **Need the SDK?** The core MCP protocol implementation is in [prism-mcp-rs](https://github.com/prismworks-ai/prism-mcp-rs)

## 🔗 Relationship to prism-mcp-rs

| Repository | Purpose | Use When |
|------------|---------|----------|
| [prism-mcp-rs](https://github.com/prismworks-ai/prism-mcp-rs) | Core MCP SDK | You need the protocol implementation, types, or transport layers |
| **prism-mcp-tools** (this repo) | Production apps & tools | You need complete applications, testing utilities, or templates |

## Repository Structure

### 🧪 Testing Utilities
- **`prism-test-utils/`** - Testing utilities for MCP SDK development
  - Mock servers and clients
  - Assertion helpers
  - Test harness (coming soon)

### 🚀 Production Servers
- **`prism-mcp-servers/`** - Production-ready MCP server implementations
  - `database-server/` - In-memory database server with CRUD operations
  - `http-server/` - HTTP/SSE server implementation
  - `http2-server/` - High-performance HTTP/2 server
  - `websocket-server/` - Real-time WebSocket server
  - `enhanced-echo-server/` - Feature-rich echo server
  - `plugin-server/` - Server with plugin support

### 💻 Production Clients  
- **`prism-mcp-clients/`** - Production-ready MCP client implementations
  - `http-client/` - Robust HTTP client
  - `advanced-http-client/` - Feature-rich HTTP client with streaming
  - `websocket-client/` - WebSocket client with auto-reconnection
  - `conservative-http-demo/` - Memory-efficient HTTP client

### 🛠️ Development Tools
- **`prism-mcp-tools/`** - MCP development and debugging tools
  - `transport-benchmark/` - Performance testing for all transport types

### 📦 Future Components
- **`prism-cli/`** - The `prs` command-line tool for MCP development (coming soon)

## Installation

### Using Test Utilities

Add as a dev-dependency in your `Cargo.toml`:

```toml
[dev-dependencies]
prism-test-utils = { git = "https://github.com/prismworks-ai/prism-mcp-tools" }
```

### Using Production Implementations

Each implementation is a standalone crate. Clone and build:

```bash
# Clone the repository
git clone https://github.com/prismworks-ai/prism-mcp-tools
cd prism-mcp-tools

# Build a specific server
cd prism-mcp-servers/database-server
cargo build --release

# Run the server
cargo run --release
```

## Quick Start Examples

### Running a Production Server

```bash
# Database Server
cd prism-mcp-servers/database-server
cargo run --release

# WebSocket Server
cd prism-mcp-servers/websocket-server
cargo run --release
```

### Running a Production Client

```bash
# HTTP Client
cd prism-mcp-clients/http-client
cargo run --release -- --server-url http://localhost:8080

# WebSocket Client
cd prism-mcp-clients/websocket-client  
cargo run --release -- --server-url ws://localhost:3000
```

### Running Development Tools

```bash
# Transport Benchmark
cd prism-mcp-tools/transport-benchmark
cargo run --release
```

## Using as Templates

These implementations are designed to be used as templates for your own projects:

1. Copy the implementation you need
2. Update the `Cargo.toml` with your project details
3. Modify the source code for your specific requirements
4. Deploy as a standalone service

## Contributing

Contributions are welcome! Please follow these guidelines:

- **SDK Development**: Contribute to [prism-mcp-rs](https://github.com/prismworks-ai/prism-mcp-rs)
- **Production Implementations**: Contribute here
- **Test Utilities**: Enhance the prism-test-utils crate
- **New Tools**: Add to prism-mcp-tools/

## Repository Links

- [MCP SDK (prism-mcp-rs)](https://github.com/prismworks-ai/prism-mcp-rs) - Core SDK
- [Developer Resources (this repo)](https://github.com/prismworks-ai/prism-mcp-tools) - Tools & implementations
- [Documentation](https://docs.prismworks.ai)

## License

MIT License