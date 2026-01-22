#!/usr/bin/env python3
"""
Build knowledge graph tensor for ACT Expert.

Copyright (c) 2025 - Cowboy AI, LLC.

This script:
1. Parses corpus documents and extracts nodes/edges
2. Builds PyTorch Geometric graph
3. Trains GAT (Graph Attention Network) model
4. Saves graph tensor for agent runtime
"""

import argparse
import json
import re
from pathlib import Path
from typing import Dict, List, Set, Tuple

import numpy as np
import torch
import torch.nn as nn
import torch.nn.functional as F
from torch_geometric.data import Data
from torch_geometric.nn import GATConv


# Node types
NODE_TYPES = {
    "definition": 0,
    "law": 1,
    "pattern": 2,
    "violation": 3,
    "fix": 4,
    "example": 5,
    "test_template": 6,
}

# Edge types
EDGE_TYPES = {
    "defines": 0,
    "requires": 1,
    "verifies": 2,
    "violates": 3,
    "fixes": 4,
    "implements": 5,
    "relates_to": 6,
    "uses": 7,
}


class KnowledgeGraphBuilder:
    """Builds knowledge graph from corpus documents."""

    def __init__(self):
        self.nodes: Dict[str, Dict] = {}  # node_id -> node data
        self.edges: List[Tuple[str, str, str]] = []  # (src, dst, edge_type)

    def parse_corpus(self, corpus_path: Path, embeddings_path: Path):
        """
        Parse corpus documents and build graph.

        Args:
            corpus_path: Path to corpus markdown files
            embeddings_path: Path to embeddings directory
        """
        # Load embeddings
        embeddings_file = embeddings_path / "embeddings.npz"
        data = np.load(embeddings_file)
        embeddings = data["embeddings"]

        metadata_file = embeddings_path / "metadata.json"
        with open(metadata_file, "r", encoding="utf-8") as f:
            metadata = json.load(f)

        # Build embedding lookup
        embedding_lookup = {doc["id"]: embeddings[i] for i, doc in enumerate(metadata)}

        # Parse each document
        for doc in metadata:
            doc_id = doc["id"]
            doc_type = doc["type"]
            tags = doc["tags"]
            embedding = embedding_lookup.get(doc_id)

            # Add node
            self.nodes[doc_id] = {
                "id": doc_id,
                "type": doc_type,
                "tags": tags,
                "embedding": embedding,
            }

            # Extract relationships from content
            self._extract_relationships(doc_id, doc["content"], doc["source_file"])

    def _extract_relationships(self, node_id: str, content: str, source_file: str):
        """Extract relationships from document content."""
        # Extract "Related:" section
        related_pattern = r"\*\*Related:\*\*\s*([^\n]+)"
        match = re.search(related_pattern, content)

        if match:
            related_str = match.group(1)
            related_ids = [r.strip() for r in related_str.split(",")]

            for related_id in related_ids:
                # Add "relates_to" edge
                self.edges.append((node_id, related_id, "relates_to"))

        # Extract specific relationships from content patterns
        if "violation-" in node_id:
            # Violation nodes
            # Look for "Fix:" or "Related:" to fix nodes
            fix_pattern = r"fix-[\w-]+"
            fixes = re.findall(fix_pattern, content)
            for fix_id in fixes:
                self.edges.append((node_id, fix_id, "fixes"))

            # Look for law violations
            law_pattern = r"law-[\w-]+"
            laws = re.findall(law_pattern, content)
            for law_id in laws:
                self.edges.append((node_id, law_id, "violates"))

        elif "pattern-" in node_id:
            # Pattern nodes
            # Look for definitions they use
            def_pattern = r"def-[\w-]+"
            definitions = re.findall(def_pattern, content)
            for def_id in definitions:
                self.edges.append((def_id, node_id, "defines"))

            # Look for examples they implement
            example_pattern = r"example-[\w-]+"
            examples = re.findall(example_pattern, content)
            for example_id in examples:
                self.edges.append((node_id, example_id, "implements"))

        elif "test-" in node_id:
            # Test template nodes
            # Tests verify patterns
            pattern_pattern = r"pattern-[\w-]+"
            patterns = re.findall(pattern_pattern, content)
            for pattern_id in patterns:
                self.edges.append((node_id, pattern_id, "verifies"))

    def build_graph(self) -> Data:
        """
        Build PyTorch Geometric graph.

        Returns:
            PyTorch Geometric Data object
        """
        # Create node index mapping
        node_ids = sorted(self.nodes.keys())
        node_to_idx = {node_id: idx for idx, node_id in enumerate(node_ids)}

        # Build node features (embeddings)
        node_features = []
        node_types = []

        for node_id in node_ids:
            node = self.nodes[node_id]
            node_features.append(node["embedding"])
            node_types.append(NODE_TYPES[node["type"]])

        x = torch.tensor(np.array(node_features), dtype=torch.float)
        node_type = torch.tensor(node_types, dtype=torch.long)

        # Build edge index
        edge_list = []
        edge_types = []

        for src, dst, edge_type in self.edges:
            if src in node_to_idx and dst in node_to_idx:
                src_idx = node_to_idx[src]
                dst_idx = node_to_idx[dst]
                edge_list.append([src_idx, dst_idx])
                edge_types.append(EDGE_TYPES[edge_type])

        if edge_list:
            edge_index = torch.tensor(edge_list, dtype=torch.long).t().contiguous()
            edge_type = torch.tensor(edge_types, dtype=torch.long)
        else:
            edge_index = torch.empty((2, 0), dtype=torch.long)
            edge_type = torch.empty((0,), dtype=torch.long)

        # Create PyTorch Geometric Data object
        graph = Data(
            x=x, edge_index=edge_index, node_type=node_type, edge_type=edge_type
        )

        # Add node ID mapping for later use
        graph.node_ids = node_ids
        graph.node_to_idx = node_to_idx

        return graph


