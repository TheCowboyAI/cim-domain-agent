//! Semantic search capabilities for AI agents
//!
//! This module provides vector-based semantic search functionality that enables
//! agents to find similar concepts, documents, and patterns across domains.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

pub mod vector_store;
pub mod embedding_service;
pub mod search_engine;
pub mod qdrant_store;
pub mod vector_store_factory;
pub mod openai_embeddings;
pub mod anthropic_embeddings;
pub mod embedding_factory;

pub use vector_store::{VectorStore, InMemoryVectorStore, SearchFilter};
pub use embedding_service::{EmbeddingService, AIProviderEmbeddingService, MockEmbeddingService};
pub use search_engine::{SemanticSearchEngine, SearchQuery, SearchResult};
pub use qdrant_store::QdrantVectorStore;
pub use vector_store_factory::{VectorStoreFactory, VectorStoreConfig};
pub use openai_embeddings::OpenAIEmbeddingService;
pub use anthropic_embeddings::AnthropicEmbeddingService;
pub use embedding_factory::{EmbeddingServiceFactory, EmbeddingServiceConfig};

/// Errors that can occur during semantic search operations
#[derive(Debug, Error)]
pub enum SemanticSearchError {
    #[error("Embedding generation failed: {0}")]
    EmbeddingGenerationFailed(String),
    
    #[error("Vector store error: {0}")]
    VectorStoreError(String),
    
    #[error("Search failed: {0}")]
    SearchFailed(String),
    
    #[error("Invalid dimension: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    
    #[error("Not found: {0}")]
    NotFound(String),
}

pub type SemanticSearchResult<T> = Result<T, SemanticSearchError>;

/// A vector embedding with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    /// Unique identifier for the embedding
    pub id: Uuid,
    
    /// The vector representation
    pub vector: Vec<f32>,
    
    /// Source identifier (e.g., document ID, node ID)
    pub source_id: String,
    
    /// Type of the source (e.g., "document", "graph_node", "concept")
    pub source_type: String,
    
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
    
    /// When this embedding was created
    pub created_at: std::time::SystemTime,
}

impl Embedding {
    /// Calculate cosine similarity with another embedding
    pub fn cosine_similarity(&self, other: &Embedding) -> SemanticSearchResult<f32> {
        if self.vector.len() != other.vector.len() {
            return Err(SemanticSearchError::DimensionMismatch {
                expected: self.vector.len(),
                actual: other.vector.len(),
            });
        }
        
        let dot_product: f32 = self.vector.iter()
            .zip(&other.vector)
            .map(|(a, b)| a * b)
            .sum();
            
        let mag_self: f32 = self.vector.iter()
            .map(|x| x * x)
            .sum::<f32>()
            .sqrt();
            
        let mag_other: f32 = other.vector.iter()
            .map(|x| x * x)
            .sum::<f32>()
            .sqrt();
        
        if mag_self == 0.0 || mag_other == 0.0 {
            Ok(0.0)
        } else {
            Ok((dot_product / (mag_self * mag_other)).clamp(-1.0, 1.0))
        }
    }
    
    /// Calculate Euclidean distance to another embedding
    pub fn euclidean_distance(&self, other: &Embedding) -> SemanticSearchResult<f32> {
        if self.vector.len() != other.vector.len() {
            return Err(SemanticSearchError::DimensionMismatch {
                expected: self.vector.len(),
                actual: other.vector.len(),
            });
        }
        
        let sum_sq_diff: f32 = self.vector.iter()
            .zip(&other.vector)
            .map(|(a, b)| (a - b).powi(2))
            .sum();
            
        Ok(sum_sq_diff.sqrt())
    }
}

/// Request to generate an embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    /// Text to embed
    pub text: String,
    
    /// Source identifier
    pub source_id: String,
    
    /// Type of the source
    pub source_type: String,
    
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
    
    /// Model to use (optional, uses default if not specified)
    pub model: Option<String>,
}

/// Configuration for semantic search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchConfig {
    /// Default embedding model
    pub default_model: String,
    
    /// Vector dimensions
    pub dimensions: usize,
    
    /// Maximum results to return
    pub max_results: usize,
    
    /// Minimum similarity threshold
    pub min_similarity: f32,
    
    /// Whether to use caching
    pub enable_cache: bool,
    
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
}

impl Default for SemanticSearchConfig {
    fn default() -> Self {
        Self {
            default_model: "text-embedding-3-small".to_string(),
            dimensions: 1536, // OpenAI text-embedding-3-small dimensions
            max_results: 10,
            min_similarity: 0.0,
            enable_cache: true,
            cache_ttl_seconds: 3600, // 1 hour
        }
    }
} 