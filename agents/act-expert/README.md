# ACT Expert Agent - Complete Design

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Overview

The **ACT (Applied Category Theory) Expert** is a specialized verification agent for CIM systems. It verifies that implementations of categorical structures from `cim-domain::category` satisfy mathematical laws and properties.

**What it does:** Verifies functor laws, naturality conditions, and categorical structures
**What it doesn't do:** Generate code, teach category theory, or make architectural decisions

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    ACT Expert Agent                      │
│                                                          │
│  ┌──────────────┐      ┌──────────────┐                │
│  │   Llama4 8B  │◄────►│ System Prompt│                │
│  │  (0.3 temp)  │      │  (Focused)   │                │
│  └──────┬───────┘      └──────────────┘                │
│         │                                                │
│         ├─────────────┐                                 │
│         │             │                                 │
│  ┌──────▼──────┐  ┌──▼──────────────┐                 │
│  │  RAG Library│  │ Knowledge Graph  │                 │
│  │             │  │    Tensor        │                 │
│  │ • Semantic  │  │                  │                 │
│  │ • Keyword   │  │  • GNN Reasoning │                 │
│  │ • Hybrid    │  │  • Graph Traverse│                 │
│  └─────────────┘  └──────────────────┘                 │
│                                                          │
│  Input: Verification Request (JSON)                     │
│  Output: Verification Report (JSON)                     │
└─────────────────────────────────────────────────────────┘
```

## Components

### 1. Agent Definition (`act-expert.v2.md`)

The core agent configuration optimized for Llama4:

**Model Configuration:**
- **Model:** Llama4 8B (llama3.2:latest as placeholder)
- **Temperature:** 0.3 (focused, deterministic)
- **Context Window:** 16,384 tokens
- **Max Output:** 4,096 tokens
- **Task:** Mathematical verification

**System Prompt:**
- Concise, task-focused (not instructional)
- Structured JSON input/output
- Verification procedures explicitly defined
- RAG and tensor integration points specified

**Resources:**
- Memory: 8GB
- CPU: 400% (4 cores)
- GPU: Not required (CPU inference sufficient)

### 2. RAG Library (`RAG_LIBRARY_DESIGN.md`)

Retrieval-Augmented Generation infrastructure:

**Knowledge Corpus:**
```
corpus/
├── category_theory.md     # Definitions (category, functor, etc.)
├── functors.md            # Functor patterns and laws
├── natural_transformations.md
├── adjunctions.md
├── cim_patterns.md        # CIM-specific ACT patterns
└── violations.md          # Common violations and fixes
```

**Embeddings:**
- **Model:** all-MiniLM-L6-v2 (384-dim)
- **Dense:** Semantic search via FAISS
- **Sparse:** Keyword search via BM25
- **Hybrid:** Combined ranking (α=0.5)

**Search Modes:**
- `semantic`: Dense vector search
- `keyword`: BM25 keyword search
- `hybrid`: Combined dense + sparse
- `graph`: GNN-based reasoning

### 3. Knowledge Graph Tensor (`TENSOR_DESIGN.md`)

Graph neural network for categorical reasoning:

**Graph Schema:**
```
Nodes (7 types):
├── definition (Category, Functor, etc.)
├── law (Identity, Composition, etc.)
├── pattern (Aggregate as Functor, etc.)
├── violation (Side effects, etc.)
├── fix (Make pure, etc.)
├── example (Code from cim-domain)
└── test_template (Test patterns)

