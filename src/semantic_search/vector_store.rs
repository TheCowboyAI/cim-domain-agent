//! Vector storage for embeddings

use super::{Embedding, SemanticSearchError, SemanticSearchResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Trait for vector storage backends
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Store an embedding
    async fn store(&self, embedding: Embedding) -> SemanticSearchResult<()>;
    
    /// Store multiple embeddings
    async fn store_batch(&self, embeddings: Vec<Embedding>) -> SemanticSearchResult<()>;
    
    /// Retrieve an embedding by ID
    async fn get(&self, id: &Uuid) -> SemanticSearchResult<Embedding>;
    
    /// Search for similar embeddings
    async fn search(
        &self,
        query: &[f32],
        limit: usize,
        min_similarity: Option<f32>,
    ) -> SemanticSearchResult<Vec<(Embedding, f32)>>;
    
    /// Search with filters
    async fn search_with_filter(
        &self,
        query: &[f32],
        limit: usize,
        min_similarity: Option<f32>,
        filter: SearchFilter,
    ) -> SemanticSearchResult<Vec<(Embedding, f32)>>;
    
    /// Delete an embedding
    async fn delete(&self, id: &Uuid) -> SemanticSearchResult<()>;
    
    /// Delete embeddings by source
    async fn delete_by_source(&self, source_id: &str) -> SemanticSearchResult<usize>;
    
    /// Get count of stored embeddings
    async fn count(&self) -> SemanticSearchResult<usize>;
    
    /// Clear all embeddings
    async fn clear(&self) -> SemanticSearchResult<()>;
}

/// Filter criteria for vector search
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SearchFilter {
    /// Filter by source type
    pub source_types: Option<Vec<String>>,
    
    /// Filter by metadata key-value pairs
    pub metadata_filters: HashMap<String, serde_json::Value>,
    
    /// Filter by date range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<std::time::SystemTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<std::time::SystemTime>,
}


/// In-memory vector store implementation
pub struct InMemoryVectorStore {
    embeddings: Arc<RwLock<HashMap<Uuid, Embedding>>>,
    index: Arc<RwLock<Vec<Uuid>>>, // Simple index, could be optimized with HNSW or similar
}

