.PHONY: build run dev test clean install help

# Variables
BINARY_NAME = mcp-inspector
CARGO = cargo
RUST_LOG ?= info

# Default target
all: build

## help: Display this help message
help:
	@echo "MCP Inspector - Build and Run Commands"
	@echo ""
	@echo "Usage:"
	@echo "  make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^##' Makefile | sed 's/## /  /'

## build: Build the inspector in release mode
build:
	@echo "Building MCP Inspector..."
	$(CARGO) build --release
	@echo "Build complete! Binary at: target/release/$(BINARY_NAME)"

## dev: Run in development mode with hot reload
dev:
	@echo "Starting MCP Inspector in development mode..."
	RUST_LOG=debug cargo watch -x run

## run: Run the inspector
run: build
	@echo "Starting MCP Inspector..."
	RUST_LOG=$(RUST_LOG) ./target/release/$(BINARY_NAME)

## run-debug: Run with debug logging
run-debug:
	@echo "Starting MCP Inspector with debug logging..."
	RUST_LOG=debug $(CARGO) run

## test: Run tests
test:
	@echo "Running tests..."
	$(CARGO) test

## clean: Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean

## install: Install the inspector globally
install: build
	@echo "Installing MCP Inspector..."
	$(CARGO) install --path .
	@echo "Installation complete! Run 'mcp-inspector' from anywhere."

## check: Check code for errors without building
check:
	@echo "Checking code..."
	$(CARGO) check

## fmt: Format the code
fmt:
	@echo "Formatting code..."
	$(CARGO) fmt

## clippy: Run clippy linter
clippy:
	@echo "Running clippy..."
	$(CARGO) clippy -- -D warnings

## serve: Start a simple HTTP server for frontend development
serve:
	@echo "Starting frontend development server on http://localhost:8080"
	python3 -m http.server 8080 --directory static/