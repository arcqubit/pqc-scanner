# WSL → Windows Ollama Integration Guide

## 🎯 Overview

This guide shows how to use **Ollama running on Windows** from **WSL (Ubuntu)** with agentic-flow for 100% free, private AI agent execution using your Windows installation.

## 📋 Prerequisites

- ✅ WSL2 (Ubuntu on Windows)
- ✅ Ollama installed and running on Windows
- ✅ `llama3:70b` model downloaded on Windows
- ✅ Node.js 18+ installed in WSL

## 🚀 Quick Start (One Command)

```bash
# Run the automated setup script
./scripts/setup-wsl-ollama.sh

# That's it! Now you can run agents:
npx agentic-flow --agent coder --task "Create a REST API"
```

## 📖 What the Script Does

1. **Auto-detects Windows host IP** from WSL
2. **Tests connectivity** to Windows Ollama
3. **Verifies model availability** (llama3:70b)
4. **Starts proxy server** that translates API calls
5. **Configures environment** for agentic-flow

## 🔧 Manual Setup (If Needed)

### Step 1: Find Windows Host IP

```bash
# Get Windows IP from WSL
ip route show | grep -i default | awk '{ print $3}'

# Example output: 172.25.176.1
```

### Step 2: Test Windows Ollama

```bash
# Replace with your Windows IP
WINDOWS_IP="172.25.176.1"

# Test connectivity
curl http://$WINDOWS_IP:11434/api/version

# Should return: {"version":"0.12.9"}

# Check available models
curl http://$WINDOWS_IP:11434/api/tags | grep -o '"name":"[^"]*"'
```

### Step 3: Start Proxy Server

```bash
# Start the WSL → Windows proxy
node scripts/wsl-ollama-proxy.js

# You should see:
# ✅ Proxy server: http://localhost:3000
# 🪟 Windows host IP: 172.25.176.1
# 📡 Windows Ollama: http://172.25.176.1:11434
# 🎯 Default model: llama3:70b
```

### Step 4: Configure Environment

```bash
# Set environment variables
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-wsl"

# Add to ~/.bashrc for persistence
echo 'export ANTHROPIC_BASE_URL="http://localhost:3000"' >> ~/.bashrc
echo 'export ANTHROPIC_API_KEY="ollama-wsl"' >> ~/.bashrc
```

### Step 5: Test It!

```bash
# Test simple task
npx agentic-flow --agent coder --task "Create a Python hello world"

# Run SPARC workflow
npx claude-flow sparc tdd "User authentication"
```

## 🎯 Your Available Models on Windows

Based on your Windows Ollama installation:

| Model | Size | Best For |
|-------|------|----------|
| **llama3:70b** | 40GB | High-quality coding, complex reasoning (DEFAULT) |
| **deepseek-r1:1.5b** | 1.1GB | Fast reasoning, quick tasks |
| **phi4:latest** | 9GB | Microsoft's latest, balanced |
| **granite-code:20b** | 11.5GB | IBM's code-focused model |
| **granite3.1-dense:8b** | 5GB | General purpose |
| **gemma3:12b** | 8GB | Google's model |
| **gpt-oss:20b** | 13.8GB | Open-source GPT alternative |

## 💡 Using Different Models

```bash
# Use llama3:70b (default, best quality)
npx agentic-flow --agent coder --task "Complex feature"

# Use faster model for quick tasks
OLLAMA_MODEL="deepseek-r1:1.5b" npx agentic-flow --agent researcher --task "Quick research"

# Use code-specialized model
OLLAMA_MODEL="granite-code:20b" npx agentic-flow --agent coder --task "Refactor code"

# Use phi4 for balanced performance
OLLAMA_MODEL="phi4:latest" npx agentic-flow --agent reviewer --task "Code review"
```

## 🔥 Advanced Usage

### Integration with SPARC Workflow

```bash
# Set environment
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-wsl"

# Run full TDD workflow with llama3:70b
npx claude-flow sparc tdd "Payment processing feature"

# Batch processing
npx claude-flow sparc batch "spec-pseudocode,architect" "Microservices design"

# Specific phase with custom model
OLLAMA_MODEL="granite-code:20b" npx claude-flow sparc run architect "System design"
```

### Multi-Agent Coordination

```bash
# Start proxy
node scripts/wsl-ollama-proxy.js &

# Initialize swarm with MCP
npx agentic-flow mcp start

# Run coordinated agents
npx agentic-flow --agent coder --task "Build API endpoints" &
npx agentic-flow --agent tester --task "Create test suite" &
npx agentic-flow --agent reviewer --task "Review code quality" &

wait
```

### AgentDB with Local Embeddings

```bash
# Initialize AgentDB
npx agentic-flow agentdb init

# Use nomic-embed-text model (already on your Windows)
# The proxy will automatically use it for embeddings

# Search patterns
npx agentic-flow agentdb search --query "authentication patterns"
```

