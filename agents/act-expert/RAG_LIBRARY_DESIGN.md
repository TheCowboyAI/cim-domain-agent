# ACT Expert RAG Library Design

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Overview

The RAG (Retrieval-Augmented Generation) library provides the ACT expert with semantic search over category theory knowledge, CIM patterns, and verification procedures.

## Architecture

```
act-expert/
├── rag/
│   ├── embeddings/           # Vector embeddings for semantic search
│   │   ├── definitions.npz   # Category theory definitions
│   │   ├── laws.npz          # Categorical laws and identities
│   │   ├── patterns.npz      # CIM-specific ACT patterns
│   │   ├── violations.npz    # Common violations and fixes
│   │   └── examples.npz      # Code examples from cim-domain
│   ├── tensors/              # Knowledge graph tensors
│   │   ├── category_graph.pt # Category structure tensor
│   │   ├── functor_graph.pt  # Functor relationships
│   │   └── law_graph.pt      # Law dependency graph
│   ├── corpus/               # Source documents for embeddings
│   │   ├── category_theory.md
│   │   ├── functors.md
│   │   ├── natural_transformations.md
│   │   ├── adjunctions.md
│   │   └── cim_patterns.md
│   └── index/                # Search indices
│       ├── semantic.faiss    # FAISS vector index
│       └── keyword.idx       # Keyword index
└── tools/
    ├── build_embeddings.py   # Generate embeddings from corpus
    ├── build_tensors.py      # Build knowledge graph tensors
    └── query_rag.py          # Query interface for agent
```

## Knowledge Corpus Structure

### 1. Category Theory Definitions (`corpus/category_theory.md`)

```markdown
# Category Theory Definitions

## Category
**ID:** def-category-001
**Tags:** fundamental, category, structure

A category C consists of:
- Objects: ob(C) = {A, B, C, ...}
- Morphisms: hom(A,B) = arrows from A to B
- Composition: ∘ : hom(B,C) × hom(A,B) → hom(A,C)
- Identity: id_A : A → A for each object A

**Laws:**
1. Associativity: (f ∘ g) ∘ h = f ∘ (g ∘ h)
2. Identity: id_B ∘ f = f = f ∘ id_A

**CIM Application:** Categories represent domain boundaries

**Example:**
```rust
// Category: Person domain
// Objects: Person aggregates
// Morphisms: PersonEvent variants
// Composition: Event causation via causation_id
```

**Related:** def-functor-001, def-morphism-001
```

### 2. Functor Patterns (`corpus/functors.md`)

```markdown
# Functor Patterns in CIM

## DomainFunctor
**ID:** pattern-functor-001
**Tags:** functor, cim-domain, verification

Functors map between domain categories while preserving structure.

**Implementation:**
```rust
#[async_trait]
pub trait DomainFunctor: Send + Sync {
    type Source: Send + Sync;
    type Target: Send + Sync;

    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError>;
    async fn map_morphism(&self, morph: DomainMorphism) -> Result<DomainMorphism, DomainError>;

    fn source_category(&self) -> String;
    fn target_category(&self) -> String;
}
```

**Verification:**
- Identity law: F(id) = id
- Composition law: F(g ∘ f) = F(g) ∘ F(f)

**Test Template:**
```rust
#[tokio::test]
async fn verify_functor_laws() {
    let functor = /* your functor */;
    // Identity test
    // Composition test
}
```

**Common Violations:** violation-functor-001, violation-functor-002
**Related:** pattern-natural-transformation-001
```

### 3. CIM Patterns (`corpus/cim_patterns.md`)

```markdown
# CIM-Specific ACT Patterns

## Pattern: Aggregate as Functor
**ID:** cim-pattern-001
**Tags:** aggregate, functor, event-sourcing

Aggregates in CIM are functors from Event category to State category.

**Structure:**
```
Event Category → State Category
  Objects: Event types
  Morphisms: Event causation
        ↓ Functor F
  Objects: Aggregate states
  Morphisms: State transitions
```

**Implementation Pattern:**
```rust
impl EventSourcedAggregate for Person {
    type Event = PersonEvent;

    // Functor: map event to state transition
    fn apply_event(&self, event: &PersonEvent) -> Result<Self, Error> {
        match event {
            PersonEvent::Created(e) => Ok(Person { ... }),
            PersonEvent::Hired(e) => Ok(Person {
                version: self.version + 1,
                employment_status: EmploymentStatus::Employed,
                ..self.clone()
            }),
        }
    }
}
```

**Verification:**
- apply_event is pure (no side effects)
- Identity event produces same state
- Sequential events compose correctly

**Example:** cim-domain-person::Person aggregate
**Related:** def-functor-001, pattern-event-sourcing-001
```

### 4. Common Violations (`corpus/violations.md`)

