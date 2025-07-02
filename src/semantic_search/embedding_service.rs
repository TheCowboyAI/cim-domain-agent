//! Embedding generation service for semantic search

use super::{Embedding, EmbeddingRequest, SemanticSearchConfig, SemanticSearchError, SemanticSearchResult};
use crate::ai_providers::{AIProviderFactory, GraphAnalysisProvider, ProviderConfig};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Service for generating embeddings
#[async_trait]
pub trait EmbeddingService: Send + Sync {
    /// Generate embedding for text
    async fn generate_embedding(&self, request: EmbeddingRequest) -> SemanticSearchResult<Embedding>;
    
    /// Generate embeddings for multiple texts
    async fn generate_embeddings(&self, requests: Vec<EmbeddingRequest>) -> SemanticSearchResult<Vec<Embedding>>;
    
    /// Get the dimensions of embeddings produced by this service
    fn dimensions(&self) -> usize;
}

/// Cache for embeddings to avoid redundant API calls
struct EmbeddingCache {
    cache: Arc<RwLock<HashMap<String, CachedEmbedding>>>,
    ttl_seconds: u64,
}

#[derive(Clone)]
struct CachedEmbedding {
    vector: Vec<f32>,
    cached_at: std::time::Instant,
}

impl EmbeddingCache {
    fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl_seconds,
        }
    }
    
    async fn get(&self, key: &str) -> Option<Vec<f32>> {
        let cache = self.cache.read().await;
        
        if let Some(cached) = cache.get(key) {
            // Check if TTL has expired
            if cached.cached_at.elapsed().as_secs() < self.ttl_seconds {
                return Some(cached.vector.clone());
            }
        }
        
        None
    }
    
    async fn put(&self, key: String, value: Vec<f32>) {
        let mut cache = self.cache.write().await;
        
        // Insert new cached embedding
        cache.insert(key, CachedEmbedding {
            vector: value,
            cached_at: std::time::Instant::now(),
        });
        
        // Implement TTL-based eviction - remove expired entries
        let ttl_seconds = self.ttl_seconds;
        cache.retain(|_, cached| {
            cached.cached_at.elapsed().as_secs() < ttl_seconds
        });
    }
}

/// Embedding service that uses AI providers
pub struct AIProviderEmbeddingService {
    #[allow(dead_code)]
    provider: Arc<dyn GraphAnalysisProvider>,
    config: SemanticSearchConfig,
    cache: Option<EmbeddingCache>,
}

impl AIProviderEmbeddingService {
    pub fn new(provider: Arc<dyn GraphAnalysisProvider>, config: SemanticSearchConfig) -> Self {
        let cache = if config.enable_cache {
            Some(EmbeddingCache::new(config.cache_ttl_seconds))
        } else {
            None
        };
        
        Self {
            provider,
            config,
            cache,
        }
    }
    
    pub async fn from_provider_config(
        provider_config: ProviderConfig,
        config: SemanticSearchConfig,
    ) -> SemanticSearchResult<Self> {
        let provider = AIProviderFactory::create_provider(&provider_config)
            .map_err(|e| SemanticSearchError::EmbeddingGenerationFailed(e.to_string()))?;
        
        Ok(Self::new(Arc::from(provider), config))
    }
    
    /// Parse embedding from provider response
    #[allow(dead_code)]
    fn parse_embedding_response(&self, response: &str) -> SemanticSearchResult<Vec<f32>> {
        // This is a simplified parser - real implementation would handle different provider formats
        let parsed: serde_json::Value = serde_json::from_str(response)
            .map_err(|e| SemanticSearchError::EmbeddingGenerationFailed(
                format!("Failed to parse response: {}", e)
            ))?;
        
        // Try OpenAI format
        if let Some(data) = parsed.get("data") {
            if let Some(first) = data.get(0) {
                if let Some(embedding) = first.get("embedding") {
                    if let Some(vec) = embedding.as_array() {
                        return vec.iter()
                            .map(|v| v.as_f64().map(|f| f as f32))
                            .collect::<Option<Vec<f32>>>()
                            .ok_or_else(|| SemanticSearchError::EmbeddingGenerationFailed(
                                "Invalid embedding format".to_string()
                            ));
                    }
                }
            }
        }
        
        // Try direct array format (some providers return just the vector)
        if let Some(vec) = parsed.as_array() {
            return vec.iter()
                .map(|v| v.as_f64().map(|f| f as f32))
                .collect::<Option<Vec<f32>>>()
                .ok_or_else(|| SemanticSearchError::EmbeddingGenerationFailed(
                    "Invalid embedding format".to_string()
                ));
        }
        
        Err(SemanticSearchError::EmbeddingGenerationFailed(
            "Unrecognized embedding response format".to_string()
        ))
    }
}

