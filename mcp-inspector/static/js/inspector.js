// MCP Inspector Frontend JavaScript

class MCPInspector {
    constructor() {
        this.connected = false;
        this.currentTool = null;
        this.ws = null;
        this.tools = [];
        this.sessions = [];
        
        this.initializeEventListeners();
        this.loadSessions();
    }

    initializeEventListeners() {
        // Connection controls
        document.getElementById('connectBtn').addEventListener('click', () => this.toggleConnection());
        document.getElementById('serverUrl').addEventListener('keypress', (e) => {
            if (e.key === 'Enter') this.toggleConnection();
        });

        // Tool invocation controls
        document.getElementById('invokeBtn')?.addEventListener('click', () => this.invokeTool());
        document.getElementById('validateBtn')?.addEventListener('click', () => this.validateInput());
        document.getElementById('formatBtn')?.addEventListener('click', () => this.formatInput());
        document.getElementById('clearBtn')?.addEventListener('click', () => this.clearInput());

        // Code tabs
        document.querySelectorAll('.code-tab').forEach(tab => {
            tab.addEventListener('click', (e) => this.switchCodeTab(e.target));
        });

        // Copy code button
        document.getElementById('copyCodeBtn')?.addEventListener('click', () => this.copyCode());
    }

    async toggleConnection() {
        if (this.connected) {
            await this.disconnect();
        } else {
            await this.connect();
        }
    }