```markdown
# Common Categorical Violations

## Violation: Side Effects in Functor
**ID:** violation-functor-001
**Tags:** functor, side-effect, purity
**Severity:** Critical

**Problem:** Functor operations contain side effects, breaking composition law.

**Example:**
```rust
// ❌ VIOLATION
impl DomainFunctor for PersonToEmployee {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        log_to_database(&obj); // Side effect!
        Ok(transform(obj))
    }
}
```

**Why it's wrong:**
- Side effects make function non-deterministic
- Breaks composition law: F(g ∘ f) ≠ F(g) ∘ F(f) when ordering matters
- Violates purity required for mathematical reasoning

**Detection:**
- Composition tests fail
- Function produces different results on repeated calls
- Logs show unexpected ordering

**Fix:**
```rust
// ✓ CORRECT
impl DomainFunctor for PersonToEmployee {
    async fn map_object(&self, obj: DomainObject) -> Result<DomainObject, DomainError> {
        // Pure transformation only
        Ok(transform(obj))
    }
}

// Log in separate layer
async fn logged_map_object(functor: &impl DomainFunctor, obj: DomainObject) -> Result<DomainObject, DomainError> {
    let result = functor.map_object(obj).await?;
    log_to_database(&result); // Side effect isolated
    Ok(result)
}
```

**Test:**
```rust
#[tokio::test]
async fn test_functor_purity() {
    let functor = PersonToEmployee::new();
    let obj = /* test object */;

    // Call twice, should get same result
    let result1 = functor.map_object(obj.clone()).await.unwrap();
    let result2 = functor.map_object(obj.clone()).await.unwrap();

    assert_eq!(result1, result2); // Purity check
}
```

**Related:** def-functor-001, pattern-functor-001
```

## Embedding Strategy

### 1. Dense Embeddings (sentence-transformers)

Use `all-MiniLM-L6-v2` model for general semantic embeddings:

```python
from sentence_transformers import SentenceTransformer

model = SentenceTransformer('all-MiniLM-L6-v2')

# Embed each document section
def embed_document(doc_id, content, tags):
    embedding = model.encode(content)
    return {
        'id': doc_id,
        'embedding': embedding,
        'tags': tags,
        'content': content
    }
```

### 2. Sparse Embeddings (BM25 for keywords)

Maintain keyword index for exact matches:

```python
from rank_bm25 import BM25Okapi

# Build BM25 index
corpus = [doc['content'].split() for doc in documents]
bm25 = BM25Okapi(corpus)

def keyword_search(query, top_k=5):
    tokenized_query = query.split()
    scores = bm25.get_scores(tokenized_query)
    top_indices = np.argsort(scores)[-top_k:][::-1]
    return [documents[i] for i in top_indices]
```

### 3. Hybrid Search

Combine dense and sparse retrieval:

```python
def hybrid_search(query, top_k=5, alpha=0.5):
    # Dense retrieval
    query_embedding = model.encode(query)
    dense_scores = cosine_similarity(query_embedding, all_embeddings)

    # Sparse retrieval
    sparse_scores = bm25.get_scores(query.split())

    # Combine
    hybrid_scores = alpha * dense_scores + (1 - alpha) * sparse_scores
    top_indices = np.argsort(hybrid_scores)[-top_k:][::-1]

    return [documents[i] for i in top_indices]
```

## Knowledge Graph Tensor Design

### Graph Structure

```python
import torch
from torch_geometric.data import Data

# Nodes: ACT concepts (categories, functors, laws, etc.)
# Edges: Relationships (defines, uses, verifies, violates, etc.)

class ACTKnowledgeGraph:
    def __init__(self):
        self.node_types = {
            'definition': 0,
            'law': 1,
            'pattern': 2,
            'violation': 3,
            'example': 4,
        }

        self.edge_types = {
            'defines': 0,
            'uses': 1,
            'verifies': 2,
            'violates': 3,
            'relates_to': 4,
        }

    def build_graph(self, documents):
        # Extract nodes and edges from documents
        nodes = []  # [(node_id, node_type, embedding)]
        edges = []  # [(src, dst, edge_type)]

        for doc in documents:
            node_id = doc['id']
            node_type = self.node_types[doc['type']]
            embedding = doc['embedding']
            nodes.append((node_id, node_type, embedding))

            # Extract relationships from "Related:" field
            for related_id in doc.get('related', []):
                edges.append((node_id, related_id, self.edge_types['relates_to']))

        # Build PyTorch Geometric graph
        node_features = torch.tensor([n[2] for n in nodes])  # Embeddings
        node_types_tensor = torch.tensor([n[1] for n in nodes])

        edge_index = torch.tensor([[e[0], e[1]] for e in edges]).t()
        edge_types_tensor = torch.tensor([e[2] for e in edges])

        graph = Data(
            x=node_features,
            edge_index=edge_index,
            node_type=node_types_tensor,
            edge_type=edge_types_tensor
        )

        return graph
```

### Graph Neural Network for Reasoning

