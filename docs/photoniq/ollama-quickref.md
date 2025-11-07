# Ollama + Agentic-Flow Quick Reference

## 🚀 One-Line Setup

```bash
# Start everything
./scripts/start-ollama-env.sh

# Or quick start with default model
./scripts/quick-start-ollama.sh

# Or with specific model
./scripts/quick-start-ollama.sh llama3.1:70b
```

## 📋 Essential Commands

### Start Services
```bash
# Terminal 1: Ollama service
ollama serve

# Terminal 2: Proxy server
node scripts/ollama-proxy.js

# Terminal 3: Set environment
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-local"
```

### Model Management
```bash
# List installed models
ollama list

# Pull recommended models
ollama pull llama3.1:8b      # Fast, general purpose
ollama pull deepseek-coder:6.7b  # Best for coding
ollama pull llama3.1:70b     # Highest quality

# Remove model
ollama rm llama3.1:8b
```

### Test Commands
```bash
# Test Ollama directly
curl http://localhost:11434/api/version

# Test proxy
curl http://localhost:3000/v1/health

# List available models through proxy
curl http://localhost:3000/v1/models
```

## 🎯 Agent Execution Examples

### Basic Usage
```bash
# Simple coding task
npx agentic-flow --agent coder --task "Create a FastAPI hello world"

# Code review
npx agentic-flow --agent reviewer --task "Review this code for security issues"

# Research task
npx agentic-flow --agent researcher --task "Research best practices for JWT auth"
```

### SPARC Workflow
```bash
# Full TDD workflow
npx claude-flow sparc tdd "User authentication system"

# Specific phase
npx claude-flow sparc run spec-pseudocode "Payment processing"

# Batch processing
npx claude-flow sparc batch "spec-pseudocode,architect" "Microservices architecture"
```

### Advanced Features
```bash
# Use specific model
OLLAMA_MODEL="llama3.1:70b" npx agentic-flow --agent coder --task "Complex algorithm"

# AgentDB with local embeddings
npx agentic-flow agentdb init
npx agentic-flow agentdb search --query "authentication patterns"

# MCP integration
npx agentic-flow mcp start
npx agentic-flow mcp list
```

## 🎛️ Configuration

### Environment Variables
```bash
export OLLAMA_BASE_URL="http://localhost:11434"
export OLLAMA_MODEL="llama3.1:8b"
export OLLAMA_PROXY_PORT="3000"
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-local"
```

### Persistent Configuration
```bash
# Set defaults
npx agentic-flow config set PROVIDER ollama
npx agentic-flow config set COMPLETION_MODEL "llama3.1:8b"

# View configuration
npx agentic-flow config list
```

## 🛠️ Troubleshooting

### Ollama Not Running
```bash
# Check status
curl http://localhost:11434/api/version

# Start manually
ollama serve

# Check logs
tail -f /tmp/ollama.log
```

### Proxy Issues
```bash
# Check proxy health
curl http://localhost:3000/v1/health

# Restart proxy
pkill -f ollama-proxy
node scripts/ollama-proxy.js
```

### Model Not Found
```bash
# Pull missing model
ollama pull llama3.1:8b

# Verify installation
ollama list
```

## 📊 Model Recommendations

| Task | Model | RAM | Speed | Quality |
|------|-------|-----|-------|---------|
| Quick coding | `gemma2:9b` | 6GB | ⚡⚡⚡ | ★★★☆☆ |
| General coding | `llama3.1:8b` | 8GB | ⚡⚡☆ | ★★★★☆ |
| Code specialist | `deepseek-coder:6.7b` | 7GB | ⚡⚡☆ | ★★★★★ |
| Complex tasks | `llama3.1:70b` | 40GB | ⚡☆☆ | ★★★★★ |
| Large context | `llama3.1:70b` | 40GB | ⚡☆☆ | ★★★★★ |

## 💰 Cost Comparison

| Provider | Cost/1M tokens | Privacy | Speed |
|----------|----------------|---------|-------|
| **Ollama (Local)** | **$0** | **100%** | Depends on hardware |
| OpenRouter (DeepSeek) | $0.14 | Cloud | Fast |
| Claude API | $15.00 | Cloud | Fast |

## 🎯 Pro Tips

1. **Use GPU if available:** `ollama serve --gpu 1`
2. **Increase context:** `ollama run llama3.1:8b --ctx-size 8192`
3. **Multiple models:** Keep both fast (8B) and quality (70B) models
4. **Cache frequently used:** Models load faster on second use
5. **Monitor resources:** `htop` or `nvidia-smi` for GPU

## 🔗 Quick Links

- [Full Integration Guide](./ollama-integration-guide.md)
- [Ollama Documentation](https://github.com/ollama/ollama)
- [Agentic-Flow GitHub](https://github.com/ruvnet/agentic-flow)
- [Model Library](https://ollama.ai/library)

---

**Start coding with $0 API costs! 🚀**
