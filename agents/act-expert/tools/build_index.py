#!/usr/bin/env python3
"""
Build search indices for ACT Expert RAG library.

Copyright (c) 2025 - Cowboy AI, LLC.

This script:
1. Loads embeddings from build_embeddings.py output
2. Builds FAISS vector index for semantic search
3. Builds BM25 index for keyword search
4. Saves indices for fast loading during agent runtime
"""

import argparse
import json
import pickle
from pathlib import Path
from typing import Dict, List

import faiss
import numpy as np
from rank_bm25 import BM25Okapi


def load_embeddings(embeddings_path: Path) -> tuple[np.ndarray, List[Dict]]:
    """
    Load embeddings and metadata.

    Args:
        embeddings_path: Path to embeddings directory

    Returns:
        Tuple of (embeddings array, metadata list)
    """
    # Load embeddings
    embeddings_file = embeddings_path / "embeddings.npz"
    data = np.load(embeddings_file)
    embeddings = data["embeddings"]

    # Load metadata
    metadata_file = embeddings_path / "metadata.json"
    with open(metadata_file, "r", encoding="utf-8") as f:
        metadata = json.load(f)

    print(f"Loaded {len(metadata)} documents with {embeddings.shape[1]}-dim embeddings")
    return embeddings, metadata


def build_faiss_index(
    embeddings: np.ndarray, index_type: str = "Flat"
) -> faiss.Index:
    """
    Build FAISS index for semantic search.

    Args:
        embeddings: Numpy array of embeddings (N x D)
        index_type: FAISS index type ("Flat", "IVF", "HNSW")

    Returns:
        FAISS index
    """
    dimension = embeddings.shape[0]
    embedding_dim = embeddings.shape[1]

    print(f"\nBuilding FAISS index (type: {index_type})...")

    if index_type == "Flat":
        # L2 distance (exact search)
        index = faiss.IndexFlatL2(embedding_dim)
    elif index_type == "IVF":
        # Inverted file index (approximate search, faster)
        nlist = min(100, dimension // 10)  # Number of clusters
        quantizer = faiss.IndexFlatL2(embedding_dim)
        index = faiss.IndexIVFFlat(quantizer, embedding_dim, nlist)
        # Train index
        print(f"  Training IVF index with {nlist} clusters...")
        index.train(embeddings)
    elif index_type == "HNSW":
        # Hierarchical Navigable Small World (very fast approximate search)
        M = 32  # Number of connections per layer
        index = faiss.IndexHNSWFlat(embedding_dim, M)
    else:
        raise ValueError(f"Unknown index type: {index_type}")

    # Add vectors to index
    print(f"  Adding {dimension} vectors...")
    index.add(embeddings)

    print(f"  Index built: {index.ntotal} vectors indexed")
    return index


def build_bm25_index(metadata: List[Dict]) -> BM25Okapi:
    """
    Build BM25 keyword index.

    Args:
        metadata: List of document metadata

    Returns:
        BM25Okapi index
    """
    print("\nBuilding BM25 keyword index...")

    # Tokenize documents (simple whitespace + lowercase)
    corpus = []
    for doc in metadata:
        # Combine title, tags, and content
        text = f"{doc['title']} {' '.join(doc['tags'])} {doc['content']}"
        tokens = text.lower().split()
        corpus.append(tokens)

    # Build BM25 index
    bm25 = BM25Okapi(corpus)

    print(f"  BM25 index built: {len(corpus)} documents indexed")
    return bm25


def save_indices(
    faiss_index: faiss.Index, bm25_index: BM25Okapi, output_path: Path
) -> None:
    """
    Save indices to disk.

    Args:
        faiss_index: FAISS vector index
        bm25_index: BM25 keyword index
        output_path: Directory to save indices
    """
    output_path.mkdir(parents=True, exist_ok=True)

    # Save FAISS index
    faiss_file = output_path / "semantic.faiss"
    faiss.write_index(faiss_index, str(faiss_file))
    print(f"\nSaved FAISS index to {faiss_file}")

    # Save BM25 index (pickle)
    bm25_file = output_path / "keyword.pkl"
    with open(bm25_file, "wb") as f:
        pickle.dump(bm25_index, f)
    print(f"Saved BM25 index to {bm25_file}")

    # Save index configuration
    config = {
        "faiss_index_type": type(faiss_index).__name__,
        "faiss_ntotal": faiss_index.ntotal,
        "bm25_num_docs": len(bm25_index.doc_freqs),
    }

    config_file = output_path / "index_config.json"
    with open(config_file, "w", encoding="utf-8") as f:
        json.dump(config, f, indent=2)
    print(f"Saved index configuration to {config_file}")


def test_indices(
    faiss_index: faiss.Index,
    bm25_index: BM25Okapi,
    embeddings: np.ndarray,
    metadata: List[Dict],
) -> None:
    """
    Test indices with sample queries.

    Args:
        faiss_index: FAISS vector index
        bm25_index: BM25 keyword index
        embeddings: Original embeddings
        metadata: Document metadata
    """
    print("\n" + "=" * 60)
    print("Testing Indices")
    print("=" * 60)

    # Test FAISS (semantic search)
    print("\nTest 1: FAISS Semantic Search")
    print("Query: Find documents about functor laws")

    # Use first document as query
    query_embedding = embeddings[0:1]
    distances, indices = faiss_index.search(query_embedding, k=3)

    print("\nTop 3 results:")
    for rank, (idx, dist) in enumerate(zip(indices[0], distances[0]), 1):
        doc = metadata[idx]
        print(f"  {rank}. {doc['title']} (distance: {dist:.4f})")
        print(f"     ID: {doc['id']}, Tags: {', '.join(doc['tags'])}")

    # Test BM25 (keyword search)
    print("\nTest 2: BM25 Keyword Search")
    print("Query: 'functor composition law'")

    query_tokens = "functor composition law".lower().split()
    scores = bm25_index.get_scores(query_tokens)
    top_indices = np.argsort(scores)[-3:][::-1]

    print("\nTop 3 results:")
    for rank, idx in enumerate(top_indices, 1):
        doc = metadata[idx]
        print(f"  {rank}. {doc['title']} (score: {scores[idx]:.4f})")
        print(f"     ID: {doc['id']}, Tags: {', '.join(doc['tags'])}")


def main():
    parser = argparse.ArgumentParser(
        description="Build search indices for ACT Expert RAG library"
    )
    parser.add_argument(
        "--embeddings",
        type=Path,
        required=True,
        help="Path to embeddings directory (output of build_embeddings.py)",
    )
    parser.add_argument(
        "--output",
        type=Path,
        required=True,
        help="Path to output directory for indices",
    )
    parser.add_argument(
        "--index-type",
        type=str,
        choices=["Flat", "IVF", "HNSW"],
        default="Flat",
        help="FAISS index type (default: Flat)",
    )
    parser.add_argument(
        "--test",
        action="store_true",
        help="Run test queries after building indices",
    )

    args = parser.parse_args()

    # Validate paths
    if not args.embeddings.exists():
        print(f"Error: Embeddings directory does not exist: {args.embeddings}")
        return 1

    # Load embeddings
    embeddings, metadata = load_embeddings(args.embeddings)

    # Build FAISS index
    faiss_index = build_faiss_index(embeddings, args.index_type)

    # Build BM25 index
    bm25_index = build_bm25_index(metadata)

    # Save indices
    save_indices(faiss_index, bm25_index, args.output)

    # Test indices
    if args.test:
        test_indices(faiss_index, bm25_index, embeddings, metadata)

    print("\n✓ Index building complete!")
    return 0


if __name__ == "__main__":
    exit(main())