class ACTReasoningGNN(nn.Module):
    """Graph Attention Network for ACT reasoning."""

    def __init__(
        self,
        node_feature_dim: int = 384,
        num_node_types: int = 7,
        num_edge_types: int = 8,
        hidden_dim: int = 256,
        num_heads: int = 4,
        num_layers: int = 3,
        output_dim: int = 128,
    ):
        super().__init__()

        # Node type embedding
        self.node_type_embed = nn.Embedding(num_node_types, hidden_dim)

        # Edge type embedding (not used in basic GAT, but useful for extension)
        self.edge_type_embed = nn.Embedding(num_edge_types, hidden_dim)

        # Input projection
        self.input_proj = nn.Linear(node_feature_dim + hidden_dim, hidden_dim)

        # GAT layers
        self.gat_layers = nn.ModuleList()
        for i in range(num_layers):
            in_channels = hidden_dim if i == 0 else hidden_dim * num_heads
            self.gat_layers.append(
                GATConv(
                    in_channels,
                    hidden_dim,
                    heads=num_heads,
                    dropout=0.1,
                    concat=True,  # Concatenate attention heads
                )
            )

        # Output projection
        self.output_proj = nn.Linear(hidden_dim * num_heads, output_dim)

    def forward(self, x, edge_index, node_type, edge_type=None):
        """
        Forward pass.

        Args:
            x: Node features (N x D)
            edge_index: Edge connectivity (2 x E)
            node_type: Node types (N,)
            edge_type: Edge types (E,) - optional

        Returns:
            Node embeddings (N x output_dim)
        """
        # Combine node features with type embeddings
        type_emb = self.node_type_embed(node_type)
        h = torch.cat([x, type_emb], dim=-1)
        h = self.input_proj(h)

        # Apply GAT layers
        for i, gat in enumerate(self.gat_layers):
            h = gat(h, edge_index)
            if i < len(self.gat_layers) - 1:
                h = F.elu(h)
                h = F.dropout(h, p=0.1, training=self.training)

        # Output projection
        h = self.output_proj(h)

        return h


def train_gnn(
    graph: Data, model: ACTReasoningGNN, num_epochs: int = 100
) -> ACTReasoningGNN:
    """
    Train GNN model (optional self-supervised pretraining).

    Args:
        graph: PyTorch Geometric graph
        model: GNN model
        num_epochs: Number of training epochs

    Returns:
        Trained model
    """
    print("\nTraining GNN model...")

    optimizer = torch.optim.Adam(model.parameters(), lr=0.001)

    # Self-supervised task: Link prediction
    # Predict whether edges exist between nodes
    model.train()

    for epoch in range(num_epochs):
        optimizer.zero_grad()

        # Forward pass
        h = model(graph.x, graph.edge_index, graph.node_type, graph.edge_type)

        # Link prediction loss (sample positive and negative pairs)
        # Positive pairs: existing edges
        pos_edges = graph.edge_index[:, : min(100, graph.edge_index.size(1))]

        # Negative pairs: random non-edges
        neg_edges = torch.randint(
            0, graph.x.size(0), (2, pos_edges.size(1)), dtype=torch.long
        )

        # Compute scores
        pos_scores = (h[pos_edges[0]] * h[pos_edges[1]]).sum(dim=-1)
        neg_scores = (h[neg_edges[0]] * h[neg_edges[1]]).sum(dim=-1)

        # Binary cross-entropy loss
        loss = -torch.mean(F.logsigmoid(pos_scores)) - torch.mean(
            F.logsigmoid(-neg_scores)
        )

        loss.backward()
        optimizer.step()

        if (epoch + 1) % 20 == 0:
            print(f"  Epoch {epoch+1}/{num_epochs}, Loss: {loss.item():.4f}")

    print("  Training complete!")
    return model


