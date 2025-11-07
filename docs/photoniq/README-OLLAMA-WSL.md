# 🚀 WSL → Windows Ollama Setup Complete!

## ✅ What's Been Configured

Your agentic-flow environment is now ready to use **Ollama on Windows** from **WSL (Ubuntu)** with your **llama3:70b** model (70.6B parameters, Q4_0 quantization).

## 🎯 Quick Start (3 Commands)

```bash
# 1. Start the proxy (connects WSL to Windows Ollama)
npm run ollama:wsl
# or: ./scripts/setup-wsl-ollama.sh

# 2. In another terminal, set environment
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-wsl"

# 3. Run your first AI agent
npm run agent:test
```

## 📋 Available npm Scripts

### Ollama Management
```bash
npm run ollama:setup      # Full automated setup
npm run ollama:wsl        # Start WSL → Windows proxy
npm run ollama:test       # Test proxy health
npm run ollama:models     # List available models
npm run ollama:windows    # Check Windows Ollama directly
```

### Agent Execution
```bash
npm run agent:list        # List all 150+ agents
npm run agent:test        # Test with hello world
npm run agent:coder -- "Your task here"  # Run custom task
```

### SPARC Workflow
```bash
npm run sparc:tdd -- "Feature name"     # Full TDD workflow
npm run sparc:spec -- "Feature name"    # Specification phase
```

### MCP Tools
```bash
npm run mcp:start         # Start all MCP servers
npm run mcp:status        # Check MCP status
npm run mcp:list          # List 223+ available tools
```

## 🎯 Your Configuration

**Windows Ollama:**
- IP: `172.25.176.1:11434`
- Version: `0.12.9`
- Default Model: `llama3:70b` (70.6B params, 40GB)

**Available Models on Windows:**
1. **llama3:70b** - 70.6B params - Best quality (your default)
2. **deepseek-r1:1.5b** - 1.8B params - Fast reasoning
3. **phi4:latest** - 14.7B params - Microsoft's latest
4. **granite-code:20b** - 20.1B params - Code specialist
5. **granite3.1-dense:8b** - 8.2B params - General purpose
6. **gemma3:12b** - 12.2B params - Google's model
7. **gpt-oss:20b** - 20.9B params - GPT alternative

**Proxy Server:**
- URL: `http://localhost:3000`
- Context Window: 8192 tokens (optimized for 70B)
- Auto-detects Windows IP

## 🚀 Example Workflows

### 1. Simple Coding Task
```bash
# Terminal 1: Start proxy
npm run ollama:wsl

# Terminal 2: Run agent
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-wsl"

npx agentic-flow --agent coder --task "Create a FastAPI REST API with authentication"
```

### 2. Full SPARC TDD Workflow
```bash
# Ensure proxy is running
npm run ollama:test

# Set environment
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-wsl"

# Run complete TDD workflow
npm run sparc:tdd -- "User authentication with JWT tokens"
```

### 3. Multi-Model Strategy
```bash
# Fast model for research
OLLAMA_MODEL="deepseek-r1:1.5b" npx agentic-flow --agent researcher --task "Research OAuth2 best practices"

# Code specialist for implementation
OLLAMA_MODEL="granite-code:20b" npx agentic-flow --agent coder --task "Implement OAuth2 server"

# High quality for review (default llama3:70b)
npx agentic-flow --agent reviewer --task "Review OAuth2 implementation for security"
```

### 4. MCP + Multi-Agent Coordination
```bash
# Start MCP servers
npm run mcp:start

# Run coordinated agents
npx agentic-flow --agent coder --task "Build user service" &
npx agentic-flow --agent tester --task "Write tests for user service" &
npx agentic-flow --agent reviewer --task "Review code quality" &

wait
```

## 📁 Files Created

### Scripts
- `scripts/wsl-ollama-proxy.js` - WSL → Windows API translator
- `scripts/setup-wsl-ollama.sh` - Automated setup script
- `scripts/ollama-proxy.js` - Original proxy (for local Ollama)
- `scripts/start-ollama-env.sh` - Full environment setup
- `scripts/quick-start-ollama.sh` - Quick test script

### Documentation
- `docs/wsl-ollama-setup.md` - Complete WSL integration guide
- `docs/ollama-integration-guide.md` - General Ollama guide
- `docs/ollama-quickref.md` - Quick reference cheat sheet
- `docs/quick-start-wsl.md` - WSL quick start
- `README-OLLAMA-WSL.md` - This file

### Configuration
- `package.json` - npm scripts and dependencies

## 🔧 Configuration Details

