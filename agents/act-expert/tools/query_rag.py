#!/usr/bin/env python3
"""
Query interface for ACT Expert RAG library.

Copyright (c) 2025 - Cowboy AI, LLC.

This script provides a command-line interface for querying the RAG library
and demonstrates how the ACT expert agent will use the knowledge base.
"""

import argparse
import json
import pickle
from pathlib import Path
from typing import Dict, List, Optional

import faiss
import numpy as np
import torch
from sentence_transformers import SentenceTransformer
from rank_bm25 import BM25Okapi
from torch_geometric.data import Data


class ACTKnowledgeBase:
    """
    ACT Expert knowledge base with semantic, keyword, and graph search.
    """

    def __init__(
        self,
        embeddings_path: Path,
        index_path: Path,
        tensor_path: Optional[Path] = None,
    ):
        """
        Initialize knowledge base.

        Args:
            embeddings_path: Path to embeddings directory
            index_path: Path to index directory
            tensor_path: Path to tensor directory (optional)
        """
        print("Loading ACT Knowledge Base...")

        # Load embeddings and metadata
        self._load_embeddings(embeddings_path)

        # Load search indices
        self._load_indices(index_path)

        # Load knowledge graph (optional)
        if tensor_path and tensor_path.exists():
            self._load_graph(tensor_path)
        else:
            self.graph = None
            self.gnn_model = None

        # Load sentence transformer model for query encoding
        print("Loading sentence transformer model...")
        self.model = SentenceTransformer("all-MiniLM-L6-v2")

        print("✓ Knowledge base loaded successfully!")

    def _load_embeddings(self, embeddings_path: Path):
        """Load embeddings and metadata."""
        print(f"  Loading embeddings from {embeddings_path}...")

        # Load embeddings
        embeddings_file = embeddings_path / "embeddings.npz"
        data = np.load(embeddings_file)
        self.embeddings = data["embeddings"]

        # Load metadata
        metadata_file = embeddings_path / "metadata.json"
        with open(metadata_file, "r", encoding="utf-8") as f:
            self.metadata = json.load(f)

        print(f"    Loaded {len(self.metadata)} documents")

    def _load_indices(self, index_path: Path):
        """Load FAISS and BM25 indices."""
        print(f"  Loading indices from {index_path}...")

        # Load FAISS index
        faiss_file = index_path / "semantic.faiss"
        self.faiss_index = faiss.read_index(str(faiss_file))

        # Load BM25 index
        bm25_file = index_path / "keyword.pkl"
        with open(bm25_file, "rb") as f:
            self.bm25_index = pickle.load(f)

        print(f"    FAISS: {self.faiss_index.ntotal} vectors indexed")
        print(f"    BM25: {len(self.bm25_index.doc_freqs)} documents indexed")

    def _load_graph(self, tensor_path: Path):
        """Load knowledge graph tensor."""
        print(f"  Loading knowledge graph from {tensor_path}...")

        graph_file = tensor_path / "category_graph.pt"
        data = torch.load(graph_file)
        self.graph = data["graph"]

        print(f"    Graph: {self.graph.x.size(0)} nodes, {self.graph.edge_index.size(1)} edges")

    def query_semantic(self, query: str, top_k: int = 5) -> List[Dict]:
        """
        Semantic search using FAISS.

        Args:
            query: Natural language query
            top_k: Number of results to return

        Returns:
            List of matching documents with scores
        """
        # Encode query
        query_embedding = self.model.encode([query], convert_to_numpy=True)

        # Search FAISS index
        distances, indices = self.faiss_index.search(query_embedding, k=top_k)

        # Prepare results
        results = []
        for idx, dist in zip(indices[0], distances[0]):
            doc = self.metadata[idx].copy()
            doc["score"] = float(dist)
            doc["search_type"] = "semantic"
            results.append(doc)

        return results

    def query_keyword(self, query: str, top_k: int = 5) -> List[Dict]:
        """
        Keyword search using BM25.

        Args:
            query: Natural language query
            top_k: Number of results to return

        Returns:
            List of matching documents with scores
        """
        # Tokenize query
        query_tokens = query.lower().split()

        # Search BM25 index
        scores = self.bm25_index.get_scores(query_tokens)
        top_indices = np.argsort(scores)[-top_k:][::-1]

        # Prepare results
        results = []
        for idx in top_indices:
            doc = self.metadata[idx].copy()
            doc["score"] = float(scores[idx])
            doc["search_type"] = "keyword"
            results.append(doc)

        return results

    def query_hybrid(
        self, query: str, top_k: int = 5, alpha: float = 0.5
    ) -> List[Dict]:
        """
        Hybrid search combining semantic and keyword search.

        Args:
            query: Natural language query
            top_k: Number of results to return
            alpha: Weight for semantic search (1-alpha for keyword)

        Returns:
            List of matching documents with combined scores
        """
        # Get semantic scores
        semantic_results = self.query_semantic(query, top_k=len(self.metadata))
        semantic_scores = {doc["id"]: doc["score"] for doc in semantic_results}

        # Get keyword scores
        keyword_results = self.query_keyword(query, top_k=len(self.metadata))
        keyword_scores = {doc["id"]: doc["score"] for doc in keyword_results}

        # Normalize scores to [0, 1]
        def normalize(scores: Dict[str, float]) -> Dict[str, float]:
            values = list(scores.values())
            min_val = min(values)
            max_val = max(values)
            if max_val == min_val:
                return {k: 0.5 for k in scores}
            return {k: (v - min_val) / (max_val - min_val) for k, v in scores.items()}

        semantic_norm = normalize(semantic_scores)
        keyword_norm = normalize(keyword_scores)

        # Combine scores
        all_doc_ids = set(semantic_norm.keys()) | set(keyword_norm.keys())
        combined_scores = {}

        for doc_id in all_doc_ids:
            sem_score = semantic_norm.get(doc_id, 0.0)
            kw_score = keyword_norm.get(doc_id, 0.0)
            combined_scores[doc_id] = alpha * sem_score + (1 - alpha) * kw_score

        # Sort and get top k
        top_doc_ids = sorted(
            combined_scores.keys(), key=lambda x: combined_scores[x], reverse=True
        )[:top_k]

        # Prepare results
        results = []
        for doc_id in top_doc_ids:
            # Find document
            doc = next((d for d in self.metadata if d["id"] == doc_id), None)
            if doc:
                doc = doc.copy()
                doc["score"] = combined_scores[doc_id]
                doc["search_type"] = "hybrid"
                doc["semantic_score"] = semantic_norm.get(doc_id, 0.0)
                doc["keyword_score"] = keyword_norm.get(doc_id, 0.0)
                results.append(doc)

        return results

    def query(
        self, query: str, query_type: str = "hybrid", top_k: int = 5
    ) -> List[Dict]:
        """
        Main query interface.

        Args:
            query: Natural language query
            query_type: "semantic", "keyword", or "hybrid"
            top_k: Number of results to return

        Returns:
            List of matching documents
        """
        if query_type == "semantic":
            return self.query_semantic(query, top_k)
        elif query_type == "keyword":
            return self.query_keyword(query, top_k)
        elif query_type == "hybrid":
            return self.query_hybrid(query, top_k)
        else:
            raise ValueError(f"Unknown query type: {query_type}")


