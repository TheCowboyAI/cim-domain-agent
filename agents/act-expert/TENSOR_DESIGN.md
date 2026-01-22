# ACT Knowledge Graph Tensor Design

<!-- Copyright (c) 2025 - Cowboy AI, LLC. -->

## Overview

The knowledge graph tensor provides structured categorical reasoning for the ACT verification expert. It encodes relationships between category theory concepts, CIM patterns, laws, violations, and fixes in a format suitable for graph neural network (GNN) reasoning.

## Tensor Structure

### Node Types

```python
NODE_TYPES = {
    'definition': 0,      # Category theory definitions
    'law': 1,             # Categorical laws (identity, composition, etc.)
    'pattern': 2,         # CIM-specific ACT patterns
    'violation': 3,       # Common violations
    'fix': 4,             # Fixes for violations
    'example': 5,         # Code examples from cim-domain
    'test_template': 6,   # Test templates
}
```

### Edge Types

```python
EDGE_TYPES = {
    'defines': 0,         # Definition → Concept
    'requires': 1,        # Law → Definition (prerequisite)
    'verifies': 2,        # Test → Law
    'violates': 3,        # Pattern → Law (negative relationship)
    'fixes': 4,           # Fix → Violation
    'implements': 5,      # Example → Pattern
    'relates_to': 6,      # General relationship
    'uses': 7,            # Pattern → Definition
}
```

## Graph Schema

```
Definitions (Category, Functor, NatTrans, etc.)
     ↓ defines
  Concepts (Objects, Morphisms, Composition, etc.)
     ↓ requires
   Laws (Identity, Composition, Naturality, etc.)
     ↓ verifies
   Tests (test_identity, test_composition, etc.)
     ↑ violates
Violations (side_effect, non_associative, etc.)
     ↓ fixes
   Fixes (remove_side_effect, make_pure, etc.)
     ↑ implements
 Examples (PersonToEmployee, etc.)
```

## Tensor Format

### PyTorch Geometric Data Structure

```python
import torch
from torch_geometric.data import Data

class ACTKnowledgeGraphTensor:
    def __init__(self):
        self.data = Data(
            # Node features [num_nodes, feature_dim]
            x=None,           # Dense embeddings (384-dim from MiniLM)

            # Node types [num_nodes]
            node_type=None,   # Integer node type IDs

            # Node IDs [num_nodes]
            node_id=None,     # String IDs (e.g., "def-functor-001")

            # Edge connections [2, num_edges]
            edge_index=None,  # COO format: [source_nodes, target_nodes]

            # Edge types [num_edges]
            edge_type=None,   # Integer edge type IDs

            # Edge attributes [num_edges, edge_feature_dim]
            edge_attr=None,   # Optional edge features

            # Metadata
            metadata={
                'num_node_types': len(NODE_TYPES),
                'num_edge_types': len(EDGE_TYPES),
                'embedding_dim': 384,
            }
        )
```

### Example Graph Construction

