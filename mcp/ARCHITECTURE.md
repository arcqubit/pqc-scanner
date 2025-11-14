# MCP Server Architecture

Technical architecture documentation for the PQC Scanner MCP implementation.

## Overview

The PQC Scanner MCP server bridges AI assistants (like Claude) with the Rust-based quantum cryptography scanner through the Model Context Protocol 2025-11 specification.

## Architecture Diagram

```
┌──────────────────────────────────────────────────────────────┐
│                     AI Assistant (Claude)                     │
│  - Natural language interaction                              │
│  - Tool discovery and selection                              │
│  - Result interpretation                                      │
└────────────────┬─────────────────────────────────────────────┘
                 │
                 │ MCP Protocol (STDIO/JSON-RPC)
                 │
┌────────────────▼─────────────────────────────────────────────┐
│                    MCP Server (Node.js)                       │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  Tool Registry                                         │  │
│  │  - Load JSON tool definitions                          │  │
│  │  - Validate schemas                                    │  │
│  │  - Expose tool metadata                                │  │
│  └────────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  Request Handler                                       │  │
│  │  - ListTools: Return available tools                   │  │
│  │  - CallTool: Execute tool with arguments              │  │
│  │  - Validate inputs against schemas                     │  │
│  └────────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  Tool Executors                                        │  │
│  │  ├─ scanCode()       ─────────┐                        │  │
│  │  ├─ analyzeFile()    ─────────┤                        │  │
│  │  ├─ getRemediation() ─────────┤                        │  │
│  │  └─ validateCompliance() ─────┤                        │  │
│  └────────────────────────────────┼────────────────────────┘  │
└─────────────────────────────────────┼───────────────────────┘
                                      │
                                      │ Subprocess (spawn)
                                      │
┌─────────────────────────────────────▼───────────────────────┐
│              PQC Scanner (Rust + Cargo)                      │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  scan_directory example                                │  │
│  │  - Parse CLI arguments                                 │  │
│  │  - Scan source files                                   │  │
│  │  - Detect crypto patterns                              │  │
│  │  - Generate SC-13/OSCAL reports                        │  │
│  └────────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  Core Scanner Library                                  │  │
│  │  - Multi-language parsing                              │  │
│  │  - Pattern detection (RSA, ECDSA, MD5, etc.)          │  │
│  │  - Compliance scoring                                  │  │
│  │  - Report generation                                   │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
```

## Component Breakdown

### 1. Tool Definitions (`tools/*.json`)

File-based tool descriptors following JSON Schema:

```javascript
{
  "tool_id": "scan_code",           // Unique identifier
  "name": "Human-readable name",
  "description": "What the tool does",
  "input_schema": {                  // JSON Schema for inputs
    "type": "object",
    "properties": { ... },
    "required": [ ... ]
  },
  "output_schema": {                 // JSON Schema for outputs
    "type": "object",
    "properties": { ... }
  },
  "metadata": {                      // Tool metadata
    "cost_estimate": 0.001,
    "latency": "100-2000ms",
    "category": "security"
  }
}
```

**Benefits:**
- Declarative tool definition
- Schema validation
- Self-documenting API
- Easy to extend

### 2. MCP Server (`src/index.js`)

Node.js server implementing MCP specification:

#### Server Initialization
```javascript
const server = new Server(
  {
    name: 'pqc-scanner',
    version: '1.0.0',
  },
  {
    capabilities: {
      tools: {},  // Supports tool invocation
    },
  }
);
```

#### Request Handlers

**ListTools**: Returns available tools
```javascript
server.setRequestHandler(ListToolsRequestSchema, async () => {
  return {
    tools: TOOLS.map(t => ({
      name: t.tool_id,
      description: t.description,
      inputSchema: t.input_schema
    }))
  };
});
```

**CallTool**: Executes tool with validation
```javascript
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  // Route to appropriate handler
  switch (name) {
    case 'scan_code':
      result = await scanCode(args);
      break;
    // ...
  }

  return {
    content: [{ type: 'text', text: JSON.stringify(result) }]
  };
});
```

### 3. Scanner Execution

Tools execute the Rust scanner via subprocess:

```javascript
async function executeScanner(args) {
  return new Promise((resolve, reject) => {
    const proc = spawn('cargo', [
      'run', '--quiet', '--example', 'scan_directory', '--', ...args
    ], {
      cwd: PROJECT_ROOT,
      stdio: ['ignore', 'pipe', 'pipe']
    });

    // Capture stdout/stderr
    // Handle exit codes
    // Parse JSON output
  });
}
```

**Flow:**
1. Node.js receives tool call
2. Validates input against schema
3. Constructs cargo command
4. Spawns subprocess
5. Waits for completion
6. Parses JSON output
7. Returns structured result

### 4. Tool Implementations

#### scanCode
```javascript
async function scanCode(args) {
  const { path, format = 'sc13' } = args;
  const outputFile = `/tmp/pqc-scan-${Date.now()}.json`;

  // Execute scanner
  await executeScanner([
    '--path', path,
    '--output', outputFile,
    '--format', format
  ]);

  // Read and parse report
  const report = JSON.parse(readFileSync(outputFile));

  // Format response
  return {
    report,
    summary: { ... },
    vulnerabilities: [ ... ]
  };
}
```