def format_result(doc: Dict, rank: int, show_content: bool = False) -> str:
    """Format a search result for display."""
    lines = []
    lines.append(f"\n{rank}. {doc['title']}")
    lines.append(f"   ID: {doc['id']}")
    lines.append(f"   Type: {doc['type']}")
    lines.append(f"   Tags: {', '.join(doc['tags'])}")
    lines.append(f"   Score: {doc['score']:.4f} ({doc['search_type']})")

    if "semantic_score" in doc and "keyword_score" in doc:
        lines.append(
            f"   Breakdown: semantic={doc['semantic_score']:.4f}, keyword={doc['keyword_score']:.4f}"
        )

    if show_content:
        content_preview = doc["content"][:200] + "..." if len(doc["content"]) > 200 else doc["content"]
        lines.append(f"\n   Content:\n   {content_preview}")

    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(
        description="Query ACT Expert RAG library"
    )
    parser.add_argument(
        "--embeddings",
        type=Path,
        required=True,
        help="Path to embeddings directory",
    )
    parser.add_argument(
        "--index",
        type=Path,
        required=True,
        help="Path to index directory",
    )
    parser.add_argument(
        "--tensor",
        type=Path,
        help="Path to tensor directory (optional)",
    )
    parser.add_argument(
        "--query",
        type=str,
        help="Query string (if not provided, enter interactive mode)",
    )
    parser.add_argument(
        "--type",
        type=str,
        choices=["semantic", "keyword", "hybrid"],
        default="hybrid",
        help="Query type (default: hybrid)",
    )
    parser.add_argument(
        "--top-k",
        type=int,
        default=5,
        help="Number of results to return (default: 5)",
    )
    parser.add_argument(
        "--show-content",
        action="store_true",
        help="Show document content in results",
    )

    args = parser.parse_args()

    # Initialize knowledge base
    kb = ACTKnowledgeBase(args.embeddings, args.index, args.tensor)

    # Single query or interactive mode
    if args.query:
        # Single query
        print(f"\nQuery: {args.query}")
        print(f"Type: {args.type}")
        print("=" * 60)

        results = kb.query(args.query, args.type, args.top_k)

        for rank, doc in enumerate(results, 1):
            print(format_result(doc, rank, args.show_content))

    else:
        # Interactive mode
        print("\n" + "=" * 60)
        print("ACT Expert Knowledge Base - Interactive Query")
        print("=" * 60)
        print("\nCommands:")
        print("  :type <semantic|keyword|hybrid> - Change query type")
        print("  :k <number> - Change top-k results")
        print("  :content <on|off> - Toggle content display")
        print("  :quit - Exit")
        print()

        query_type = args.type
        top_k = args.top_k
        show_content = args.show_content

        while True:
            try:
                query = input("Query> ").strip()

                if not query:
                    continue

                # Handle commands
                if query.startswith(":"):
                    parts = query.split(maxsplit=1)
                    cmd = parts[0]

                    if cmd == ":quit":
                        break
                    elif cmd == ":type" and len(parts) > 1:
                        query_type = parts[1]
                        print(f"Query type set to: {query_type}")
                    elif cmd == ":k" and len(parts) > 1:
                        top_k = int(parts[1])
                        print(f"Top-k set to: {top_k}")
                    elif cmd == ":content" and len(parts) > 1:
                        show_content = parts[1].lower() == "on"
                        print(f"Content display: {'on' if show_content else 'off'}")
                    else:
                        print("Unknown command")
                    continue

                # Execute query
                results = kb.query(query, query_type, top_k)

                print()
                for rank, doc in enumerate(results, 1):
                    print(format_result(doc, rank, show_content))
                print()

            except KeyboardInterrupt:
                print("\n\nExiting...")
                break
            except Exception as e:
                print(f"Error: {e}")

    return 0


if __name__ == "__main__":
    exit(main())
