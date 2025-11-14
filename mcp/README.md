# PQC Scanner MCP Server

Model Context Protocol (MCP) server for the ArcQubit PQC Scanner, enabling AI assistants to analyze code for quantum-vulnerable cryptographic algorithms.

## Overview

This MCP server exposes the PQC Scanner's capabilities through a standardized protocol, allowing AI assistants like Claude to:
- Scan code for cryptographic vulnerabilities
- Generate NIST 800-53 SC-13 compliance reports
- Validate quantum-safety of codebases
- Provide remediation recommendations

## MCP Specification

Implements **MCP 2025-11** with the following capabilities:
- ✅ Tool discovery via JSON descriptors
- ✅ STDIO transport for local development
- ✅ Structured input/output schemas
- ✅ Async operation support
- ✅ Security and compliance validation

## Available Tools

### 1. `scan_code`
Analyzes source code directory for cryptographic vulnerabilities.

**Input:**
```json
{
  "path": "/path/to/code",
  "language": "javascript",  // optional, auto-detected
  "format": "sc13"           // or "oscal"
}
```

**Output:**
```json
{
  "report": { ... },         // Full compliance report
  "summary": {
    "files_scanned": 5,
    "lines_scanned": 378,
    "vulnerabilities_found": 40,
    "compliance_score": 28,
    "risk_score": 78
  },
  "vulnerabilities": [ ... ]
}
```

### 2. `analyze_file`
Analyzes a single source file for vulnerabilities.

**Input:**
```json
{
  "source_code": "const hash = crypto.createHash('md5');",
  "language": "javascript",
  "filename": "crypto.js"  // optional
}
```

**Output:**
```json
{
  "vulnerabilities": [ ... ],
  "stats": { ... },
  "risk_score": 85,
  "recommendations": [ ... ]
}
```

### 3. `get_remediation`
Provides detailed remediation recommendations.

**Input:**
```json
{
  "vulnerability_type": "RSA",
  "language": "python",
  "context": "key generation"  // optional
}
```

**Output:**
```json
{
  "vulnerability": "RSA",
  "severity": "high",
  "recommendation": "Migrate to CRYSTALS-Kyber",
  "pqc_alternative": {
    "algorithm": "CRYSTALS-Kyber (NIST)",
    "nist_status": "Standardized",
    "code_example": "..."
  },
  "migration_steps": [ ... ]
}
```

### 4. `validate_compliance`
Validates code against NIST 800-53 SC-13 requirements.

**Input:**
```json
{
  "path": "/path/to/code",
  "target_score": 80,
  "strict_mode": false
}
```

**Output:**
```json
{
  "compliant": false,
  "compliance_score": 28,
  "target_score": 80,
  "control_status": "notsatisfied",
  "blocking_issues": [ ... ],
  "recommendations": [ ... ]
}
```

## Installation

### Prerequisites
- Node.js 18+
- Rust toolchain
- PQC Scanner (parent project)

### Setup

```bash
cd mcp
npm install
```

## Usage

### With Claude Desktop

Add to your Claude Desktop configuration (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "pqc-scanner": {
      "command": "node",
      "args": [
        "/path/to/pqc-scanner/mcp/src/index.js"
      ]
    }
  }
}
```

**Location:**
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`

### With Other MCP Clients

The server uses STDIO transport and follows MCP 2025-11 specification:

```bash
# Run the server
node src/index.js

# Or use npm
npm start
```

## Example Conversations

### Scan Code for Vulnerabilities

**User:** "Scan the legacy-banking sample for crypto vulnerabilities"

**Claude:** Uses `scan_code` tool:
```json
{
  "path": "samples/legacy-banking/src",
  "format": "sc13"
}
```

**Result:** Complete compliance report with 15 vulnerabilities detected

### Validate Compliance

**User:** "Check if our codebase meets NIST standards with a target score of 80"

**Claude:** Uses `validate_compliance` tool:
```json
{
  "path": "src/",
  "target_score": 80,
  "strict_mode": true
}
```

**Result:** Compliance status with blocking issues and recommendations

### Get Remediation Guidance

**User:** "How do I migrate from RSA to post-quantum cryptography in Python?"

**Claude:** Uses `get_remediation` tool:
```json
{
  "vulnerability_type": "RSA",
  "language": "python"
}
```

**Result:** Step-by-step migration guide with code examples

## Tool Metadata

Each tool includes metadata for cost estimation and categorization:

```json
{
  "metadata": {
    "cost_estimate": 0.001,
    "latency": "100-2000ms",
    "category": "security",
    "tags": ["cryptography", "quantum-safe", "compliance"]
  }
}
```

## Development

### Testing

```bash
# Run manual tests
npm test

# Validate MCP compliance
npm run validate
```

### Tool Discovery

Tools are defined in `tools/*.json` using JSON Schema:

```
mcp/
├── tools/
│   ├── scan_code.json
│   ├── analyze_file.json
│   ├── get_remediation.json
│   └── validate_compliance.json
├── resources/
│   └── schemas/
└── src/
    └── index.js
```

## Architecture

```
┌─────────────┐          ┌──────────────┐          ┌─────────────┐
│   Claude    │ ◄──MCP──►│  MCP Server  │ ◄────────►│ PQC Scanner │
│  Assistant  │          │  (Node.js)   │          │   (Rust)    │
└─────────────┘          └──────────────┘          └─────────────┘
                                │
                                ▼
                         ┌──────────────┐
                         │ Tool Defs    │
                         │ (JSON)       │
                         └──────────────┘
```

1. **Claude** requests available tools via MCP
2. **MCP Server** loads tool definitions from JSON files
3. **Claude** invokes tools with structured arguments
4. **MCP Server** executes Rust scanner via subprocess
5. **Results** returned in structured format to Claude

## Security

- Tools run in subprocess isolation
- Input validation via JSON Schema
- Audit logging of all invocations
- No credential handling (scanner reads local files only)

## Troubleshooting

### Server Not Starting

```bash
# Check Node version
node --version  # Should be 18+

# Verify dependencies
npm install

# Check MCP SDK
npm list @modelcontextprotocol/sdk
```

### Tool Execution Fails

```bash
# Ensure scanner is built
cd ..
cargo build --release

# Test scanner directly
cargo run --example scan_directory -- --help
```

### Claude Not Finding Server

1. Verify configuration path
2. Restart Claude Desktop
3. Check Claude Desktop logs
4. Ensure absolute paths in config

## Resources

- [MCP Specification](https://modelcontextprotocol.io)
- [PQC Scanner Documentation](../README.md)
- [NIST PQC](https://csrc.nist.gov/projects/post-quantum-cryptography)

## License

MIT - Same as parent PQC Scanner project