Edges (8 types):
├── defines
├── requires
├── verifies
├── violates
├── fixes
├── implements
├── relates_to
└── uses
```

**GNN Architecture:**
- **Model:** Graph Attention Network (GAT)
- **Layers:** 3 GAT layers with 4 attention heads
- **Hidden Dim:** 256
- **Output Dim:** 128
- **Operations:** k-hop subgraph, relevance ranking

**Reasoning:**
- Law → Definitions (what does this law require?)
- Violation → Fix (how do I fix this?)
- Pattern → Examples (show me implementations)

### 4. Role Definition (`ACT_ROLE_IN_CIM.md`)

Precise specification of the agent's responsibilities:

**Scope: What ACT Expert Does**
1. ✓ Verify functor laws (identity, composition)
2. ✓ Verify naturality conditions
3. ✓ Validate anti-corruption layers
4. ✓ Validate context mappings
5. ✓ Detect categorical violations

**Scope: What ACT Expert Does NOT Do**
1. ✗ Generate code implementations
2. ✗ Teach category theory
3. ✗ Make architectural decisions
4. ✗ Handle non-categorical concerns

**Integration:**
- Works with `cim-domain::category` types
- Collaborates with cim-expert, fp-expert, tdd-expert
- Invoked by sage for multi-domain verification

## Usage

### Input Format

```json
{
  "verification_type": "functor_laws",
  "implementation": {
    "type": "DomainFunctor",
    "name": "PersonToEmployee",
    "source_code": "impl DomainFunctor for PersonToEmployee { ... }",
    "source_file": "src/functors/person_to_employee.rs"
  },
  "test_cases": [
    {
      "name": "test_identity",
      "objects": ["Person(id=1)"],
      "morphisms": ["identity"]
    },
    {
      "name": "test_composition",
      "morphisms": ["PersonCreated", "PersonHired"],
      "expected": "Employee(id=1, status=Hired)"
    }
  ]
}
```

### Output Format

```json
{
  "verification_id": "uuid",
  "status": "pass" | "fail",
  "laws_verified": [
    {
      "law": "identity",
      "status": "pass",
      "evidence": "F(id_Person) = id_Employee verified"
    },
    {
      "law": "composition",
      "status": "fail",
      "evidence": "F(g ∘ f) ≠ F(g) ∘ F(f)",
      "counterexample": {...}
    }
  ],
  "violations": [
    {
      "law": "composition",
      "severity": "critical",
      "location": "src/functors/person_to_employee.rs:42",
      "explanation": "map_morphism contains side effect",
      "fix": "Remove side effect from map_morphism"
    }
  ],
  "recommendations": [
    "Make map_morphism pure",
    "Add property-based tests using proptest"
  ]
}
```

### Verification Workflow

```
1. Developer implements DomainFunctor
2. Developer writes tests invoking ACT expert via NATS
3. ACT expert receives verification request
4. ACT expert:
   a. Queries RAG library for relevant knowledge
   b. Queries knowledge graph tensor for reasoning
   c. Analyzes implementation for violations
   d. Executes verification procedures
   e. Collects evidence (pass/fail)
   f. Generates structured report