impl InMemoryVectorStore {
    pub fn new() -> Self {
        Self {
            embeddings: Arc::new(RwLock::new(HashMap::new())),
            index: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Default for InMemoryVectorStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl VectorStore for InMemoryVectorStore {
    async fn store(&self, embedding: Embedding) -> SemanticSearchResult<()> {
        let mut embeddings = self.embeddings.write()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        let mut index = self.index.write()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        
        let id = embedding.id;
        embeddings.insert(id, embedding);
        if !index.contains(&id) {
            index.push(id);
        }
        
        Ok(())
    }
    
    async fn store_batch(&self, batch: Vec<Embedding>) -> SemanticSearchResult<()> {
        let mut embeddings = self.embeddings.write()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        let mut index = self.index.write()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        
        for embedding in batch {
            let id = embedding.id;
            embeddings.insert(id, embedding);
            if !index.contains(&id) {
                index.push(id);
            }
        }
        
        Ok(())
    }
    
    async fn get(&self, id: &Uuid) -> SemanticSearchResult<Embedding> {
        let embeddings = self.embeddings.read()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        
        embeddings.get(id)
            .cloned()
            .ok_or_else(|| SemanticSearchError::NotFound(format!("Embedding {id} not found")))
    }
    
    async fn search(
        &self,
        query: &[f32],
        limit: usize,
        min_similarity: Option<f32>,
    ) -> SemanticSearchResult<Vec<(Embedding, f32)>> {
        let embeddings = self.embeddings.read()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        
        let mut results: Vec<(Embedding, f32)> = Vec::new();
        
        // Calculate similarities for all embeddings
        for embedding in embeddings.values() {
            if embedding.vector.len() != query.len() {
                continue; // Skip embeddings with different dimensions
            }
            
            let similarity = cosine_similarity(&embedding.vector, query);
            
            if let Some(threshold) = min_similarity {
                if similarity < threshold {
                    continue;
                }
            }
            
            results.push((embedding.clone(), similarity));
        }
        
        // Sort by similarity (descending)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Limit results
        results.truncate(limit);
        
        Ok(results)
    }
    
    async fn search_with_filter(
        &self,
        query: &[f32],
        limit: usize,
        min_similarity: Option<f32>,
        filter: SearchFilter,
    ) -> SemanticSearchResult<Vec<(Embedding, f32)>> {
        let embeddings = self.embeddings.read()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        
        let mut results: Vec<(Embedding, f32)> = Vec::new();
        
        for embedding in embeddings.values() {
            // Apply filters
            if let Some(ref types) = filter.source_types {
                if !types.contains(&embedding.source_type) {
                    continue;
                }
            }
            
            if let Some(after) = filter.created_after {
                if embedding.created_at < after {
                    continue;
                }
            }
            
            if let Some(before) = filter.created_before {
                if embedding.created_at > before {
                    continue;
                }
            }
            
            // Check metadata filters
            let mut metadata_match = true;
            for (key, value) in &filter.metadata_filters {
                if let Some(emb_value) = embedding.metadata.get(key) {
                    if emb_value != value {
                        metadata_match = false;
                        break;
                    }
                } else {
                    metadata_match = false;
                    break;
                }
            }
            
            if !metadata_match {
                continue;
            }
            
            // Calculate similarity
            if embedding.vector.len() != query.len() {
                continue;
            }
            
            let similarity = cosine_similarity(&embedding.vector, query);
            
            if let Some(threshold) = min_similarity {
                if similarity < threshold {
                    continue;
                }
            }
            
            results.push((embedding.clone(), similarity));
        }
        
        // Sort and limit
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        
        Ok(results)
    }
    
    async fn delete(&self, id: &Uuid) -> SemanticSearchResult<()> {
        let mut embeddings = self.embeddings.write()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        let mut index = self.index.write()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        
        embeddings.remove(id);
        index.retain(|&x| x != *id);
        
        Ok(())
    }
    
    async fn delete_by_source(&self, source_id: &str) -> SemanticSearchResult<usize> {
        let mut embeddings = self.embeddings.write()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        let mut index = self.index.write()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        
        let mut deleted = 0;
        let to_delete: Vec<Uuid> = embeddings
            .iter()
            .filter(|(_, emb)| emb.source_id == source_id)
            .map(|(id, _)| *id)
            .collect();
        
        for id in to_delete {
            embeddings.remove(&id);
            index.retain(|&x| x != id);
            deleted += 1;
        }
        
        Ok(deleted)
    }
    
    async fn count(&self) -> SemanticSearchResult<usize> {
        let embeddings = self.embeddings.read()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        
        Ok(embeddings.len())
    }
    
    async fn clear(&self) -> SemanticSearchResult<()> {
        let mut embeddings = self.embeddings.write()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        let mut index = self.index.write()
            .map_err(|e| SemanticSearchError::VectorStoreError(e.to_string()))?;
        
        embeddings.clear();
        index.clear();
        
        Ok(())
    }
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if mag_a == 0.0 || mag_b == 0.0 {
        0.0
    } else {
        (dot_product / (mag_a * mag_b)).clamp(-1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[tokio::test]
    async fn test_in_memory_store() {
        let store = InMemoryVectorStore::new();
        
        // Create test embedding
        let embedding = Embedding {
            id: Uuid::new_v4(),
            vector: vec![0.1, 0.2, 0.3],
            source_id: "test_doc".to_string(),
            source_type: "document".to_string(),
            metadata: HashMap::from([
                ("category".to_string(), json!("test")),
            ]),
            created_at: std::time::SystemTime::now(),
        };
        
        // Store
        store.store(embedding.clone()).await.unwrap();
        
        // Retrieve
        let retrieved = store.get(&embedding.id).await.unwrap();
        assert_eq!(retrieved.source_id, embedding.source_id);
        
        // Search
        let results = store.search(&embedding.vector, 10, None).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].1 > 0.99); // Should be very similar to itself
        
        // Count
        let count = store.count().await.unwrap();
        assert_eq!(count, 1);
        
        // Delete
        store.delete(&embedding.id).await.unwrap();
        let count = store.count().await.unwrap();
        assert_eq!(count, 0);
    }
} 