### Environment Variables
```bash
# Required for agentic-flow
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-wsl"

# Optional customization
export OLLAMA_MODEL="llama3:70b"              # Change default model
export OLLAMA_PROXY_PORT="3000"               # Change proxy port
export OLLAMA_BASE_URL="http://172.25.176.1:11434"  # Windows Ollama URL
```

### Make Permanent (Add to ~/.bashrc)
```bash
echo 'export ANTHROPIC_BASE_URL="http://localhost:3000"' >> ~/.bashrc
echo 'export ANTHROPIC_API_KEY="ollama-wsl"' >> ~/.bashrc
source ~/.bashrc
```

## 💡 Pro Tips

1. **Keep Windows Ollama Running** - Ensure Ollama is running on Windows for best performance
2. **Use GPU** - If you have a GPU, Windows Ollama will use it automatically
3. **Model Selection** - Use `llama3:70b` for quality, `deepseek-r1:1.5b` for speed
4. **Context Size** - Proxy automatically uses 8192 token context for large prompts
5. **Multiple Terminals** - Keep proxy running in one terminal, run agents in others

## 🛠️ Troubleshooting

### Proxy Won't Start
```bash
# Check if port is in use
lsof -i :3000

# Kill existing proxy
pkill -f wsl-ollama-proxy

# Restart
npm run ollama:wsl
```

### Cannot Connect to Windows
```bash
# Test Windows Ollama directly
curl http://172.25.176.1:11434/api/version

# If fails, check Windows Firewall
# Run in Windows PowerShell as Admin:
New-NetFirewallRule -DisplayName "Ollama for WSL" -Direction Inbound -Protocol TCP -LocalPort 11434 -Action Allow
```

### Wrong Model
```bash
# Check available models
npm run ollama:windows

# Change default model
export OLLAMA_MODEL="your-model-name"
npm run ollama:wsl
```

### Slow Performance
```bash
# Use smaller model for quick tasks
OLLAMA_MODEL="deepseek-r1:1.5b" npx agentic-flow --agent coder --task "Simple task"

# Or use code-optimized model
OLLAMA_MODEL="granite-code:20b" npx agentic-flow --agent coder --task "Code task"
```

### Check Proxy Logs
```bash
tail -f /tmp/wsl-ollama-proxy.log
```

## 📊 Performance & Cost

### Your Setup (llama3:70b)
- **Cost:** $0 forever
- **Privacy:** 100% private (data never leaves your machine)
- **Quality:** Rivals GPT-4 (70.6B parameters)
- **Speed:** Depends on your hardware
  - CPU: 2-5 tokens/sec
  - RTX 3060: 15-25 tokens/sec
  - RTX 4090: 40-60 tokens/sec

### Comparison (1M tokens)
| Provider | Cost | Privacy | Quality |
|----------|------|---------|---------|
| **Your Ollama** | **$0** | **100%** | ★★★★★ |
| OpenRouter (DeepSeek) | $0.14 | Cloud | ★★★★☆ |
| Claude API | $15.00 | Cloud | ★★★★★ |

## 🎓 Next Steps

1. ✅ **Test basic task** - Run `npm run agent:test`
2. ✅ **Try SPARC workflow** - Build a real feature with TDD
3. ✅ **Explore models** - Test different Windows models for different tasks
4. ✅ **Set up MCP** - Enable 223+ additional tools
5. ✅ **Build something!** - Use for your actual development work

## 📚 Resources

- [Full WSL Setup Guide](./docs/wsl-ollama-setup.md)
- [Quick Reference](./docs/ollama-quickref.md)
- [Ollama Documentation](https://github.com/ollama/ollama)
- [Agentic-Flow GitHub](https://github.com/ruvnet/agentic-flow)
- [Claude-Flow SPARC](https://github.com/ruvnet/claude-flow)

## 🎯 Quick Reference

```bash
# Start everything
npm run ollama:wsl

# In another terminal
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-wsl"

# Run agents
npx agentic-flow --agent coder --task "Your task"
npx agentic-flow --list
npm run sparc:tdd -- "Feature name"

# Check status
npm run ollama:test
npm run mcp:status
curl http://localhost:3000/v1/models
```

---

## 🎉 You're All Set!

Start using your Windows `llama3:70b` model with agentic-flow for **$0 API costs** and **100% privacy**!

**Quick Start:**
```bash
npm run ollama:wsl
```

Then in another terminal:
```bash
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-wsl"
npx agentic-flow --agent coder --task "Create a REST API"
```

**Happy coding! 🚀**