def save_graph_tensor(
    graph: Data, model: ACTReasoningGNN, output_path: Path
) -> None:
    """
    Save graph tensor and model.

    Args:
        graph: PyTorch Geometric graph
        model: Trained GNN model
        output_path: Directory to save tensors
    """
    output_path.mkdir(parents=True, exist_ok=True)

    # Save graph
    graph_file = output_path / "category_graph.pt"
    torch.save(
        {
            "graph": graph,
            "node_types": NODE_TYPES,
            "edge_types": EDGE_TYPES,
        },
        graph_file,
    )
    print(f"\nSaved graph tensor to {graph_file}")

    # Save model
    model_file = output_path / "gnn_model.pt"
    torch.save(
        {
            "model_state_dict": model.state_dict(),
            "model_config": {
                "node_feature_dim": 384,
                "num_node_types": 7,
                "num_edge_types": 8,
                "hidden_dim": 256,
                "num_heads": 4,
                "num_layers": 3,
                "output_dim": 128,
            },
        },
        model_file,
    )
    print(f"Saved GNN model to {model_file}")

    # Save statistics
    stats = {
        "num_nodes": graph.x.size(0),
        "num_edges": graph.edge_index.size(1),
        "node_feature_dim": graph.x.size(1),
        "nodes_by_type": {},
        "edges_by_type": {},
    }

    for node_type_name, node_type_id in NODE_TYPES.items():
        count = (graph.node_type == node_type_id).sum().item()
        stats["nodes_by_type"][node_type_name] = count

    for edge_type_name, edge_type_id in EDGE_TYPES.items():
        count = (graph.edge_type == edge_type_id).sum().item()
        stats["edges_by_type"][edge_type_name] = count

    stats_file = output_path / "graph_stats.json"
    with open(stats_file, "w") as f:
        json.dump(stats, f, indent=2)
    print(f"Saved graph statistics to {stats_file}")

    print(f"\nGraph Statistics:")
    print(f"  Nodes: {stats['num_nodes']}")
    print(f"  Edges: {stats['num_edges']}")
    print(f"  Node feature dim: {stats['node_feature_dim']}")
    print(f"  Nodes by type:")
    for node_type, count in stats["nodes_by_type"].items():
        print(f"    {node_type}: {count}")
    print(f"  Edges by type:")
    for edge_type, count in stats["edges_by_type"].items():
        print(f"    {edge_type}: {count}")


def main():
    parser = argparse.ArgumentParser(
        description="Build knowledge graph tensor for ACT Expert"
    )
    parser.add_argument(
        "--corpus",
        type=Path,
        required=True,
        help="Path to corpus directory",
    )
    parser.add_argument(
        "--embeddings",
        type=Path,
        required=True,
        help="Path to embeddings directory",
    )
    parser.add_argument(
        "--output",
        type=Path,
        required=True,
        help="Path to output directory for tensors",
    )
    parser.add_argument(
        "--train",
        action="store_true",
        help="Train GNN model (self-supervised link prediction)",
    )
    parser.add_argument(
        "--epochs",
        type=int,
        default=100,
        help="Number of training epochs (default: 100)",
    )

    args = parser.parse_args()

    # Validate paths
    if not args.corpus.exists():
        print(f"Error: Corpus directory does not exist: {args.corpus}")
        return 1

    if not args.embeddings.exists():
        print(f"Error: Embeddings directory does not exist: {args.embeddings}")
        return 1

    # Build knowledge graph
    print("Building knowledge graph...")
    builder = KnowledgeGraphBuilder()
    builder.parse_corpus(args.corpus, args.embeddings)

    graph = builder.build_graph()
    print(f"Graph built: {graph.x.size(0)} nodes, {graph.edge_index.size(1)} edges")

    # Initialize GNN model
    model = ACTReasoningGNN(
        node_feature_dim=graph.x.size(1),
        num_node_types=len(NODE_TYPES),
        num_edge_types=len(EDGE_TYPES),
    )

    # Train model (optional)
    if args.train:
        model = train_gnn(graph, model, args.epochs)

    # Save graph and model
    save_graph_tensor(graph, model, args.output)

    print("\n✓ Knowledge graph tensor building complete!")
    return 0


if __name__ == "__main__":
    exit(main())