5. ACT expert publishes report to NATS
6. Developer fixes violations if any
7. Repeat until verified
```

## Implementation Steps

### Phase 1: RAG Library Construction

1. **Create Knowledge Corpus** (10-20 documents)
   - Category theory definitions
   - Functor patterns
   - Natural transformation patterns
   - Common violations and fixes
   - Code examples from `cim-domain::category`

2. **Generate Embeddings**
   ```bash
   cd agents/act-expert/tools
   python build_embeddings.py \
     --corpus ../rag/corpus/ \
     --output ../rag/embeddings/ \
     --model all-MiniLM-L6-v2
   ```

3. **Build Search Index**
   ```bash
   python build_index.py \
     --embeddings ../rag/embeddings/ \
     --output ../rag/index/ \
     --index-type faiss
   ```

### Phase 2: Knowledge Graph Tensor

1. **Define Graph Schema**
   - Node types and relationships
   - Extraction rules from corpus

2. **Build Knowledge Graph**
   ```bash
   python build_tensors.py \
     --corpus ../rag/corpus/ \
     --embeddings ../rag/embeddings/ \
     --output ../tensors/ \
     --graph-type gat
   ```

3. **Train GNN (optional)**
   - Pre-training on link prediction
   - Fine-tuning on verification tasks

### Phase 3: Agent Integration

1. **Deploy RAG Service**
   - FastAPI endpoint for RAG queries
   - FAISS index loading
   - Caching layer

2. **Deploy GNN Service**
   - TorchServe for GNN inference
   - Graph tensor loading
   - k-hop subgraph extraction

3. **Deploy ACT Agent**
   - Llama4 model via Ollama
   - NATS subscription to `agent.commands.{agent_id}`
   - Integration with RAG and GNN services

### Phase 4: Testing

1. **Unit Tests**
   - RAG query accuracy
   - GNN reasoning correctness
   - Verification logic

2. **Integration Tests**
   - End-to-end verification workflows
   - Real implementations from cim-domain-person

3. **Performance Tests**
   - Latency benchmarks (target: <30s)
   - Throughput tests (concurrent verifications)

## Performance Targets

| Metric | Target | Actual |
|--------|--------|--------|
| RAG query latency | <100ms | TBD |
| GNN inference latency | <50ms | TBD |
| Total verification time | <30s | TBD |
| False positive rate | <5% | TBD |
| False negative rate | <1% | TBD |

## Dependencies

### Python Dependencies
```toml
# pyproject.toml
[dependencies]
torch = "^2.0.0"
torch-geometric = "^2.3.0"
sentence-transformers = "^2.2.0"
faiss-cpu = "^1.7.4"  # or faiss-gpu
rank-bm25 = "^0.2.2"
fastapi = "^0.100.0"
uvicorn = "^0.23.0"
```

### Rust Dependencies
```toml
# Cargo.toml (for agent runtime)
[dependencies]
cim-domain = { version = "0.13", features = ["category"] }
async-nats = "0.36"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
```

## Future Enhancements

### Short-term (v0.4.0)
- [ ] Add adjunction verification (CQRS validation)
- [ ] Add limit/colimit verification
- [ ] Property-based test generation
- [ ] Violation auto-fix suggestions (code patches)

### Medium-term (v0.5.0)
- [ ] Active learning: improve GNN from verification results
- [ ] Multi-agent collaboration: act-expert + fp-expert
- [ ] Proof assistant integration: generate Coq/Lean proofs
- [ ] Performance optimization: <10s verification time

### Long-term (v1.0.0)
- [ ] Automatic functor synthesis from specifications
- [ ] Continuous verification in CI/CD
- [ ] Formal verification certificate generation
- [ ] Integration with other CIM verification agents

## Status

- [x] Role definition complete
- [x] RAG library design complete
- [x] Knowledge graph tensor design complete
- [x] Agent configuration (v0.3.0) complete
- [x] **RAG library implementation (Phase 1) COMPLETE**
  - [x] Knowledge corpus created (5 comprehensive documents)
  - [x] Embedding generation tool (build_embeddings.py)
  - [x] Search index builder (build_index.py - FAISS + BM25)
  - [x] Knowledge graph tensor builder (build_tensors.py - GAT)
  - [x] Query interface (query_rag.py)
  - [x] Automated build script (build_all.sh)
- [ ] RAG service deployment (Phase 2)
- [ ] Agent integration (Phase 3)
- [ ] Testing and validation (Phase 4)

---

## Next Steps

### Phase 1 Complete ✓
1. ✅ **Knowledge Corpus Created** - 5 comprehensive markdown documents covering:
   - Category theory definitions
   - Functor patterns and implementations
   - Common violations and fixes
   - Natural transformation patterns
   - CIM-specific ACT patterns
2. ✅ **RAG Tooling Complete** - Build scripts for embeddings, indices, and tensors
3. ✅ **Query Interface** - Interactive testing tool

### Phase 2: RAG Service Deployment
1. **Deploy FastAPI RAG Service**
   - HTTP endpoint for RAG queries
   - FAISS index loading and caching
   - Hybrid search (semantic + keyword)
2. **Deploy GNN Inference Service**
   - TorchServe for GAT model inference
   - k-hop subgraph extraction
   - Violation → Fix reasoning paths
3. **Docker Containers**
   - RAG service container
   - GNN service container
   - NATS integration

### Phase 3: Agent Integration
1. **Connect Llama4 via Ollama**
   - Load llama3.2:latest (placeholder for Llama4)
   - Configure context window (16K) and temperature (0.3)
2. **NATS Subscription**
   - Subscribe to `agent.commands.{agent_id}`
   - Parse verification requests (JSON)
3. **RAG Integration**
   - Query RAG service for relevant knowledge
   - Build Llama4 context from retrieved documents
   - Include GNN reasoning results
4. **Verification Workflow**
   - Receive DomainFunctor implementation
   - Query RAG for functor laws and patterns
   - Analyze for violations using GNN
   - Generate structured verification report (JSON)
   - Publish to NATS events

### Phase 4: Testing and Validation
1. **Unit Tests**
   - RAG query accuracy and relevance
   - GNN reasoning correctness
   - Verification logic edge cases
2. **Integration Tests**
   - End-to-end verification with cim-domain-person
   - Verify Person → Employee functor laws
   - Test composition law detection
3. **Performance Tests**
   - Latency benchmarks (<30s target)
   - Throughput tests (concurrent verifications)
   - Resource usage monitoring

### To Build RAG Library Now:
```bash
cd /git/thecowboyai/cim-domain-agent/agents/act-expert/tools
pip install -r requirements.txt
./build_all.sh
```

This will generate the complete RAG library ready for Phase 2 deployment.

---

**Status:** Phase 1 (RAG Library Implementation) is COMPLETE. Ready to proceed with Phase 2 (Service Deployment).
