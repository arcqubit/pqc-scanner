# Quick Start: WSL → Windows Ollama with Agentic-Flow

## 🚀 One-Command Setup

```bash
./scripts/setup-wsl-ollama.sh
```

## 🎯 What You Get

- ✅ **Free AI Agents** using your Windows `llama3:70b` model
- ✅ **Zero API Costs** - No cloud services needed
- ✅ **100% Private** - Data stays on your machine
- ✅ **High Quality** - 70B parameter model rivals GPT-4

## 📋 After Setup, Try These:

### Test Simple Task
```bash
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-wsl"

npx agentic-flow --agent coder --task "Create a Python FastAPI hello world"
```

### List Available Agents
```bash
npx agentic-flow --list
```

### Run SPARC TDD Workflow
```bash
npx claude-flow sparc tdd "User authentication system"
```

### Use Different Windows Models
```bash
# Fast reasoning (1.5B)
OLLAMA_MODEL="deepseek-r1:1.5b" npx agentic-flow --agent researcher --task "Research task"

# Code specialist (20B)
OLLAMA_MODEL="granite-code:20b" npx agentic-flow --agent coder --task "Refactor code"

# Best quality (70B) - default
npx agentic-flow --agent reviewer --task "Code review"
```

## 🎛️ Your Windows Models

You have these models available on Windows:
- **llama3:70b** - 70.6B params (default, best quality)
- **deepseek-r1:1.5b** - 1.8B params (fast)
- **phi4:latest** - 14.7B params (balanced)
- **granite-code:20b** - 20.1B params (code specialist)
- **granite3.1-dense:8b** - 8.2B params (general)
- **gemma3:12b** - 12.2B params (Google)
- **gpt-oss:20b** - 20.9B params (GPT alternative)

## 🛠️ Troubleshooting

### Proxy not connecting?
```bash
# Check Windows Ollama
curl http://172.25.176.1:11434/api/version

# Restart proxy
pkill -f wsl-ollama-proxy
./scripts/setup-wsl-ollama.sh
```

### Check proxy status
```bash
curl http://localhost:3000/v1/health
```

### View proxy logs
```bash
tail -f /tmp/wsl-ollama-proxy.log
```

## 💰 Cost Comparison

| Task | Ollama (You) | Claude API | OpenRouter |
|------|-------------|------------|------------|
| 100K tokens | **$0** | $1.50 | $0.014 |
| 1M tokens | **$0** | $15.00 | $0.14 |
| Unlimited | **$0** | $$$ | $$ |

## 📚 Full Documentation

- [Complete WSL Setup Guide](./wsl-ollama-setup.md)
- [Ollama Integration Guide](./ollama-integration-guide.md)
- [Quick Reference](./ollama-quickref.md)

---

**Start coding with your Windows Ollama for $0! 🚀**
