//! Factory for creating vector store instances based on configuration
//!
//! This module provides a flexible way to instantiate different vector store
//! implementations based on runtime configuration.

use super::{VectorStore, InMemoryVectorStore, QdrantVectorStore, SemanticSearchResult, SemanticSearchError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use qdrant_client::prelude::*;

/// Configuration for vector store backend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum VectorStoreConfig {
    /// In-memory vector store (for development/testing)
    Memory,
    
    /// Qdrant vector database
    Qdrant {
        /// Qdrant server URL
        url: String,
        /// Collection name for embeddings
        collection_name: String,
        /// Vector dimensionality
        vector_size: usize,
        /// Optional API key
        #[serde(skip_serializing_if = "Option::is_none")]
        api_key: Option<String>,
        /// Connection timeout in seconds
        #[serde(default = "default_timeout")]
        timeout_secs: u64,
    },
}

fn default_timeout() -> u64 {
    30
}

impl Default for VectorStoreConfig {
    fn default() -> Self {
        Self::Memory
    }
}

/// Factory for creating vector store instances
pub struct VectorStoreFactory;

impl VectorStoreFactory {
    /// Create a vector store from configuration
    pub async fn create(config: &VectorStoreConfig) -> SemanticSearchResult<Arc<dyn VectorStore>> {
        match config {
            VectorStoreConfig::Memory => {
                Ok(Arc::new(InMemoryVectorStore::new()))
            }
            
            VectorStoreConfig::Qdrant {
                url,
                collection_name,
                vector_size,
                api_key,
                timeout_secs,
            } => {
                // Build client directly with the URL
                let client = QdrantClient::from_url(url).build()
                    .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
                
                let store = QdrantVectorStore::with_client(
                    client,
                    collection_name.clone(),
                    *vector_size,
                ).await?;
                
                Ok(Arc::new(store))
            }
        }
    }
    
    /// Create a vector store with environment-based configuration
    pub async fn from_env() -> SemanticSearchResult<Arc<dyn VectorStore>> {
        let config = if let Ok(qdrant_url) = std::env::var("QDRANT_URL") {
            VectorStoreConfig::Qdrant {
                url: qdrant_url,
                collection_name: std::env::var("QDRANT_COLLECTION")
                    .unwrap_or_else(|_| "embeddings".to_string()),
                vector_size: std::env::var("EMBEDDING_SIZE")
                    .unwrap_or_else(|_| "384".to_string())
                    .parse()
                    .unwrap_or(384),
                api_key: std::env::var("QDRANT_API_KEY").ok(),
                timeout_secs: std::env::var("QDRANT_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
            }
        } else {
            VectorStoreConfig::Memory
        };
        
        Self::create(&config).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_memory_store_creation() {
        let config = VectorStoreConfig::Memory;
        let store = VectorStoreFactory::create(&config).await.unwrap();
        
        // Basic smoke test
        let count = store.count().await.unwrap();
        assert_eq!(count, 0);
    }
    
    #[test]
    fn test_config_serialization() {
        let config = VectorStoreConfig::Qdrant {
            url: "http://localhost:6333".to_string(),
            collection_name: "test".to_string(),
            vector_size: 384,
            api_key: None,
            timeout_secs: 30,
        };
        
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("qdrant"));
        assert!(json.contains("http://localhost:6333"));
        
        let deserialized: VectorStoreConfig = serde_json::from_str(&json).unwrap();
        match deserialized {
            VectorStoreConfig::Qdrant { url, .. } => {
                assert_eq!(url, "http://localhost:6333");
            }
            _ => panic!("Expected Qdrant config"),
        }
    }
}