#### analyzeFile
```javascript
async function analyzeFile(args) {
  const { source_code, language } = args;

  // Write source to temp file
  const tempFile = `/tmp/input.${getExtension(language)}`;
  writeFileSync(tempFile, source_code);

  // Scan temp file
  const result = await scanCode({ path: tempFile });

  // Return formatted results
  return {
    vulnerabilities: result.vulnerabilities,
    stats: result.summary,
    risk_score: result.summary.risk_score
  };
}
```

#### getRemediation
```javascript
async function getRemediation(args) {
  const { vulnerability_type, language } = args;

  // Static remediation database
  const remediations = {
    'RSA': {
      pqc_alternative: {
        algorithm: 'CRYSTALS-Kyber',
        code_example: '...'
      },
      migration_steps: [ ... ]
    }
  };

  return remediations[vulnerability_type];
}
```

#### validateCompliance
```javascript
async function validateCompliance(args) {
  const { path, target_score, strict_mode } = args;

  // Run scan
  const result = await scanCode({ path });

  // Evaluate compliance
  const compliant = result.summary.compliance_score >= target_score;

  return {
    compliant,
    compliance_score: result.summary.compliance_score,
    blocking_issues: [ ... ]
  };
}
```

## Data Flow

### Scan Code Example

```
1. User: "Scan /path/to/code for vulnerabilities"
   ↓
2. Claude: Selects scan_code tool
   ↓
3. MCP Request:
   {
     "method": "tools/call",
     "params": {
       "name": "scan_code",
       "arguments": {
         "path": "/path/to/code",
         "format": "sc13"
       }
     }
   }
   ↓
4. MCP Server: Validates input schema
   ↓
5. Execute Scanner:
   $ cargo run --example scan_directory -- \
     --path /path/to/code \
     --output /tmp/pqc-scan-123.json
   ↓
6. Scanner: Analyzes code, generates report
   ↓
7. MCP Server: Reads report, formats response
   ↓
8. MCP Response:
   {
     "content": [{
       "type": "text",
       "text": "{\"report\": {...}, \"summary\": {...}}"
     }]
   }
   ↓
9. Claude: Interprets results, presents to user
   ↓
10. User sees: "Found 15 vulnerabilities..."
```

## Transport Layer

### STDIO Transport

The server uses STDIO for communication:

```javascript
const transport = new StdioServerTransport();
await server.connect(transport);
```

**Characteristics:**
- JSON-RPC over stdin/stdout
- Process-based isolation
- No network exposure
- Managed by MCP client (Claude)

**Message Format:**
```json
// Request
{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {...}}

// Response
{"jsonrpc": "2.0", "id": 1, "result": {...}}
```

## Error Handling

### Input Validation
```javascript
// Schema validation happens automatically
// Invalid inputs are rejected before execution
```

### Execution Errors
```javascript
try {
  result = await scanCode(args);
} catch (error) {
  return {
    content: [{
      type: 'text',
      text: `Error: ${error.message}`
    }],
    isError: true
  };
}
```

### Scanner Errors
```javascript
proc.on('close', (code) => {
  // Code 1 = vulnerabilities found (expected)
  if (code === 0 || code === 1) {
    resolve({ stdout, stderr, code });
  } else {
    reject(new Error(`Scanner failed: ${stderr}`));
  }
});
```

## Performance Considerations

### Subprocess Overhead
- Scanner startup: ~100-500ms
- Analysis: Variable (depends on code size)
- Total latency: 100-2000ms

### Caching Opportunities
- Tool definitions cached in memory
- Could cache scan results (not implemented)
- Subprocess pool (not implemented)

### Optimization Strategies
1. Keep scanner binary built
2. Use `--quiet` flag to reduce output
3. Temporary file cleanup
4. Async operation support

## Security Model

### Isolation
- Scanner runs as subprocess
- Separate process space
- No network access required
- File system access controlled by user

### Input Validation
- JSON Schema validation
- Path validation (no path traversal)
- Language enumeration (prevent injection)

### Output Sanitization
- Structured JSON only
- No arbitrary code execution
- Error messages sanitized

## Extensibility

### Adding New Tools

1. Create tool definition:
```bash
cat > mcp/tools/new_tool.json
```

2. Add to TOOLS array:
```javascript
const TOOLS = [..., 'new_tool']
```

3. Implement handler:
```javascript
async function newTool(args) {
  // Implementation
}
```

4. Add to switch statement:
```javascript
case 'new_tool':
  result = await newTool(args);
  break;
```

### Supported Transports

Current: STDIO
Future: HTTP with SSE, WebSocket

### Future Enhancements

- [ ] Async job support for large scans
- [ ] Streaming results
- [ ] Caching layer
- [ ] Batch operations
- [ ] Registry integration
- [ ] Webhook notifications

## Testing

### Manual Testing
```bash
# Start server
npm start

# Send MCP requests via stdin
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | npm start
```

### Integration Testing
```javascript
// Test tool execution
const result = await scanCode({
  path: 'samples/legacy-banking/src',
  format: 'sc13'
});

assert(result.summary.vulnerabilities_found > 0);
```

## Monitoring

### Logging
```javascript
console.error('PQC Scanner MCP Server running');  // Startup
console.error(`Tool called: ${name}`);             // Tool invocation
```

### Metrics (Future)
- Tool invocation count
- Execution latency
- Error rates
- Cache hit rates

## References

- [MCP Specification 2025-11](https://modelcontextprotocol.io)
- [PQC Scanner Documentation](../README.md)
- [Tool Definitions](tools/)
- [Quick Start Guide](QUICKSTART.md)
