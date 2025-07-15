//! High-level semantic search engine

use super::{
    EmbeddingRequest, EmbeddingService, 
    SemanticSearchResult,
    VectorStore,
};
use crate::semantic_search::SemanticSearchConfig;
use super::vector_store::SearchFilter;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// A search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// The text to search for
    pub query: String,
    
    /// Maximum number of results to return
    pub limit: Option<usize>,
    
    /// Minimum similarity threshold (0.0 to 1.0)
    pub min_similarity: Option<f32>,
    
    /// Filter criteria
    pub filter: Option<SearchFilter>,
    
    /// Whether to include metadata in results
    pub include_metadata: bool,
}

impl SearchQuery {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            limit: None,
            min_similarity: None,
            filter: None,
            include_metadata: true,
        }
    }
    
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    pub fn with_min_similarity(mut self, threshold: f32) -> Self {
        self.min_similarity = Some(threshold);
        self
    }
    
    pub fn with_filter(mut self, filter: SearchFilter) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// A search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// The source ID of the matching item
    pub source_id: String,
    
    /// The type of the source
    pub source_type: String,
    
    /// Similarity score (0.0 to 1.0)
    pub similarity: f32,
    
    /// Metadata if requested
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    
    /// The embedding ID
    pub embedding_id: uuid::Uuid,
}

/// Semantic search engine
#[async_trait]
pub trait SemanticSearchEngine: Send + Sync {
    /// Index a document or item
    async fn index(&self, request: EmbeddingRequest) -> SemanticSearchResult<uuid::Uuid>;
    
    /// Index multiple items
    async fn index_batch(&self, requests: Vec<EmbeddingRequest>) -> SemanticSearchResult<Vec<uuid::Uuid>>;
    
    /// Search for similar items
    async fn search(&self, query: SearchQuery) -> SemanticSearchResult<Vec<SearchResult>>;
    
    /// Delete an indexed item by source ID
    async fn delete(&self, source_id: &str) -> SemanticSearchResult<usize>;
    
    /// Clear all indexed items
    async fn clear(&self) -> SemanticSearchResult<()>;
    
    /// Get statistics about the index
    async fn stats(&self) -> SemanticSearchResult<IndexStats>;
}

/// Statistics about the search index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    /// Total number of indexed items
    pub total_items: usize,
    
    /// Breakdown by source type
    pub items_by_type: HashMap<String, usize>,
    
    /// Vector dimensions
    pub dimensions: usize,
}

/// Default implementation of semantic search engine
pub struct DefaultSemanticSearchEngine {
    embedding_service: Arc<dyn EmbeddingService>,
    vector_store: Arc<dyn VectorStore>,
    config: SemanticSearchConfig,
}

impl DefaultSemanticSearchEngine {
    pub fn new(
        embedding_service: Arc<dyn EmbeddingService>,
        vector_store: Arc<dyn VectorStore>,
        config: SemanticSearchConfig,
    ) -> Self {
        Self {
            embedding_service,
            vector_store,
            config,
        }
    }
}

#[async_trait]
impl SemanticSearchEngine for DefaultSemanticSearchEngine {
    async fn index(&self, request: EmbeddingRequest) -> SemanticSearchResult<uuid::Uuid> {
        // Generate embedding
        let embedding = self.embedding_service.generate_embedding(request).await?;
        let id = embedding.id;
        
        // Store in vector store
        self.vector_store.store(embedding).await?;
        
        Ok(id)
    }
    
    async fn index_batch(&self, requests: Vec<EmbeddingRequest>) -> SemanticSearchResult<Vec<uuid::Uuid>> {
        // Generate embeddings
        let embeddings = self.embedding_service.generate_embeddings(requests).await?;
        let ids: Vec<uuid::Uuid> = embeddings.iter().map(|e| e.id).collect();
        
        // Store in vector store
        self.vector_store.store_batch(embeddings).await?;
        
        Ok(ids)
    }
    
