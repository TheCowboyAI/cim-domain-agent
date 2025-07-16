//! OpenAI embeddings integration for production semantic search
//!
//! This module provides production-ready embedding generation using OpenAI's
//! text-embedding models.

use super::{Embedding, EmbeddingRequest, EmbeddingService, SemanticSearchError, SemanticSearchResult};
use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Semaphore;
use uuid::Uuid;

/// OpenAI embedding service implementation
pub struct OpenAIEmbeddingService {
    client: Client,
    api_key: String,
    model: String,
    dimensions: usize,
    /// Rate limiting
    semaphore: Arc<Semaphore>,
    /// Retry configuration
    max_retries: u32,
    retry_delay_ms: u64,
}

impl OpenAIEmbeddingService {
    /// Create a new OpenAI embedding service
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| "text-embedding-3-small".to_string());
        let dimensions = match model.as_str() {
            "text-embedding-3-small" => 1536,
            "text-embedding-3-large" => 3072,
            "text-embedding-ada-002" => 1536,
            _ => 1536, // default
        };

        Self {
            client: Client::new(),
            api_key,
            model,
            dimensions,
            semaphore: Arc::new(Semaphore::new(10)), // Max 10 concurrent requests
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }

    /// Create with custom configuration
    pub fn with_config(
        api_key: String,
        model: String,
        dimensions: Option<usize>,
        max_concurrent_requests: usize,
        max_retries: u32,
    ) -> Self {
        let dimensions = dimensions.unwrap_or_else(|| match model.as_str() {
            "text-embedding-3-small" => 1536,
            "text-embedding-3-large" => 3072,
            "text-embedding-ada-002" => 1536,
            _ => 1536,
        });

        Self {
            client: Client::new(),
            api_key,
            model,
            dimensions,
            semaphore: Arc::new(Semaphore::new(max_concurrent_requests)),
            max_retries,
            retry_delay_ms: 1000,
        }
    }

    /// Call OpenAI API with retry logic
    async fn call_api(&self, texts: Vec<String>) -> SemanticSearchResult<EmbeddingResponse> {
        let _permit = self.semaphore.acquire().await
            .map_err(|e| SemanticSearchError::EmbeddingGenerationFailed(format!("Semaphore error: {}", e)))?;

        let request = OpenAIApiRequest {
            input: texts,
            model: self.model.clone(),
            encoding_format: Some("float".to_string()),
            dimensions: if self.model.starts_with("text-embedding-3") {
                Some(self.dimensions)
            } else {
                None
            },
            user: None,
        };

        let mut retries = 0;
        loop {
            let response = self.client
                .post("https://api.openai.com/v1/embeddings")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await;

            match response {
                Ok(resp) => {
                    match resp.status() {
                        StatusCode::OK => {
                            return resp.json::<EmbeddingResponse>()
                                .await
                                .map_err(|e| SemanticSearchError::EmbeddingGenerationFailed(
                                    format!("Failed to parse response: {}", e)
                                ));
                        }
                        StatusCode::TOO_MANY_REQUESTS => {
                            if retries < self.max_retries {
                                retries += 1;
                                tokio::time::sleep(tokio::time::Duration::from_millis(
                                    self.retry_delay_ms * retries as u64
                                )).await;
                                continue;
                            }
                            return Err(SemanticSearchError::EmbeddingGenerationFailed(
                                "Rate limit exceeded".to_string()
                            ));
                        }
                        StatusCode::UNAUTHORIZED => {
                            return Err(SemanticSearchError::EmbeddingGenerationFailed(
                                "Invalid API key".to_string()
                            ));
                        }
                        _ => {
                            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                            return Err(SemanticSearchError::EmbeddingGenerationFailed(
                                format!("API error: {}", error_text)
                            ));
                        }
                    }
                }
                Err(e) => {
                    if retries < self.max_retries {
                        retries += 1;
                        tokio::time::sleep(tokio::time::Duration::from_millis(
                            self.retry_delay_ms * retries as u64
                        )).await;
                        continue;
                    }
                    return Err(SemanticSearchError::EmbeddingGenerationFailed(
                        format!("Request failed: {}", e)
                    ));
                }
            }
        }
    }
}

#[async_trait]
impl EmbeddingService for OpenAIEmbeddingService {
    async fn generate_embedding(&self, request: EmbeddingRequest) -> SemanticSearchResult<Embedding> {
        let response = self.call_api(vec![request.text.clone()]).await?;

        if response.data.is_empty() {
            return Err(SemanticSearchError::EmbeddingGenerationFailed(
                "No embedding returned".to_string()
            ));
        }

        let vector = response.data[0].embedding.clone();
        
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
        if requests.is_empty() {
            return Ok(Vec::new());
        }

        // Batch requests to OpenAI API (max 2048 inputs per request)
        const BATCH_SIZE: usize = 100;
        let mut all_embeddings = Vec::new();

        for chunk in requests.chunks(BATCH_SIZE) {
            let texts: Vec<String> = chunk.iter().map(|r| r.text.clone()).collect();
            let response = self.call_api(texts).await?;

            if response.data.len() != chunk.len() {
                return Err(SemanticSearchError::EmbeddingGenerationFailed(
                    format!("Expected {} embeddings, got {}", chunk.len(), response.data.len())
                ));
            }

            for (i, embedding_data) in response.data.iter().enumerate() {
                let request = &chunk[i];
                all_embeddings.push(Embedding {
                    id: Uuid::new_v4(),
                    vector: embedding_data.embedding.clone(),
                    source_id: request.source_id.clone(),
                    source_type: request.source_type.clone(),
                    metadata: request.metadata.clone(),
                    created_at: std::time::SystemTime::now(),
                });
            }
        }

        Ok(all_embeddings)
    }

    fn dimensions(&self) -> usize {
        self.dimensions
    }
}

/// OpenAI API request structure  
#[derive(Debug, Serialize)]
struct OpenAIApiRequest {
    input: Vec<String>,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

/// OpenAI API response structure
#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
    #[allow(dead_code)]
    model: String,
    #[allow(dead_code)]
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
    #[allow(dead_code)]
    index: i32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Usage {
    prompt_tokens: i32,
    total_tokens: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_dimensions() {
        let service = OpenAIEmbeddingService::new("test_key".to_string(), None);
        assert_eq!(service.dimensions(), 1536);

        let service = OpenAIEmbeddingService::new(
            "test_key".to_string(),
            Some("text-embedding-3-large".to_string())
        );
        assert_eq!(service.dimensions(), 3072);
    }

    #[tokio::test]
    async fn test_embedding_generation() {
        // This test requires a valid API key to run
        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            let service = OpenAIEmbeddingService::new(api_key, None);
            
            let request = super::super::EmbeddingRequest {
                text: "Test embedding generation".to_string(),
                source_id: "test_1".to_string(),
                source_type: "test".to_string(),
                metadata: HashMap::new(),
                model: None,
            };

            let result = service.generate_embedding(request).await;
            assert!(result.is_ok());
            
            let embedding = result.unwrap();
            assert_eq!(embedding.vector.len(), service.dimensions());
        } else {
            eprintln!("Skipping test: OPENAI_API_KEY not set");
        }
    }
}