```python
def build_act_knowledge_graph():
    # Example nodes
    nodes = [
        {'id': 'def-category-001', 'type': 'definition', 'text': 'A category C consists of...'},
        {'id': 'def-functor-001', 'type': 'definition', 'text': 'A functor F: C → D...'},
        {'id': 'law-identity-001', 'type': 'law', 'text': 'F(id) = id'},
        {'id': 'law-composition-001', 'type': 'law', 'text': 'F(g ∘ f) = F(g) ∘ F(f)'},
        {'id': 'violation-side-effect-001', 'type': 'violation', 'text': 'Side effect in functor'},
        {'id': 'fix-purity-001', 'type': 'fix', 'text': 'Remove side effects'},
        {'id': 'pattern-aggregate-functor-001', 'type': 'pattern', 'text': 'Aggregate as functor'},
        {'id': 'example-person-to-employee-001', 'type': 'example', 'text': 'impl DomainFunctor...'},
    ]

    # Example edges
    edges = [
        {'src': 'def-functor-001', 'dst': 'law-identity-001', 'type': 'requires'},
        {'src': 'def-functor-001', 'dst': 'law-composition-001', 'type': 'requires'},
        {'src': 'violation-side-effect-001', 'dst': 'law-composition-001', 'type': 'violates'},
        {'src': 'fix-purity-001', 'dst': 'violation-side-effect-001', 'type': 'fixes'},
        {'src': 'pattern-aggregate-functor-001', 'dst': 'def-functor-001', 'type': 'uses'},
        {'src': 'example-person-to-employee-001', 'dst': 'pattern-aggregate-functor-001', 'type': 'implements'},
    ]

    # Embed nodes
    from sentence_transformers import SentenceTransformer
    model = SentenceTransformer('all-MiniLM-L6-v2')
    texts = [n['text'] for n in nodes]
    embeddings = model.encode(texts)

    # Build tensor
    node_ids = [n['id'] for n in nodes]
    node_types = [NODE_TYPES[n['type']] for n in nodes]
    node_features = torch.tensor(embeddings, dtype=torch.float)
    node_types_tensor = torch.tensor(node_types, dtype=torch.long)

    # Build edge index (COO format)
    node_id_to_idx = {nid: idx for idx, nid in enumerate(node_ids)}
    edge_src = [node_id_to_idx[e['src']] for e in edges]
    edge_dst = [node_id_to_idx[e['dst']] for e in edges]
    edge_index = torch.tensor([edge_src, edge_dst], dtype=torch.long)
    edge_types = torch.tensor([EDGE_TYPES[e['type']] for e in edges], dtype=torch.long)

    # Create Data object
    data = Data(
        x=node_features,
        node_type=node_types_tensor,
        edge_index=edge_index,
        edge_type=edge_types,
    )

    # Save metadata
    data.node_ids = node_ids
    data.metadata = {
        'num_nodes': len(nodes),
        'num_edges': len(edges),
        'num_node_types': len(NODE_TYPES),
        'num_edge_types': len(EDGE_TYPES),
        'embedding_dim': 384,
    }

    return data
```

## Graph Neural Network Architecture

### GAT (Graph Attention Network) Model

```python
import torch.nn as nn
import torch.nn.functional as F
from torch_geometric.nn import GATConv, global_mean_pool

class ACTReasoningGNN(nn.Module):
    """
    Graph Attention Network for ACT knowledge reasoning.

    Architecture:
    1. Node embedding layer (type + feature)
    2. GAT layers with multi-head attention
    3. Global pooling for graph-level reasoning
    4. Output layer for verification predictions
    """

    def __init__(
        self,
        node_feature_dim=384,
        num_node_types=7,
        num_edge_types=8,
        hidden_dim=256,
        num_heads=4,
        num_layers=3,
        output_dim=128,
    ):
        super().__init__()

        # Node type embeddings
        self.node_type_embed = nn.Embedding(num_node_types, hidden_dim // 2)

        # Edge type embeddings
        self.edge_type_embed = nn.Embedding(num_edge_types, hidden_dim // 2)

        # Input projection
        self.input_proj = nn.Linear(node_feature_dim + hidden_dim // 2, hidden_dim)

        # GAT layers
        self.gat_layers = nn.ModuleList([
            GATConv(
                in_channels=hidden_dim,
                out_channels=hidden_dim // num_heads,
                heads=num_heads,
                dropout=0.1,
                concat=True,
            )
            for _ in range(num_layers)
        ])

        # Layer normalization
        self.layer_norms = nn.ModuleList([
            nn.LayerNorm(hidden_dim)
            for _ in range(num_layers)
        ])

        # Output projection
        self.output_proj = nn.Linear(hidden_dim, output_dim)

    def forward(self, data):
        """
        Forward pass through GNN.

        Args:
            data: PyG Data object with x, edge_index, node_type, edge_type

        Returns:
            Node-level embeddings [num_nodes, output_dim]
        """
        x = data.x                  # [num_nodes, 384]
        edge_index = data.edge_index  # [2, num_edges]
        node_type = data.node_type   # [num_nodes]

        # Embed node types
        type_embed = self.node_type_embed(node_type)  # [num_nodes, hidden_dim//2]

        # Concatenate features and type embeddings
        x = torch.cat([x, type_embed], dim=-1)  # [num_nodes, 384 + hidden_dim//2]

        # Project to hidden dimension
        x = self.input_proj(x)  # [num_nodes, hidden_dim]
        x = F.relu(x)

        # Apply GAT layers with residual connections
        for gat, ln in zip(self.gat_layers, self.layer_norms):
            x_residual = x
            x = gat(x, edge_index)  # [num_nodes, hidden_dim]
            x = ln(x + x_residual)  # Residual + layer norm
            x = F.relu(x)

        # Output projection
        x = self.output_proj(x)  # [num_nodes, output_dim]

        return x

    def get_subgraph_embedding(self, data, node_indices):
        """
        Get embedding for a subgraph (e.g., all nodes within k hops of a query node).

        Args:
            data: Full graph Data object
            node_indices: Indices of nodes in subgraph

        Returns:
            Subgraph embedding [output_dim]
        """
        # Extract subgraph
        subgraph_data = data.subgraph(node_indices)

        # Forward pass
        node_embeddings = self.forward(subgraph_data)  # [subgraph_size, output_dim]

        # Global pooling (mean)
        subgraph_embedding = global_mean_pool(node_embeddings, batch=None)  # [1, output_dim]

        return subgraph_embedding.squeeze(0)
```

