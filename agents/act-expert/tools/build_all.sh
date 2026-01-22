#!/usr/bin/env bash
#
# Build complete ACT Expert RAG library
# Copyright (c) 2025 - Cowboy AI, LLC.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AGENT_DIR="$(dirname "$SCRIPT_DIR")"

# Paths
CORPUS_DIR="$AGENT_DIR/rag/corpus"
EMBEDDINGS_DIR="$AGENT_DIR/rag/embeddings"
INDEX_DIR="$AGENT_DIR/rag/index"
TENSOR_DIR="$AGENT_DIR/tensors"

echo -e "${BLUE}=====================================${NC}"
echo -e "${BLUE}  ACT Expert RAG Library Builder${NC}"
echo -e "${BLUE}=====================================${NC}"
echo ""

# Check if corpus exists
if [ ! -d "$CORPUS_DIR" ]; then
    echo -e "${RED}✗ Error: Corpus directory not found: $CORPUS_DIR${NC}"
    exit 1
fi

# Count corpus files
CORPUS_COUNT=$(find "$CORPUS_DIR" -name "*.md" | wc -l)
if [ "$CORPUS_COUNT" -eq 0 ]; then
    echo -e "${RED}✗ Error: No markdown files found in corpus directory${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Found $CORPUS_COUNT corpus documents${NC}"

# Check Python dependencies
echo -e "\n${BLUE}[1/5] Checking dependencies...${NC}"

if ! python3 -c "import sentence_transformers" 2>/dev/null; then
    echo -e "${YELLOW}⚠ Missing: sentence-transformers${NC}"
    echo -e "  Install with: pip install -r requirements.txt"
    exit 1
fi

if ! python3 -c "import faiss" 2>/dev/null; then
    echo -e "${YELLOW}⚠ Missing: faiss${NC}"
    echo -e "  Install with: pip install faiss-cpu (or faiss-gpu)"
    exit 1
fi

if ! python3 -c "import torch" 2>/dev/null; then
    echo -e "${YELLOW}⚠ Missing: torch${NC}"
    echo -e "  Install with: pip install torch"
    exit 1
fi

if ! python3 -c "import torch_geometric" 2>/dev/null; then
    echo -e "${YELLOW}⚠ Missing: torch-geometric${NC}"
    echo -e "  Install with: pip install torch-geometric"
    exit 1
fi

echo -e "${GREEN}✓ All dependencies installed${NC}"

# Build embeddings
echo -e "\n${BLUE}[2/5] Building embeddings...${NC}"

python3 "$SCRIPT_DIR/build_embeddings.py" \
    --corpus "$CORPUS_DIR" \
    --output "$EMBEDDINGS_DIR"

if [ $? -ne 0 ]; then
    echo -e "${RED}✗ Failed to build embeddings${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Embeddings built successfully${NC}"

# Build indices
echo -e "\n${BLUE}[3/5] Building search indices...${NC}"

python3 "$SCRIPT_DIR/build_index.py" \
    --embeddings "$EMBEDDINGS_DIR" \
    --output "$INDEX_DIR" \
    --index-type Flat

if [ $? -ne 0 ]; then
    echo -e "${RED}✗ Failed to build indices${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Search indices built successfully${NC}"

# Build knowledge graph tensor
echo -e "\n${BLUE}[4/5] Building knowledge graph tensor...${NC}"

python3 "$SCRIPT_DIR/build_tensors.py" \
    --corpus "$CORPUS_DIR" \
    --embeddings "$EMBEDDINGS_DIR" \
    --output "$TENSOR_DIR" \
    --train \
    --epochs 100

if [ $? -ne 0 ]; then
    echo -e "${RED}✗ Failed to build knowledge graph${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Knowledge graph tensor built successfully${NC}"

# Test queries
echo -e "\n${BLUE}[5/5] Running test queries...${NC}"

TEST_QUERIES=(
    "How do I verify functor laws?"
    "What are common functor violations?"
    "Natural transformation naturality condition"
)

for query in "${TEST_QUERIES[@]}"; do
    echo -e "\n${YELLOW}Testing: \"$query\"${NC}"

    python3 "$SCRIPT_DIR/query_rag.py" \
        --embeddings "$EMBEDDINGS_DIR" \
        --index "$INDEX_DIR" \
        --tensor "$TENSOR_DIR" \
        --query "$query" \
        --type hybrid \
        --top-k 3

    echo ""
done

echo -e "${GREEN}✓ Test queries completed${NC}"

# Print summary
echo -e "\n${BLUE}=====================================${NC}"
echo -e "${BLUE}  Build Complete!${NC}"
echo -e "${BLUE}=====================================${NC}"

echo -e "\nDirectory structure:"
echo -e "  Corpus:     $CORPUS_DIR ($CORPUS_COUNT files)"
echo -e "  Embeddings: $EMBEDDINGS_DIR"
echo -e "  Indices:    $INDEX_DIR"
echo -e "  Tensors:    $TENSOR_DIR"

echo -e "\nTo query the knowledge base:"
echo -e "  ${YELLOW}python3 tools/query_rag.py \\${NC}"
echo -e "    ${YELLOW}--embeddings $EMBEDDINGS_DIR \\${NC}"
echo -e "    ${YELLOW}--index $INDEX_DIR \\${NC}"
echo -e "    ${YELLOW}--tensor $TENSOR_DIR${NC}"

echo -e "\nNext steps:"
echo -e "  1. Deploy RAG service (FastAPI)"
echo -e "  2. Integrate with Llama4 agent"
echo -e "  3. Connect to NATS"
echo -e "  4. Test with real DomainFunctor implementations"

echo -e "\n${GREEN}✓ ACT Expert RAG library ready!${NC}\n"