    async connect() {
        const url = document.getElementById('serverUrl').value;
        const transport = document.getElementById('transportSelect').value;
        
        if (!url) {
            alert('Please enter a server URL');
            return;
        }

        try {
            const response = await fetch('/api/connect', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ url, transport })
            });

            const data = await response.json();
            
            if (data.success) {
                this.connected = true;
                this.updateConnectionStatus(true, url);
                await this.loadTools();
                this.connectWebSocket();
                
                // Update UI
                document.getElementById('connectBtn').textContent = 'Disconnect';
                document.getElementById('welcomeScreen').style.display = 'none';
            } else {
                alert(`Connection failed: ${data.message}`);
            }
        } catch (error) {
            console.error('Connection error:', error);
            alert('Failed to connect to server');
        }
    }

    async disconnect() {
        try {
            await fetch('/api/disconnect', { method: 'POST' });
            this.connected = false;
            this.updateConnectionStatus(false);
            this.disconnectWebSocket();
            
            // Update UI
            document.getElementById('connectBtn').textContent = 'Connect';
            document.getElementById('toolsList').innerHTML = '<p class="empty-state">Connect to a server to see available tools</p>';
            document.getElementById('welcomeScreen').style.display = 'block';
            document.getElementById('toolDetails').style.display = 'none';
        } catch (error) {
            console.error('Disconnect error:', error);
        }
    }

    updateConnectionStatus(connected, url = null) {
        const statusIndicator = document.querySelector('.status-indicator');
        const statusText = document.querySelector('.status-text');
        
        if (connected) {
            statusIndicator.classList.remove('disconnected');
            statusIndicator.classList.add('connected');
            statusText.textContent = `Connected to ${new URL(url).hostname}`;
        } else {
            statusIndicator.classList.remove('connected');
            statusIndicator.classList.add('disconnected');
            statusText.textContent = 'Disconnected';
        }
    }

    async loadTools() {
        try {
            const response = await fetch('/api/tools');
            this.tools = await response.json();
            this.renderTools();
        } catch (error) {
            console.error('Failed to load tools:', error);
        }
    }

    renderTools() {
        const toolsList = document.getElementById('toolsList');
        
        if (this.tools.length === 0) {
            toolsList.innerHTML = '<p class="empty-state">No tools available</p>';
            return;
        }

        toolsList.innerHTML = this.tools.map(tool => `
            <div class="tool-item" data-tool="${tool.name}">
                <div class="tool-name">${tool.name}</div>
                ${tool.description ? `<div class="tool-desc">${tool.description}</div>` : ''}
            </div>
        `).join('');

        // Add click handlers
        toolsList.querySelectorAll('.tool-item').forEach(item => {
            item.addEventListener('click', () => this.selectTool(item.dataset.tool));
        });
    }

    async selectTool(toolName) {
        // Update active state
        document.querySelectorAll('.tool-item').forEach(item => {
            item.classList.toggle('active', item.dataset.tool === toolName);
        });

        // Load tool details
        try {
            const response = await fetch(`/api/tools/${toolName}`);
            const tool = await response.json();
            this.currentTool = tool;
            this.displayTool(tool);
        } catch (error) {
            console.error('Failed to load tool:', error);
        }
    }

    displayTool(tool) {
        // Show tool details
        document.getElementById('welcomeScreen').style.display = 'none';
        document.getElementById('toolDetails').style.display = 'block';
        
        // Update tool info
        document.getElementById('toolName').textContent = tool.name;
        document.getElementById('toolDescription').textContent = tool.description || 'No description available';
        
        // Display schema
        document.getElementById('schemaCode').textContent = JSON.stringify(tool.input_schema, null, 2);
        
        // Generate example input
        const exampleInput = this.generateExampleInput(tool.input_schema);
        document.getElementById('inputJson').value = JSON.stringify(exampleInput, null, 2);
        
        // Generate code examples
        this.generateCodeExamples(tool);
    }

    generateExampleInput(schema) {
        if (schema.type === 'object' && schema.properties) {
            const example = {};
            for (const [key, prop] of Object.entries(schema.properties)) {
                if (prop.type === 'string') {
                    example[key] = prop.example || 'example string';
                } else if (prop.type === 'number' || prop.type === 'integer') {
                    example[key] = prop.example || 42;
                } else if (prop.type === 'boolean') {
                    example[key] = prop.example || true;
                } else if (prop.type === 'array') {
                    example[key] = prop.example || [];
                } else if (prop.type === 'object') {
                    example[key] = this.generateExampleInput(prop);
                }
            }
            return example;
        }
        return {};
    }

    async invokeTool() {
        if (!this.currentTool) {
            alert('Please select a tool first');
            return;
        }

        const inputJson = document.getElementById('inputJson').value;
        let arguments;
        
        try {
            arguments = JSON.parse(inputJson);
        } catch (error) {
            alert('Invalid JSON input');
            return;
        }

        // Disable invoke button
        const invokeBtn = document.getElementById('invokeBtn');
        invokeBtn.disabled = true;
        invokeBtn.textContent = 'Invoking...';

        try {
            const response = await fetch(`/api/tools/${this.currentTool.name}/invoke`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ arguments })
            });

            const data = await response.json();
            this.displayResponse(data);
        } catch (error) {
            console.error('Invocation error:', error);
            this.displayResponse({
                success: false,
                error: error.message,
                duration_ms: 0
            });
        } finally {
            invokeBtn.disabled = false;
            invokeBtn.textContent = 'Invoke Tool';
        }
    }

    displayResponse(response) {
        const responseMeta = document.getElementById('responseMeta');
        const responseStatus = document.getElementById('responseStatus');
        const responseTime = document.getElementById('responseTime');
        const responseCode = document.getElementById('responseCode');

        responseMeta.style.display = 'block';
        
        if (response.success) {
            responseStatus.className = 'response-status success';
            responseStatus.textContent = '✓ Success';
            responseCode.textContent = JSON.stringify(response.result, null, 2);
        } else {
            responseStatus.className = 'response-status error';
            responseStatus.textContent = '✗ Error';
            responseCode.textContent = response.error || 'Unknown error occurred';
        }
        
        responseTime.textContent = `${response.duration_ms}ms`;
    }

    validateInput() {
        try {
            const input = document.getElementById('inputJson').value;
            JSON.parse(input);
            alert('✓ Valid JSON');
        } catch (error) {
            alert(`✗ Invalid JSON: ${error.message}`);
        }
    }

    formatInput() {
        try {
            const input = document.getElementById('inputJson').value;
            const parsed = JSON.parse(input);
            document.getElementById('inputJson').value = JSON.stringify(parsed, null, 2);
        } catch (error) {
            alert('Cannot format invalid JSON');
        }
    }

    clearInput() {
        document.getElementById('inputJson').value = '';
    }

    generateCodeExamples(tool) {
        const examples = {
            rust: this.generateRustExample(tool),
            python: this.generatePythonExample(tool),
            typescript: this.generateTypeScriptExample(tool)
        };
        
        // Show Rust example by default
        document.getElementById('codeExample').textContent = examples.rust;
        
        // Store examples for tab switching
        this.codeExamples = examples;
    }

    generateRustExample(tool) {
        const args = this.generateExampleInput(tool.input_schema);
        return `use prism_mcp_rs::client::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to MCP server
    let client = Client::connect("http://localhost:8000/mcp").await?;
    
    // Invoke the tool
    let result = client.invoke_tool(
        "${tool.name}",
        json!(${JSON.stringify(args, null, 8).split('\n').join('\n        ')})
    ).await?;
    
    println!("Result: {:#?}", result);
    Ok(())
}`;
    }

    generatePythonExample(tool) {
        const args = this.generateExampleInput(tool.input_schema);
        return `import asyncio
import json
from mcp_client import MCPClient

async def main():
    # Connect to MCP server
    client = MCPClient("http://localhost:8000/mcp")
    await client.connect()
    
    # Invoke the tool
    result = await client.invoke_tool(
        "${tool.name}",
        ${JSON.stringify(args, null, 8).split('\n').join('\n        ')}
    )
    
    print(f"Result: {json.dumps(result, indent=2)}")

if __name__ == "__main__":
    asyncio.run(main())`;
    }

    generateTypeScriptExample(tool) {
        const args = this.generateExampleInput(tool.input_schema);
        return `import { MCPClient } from '@prismworks/mcp-client';

async function main() {
    // Connect to MCP server
    const client = new MCPClient('http://localhost:8000/mcp');
    await client.connect();
    
    // Invoke the tool
    const result = await client.invokeTool(
        '${tool.name}',
        ${JSON.stringify(args, null, 8).split('\n').join('\n        ')}
    );
    
    console.log('Result:', result);
}

main().catch(console.error);`;
    }

    switchCodeTab(tab) {
        // Update active tab
        document.querySelectorAll('.code-tab').forEach(t => {
            t.classList.toggle('active', t === tab);
        });
        
        // Update code display
        const lang = tab.dataset.lang;
        if (this.codeExamples && this.codeExamples[lang]) {
            document.getElementById('codeExample').textContent = this.codeExamples[lang];
        }
    }

    copyCode() {
        const code = document.getElementById('codeExample').textContent;
        navigator.clipboard.writeText(code).then(() => {
            const btn = document.getElementById('copyCodeBtn');
            const originalText = btn.textContent;
            btn.textContent = 'Copied!';
            setTimeout(() => {
                btn.textContent = originalText;
            }, 2000);
        });
    }

    connectWebSocket() {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws`;
        
        this.ws = new WebSocket(wsUrl);
        
        this.ws.onopen = () => {
            console.log('WebSocket connected');
        };
        
        this.ws.onmessage = (event) => {
            try {
                const message = JSON.parse(event.data);
                this.handleWebSocketMessage(message);
            } catch (error) {
                console.error('WebSocket message error:', error);
            }
        };
        
        this.ws.onerror = (error) => {
            console.error('WebSocket error:', error);
        };
        
        this.ws.onclose = () => {
            console.log('WebSocket disconnected');
            this.ws = null;
        };
    }

    disconnectWebSocket() {
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
    }

    handleWebSocketMessage(message) {
        switch (message.type) {
            case 'ConnectionStatus':
                this.updateConnectionStatus(message.connected, message.url);
                break;
            case 'ToolResponse':
                // Real-time tool response updates
                break;
            case 'MetricsUpdate':
                // Performance metrics updates
                break;
            case 'Error':
                console.error('Server error:', message.message);
                break;
        }
    }

    async loadSessions() {
        try {
            const response = await fetch('/api/sessions');
            this.sessions = await response.json();
            this.renderSessions();
        } catch (error) {
            console.error('Failed to load sessions:', error);
        }
    }

    renderSessions() {
        const sessionsList = document.getElementById('sessionsList');
        
        if (this.sessions.length === 0) {
            sessionsList.innerHTML = '<p class="empty-state">No saved sessions</p>';
            return;
        }

        sessionsList.innerHTML = this.sessions.map(session => `
            <div class="session-item" data-session="${session.id}">
                <div class="session-name">${session.name}</div>
                <div class="session-date">${new Date(session.created_at).toLocaleDateString()}</div>
            </div>
        `).join('');

        // Add click handlers
        sessionsList.querySelectorAll('.session-item').forEach(item => {
            item.addEventListener('click', () => this.loadSession(item.dataset.session));
        });
    }

    async loadSession(sessionId) {
        try {
            const response = await fetch(`/api/sessions/${sessionId}`);
            const session = await response.json();
            
            // Restore connection info
            document.getElementById('serverUrl').value = session.connection_info.url;
            document.getElementById('transportSelect').value = session.connection_info.transport;
            
            // Connect if not already connected
            if (!this.connected) {
                await this.connect();
            }
        } catch (error) {
            console.error('Failed to load session:', error);
        }
    }
}

// Initialize the inspector when the page loads
document.addEventListener('DOMContentLoaded', () => {
    window.inspector = new MCPInspector();
});