## Reasoning Operations

### 1. Law Verification Knowledge Retrieval

```python
def retrieve_verification_knowledge(graph, law_name, k_hops=2):
    """
    Given a law (e.g., "composition law"), retrieve all relevant knowledge
    within k hops: definitions, patterns, violations, fixes, examples.

    Args:
        graph: ACT knowledge graph tensor
        law_name: Name of the law (e.g., "law-composition-001")
        k_hops: Number of hops to traverse

    Returns:
        Subgraph with all relevant nodes
    """
    # Find law node
    law_idx = graph.node_ids.index(law_name)

    # k-hop neighborhood extraction
    subset, edge_index, mapping, edge_mask = k_hop_subgraph(
        node_idx=law_idx,
        num_hops=k_hops,
        edge_index=graph.edge_index,
    )

    # Extract subgraph
    subgraph = Data(
        x=graph.x[subset],
        edge_index=edge_index,
        node_type=graph.node_type[subset],
        edge_type=graph.edge_type[edge_mask],
    )

    subgraph.node_ids = [graph.node_ids[i] for i in subset.tolist()]

    return subgraph
```

### 2. Violation-to-Fix Reasoning

```python
def find_fix_for_violation(graph, violation_id):
    """
    Given a violation, find the fix by traversing 'fixes' edges.

    Args:
        graph: ACT knowledge graph tensor
        violation_id: Node ID of violation (e.g., "violation-side-effect-001")

    Returns:
        Fix node ID and content
    """
    violation_idx = graph.node_ids.index(violation_id)

    # Find edges of type 'fixes' pointing away from violation
    edge_mask = graph.edge_type == EDGE_TYPES['fixes']
    relevant_edges = graph.edge_index[:, edge_mask]

    # Find targets of edges from violation_idx
    fix_indices = relevant_edges[1, relevant_edges[0] == violation_idx]

    if len(fix_indices) == 0:
        return None

    # Return first fix (could rank by relevance using GNN)
    fix_idx = fix_indices[0].item()
    fix_id = graph.node_ids[fix_idx]

    return fix_id
```

### 3. Pattern-to-Example Lookup

```python
def find_examples_for_pattern(graph, pattern_id):
    """
    Given a pattern, find code examples that implement it.

    Args:
        graph: ACT knowledge graph tensor
        pattern_id: Node ID of pattern (e.g., "pattern-aggregate-functor-001")

    Returns:
        List of example node IDs
    """
    pattern_idx = graph.node_ids.index(pattern_id)

    # Find edges of type 'implements' pointing TO pattern
    edge_mask = graph.edge_type == EDGE_TYPES['implements']
    relevant_edges = graph.edge_index[:, edge_mask]

    # Find sources of edges to pattern_idx
    example_indices = relevant_edges[0, relevant_edges[1] == pattern_idx]

    example_ids = [graph.node_ids[idx.item()] for idx in example_indices]

    return example_ids
```

### 4. GNN-based Relevance Ranking

```python
def rank_knowledge_by_relevance(graph, gnn_model, query_node_id, candidate_node_ids):
    """
    Use GNN embeddings to rank candidate nodes by relevance to query.

    Args:
        graph: ACT knowledge graph tensor
        gnn_model: Trained ACTReasoningGNN
        query_node_id: Query node ID
        candidate_node_ids: List of candidate node IDs

    Returns:
        Ranked list of node IDs (most relevant first)
    """
    # Get embeddings
    embeddings = gnn_model(graph)  # [num_nodes, output_dim]

    # Query embedding
    query_idx = graph.node_ids.index(query_node_id)
    query_embed = embeddings[query_idx]  # [output_dim]

    # Candidate embeddings
    candidate_indices = [graph.node_ids.index(cid) for cid in candidate_node_ids]
    candidate_embeds = embeddings[candidate_indices]  # [num_candidates, output_dim]

    # Cosine similarity
    similarities = F.cosine_similarity(query_embed.unsqueeze(0), candidate_embeds, dim=-1)

    # Sort by similarity
    sorted_indices = torch.argsort(similarities, descending=True)
    ranked_ids = [candidate_node_ids[i] for i in sorted_indices.tolist()]

    return ranked_ids
```