## 🛠️ Troubleshooting

### Cannot Connect to Windows Ollama

**Issue:** Proxy can't reach Windows Ollama
```bash
# Check if Ollama is running on Windows
# Open PowerShell on Windows:
ollama serve

# Check Windows firewall
# Allow incoming connections on port 11434
```

**Solution:** Allow WSL through Windows Firewall
```powershell
# Run in Windows PowerShell as Administrator
New-NetFirewallRule -DisplayName "Ollama for WSL" -Direction Inbound -Protocol TCP -LocalPort 11434 -Action Allow
```

### Wrong Windows IP

**Issue:** Script detects wrong IP
```bash
# Manually set the IP
export OLLAMA_BASE_URL="http://YOUR_WINDOWS_IP:11434"
node scripts/wsl-ollama-proxy.js
```

### Model Not Found

**Issue:** llama3:70b not available
```bash
# Check available models
curl http://YOUR_WINDOWS_IP:11434/api/tags

# Install on Windows if needed
# In Windows PowerShell:
ollama pull llama3:70b
```

### Proxy Port in Use

**Issue:** Port 3000 already taken
```bash
# Use different port
OLLAMA_PROXY_PORT=3001 node scripts/wsl-ollama-proxy.js

# Update environment
export ANTHROPIC_BASE_URL="http://localhost:3001"
```

### Slow Performance

**Performance Tips:**

1. **Use GPU on Windows** (if available):
```powershell
# Windows PowerShell
$env:OLLAMA_GPU=1
ollama serve
```

2. **Increase context window**:
```bash
# Already optimized in wsl-ollama-proxy.js
# Uses 8192 token context for 70B model
```

3. **Use smaller model for quick tasks**:
```bash
OLLAMA_MODEL="deepseek-r1:1.5b" npx agentic-flow --agent coder --task "Simple task"
```

## 📊 Performance Comparison

### llama3:70b Performance

| Hardware | Tokens/sec | Quality |
|----------|-----------|---------|
| CPU Only | 2-5 tok/s | ★★★★★ |
| RTX 3060 (12GB) | 15-25 tok/s | ★★★★★ |
| RTX 4090 (24GB) | 40-60 tok/s | ★★★★★ |

### Cost Comparison (1M tokens)

| Provider | Cost | Privacy |
|----------|------|---------|
| **Windows Ollama (llama3:70b)** | **$0** | **100% Private** |
| OpenRouter (DeepSeek) | $0.14 | Cloud |
| Claude API | $15.00 | Cloud |

## 🎯 Why This Setup?

### ✅ Advantages

1. **Zero API Costs** - Use your Windows GPU/CPU for free
2. **100% Private** - Data never leaves your machine
3. **Best Quality** - llama3:70b (70.6B params) rivals GPT-4
4. **No Rate Limits** - Process as much as your hardware allows
5. **Offline** - Works without internet
6. **Multiple Models** - Switch between 14+ models on your Windows

### ⚠️ Considerations

1. **Speed** - Depends on your hardware (GPU recommended)
2. **RAM** - llama3:70b needs ~40GB RAM
3. **Startup** - First response slower (model loading)

## 🚀 Optimization Tips

### For Best Performance

```bash
# 1. Ensure Windows Ollama uses GPU
# Check in Windows PowerShell:
ollama ps

# 2. Keep model warm (reduce startup time)
# In Windows, run:
ollama run llama3:70b "test"

# 3. Use smaller models for simple tasks
OLLAMA_MODEL="deepseek-r1:1.5b"  # Fast inference
OLLAMA_MODEL="granite3.1-dense:8b"  # Balanced
OLLAMA_MODEL="llama3:70b"  # Best quality
```

### Persistent Configuration

Add to `~/.bashrc`:

```bash
# Ollama WSL Configuration
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-wsl"

# Default model
export OLLAMA_MODEL="llama3:70b"

# Auto-start proxy on terminal open (optional)
# if ! curl -s http://localhost:3000/v1/health > /dev/null 2>&1; then
#     node ~/your-project/scripts/wsl-ollama-proxy.js &
# fi
```

## 🎓 Next Steps

1. ✅ Run automated setup: `./scripts/setup-wsl-ollama.sh`
2. ✅ Test simple task with llama3:70b
3. ✅ Try SPARC workflow for complex feature
4. ✅ Explore your other Windows models
5. ✅ Create custom agents for your domain

## 📚 Resources

- [WSL Documentation](https://docs.microsoft.com/en-us/windows/wsl/)
- [Ollama Documentation](https://github.com/ollama/ollama)
- [Agentic-Flow GitHub](https://github.com/ruvnet/agentic-flow)
- [Model Library](https://ollama.ai/library)

---

**Start using llama3:70b with $0 API costs from WSL! 🚀**
