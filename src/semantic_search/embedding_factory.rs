//! Factory for creating embedding services based on configuration
//!
//! This module provides a flexible way to instantiate different embedding
//! service implementations based on runtime configuration.

use super::{
    EmbeddingService, SemanticSearchResult, SemanticSearchError,
    openai_embeddings::OpenAIEmbeddingService,
    anthropic_embeddings::AnthropicEmbeddingService,
    embedding_service::MockEmbeddingService,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Configuration for embedding service
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "provider", rename_all = "lowercase")]
pub enum EmbeddingServiceConfig {
    /// OpenAI embeddings
    OpenAI {
        /// API key (can also be set via OPENAI_API_KEY env var)
        #[serde(skip_serializing_if = "Option::is_none")]
        api_key: Option<String>,
        /// Model name (e.g., "text-embedding-3-small", "text-embedding-3-large")
        #[serde(default = "default_openai_model")]
        model: String,
        /// Optional dimension override for text-embedding-3 models
        #[serde(skip_serializing_if = "Option::is_none")]
        dimensions: Option<usize>,
        /// Max concurrent requests
        #[serde(default = "default_max_concurrent")]
        max_concurrent_requests: usize,
    },
    
    /// Anthropic embeddings (using Claude for semantic representation)
    Anthropic {
        /// API key (can also be set via ANTHROPIC_API_KEY env var)
        #[serde(skip_serializing_if = "Option::is_none")]
        api_key: Option<String>,
        /// Model name (e.g., "claude-3-haiku-20240307")
        #[serde(default = "default_anthropic_model")]
        model: String,
    },
    
    /// Ollama local embeddings
    Ollama {
        /// Ollama server URL
        #[serde(default = "default_ollama_url")]
        url: String,
        /// Model name (e.g., "nomic-embed-text", "all-minilm")
        model: String,
        /// Embedding dimensions
        #[serde(default = "default_ollama_dimensions")]
        dimensions: usize,
    },
    
    /// Mock embeddings for testing
    Mock {
        /// Embedding dimensions
        #[serde(default = "default_mock_dimensions")]
        dimensions: usize,
    },
}

fn default_openai_model() -> String {
    "text-embedding-3-small".to_string()
}

fn default_anthropic_model() -> String {
    "claude-3-haiku-20240307".to_string()
}

fn default_ollama_url() -> String {
    "http://localhost:11434".to_string()
}

fn default_ollama_dimensions() -> usize {
    384
}

fn default_mock_dimensions() -> usize {
    384
}

fn default_max_concurrent() -> usize {
    10
}

impl Default for EmbeddingServiceConfig {
    fn default() -> Self {
        Self::Mock {
            dimensions: default_mock_dimensions(),
        }
    }
}

/// Factory for creating embedding services
pub struct EmbeddingServiceFactory;

impl EmbeddingServiceFactory {
    /// Create an embedding service from configuration
    pub async fn create(config: &EmbeddingServiceConfig) -> SemanticSearchResult<Arc<dyn EmbeddingService>> {
        match config {
            EmbeddingServiceConfig::OpenAI {
                api_key,
                model,
                dimensions,
                max_concurrent_requests,
            } => {
                let api_key = api_key.clone()
                    .or_else(|| std::env::var("OPENAI_API_KEY").ok())
                    .ok_or_else(|| SemanticSearchError::EmbeddingGenerationFailed(
                        "OpenAI API key not provided".to_string()
                    ))?;
                
                let service = if let Some(dims) = dimensions {
                    OpenAIEmbeddingService::with_config(
                        api_key,
                        model.clone(),
                        Some(*dims),
                        *max_concurrent_requests,
                        3,
                    )
                } else {
                    OpenAIEmbeddingService::new(api_key, Some(model.clone()))
                };
                
                Ok(Arc::new(service))
            }
            
            EmbeddingServiceConfig::Anthropic { api_key, model } => {
                let api_key = api_key.clone()
                    .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
                    .ok_or_else(|| SemanticSearchError::EmbeddingGenerationFailed(
                        "Anthropic API key not provided".to_string()
                    ))?;
                
                let service = AnthropicEmbeddingService::new(api_key, Some(model.clone()));
                Ok(Arc::new(service))
            }
            
            EmbeddingServiceConfig::Ollama { url, model, dimensions } => {
                // Create Ollama service
                let service = OllamaEmbeddingService::new(url.clone(), model.clone(), *dimensions);
                Ok(Arc::new(service))
            }
            
            EmbeddingServiceConfig::Mock { dimensions } => {
                Ok(Arc::new(MockEmbeddingService::new(*dimensions)))
            }
        }
    }
    