#[async_trait]
impl EmbeddingService for AIProviderEmbeddingService {
    async fn generate_embedding(&self, request: EmbeddingRequest) -> SemanticSearchResult<Embedding> {
        // Check cache if enabled
        let cache_key = format!("{}:{}", request.text, request.model.as_ref().unwrap_or(&self.config.default_model));
        
        if let Some(ref cache) = self.cache {
            if let Some(cached_vector) = cache.get(&cache_key).await {
                return Ok(Embedding {
                    id: Uuid::new_v4(),
                    vector: cached_vector,
                    source_id: request.source_id,
                    source_type: request.source_type,
                    metadata: request.metadata,
                    created_at: std::time::SystemTime::now(),
                });
            }
        }
        
        // For now, use mock implementation since providers don't have native embedding support
        // In production, this would call the appropriate embedding API
        let mock_service = MockEmbeddingService::new(self.config.dimensions);
        let embedding = mock_service.generate_embedding(request).await?;
        
        // Cache the result
        if let Some(ref cache) = self.cache {
            cache.put(cache_key, embedding.vector.clone()).await;
        }
        
        Ok(embedding)
    }
    
    async fn generate_embeddings(&self, requests: Vec<EmbeddingRequest>) -> SemanticSearchResult<Vec<Embedding>> {
        // Simple implementation - could be optimized with batching
        let mut embeddings = Vec::new();
        
        for request in requests {
            let embedding = self.generate_embedding(request).await?;
            embeddings.push(embedding);
        }
        
        Ok(embeddings)
    }
    
    fn dimensions(&self) -> usize {
        self.config.dimensions
    }
}

/// Mock embedding service for testing
pub struct MockEmbeddingService {
    dimensions: usize,
}

impl MockEmbeddingService {
    pub fn new(dimensions: usize) -> Self {
        Self { dimensions }
    }
    
    fn generate_mock_vector(&self, text: &str) -> Vec<f32> {
        let mut vector = vec![0.0; self.dimensions];
        
        // Simple hash-based embedding for consistency
        let text_lower = text.to_lowercase();
        let words: Vec<&str> = text_lower.split_whitespace().collect();
        
        for (i, word) in words.iter().enumerate() {
            let hash = word.chars().fold(0u32, |acc, c| {
                acc.wrapping_add(c as u32).wrapping_mul(31)
            });
            
            for j in 0..self.dimensions {
                let idx = (hash.wrapping_add(j as u32) % self.dimensions as u32) as usize;
                vector[idx] += 1.0 / (i + 1) as f32;
            }
        }
        
        // Normalize
        let magnitude: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for x in &mut vector {
                *x /= magnitude;
            }
        }
        
        vector
    }
}

#[async_trait]
impl EmbeddingService for MockEmbeddingService {
    async fn generate_embedding(&self, request: EmbeddingRequest) -> SemanticSearchResult<Embedding> {
        let vector = self.generate_mock_vector(&request.text);
        
        Ok(Embedding {
            id: Uuid::new_v4(),
            vector,
            source_id: request.source_id,
            source_type: request.source_type,
            metadata: request.metadata,
            created_at: std::time::SystemTime::now(),
        })
    }
    
    async fn generate_embeddings(&self, requests: Vec<EmbeddingRequest>) -> SemanticSearchResult<Vec<Embedding>> {
        let mut embeddings = Vec::new();
        
        for request in requests {
            let embedding = self.generate_embedding(request).await?;
            embeddings.push(embedding);
        }
        
        Ok(embeddings)
    }
    
    fn dimensions(&self) -> usize {
        self.dimensions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mock_embedding_service() {
        let service = MockEmbeddingService::new(384);
        
        let request = EmbeddingRequest {
            text: "test document about graphs".to_string(),
            source_id: "doc1".to_string(),
            source_type: "document".to_string(),
            metadata: HashMap::new(),
            model: None,
        };
        
        let embedding = service.generate_embedding(request).await.unwrap();
        
        assert_eq!(embedding.vector.len(), 384);
        assert_eq!(embedding.source_id, "doc1");
        assert_eq!(embedding.source_type, "document");
        
        // Vector should be normalized
        let magnitude: f32 = embedding.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 0.01);
    }
} 