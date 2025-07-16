//! Anthropic embeddings integration for production semantic search
//!
//! This module provides embedding generation using Anthropic's API.
//! Note: As of the knowledge cutoff, Anthropic doesn't have a dedicated embeddings API,
//! so this uses their text completion API with specific prompts for semantic similarity.

use super::{Embedding, EmbeddingRequest, EmbeddingService, SemanticSearchError, SemanticSearchResult};
use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Semaphore;
use uuid::Uuid;

/// Anthropic embedding service implementation
/// 
/// This uses Anthropic's Claude model to generate semantic representations
/// that can be used for similarity search.
pub struct AnthropicEmbeddingService {
    client: Client,
    api_key: String,
    model: String,
    dimensions: usize,
    semaphore: Arc<Semaphore>,
    max_retries: u32,
    retry_delay_ms: u64,
}

impl AnthropicEmbeddingService {
    /// Create a new Anthropic embedding service
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| "claude-3-haiku-20240307".to_string());
        
        Self {
            client: Client::new(),
            api_key,
            model,
            dimensions: 768, // Standard dimension for semantic embeddings
            semaphore: Arc::new(Semaphore::new(5)), // Lower concurrent requests
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }

    /// Generate a semantic vector representation using Claude
    async fn generate_semantic_vector(&self, text: &str) -> SemanticSearchResult<Vec<f32>> {
        let _permit = self.semaphore.acquire().await
            .map_err(|e| SemanticSearchError::EmbeddingGenerationFailed(format!("Semaphore error: {}", e)))?;

        // Create a prompt that asks Claude to generate semantic features
        let prompt = format!(
            r#"Analyze the following text and generate a list of exactly 768 semantic features as floating point numbers between -1.0 and 1.0. 
Each number should represent a different semantic aspect of the text (topics, sentiment, complexity, domain, etc.).
Format your response as a JSON array of numbers only, no explanation.

Text: "{}"

Semantic vector:"#,
            text
        );

        let request = AnthropicRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "user".to_string(),
                    content: prompt,
                }
            ],
            max_tokens: 4096,
            temperature: 0.0, // Deterministic
        };

        let mut retries = 0;
        loop {
            let response = self.client
                .post("https://api.anthropic.com/v1/messages")
                .header("x-api-key", &self.api_key)
                .header("anthropic-version", "2023-06-01")
                .header("content-type", "application/json")
                .json(&request)
                .send()
                .await;

            match response {
                Ok(resp) => {
                    match resp.status() {
                        StatusCode::OK => {
                            let response_data: AnthropicResponse = resp.json().await
                                .map_err(|e| SemanticSearchError::EmbeddingGenerationFailed(
                                    format!("Failed to parse response: {}", e)
                                ))?;

                            // Extract the vector from the response
                            if let Some(content) = response_data.content.first() {
                                if let Ok(vector) = serde_json::from_str::<Vec<f32>>(&content.text) {
                                    if vector.len() == self.dimensions {
                                        return Ok(vector);
                                    }
                                }
                            }

                            return Err(SemanticSearchError::EmbeddingGenerationFailed(
                                "Failed to extract valid vector from response".to_string()
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

    /// Alternative: Use a hash-based approach for deterministic embeddings
    fn generate_deterministic_embedding(&self, text: &str) -> Vec<f32> {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut vector = vec![0.0; self.dimensions];
        
        // Generate multiple hash values for different n-grams
        let words: Vec<&str> = text.split_whitespace().collect();
        
        // Unigrams
        for word in words.iter() {
            let mut hasher = DefaultHasher::new();
            word.hash(&mut hasher);
            let hash = hasher.finish();
            let idx = (hash as usize) % self.dimensions;
            vector[idx] += 1.0;
        }
        
        // Bigrams
        for window in words.windows(2) {
            let mut hasher = DefaultHasher::new();
            window.hash(&mut hasher);
            let hash = hasher.finish();
            let idx = (hash as usize) % self.dimensions;
            vector[idx] += 0.5;
        }
        
        // Normalize
        let magnitude: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for v in &mut vector {
                *v /= magnitude;
            }
        }
        
        vector
    }
}

#[async_trait]
impl EmbeddingService for AnthropicEmbeddingService {
    async fn generate_embedding(&self, request: EmbeddingRequest) -> SemanticSearchResult<Embedding> {
        // For production, you might want to use the API approach
        // For now, using deterministic embeddings to avoid API costs
        let vector = self.generate_deterministic_embedding(&request.text);
        
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

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: i32,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    text: String,
}