```python
import torch.nn as nn
from torch_geometric.nn import GATConv, global_mean_pool

class ACTReasoningGNN(nn.Module):
    def __init__(self, in_channels, hidden_channels, out_channels, num_heads=4):
        super().__init__()
        self.conv1 = GATConv(in_channels, hidden_channels, heads=num_heads)
        self.conv2 = GATConv(hidden_channels * num_heads, out_channels, heads=1)

    def forward(self, x, edge_index):
        x = self.conv1(x, edge_index)
        x = torch.relu(x)
        x = self.conv2(x, edge_index)
        return x

# Usage: Given a verification query, find relevant concepts via graph traversal
def find_verification_knowledge(graph, query_node_id, k_hops=2):
    # k-hop neighborhood of query node
    subgraph = extract_subgraph(graph, query_node_id, k_hops)
    reasoning_output = gnn_model(subgraph.x, subgraph.edge_index)
    return reasoning_output
```

## Query Interface

```python
class ACTKnowledgeBase:
    def __init__(self, embeddings_path, tensors_path, index_path):
        self.embeddings = load_embeddings(embeddings_path)
        self.graph = load_graph_tensor(tensors_path)
        self.faiss_index = load_faiss_index(index_path)
        self.gnn = ACTReasoningGNN(...)

    def query(self, query_text, query_type='semantic', top_k=5):
        """
        Query the knowledge base.

        Args:
            query_text: Natural language query
            query_type: 'semantic', 'keyword', 'hybrid', or 'graph'
            top_k: Number of results to return

        Returns:
            List of relevant knowledge entries
        """
        if query_type == 'semantic':
            return self._semantic_search(query_text, top_k)
        elif query_type == 'keyword':
            return self._keyword_search(query_text, top_k)
        elif query_type == 'hybrid':
            return self._hybrid_search(query_text, top_k)
        elif query_type == 'graph':
            return self._graph_reasoning(query_text, top_k)

    def verify_functor(self, functor_impl):
        """
        Given a functor implementation, retrieve verification knowledge.
        """
        # Query: "functor laws verification identity composition"
        laws = self.query("functor laws", query_type='semantic', top_k=3)

        # Query: "common functor violations"
        violations = self.query("functor violations", query_type='keyword', top_k=5)

        # Query: "functor test templates"
        templates = self.query("functor test template", query_type='hybrid', top_k=3)

        return {
            'laws': laws,
            'violations': violations,
            'templates': templates
        }
```

## Integration with Llama4

### Context Construction

```python
def build_llama4_context(query, rag_results):
    """
    Build context for Llama4 from RAG results.
    """
    context = "# Relevant Knowledge\n\n"

    for result in rag_results:
        context += f"## {result['title']}\n"
        context += f"**ID:** {result['id']}\n"
        context += f"**Tags:** {', '.join(result['tags'])}\n\n"
        context += f"{result['content']}\n\n"
        if 'related' in result:
            context += f"**Related:** {', '.join(result['related'])}\n\n"

    return context

# Example usage
query = "How do I verify the composition law for a DomainFunctor?"
rag_results = kb.query(query, query_type='hybrid', top_k=3)
context = build_llama4_context(query, rag_results)

# Send to Llama4
prompt = f"""You are an ACT verification expert.

{context}

User Query: {query}

Provide verification steps:"""

response = llama4.generate(prompt, max_tokens=1024)
```

## Performance Optimization

### Caching Strategy

```python
from functools import lru_cache
import hashlib

@lru_cache(maxsize=1000)
def cached_query(query_hash, query_type, top_k):
    # Cache frequent queries
    return kb.query(query_text, query_type, top_k)

def query_with_cache(query_text, query_type='semantic', top_k=5):
    query_hash = hashlib.md5(query_text.encode()).hexdigest()
    return cached_query(query_hash, query_type, top_k)
```

### Batch Processing

```python
def batch_verify_functors(functor_impls):
    """
    Verify multiple functors in parallel using RAG.
    """
    # Batch embed queries
    queries = [f"verify functor {f.name()}" for f in functor_impls]
    embeddings = model.encode(queries, batch_size=32)

    # Batch search FAISS index
    distances, indices = faiss_index.search(embeddings, k=5)

    # Batch retrieve documents
    results = []
    for impl, doc_indices in zip(functor_impls, indices):
        docs = [documents[i] for i in doc_indices]
        results.append(verify_functor_with_docs(impl, docs))

    return results
```

## Metrics and Evaluation

### RAG Quality Metrics

1. **Relevance:** Do retrieved documents help verification?
2. **Coverage:** Are all necessary concepts retrieved?
3. **Precision:** Are retrieved documents accurate?
4. **Recall:** Are all relevant documents retrieved?

### Evaluation Dataset

```json
{
  "query": "How to verify functor composition law?",
  "expected_docs": [
    "def-functor-001",
    "pattern-functor-001",
    "violation-functor-002"
  ],
  "retrieved_docs": [...],
  "precision": 0.8,
  "recall": 1.0,
  "f1": 0.89
}
```

---

This RAG library design provides the ACT expert with semantic search, knowledge graph reasoning, and efficient retrieval for categorical verification tasks.