## Integration with RAG

### Hybrid Retrieval

```python
def hybrid_verification_retrieval(query_text, graph, gnn_model, rag_embeddings):
    """
    Combine dense vector search (RAG) with graph reasoning (GNN).

    Args:
        query_text: Natural language query
        graph: ACT knowledge graph tensor
        gnn_model: Trained GNN model
        rag_embeddings: RAG embedding index

    Returns:
        Combined ranked results
    """
    # 1. Dense retrieval (RAG)
    from sentence_transformers import SentenceTransformer
    embed_model = SentenceTransformer('all-MiniLM-L6-v2')
    query_embed = embed_model.encode(query_text)

    rag_results = rag_embeddings.search(query_embed, top_k=10)  # FAISS search
    rag_node_ids = [result['node_id'] for result in rag_results]

    # 2. Graph reasoning (GNN)
    # For each RAG result, get k-hop neighborhood
    graph_expanded_ids = set()
    for node_id in rag_node_ids:
        if node_id in graph.node_ids:
            subgraph = retrieve_verification_knowledge(graph, node_id, k_hops=1)
            graph_expanded_ids.update(subgraph.node_ids)

    graph_node_ids = list(graph_expanded_ids)

    # 3. Rank combined results using GNN
    all_candidates = list(set(rag_node_ids + graph_node_ids))
    ranked_ids = rank_knowledge_by_relevance(graph, gnn_model, rag_node_ids[0], all_candidates)

    return ranked_ids[:10]  # Top 10
```

## Tensor Serialization

### Saving

```python
def save_knowledge_graph_tensor(data, path):
    """
    Save knowledge graph tensor to disk.

    Format: PyTorch .pt file with all tensors and metadata.
    """
    torch.save({
        'x': data.x,
        'node_type': data.node_type,
        'edge_index': data.edge_index,
        'edge_type': data.edge_type,
        'node_ids': data.node_ids,
        'metadata': data.metadata,
    }, path)

# Usage
save_knowledge_graph_tensor(graph, './agents/act-expert/tensors/category_graph.pt')
```

### Loading

```python
def load_knowledge_graph_tensor(path):
    """
    Load knowledge graph tensor from disk.
    """
    checkpoint = torch.load(path)
    data = Data(
        x=checkpoint['x'],
        node_type=checkpoint['node_type'],
        edge_index=checkpoint['edge_index'],
        edge_type=checkpoint['edge_type'],
    )
    data.node_ids = checkpoint['node_ids']
    data.metadata = checkpoint['metadata']
    return data

# Usage
graph = load_knowledge_graph_tensor('./agents/act-expert/tensors/category_graph.pt')
```

## Tensor Statistics

Example knowledge graph dimensions:

```python
{
    'num_nodes': 250,              # 250 concepts
    'num_edges': 800,              # 800 relationships
    'num_node_types': 7,           # 7 node types
    'num_edge_types': 8,           # 8 edge types
    'embedding_dim': 384,          # 384-dim embeddings (MiniLM)
    'gnn_hidden_dim': 256,         # 256-dim GNN hidden state
    'gnn_output_dim': 128,         # 128-dim final embeddings
    'avg_degree': 6.4,             # Average node degree
    'diameter': 8,                 # Graph diameter
}
```

## Performance

### Inference Latency

- **k-hop subgraph extraction:** ~1ms
- **GNN forward pass (250 nodes):** ~10ms
- **Ranking candidates (50 nodes):** ~2ms
- **Total latency:** ~15-20ms

### Memory Footprint

- **Node features:** 250 × 384 × 4 bytes = 384 KB
- **Edge index:** 800 × 2 × 8 bytes = 12.8 KB
- **GNN model:** ~5 MB
- **Total:** ~6 MB

Fits easily in GPU memory for fast inference.

---

This tensor design provides efficient graph reasoning for categorical verification in CIM systems.
