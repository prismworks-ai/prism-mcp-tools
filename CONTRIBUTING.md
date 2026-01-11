# Contributing to Prism MCP Tools

We welcome contributions to the Prism MCP Tools project! This repository contains production implementations and developer tools for MCP development.

## 🎯 What Belongs Here

### Contribute to This Repository
- **Production Servers**: Complete, deployable MCP server implementations
- **Production Clients**: Full-featured MCP client applications
- **Testing Utilities**: Mock servers, clients, assertion helpers
- **Development Tools**: Benchmarks, debugging tools, performance analyzers
- **Templates**: Starter projects and boilerplate for MCP applications
- **CLI Tools**: Command-line utilities for MCP development

### Contribute to [prism-mcp-rs](https://github.com/prismworks-ai/prism-mcp-rs)
- **SDK Changes**: Protocol implementation, transport layers, core types
- **SDK Bug Fixes**: Issues in the core SDK
- **SDK Documentation**: API documentation, architectural guides
- **Simple Examples**: Focused demonstrations of SDK features

## Getting Started

1. Fork the repository
2. Clone your fork
3. Create a feature branch
4. Make your changes
5. Submit a pull request

## Development Setup

```bash
# Clone the repository
git clone https://github.com/prismworks-ai/prism-mcp-tools.git
cd prism-mcp-tools

# Build a specific implementation
cd prism-mcp-servers/database-server
cargo build --release

# Run tests
cargo test

# Run with all features
cargo build --all-features
```

## Project Structure

```
prism-mcp-tools/
├── prism-test-utils/      # Testing utilities crate
├── prism-mcp-servers/     # Production server implementations
│   ├── database-server/
│   ├── http-server/
│   └── websocket-server/
├── prism-mcp-clients/     # Production client implementations
│   ├── http-client/
│   └── websocket-client/
└── prism-mcp-tools/       # Development tools
    └── transport-benchmark/
```

## Adding a New Implementation

### For a New Server

1. Create directory: `prism-mcp-servers/your-server/`
2. Add `Cargo.toml` with proper metadata
3. Implement in `src/main.rs`
4. Add README.md with:
   - Purpose and features
   - Configuration options
   - Usage examples
   - Deployment instructions

### For a New Client

1. Create directory: `prism-mcp-clients/your-client/`
2. Add `Cargo.toml` with proper metadata
3. Implement in `src/main.rs`
4. Add README.md with usage instructions

### For a New Tool

1. Create directory: `prism-mcp-tools/your-tool/`
2. Add `Cargo.toml` with proper metadata
3. Implement the tool
4. Document its purpose and usage

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use `cargo clippy` to catch common issues
- Write descriptive commit messages
- Add tests for new functionality
- Update documentation as needed
- Each implementation should be self-contained

## Testing Guidelines

- Write unit tests for core functionality
- Add integration tests for network operations
- Test with different transport types
- Ensure examples in README work
- Test error handling paths

## Pull Request Process

1. Ensure all tests pass: `cargo test --all-features`
2. Format code: `cargo fmt --all`
3. Run clippy: `cargo clippy --all-features`
4. Update relevant README files
5. Submit PR with clear description:
   - What implementation/tool you're adding
   - Why it's useful
   - Any special features or considerations

## Documentation

- Each implementation needs its own README
- Include configuration examples
- Provide deployment instructions
- Add usage examples
- Document any environment variables

## License

All contributions must be compatible with the MIT license.

## Questions?

- Open an issue in this repository for tool/implementation questions
- For SDK questions, use [prism-mcp-rs issues](https://github.com/prismworks-ai/prism-mcp-rs/issues)
- Join our [Discord community](https://discord.gg/prismworks) for discussion