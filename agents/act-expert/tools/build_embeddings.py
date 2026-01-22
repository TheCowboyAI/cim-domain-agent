#!/usr/bin/env python3
"""
Build embeddings for ACT Expert RAG library.

Copyright (c) 2025 - Cowboy AI, LLC.

This script:
1. Parses markdown corpus documents
2. Extracts sections with ID, tags, and content
3. Generates embeddings using sentence-transformers
4. Saves embeddings in .npz format for fast loading
"""

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Dict, List, Tuple

import numpy as np
from sentence_transformers import SentenceTransformer


class CorpusDocument:
    """Represents a parsed corpus document section."""

    def __init__(
        self,
        doc_id: str,
        title: str,
        tags: List[str],
        doc_type: str,
        content: str,
        source_file: str,
    ):
        self.doc_id = doc_id
        self.title = title
        self.tags = tags
        self.doc_type = doc_type
        self.content = content
        self.source_file = source_file

    def to_dict(self) -> Dict:
        """Convert to dictionary for JSON serialization."""
        return {
            "id": self.doc_id,
            "title": self.title,
            "tags": self.tags,
            "type": self.doc_type,
            "content": self.content,
            "source_file": self.source_file,
            "content_hash": hashlib.md5(self.content.encode()).hexdigest(),
        }


def parse_markdown_corpus(corpus_path: Path) -> List[CorpusDocument]:
    """
    Parse markdown files from corpus directory.

    Expected format:
    ```markdown
    ## Section Title

    **ID:** def-category-001
    **Tags:** fundamental, category, structure
    **Type:** definition

    Content goes here...

    **Related:** def-functor-001, def-morphism-001
    ```
    """
    documents = []

    for md_file in corpus_path.glob("*.md"):
        print(f"Parsing {md_file.name}...")

        with open(md_file, "r", encoding="utf-8") as f:
            content = f.read()

        # Split by ## headers (sections)
        sections = re.split(r"\n## ", content)

        for section in sections[1:]:  # Skip file header
            lines = section.split("\n")
            title = lines[0].strip()

            # Extract metadata
            doc_id = None
            tags = []
            doc_type = None
            content_lines = []
            in_content = False

            for line in lines[1:]:
                if line.startswith("**ID:**"):
                    doc_id = line.replace("**ID:**", "").strip()
                elif line.startswith("**Tags:**"):
                    tags_str = line.replace("**Tags:**", "").strip()
                    tags = [t.strip() for t in tags_str.split(",")]
                elif line.startswith("**Type:**"):
                    doc_type = line.replace("**Type:**", "").strip()
                elif line.startswith("**Related:**"):
                    # Skip related section (used for graph edges)
                    pass
                else:
                    if doc_id and not in_content:
                        in_content = True
                    if in_content:
                        content_lines.append(line)

            if doc_id:
                doc_content = "\n".join(content_lines).strip()
                doc = CorpusDocument(
                    doc_id=doc_id,
                    title=title,
                    tags=tags,
                    doc_type=doc_type,
                    content=doc_content,
                    source_file=md_file.name,
                )
                documents.append(doc)

    return documents


def generate_embeddings(
    documents: List[CorpusDocument], model_name: str = "all-MiniLM-L6-v2"
) -> Tuple[np.ndarray, List[Dict]]:
    """
    Generate embeddings for all documents.

    Args:
        documents: List of parsed corpus documents
        model_name: Sentence transformer model name

    Returns:
        Tuple of (embeddings array, document metadata list)
    """
    print(f"\nLoading model: {model_name}...")
    model = SentenceTransformer(model_name)

    print(f"Generating embeddings for {len(documents)} documents...")

    # Combine title and content for embedding
    texts = [f"{doc.title}\n\n{doc.content}" for doc in documents]

    # Generate embeddings
    embeddings = model.encode(
        texts,
        batch_size=32,
        show_progress_bar=True,
        convert_to_numpy=True,
    )

    # Prepare metadata
    metadata = [doc.to_dict() for doc in documents]

    return embeddings, metadata


def save_embeddings(
    embeddings: np.ndarray,
    metadata: List[Dict],
    output_path: Path,
) -> None:
    """
    Save embeddings and metadata to disk.

    Args:
        embeddings: Numpy array of embeddings
        metadata: List of document metadata dictionaries
        output_path: Directory to save files
    """
    output_path.mkdir(parents=True, exist_ok=True)

    # Save embeddings as .npz
    embeddings_file = output_path / "embeddings.npz"
    np.savez_compressed(embeddings_file, embeddings=embeddings)
    print(f"\nSaved embeddings to {embeddings_file}")

    # Save metadata as JSON
    metadata_file = output_path / "metadata.json"
    with open(metadata_file, "w", encoding="utf-8") as f:
        json.dump(metadata, f, indent=2)
    print(f"Saved metadata to {metadata_file}")

    # Save statistics
    stats = {
        "num_documents": len(metadata),
        "embedding_dim": embeddings.shape[1],
        "total_size_bytes": embeddings.nbytes,
        "documents_by_type": {},
    }

    for doc in metadata:
        doc_type = doc["type"]
        stats["documents_by_type"][doc_type] = (
            stats["documents_by_type"].get(doc_type, 0) + 1
        )

    stats_file = output_path / "stats.json"
    with open(stats_file, "w", encoding="utf-8") as f:
        json.dump(stats, f, indent=2)
    print(f"Saved statistics to {stats_file}")

    print(f"\nEmbedding Statistics:")
    print(f"  Documents: {stats['num_documents']}")
    print(f"  Embedding dimension: {stats['embedding_dim']}")
    print(f"  Total size: {stats['total_size_bytes'] / 1024 / 1024:.2f} MB")
    print(f"  By type:")
    for doc_type, count in stats["documents_by_type"].items():
        print(f"    {doc_type}: {count}")


def main():
    parser = argparse.ArgumentParser(
        description="Build embeddings for ACT Expert RAG library"
    )
    parser.add_argument(
        "--corpus",
        type=Path,
        required=True,
        help="Path to corpus directory containing markdown files",
    )
    parser.add_argument(
        "--output",
        type=Path,
        required=True,
        help="Path to output directory for embeddings",
    )
    parser.add_argument(
        "--model",
        type=str,
        default="all-MiniLM-L6-v2",
        help="Sentence transformer model name (default: all-MiniLM-L6-v2)",
    )

    args = parser.parse_args()

    # Validate paths
    if not args.corpus.exists():
        print(f"Error: Corpus directory does not exist: {args.corpus}")
        return 1

    # Parse corpus
    print(f"Parsing corpus from {args.corpus}...")
    documents = parse_markdown_corpus(args.corpus)

    if not documents:
        print("Error: No documents found in corpus")
        return 1

    print(f"Found {len(documents)} documents")

    # Generate embeddings
    embeddings, metadata = generate_embeddings(documents, args.model)

    # Save results
    save_embeddings(embeddings, metadata, args.output)

    print("\n✓ Embedding generation complete!")
    return 0


if __name__ == "__main__":
    exit(main())