    async fn search(&self, query: SearchQuery) -> SemanticSearchResult<Vec<SearchResult>> {
        // Generate query embedding
        let query_request = EmbeddingRequest {
            text: query.query.clone(),
            source_id: "query".to_string(),
            source_type: "query".to_string(),
            metadata: HashMap::new(),
            model: None,
        };
        
        let query_embedding = self.embedding_service.generate_embedding(query_request).await?;
        
        // Search in vector store
        let limit = query.limit.unwrap_or(self.config.max_results);
        let min_similarity = query.min_similarity.unwrap_or(self.config.min_similarity);
        
        let results = if let Some(filter) = query.filter {
            self.vector_store.search_with_filter(
                &query_embedding.vector,
                limit,
                Some(min_similarity),
                filter,
            ).await?
        } else {
            self.vector_store.search(
                &query_embedding.vector,
                limit,
                Some(min_similarity),
            ).await?
        };
        
        // Convert to search results
        let search_results: Vec<SearchResult> = results
            .into_iter()
            .map(|(embedding, similarity)| SearchResult {
                source_id: embedding.source_id,
                source_type: embedding.source_type,
                similarity,
                metadata: if query.include_metadata {
                    Some(embedding.metadata)
                } else {
                    None
                },
                embedding_id: embedding.id,
            })
            .collect();
        
        Ok(search_results)
    }
    
    async fn delete(&self, source_id: &str) -> SemanticSearchResult<usize> {
        self.vector_store.delete_by_source(source_id).await
    }
    
    async fn clear(&self) -> SemanticSearchResult<()> {
        self.vector_store.clear().await
    }
    
    async fn stats(&self) -> SemanticSearchResult<IndexStats> {
        let total_items = self.vector_store.count().await?;
        
        // TODO: Implement breakdown by type
        // This would require iterating through all embeddings or maintaining separate counters
        let items_by_type = HashMap::new();
        
        Ok(IndexStats {
            total_items,
            items_by_type,
            dimensions: self.embedding_service.dimensions(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_search::{
        embedding_service::MockEmbeddingService,
        vector_store::InMemoryVectorStore,
    };
    
    #[tokio::test]
    async fn test_semantic_search_engine() {
        // Create components
        let embedding_service = Arc::new(MockEmbeddingService::new(384));
        let vector_store = Arc::new(InMemoryVectorStore::new());
        let config = SemanticSearchConfig::default();
        
        let engine = DefaultSemanticSearchEngine::new(
            embedding_service,
            vector_store,
            config,
        );
        
        // Index some documents
        let docs = vec![
            EmbeddingRequest {
                text: "Graph-based workflow optimization".to_string(),
                source_id: "doc1".to_string(),
                source_type: "document".to_string(),
                metadata: HashMap::from([
                    ("title".to_string(), serde_json::json!("Workflow Guide")),
                ]),
                model: None,
            },
            EmbeddingRequest {
                text: "Sequential processing patterns".to_string(),
                source_id: "doc2".to_string(),
                source_type: "document".to_string(),
                metadata: HashMap::new(),
                model: None,
            },
            EmbeddingRequest {
                text: "Parallel execution strategies".to_string(),
                source_id: "doc3".to_string(),
                source_type: "document".to_string(),
                metadata: HashMap::new(),
                model: None,
            },
        ];
        
        let ids = engine.index_batch(docs).await.unwrap();
        assert_eq!(ids.len(), 3);
        
        // Search
        let query = SearchQuery::new("workflow optimization patterns")
            .with_limit(2)
            .with_min_similarity(0.1);
        
        let results = engine.search(query).await.unwrap();
        
        assert!(!results.is_empty());
        assert!(results.len() <= 2);
        
        // Results should be sorted by similarity
        for i in 1..results.len() {
            assert!(results[i-1].similarity >= results[i].similarity);
        }
        
        // Stats
        let stats = engine.stats().await.unwrap();
        assert_eq!(stats.total_items, 3);
        assert_eq!(stats.dimensions, 384);
        
        // Delete
        let deleted = engine.delete("doc1").await.unwrap();
        assert_eq!(deleted, 1);
        
        let stats = engine.stats().await.unwrap();
        assert_eq!(stats.total_items, 2);
    }
} 