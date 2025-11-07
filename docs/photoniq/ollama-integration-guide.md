# Ollama Integration Guide for Agentic-Flow

## 🎯 Overview

This guide shows how to use your local Ollama installation with agentic-flow for **100% free, private AI agent execution**.

## 📋 Prerequisites

- ✅ Ollama installed locally
- ✅ Agentic-flow installed (`npx agentic-flow --help`)
- ✅ Node.js 18+ and npm

## 🚀 Quick Start

### Step 1: Start Ollama Service

```bash
# Start Ollama (run in background)
ollama serve

# Or on Windows
ollama serve &

# Verify it's running
curl http://localhost:11434/api/version
```

### Step 2: Pull Recommended Models

```bash
# Best for coding (8B parameters, fast)
ollama pull llama3.1:8b

# Best for reasoning (70B parameters, slower but smarter)
ollama pull llama3.1:70b

# DeepSeek for coding (lightweight)
ollama pull deepseek-coder:6.7b

# Mistral for general tasks
ollama pull mistral:7b

# Gemma for fast responses
ollama pull gemma2:9b

# Check installed models
ollama list
```

### Step 3: Configure Agentic-Flow for Ollama

**Option A: Using OpenRouter-Compatible Proxy (Recommended)**

```bash
# Create Ollama proxy configuration
npx agentic-flow config set ANTHROPIC_BASE_URL "http://localhost:11434/v1"
npx agentic-flow config set COMPLETION_MODEL "llama3.1:8b"
npx agentic-flow config set PROVIDER "ollama"
```

**Option B: Using Custom Ollama Proxy Script**

Create `scripts/ollama-proxy.js`:

```javascript
#!/usr/bin/env node
import express from 'express';
import fetch from 'node-fetch';

const app = express();
app.use(express.json());

const OLLAMA_BASE = 'http://localhost:11434';
const DEFAULT_MODEL = process.env.OLLAMA_MODEL || 'llama3.1:8b';

// Translate Anthropic API to Ollama API
app.post('/v1/messages', async (req, res) => {
  const { messages, model, max_tokens, temperature } = req.body;

  // Convert messages format
  const prompt = messages.map(m =>
    `${m.role === 'user' ? 'User' : 'Assistant'}: ${m.content}`
  ).join('\n\n');

  try {
    const response = await fetch(`${OLLAMA_BASE}/api/generate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        model: model || DEFAULT_MODEL,
        prompt: prompt + '\n\nAssistant:',
        stream: false,
        options: {
          temperature: temperature || 0.7,
          num_predict: max_tokens || 2048
        }
      })
    });

    const data = await response.json();

    // Convert back to Anthropic format
    res.json({
      id: `msg_${Date.now()}`,
      type: 'message',
      role: 'assistant',
      content: [{ type: 'text', text: data.response }],
      model: data.model,
      stop_reason: 'end_turn',
      usage: {
        input_tokens: data.prompt_eval_count || 0,
        output_tokens: data.eval_count || 0
      }
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Health check
app.get('/v1/health', (req, res) => {
  res.json({ status: 'ok', ollama: OLLAMA_BASE });
});

const PORT = process.env.OLLAMA_PROXY_PORT || 3000;
app.listen(PORT, () => {
  console.log(`🤖 Ollama Proxy running on http://localhost:${PORT}`);
  console.log(`📡 Connected to Ollama at ${OLLAMA_BASE}`);
  console.log(`🎯 Default model: ${DEFAULT_MODEL}`);
});
```

**Run the proxy:**

```bash
# Make it executable
chmod +x scripts/ollama-proxy.js

# Start proxy
node scripts/ollama-proxy.js

# Or with custom settings
OLLAMA_MODEL="deepseek-coder:6.7b" OLLAMA_PROXY_PORT=3000 node scripts/ollama-proxy.js
```

### Step 4: Use Ollama with Agentic-Flow

**Terminal 1: Start Ollama Proxy**
```bash
node scripts/ollama-proxy.js
```

**Terminal 2: Run Agentic-Flow Agents**
```bash
# Set environment for proxy
export ANTHROPIC_BASE_URL="http://localhost:3000"
export ANTHROPIC_API_KEY="ollama-local-key"

# Test with a simple task
npx agentic-flow --agent coder --task "Create a hello world function"

# Use specific model
npx agentic-flow --agent coder --task "Build REST API" --model "llama3.1:70b"

# Cost-optimized (uses local Ollama, $0 cost)
npx agentic-flow --agent coder --task "Complex feature" --optimize --priority privacy
```

## 🎯 Recommended Model Selection by Task

| Task Type | Recommended Model | Why |
|-----------|------------------|-----|
| **Coding** | `deepseek-coder:6.7b` | Specialized for code generation |
| **General** | `llama3.1:8b` | Fast, balanced performance |
| **Complex Reasoning** | `llama3.1:70b` | Best quality, slower |
| **Quick Tasks** | `gemma2:9b` | Very fast responses |
| **Code Review** | `deepseek-coder:33b` | Deep code understanding |

## 🔧 Advanced Configuration

### Multi-Model Strategy

```bash
# Use different models for different agent types
export OLLAMA_CODER_MODEL="deepseek-coder:6.7b"
export OLLAMA_REVIEWER_MODEL="llama3.1:70b"
export OLLAMA_RESEARCHER_MODEL="mistral:7b"
```

### Performance Optimization

```bash
# Enable GPU acceleration (if available)
ollama serve --gpu 1

# Increase context window
ollama run llama3.1:8b --ctx-size 8192

# Multiple concurrent requests
ollama serve --parallel 4
```

### Agentic-Flow with Ollama MCP Integration

```bash
# Start MCP servers with Ollama backend
npx agentic-flow mcp start

# Initialize AgentDB with Ollama for embeddings
npx agentic-flow agentdb init --model "nomic-embed-text"

# Run SPARC workflow with local models
npx claude-flow sparc tdd "User authentication" --provider ollama
```

## 📊 Performance Comparison

| Provider | Cost | Speed | Privacy | Quality |
|----------|------|-------|---------|---------|
| **Ollama (Local)** | $0 | Fast* | 100% | Good-Excellent |
| **OpenRouter** | $-$$ | Fast | Cloud | Excellent |
| **Claude API** | $$$ | Fast | Cloud | Excellent |

*Speed depends on your hardware (CPU vs GPU)

## 🚨 Troubleshooting

### Ollama Not Starting

```bash
# Check if port 11434 is in use
lsof -i :11434

# Start with verbose logging
OLLAMA_DEBUG=1 ollama serve

# Check logs
journalctl -u ollama -f  # Linux
```

### Model Download Issues

```bash
# Check available disk space
df -h

# Clear Ollama cache
rm -rf ~/.ollama/models/*

# Re-pull model
ollama pull llama3.1:8b
```

### Proxy Connection Issues

```bash
# Test Ollama directly
curl http://localhost:11434/api/generate -d '{
  "model": "llama3.1:8b",
  "prompt": "Why is the sky blue?",
  "stream": false
}'

# Test proxy
curl http://localhost:3000/v1/health
```

## 🎯 Integration with Claude-Flow SPARC

```bash
# Use Ollama for all SPARC phases
export ANTHROPIC_BASE_URL="http://localhost:3000"

# Run specification phase
npx claude-flow sparc run spec-pseudocode "Build authentication system"

# Run full TDD workflow with local models
npx claude-flow sparc tdd "User registration feature"

# Batch processing with multiple Ollama models
npx claude-flow sparc batch "spec-pseudocode,architect" "Microservices design"
```

## 💡 Benefits of Local Ollama

1. **Zero API Costs** - Run unlimited agents for $0
2. **100% Privacy** - Data never leaves your machine
3. **Offline Capable** - Work without internet
4. **Custom Models** - Fine-tune for your domain
5. **No Rate Limits** - Process as much as your hardware allows

## 🚀 Next Steps

1. ✅ Start Ollama service
2. ✅ Pull recommended models
3. ✅ Run proxy script
4. ✅ Test with simple agent task
5. ✅ Integrate with SPARC workflow
6. ✅ Create custom agents for your domain

## 📚 Resources

- [Ollama Documentation](https://github.com/ollama/ollama)
- [Agentic-Flow GitHub](https://github.com/ruvnet/agentic-flow)
- [Claude-Flow SPARC Guide](https://github.com/ruvnet/claude-flow)
- [Model Library](https://ollama.ai/library)

---

**Pro Tip:** Use `llama3.1:8b` for development (fast) and `llama3.1:70b` for production-grade code review.