    /// Create an embedding service from environment variables
    pub async fn from_env() -> SemanticSearchResult<Arc<dyn EmbeddingService>> {
        let provider = std::env::var("EMBEDDING_PROVIDER").unwrap_or_else(|_| "mock".to_string());
        
        let config = match provider.as_str() {
            "openai" => {
                EmbeddingServiceConfig::OpenAI {
                    api_key: std::env::var("OPENAI_API_KEY").ok(),
                    model: std::env::var("OPENAI_EMBEDDING_MODEL")
                        .unwrap_or_else(|_| default_openai_model()),
                    dimensions: std::env::var("EMBEDDING_DIMENSIONS")
                        .ok()
                        .and_then(|s| s.parse().ok()),
                    max_concurrent_requests: std::env::var("EMBEDDING_MAX_CONCURRENT")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(10),
                }
            }
            
            "anthropic" => {
                EmbeddingServiceConfig::Anthropic {
                    api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
                    model: std::env::var("ANTHROPIC_MODEL")
                        .unwrap_or_else(|_| default_anthropic_model()),
                }
            }
            
            "ollama" => {
                EmbeddingServiceConfig::Ollama {
                    url: std::env::var("OLLAMA_URL")
                        .unwrap_or_else(|_| default_ollama_url()),
                    model: std::env::var("OLLAMA_EMBEDDING_MODEL")
                        .unwrap_or_else(|_| "nomic-embed-text".to_string()),
                    dimensions: std::env::var("EMBEDDING_DIMENSIONS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(default_ollama_dimensions()),
                }
            }
            
            _ => {
                EmbeddingServiceConfig::Mock {
                    dimensions: std::env::var("EMBEDDING_DIMENSIONS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(default_mock_dimensions()),
                }
            }
        };
        
        Self::create(&config).await
    }
}

/// Ollama embedding service implementation
pub struct OllamaEmbeddingService {
    client: reqwest::Client,
    url: String,
    model: String,
    dimensions: usize,
}

impl OllamaEmbeddingService {
    pub fn new(url: String, model: String, dimensions: usize) -> Self {
        Self {
            client: reqwest::Client::new(),
            url,
            model,
            dimensions,
        }
    }
}

#[async_trait::async_trait]
impl EmbeddingService for OllamaEmbeddingService {
    async fn generate_embedding(&self, request: super::EmbeddingRequest) -> SemanticSearchResult<super::Embedding> {
        #[derive(Serialize)]
        struct OllamaRequest {
            model: String,
            prompt: String,
        }
        
        #[derive(Deserialize)]
        struct OllamaResponse {
            embedding: Vec<f32>,
        }
        
        let ollama_request = OllamaRequest {
            model: self.model.clone(),
            prompt: request.text.clone(),
        };
        
        let response = self.client
            .post(format!("{}/api/embeddings", self.url))
            .json(&ollama_request)
            .send()
            .await
            .map_err(|e| SemanticSearchError::EmbeddingGenerationFailed(format!("Ollama request failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(SemanticSearchError::EmbeddingGenerationFailed(
                format!("Ollama returned error: {}", response.status())
            ));
        }
        
        let ollama_response: OllamaResponse = response.json().await
            .map_err(|e| SemanticSearchError::EmbeddingGenerationFailed(format!("Failed to parse Ollama response: {}", e)))?;
        
        Ok(super::Embedding {
            id: uuid::Uuid::new_v4(),
            vector: ollama_response.embedding,
            source_id: request.source_id,
            source_type: request.source_type,
            metadata: request.metadata,
            created_at: std::time::SystemTime::now(),
        })
    }
    
    async fn generate_embeddings(&self, requests: Vec<super::EmbeddingRequest>) -> SemanticSearchResult<Vec<super::Embedding>> {
        let mut embeddings = Vec::new();
        for request in requests {
            embeddings.push(self.generate_embedding(request).await?);
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
    
    #[test]
    fn test_config_serialization() {
        let config = EmbeddingServiceConfig::OpenAI {
            api_key: None,
            model: "text-embedding-3-small".to_string(),
            dimensions: Some(512),
            max_concurrent_requests: 10,
        };
        
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("openai"));
        assert!(json.contains("text-embedding-3-small"));
        
        let deserialized: EmbeddingServiceConfig = serde_json::from_str(&json).unwrap();
        match deserialized {
            EmbeddingServiceConfig::OpenAI { model, dimensions, .. } => {
                assert_eq!(model, "text-embedding-3-small");
                assert_eq!(dimensions, Some(512));
            }
            _ => panic!("Wrong config type"),
        }
    }
    
    #[tokio::test]
    async fn test_mock_service_creation() {
        let config = EmbeddingServiceConfig::Mock { dimensions: 256 };
        let service = EmbeddingServiceFactory::create(&config).await.unwrap();
        assert_eq!(service.dimensions(), 256);
    }
}