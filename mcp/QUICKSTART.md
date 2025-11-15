# MCP Quick Start Guide

Get the PQC Scanner MCP server running in 5 minutes.

## 1. Install Dependencies

```bash
cd mcp
npm install
```

## 2. Test the Server

```bash
# Test that it starts
npm start

# You should see: "PQC Scanner MCP Server running on stdio"
# Press Ctrl+C to stop
```

## 3. Configure Claude Desktop

### macOS

```bash
# Edit config file
nano ~/Library/Application\ Support/Claude/claude_desktop_config.json
```

### Windows

```powershell
# Edit config file
notepad %APPDATA%\Claude\claude_desktop_config.json
```

### Configuration

Add this configuration (update the path to match your installation):

```json
{
  "mcpServers": {
    "pqc-scanner": {
      "command": "node",
      "args": [
        "/absolute/path/to/pqc-scanner/mcp/src/index.js"
      ]
    }
  }
}
```

**Important:** Use the absolute path to the `index.js` file!

To find your path:
```bash
# From the mcp directory
pwd
# Append /src/index.js to the output
```

## 4. Restart Claude Desktop

1. Quit Claude Desktop completely
2. Reopen Claude Desktop
3. The MCP server will start automatically

## 5. Test It Works

In Claude Desktop, try these prompts:

### Test 1: Scan Sample Code

```
Scan the legacy-banking sample for crypto vulnerabilities
```

Claude should use the `scan_code` tool and return vulnerability findings.

### Test 2: Analyze Code Snippet

```
Analyze this code for security issues:

const crypto = require('crypto');
const hash = crypto.createHash('md5');
const key = crypto.generateKeyPairSync('rsa', { modulusLength: 1024 });
```

Claude should use `analyze_file` and detect MD5 and RSA-1024 vulnerabilities.

### Test 3: Get Remediation

```
How do I migrate from RSA-1024 to post-quantum cryptography in JavaScript?
```

Claude should use `get_remediation` and provide migration guidance.

### Test 4: Validate Compliance

```
Check if the legacy-banking sample meets NIST 800-53 SC-13 requirements with a minimum score of 80
```

Claude should use `validate_compliance` and report non-compliance.

## Troubleshooting

### "Server not found" or tools don't appear

1. **Check Configuration Path**
   - Ensure you used absolute path (not relative)
   - Verify the path exists: `ls /path/to/index.js`

2. **Check Node Version**
   ```bash
   node --version  # Should be 18+
   ```

3. **Restart Claude Desktop**
   - Fully quit and reopen (not just close window)

4. **Check Claude Logs** (macOS)
   ```bash
   tail -f ~/Library/Logs/Claude/mcp*.log
   ```

### "Scanner execution failed"

1. **Verify Scanner is Built**
   ```bash
   cd ..  # Go to project root
   cargo build --release
   ```

2. **Test Scanner Directly**
   ```bash
   cargo run --example scan_directory -- --help
   ```

### "npm install failed"

```bash
# Clear cache and retry
npm cache clean --force
npm install
```

## Next Steps

- Read [README.md](README.md) for detailed documentation
- Explore the [tool definitions](tools/) to understand capabilities
- Check [../samples/](../samples/) for test cases
- Review MCP specification at https://modelcontextprotocol.io

## Example Workflows

### Security Audit Workflow

1. **Scan codebase**: "Scan /path/to/my/app for crypto vulnerabilities"
2. **Review findings**: Claude will list all vulnerabilities
3. **Get remediation**: "How do I fix the RSA-1024 issues in Python?"
4. **Validate fix**: "Validate compliance of /path/to/my/app with target score 90"

### CI/CD Integration

1. **Pre-commit check**: "Validate compliance of my-feature-branch"
2. **Review report**: Claude provides compliance status
3. **Block if non-compliant**: Set `strict_mode: true`

### Migration Planning

1. **Assess current state**: "Scan our legacy system"
2. **Prioritize issues**: "What are the critical vulnerabilities?"
3. **Plan migration**: "Provide migration steps for all quantum-vulnerable algorithms"
4. **Track progress**: Re-scan after each fix

## Support

- **Issues**: https://github.com/arcqubit/pqc-scanner/issues
- **Documentation**: [README.md](README.md)
- **MCP Spec**: https://modelcontextprotocol.io
