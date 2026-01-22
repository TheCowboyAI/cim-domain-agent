# 🎯 Run This To Prove System Prompt Works

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Quick Proof (30 seconds)

```bash
cd /git/thecowboyai/cim-domain-agent

# Run basic tests (no network required)
cargo test --test system_prompt_integration_test
```

**You should see:**
```
test test_system_prompt_event_application ... ok
test test_multiple_agents_same_model_different_prompts ... ok

test result: ok. 2 passed; 0 failed
```

✅ **This proves**: SystemPromptConfiguredEvent correctly updates Agent state.

---

## Full Proof Through Mistral on DGX (2 minutes)

```bash
cd /git/thecowboyai/cim-domain-agent

# Automated test with all pre-flight checks
./run_dgx_integration_test.sh
```

**This will:**
1. ✅ Check DGX connectivity (10.0.20.1)
2. ✅ Verify Ollama is running (port 11434)
3. ✅ Confirm Mistral model is available
4. ✅ Run integration test

**You should see a pirate response like:**
```
Agent response:
─────────────────────────────────────────
Arrr, matey! I be here to help ye navigate
the seven seas of knowledge! As a friendly
pirate assistant, I can answer yer questions...
─────────────────────────────────────────

✅ SUCCESS: System prompt is working!
Evidence of pirate system prompt:
  - Found pirate terms: ["arrr", "matey", "ye"]
```

🎉 **The pirate response is your proof!**

---

## Manual Test (If Automated Script Fails)

```bash
# 1. Check connectivity
ping -c 1 10.0.20.1

# 2. Check Ollama
nc -zv 10.0.20.1 11434

# 3. Set environment
export OLLAMA_HOST="http://10.0.20.1:11434"

# 4. Run test
cargo test --test system_prompt_integration_test \
  --features genai-adapter \
  -- --ignored --nocapture \
  test_system_prompt_through_genai_to_mistral_dgx
```

---

## What This Proves

### 1. Event Sourcing ✅
```rust
AgentEvent::SystemPromptConfigured(
    SystemPromptConfiguredEvent::new(
        agent_id,
        "You are a pirate assistant..."
    )
)
```
**Proof**: Agent state updated, `agent.system_prompt()` returns the prompt.

### 2. MessageService Integration ✅
**Proof**: System prompt prepended to context before sending to genai.

### 3. GenAI Adapter ✅
**Proof**: Converts to ChatMessage::system() and sends to Mistral.

### 4. Mistral Processing ✅
**Proof**: Response contains pirate vocabulary ("Arrr", "matey", "ye").

### 5. Multiple Agents, Same Model ✅
**Proof**: Test creates 3 agents sharing Mistral with different prompts:
- Agent 1: Pirate personality
- Agent 2: Shakespeare personality
- Agent 3: Technical writer personality

---

## Troubleshooting

### "Cannot reach DGX"
```bash
# Check network
ping 10.0.20.1

# If on different network, check VPN
# If SSH access: ssh 10.0.20.1
```

### "Cannot connect to Ollama"
```bash
# Check Ollama service on DGX
ssh 10.0.20.1 systemctl status ollama

# Should show: Active: active (running)
```

### "Mistral model not found"
```bash
# List available models
ssh 10.0.20.1 ollama list

# Pull Mistral if missing
ssh 10.0.20.1 ollama pull mistral:7b-instruct
```

### "Test times out"
```bash
# Check if model is loaded in memory
ssh 10.0.20.1 'curl -s http://localhost:11434/api/tags'

# First request may take 30-60 seconds to load model into memory
# Subsequent requests should be faster
```

---

## Expected Results

### Without System Prompt (Baseline)
```
"Hello! I'm an AI assistant. I can help you with various
tasks including answering questions and providing information."
```

### With Pirate System Prompt (Our Implementation)
```
"Arrr, matey! I be here to help ye navigate the seven seas
of knowledge! What be on yer mind, me hearty?"
```

**The difference IS the proof!** 🏴‍☠️

---

## Files Created for This Proof

1. `tests/system_prompt_integration_test.rs` - Integration tests
2. `run_dgx_integration_test.sh` - Automated test script
3. `PROOF_OF_SYSTEM_PROMPT.md` - Detailed technical documentation
4. `RUN_THIS_TO_PROVE_IT.md` - This quick-start guide

---

## One-Line Proof

```bash
./run_dgx_integration_test.sh && echo "✅ PROOF: System prompts work!"
```

If you see a pirate response, the proof is complete. 🎉
