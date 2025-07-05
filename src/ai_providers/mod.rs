//! AI Provider integrations for agent domain
//!
//! This module provides integrations with various AI services and models
//! to enable actual AI capabilities for agents.

use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};
use thiserror::Error;
use crate::value_objects::{
    AnalysisCapability, AnalysisResult, TransformationSuggestion,
};

pub mod mock;
pub mod openai;
pub mod anthropic;
pub mod ollama;
pub mod config;
pub mod provider_manager;

// Re-export commonly used types
pub use config::{create_provider_config, ProviderType, load_provider_config};
pub use provider_manager::{AIProviderManager, SelectionStrategy};

/// Errors that can occur during AI provider operations
#[derive(Debug, Error)]
pub enum AIProviderError {
    #[error("API request failed: {0}")]
    ApiError(String),
    
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
    
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Unsupported capability: {0:?}")]
    UnsupportedCapability(AnalysisCapability),
    
    #[error("Generic error: {0}")]
    Generic(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

/// Result type for AI provider operations
pub type AIProviderResult<T> = Result<T, AIProviderError>;

/// Trait for AI providers that can analyze graphs
#[async_trait]
pub trait GraphAnalysisProvider: Send + Sync {
    /// Analyze a graph with the given capability
    async fn analyze_graph(
        &self,
        graph_data: GraphData,
        analysis_type: AnalysisCapability,
        parameters: HashMap<String, Value>,
    ) -> AIProviderResult<AnalysisResult>;
    
    /// Generate transformation suggestions for a graph
    async fn suggest_transformations(
        &self,
        graph_data: GraphData,
        optimization_goals: Vec<String>,
        constraints: HashMap<String, Value>,
    ) -> AIProviderResult<Vec<TransformationSuggestion>>;
    
    /// Check if the provider supports a specific capability
    fn supports_capability(&self, capability: &AnalysisCapability) -> bool;
    
    /// Get provider metadata
    fn get_metadata(&self) -> ProviderMetadata;
}

/// Data structure representing a graph for analysis
#[derive(Debug, Clone)]
pub struct GraphData {
    /// Graph identifier
    pub graph_id: uuid::Uuid,
    
    /// Nodes in the graph
    pub nodes: Vec<NodeData>,
    
    /// Edges in the graph
    pub edges: Vec<EdgeData>,
    
    /// Graph metadata
    pub metadata: HashMap<String, Value>,
}

/// Data structure representing a node
#[derive(Debug, Clone)]
pub struct NodeData {
    pub id: String,
    pub node_type: String,
    pub label: String,
    pub properties: HashMap<String, Value>,
    pub position: Option<(f32, f32, f32)>,
}

/// Data structure representing an edge
#[derive(Debug, Clone)]
pub struct EdgeData {
    pub id: String,
    pub source: String,
    pub target: String,
    pub edge_type: String,
    pub properties: HashMap<String, Value>,
}

/// Metadata about an AI provider
#[derive(Debug, Clone)]
pub struct ProviderMetadata {
    pub name: String,
    pub version: String,
    pub model: String,
    pub capabilities: Vec<AnalysisCapability>,
    pub rate_limits: Option<RateLimits>,
}

/// Rate limit information
#[derive(Debug, Clone)]
pub struct RateLimits {
    pub requests_per_minute: u32,
    pub tokens_per_minute: u32,
    pub concurrent_requests: u32,
}

/// Factory for creating AI providers
pub struct AIProviderFactory;

impl AIProviderFactory {
    /// Create a provider based on configuration
    pub fn create_provider(config: &ProviderConfig) -> AIProviderResult<Box<dyn GraphAnalysisProvider>> {
        match config {
            ProviderConfig::Mock => {
                Ok(Box::new(mock::MockAIProvider::new()))
            }
            #[cfg(feature = "ai-openai")]
            ProviderConfig::OpenAI { api_key, model } => {
                Ok(Box::new(openai::OpenAIProvider::new(api_key.clone(), model.clone())?))
            }
            #[cfg(feature = "ai-anthropic")]
            ProviderConfig::Anthropic { api_key, model } => {
                Ok(Box::new(anthropic::AnthropicProvider::new(api_key.clone(), model.clone())?))
            }
            #[cfg(feature = "ai-ollama")]
            ProviderConfig::Ollama { host, model } => {
                Ok(Box::new(ollama::OllamaProvider::new(model.clone(), Some(host.clone()))?))
            }
            #[cfg(not(feature = "ai-openai"))]
            ProviderConfig::OpenAI { .. } => Err(AIProviderError::ConfigurationError(
                "OpenAI provider not available (feature not enabled)".to_string()
            )),
            #[cfg(not(feature = "ai-anthropic"))]
            ProviderConfig::Anthropic { .. } => Err(AIProviderError::ConfigurationError(
                "Anthropic provider not available (feature not enabled)".to_string()
            )),
            #[cfg(not(feature = "ai-ollama"))]
            ProviderConfig::Ollama { .. } => Err(AIProviderError::ConfigurationError(
                "Ollama provider not available (feature not enabled)".to_string()
            )),
        }
    }
}

/// Configuration for AI providers
#[derive(Debug, Clone)]
pub enum ProviderConfig {
    /// Mock provider for testing
    Mock,
    
    /// OpenAI API provider
    OpenAI {
        api_key: String,
        model: String,
    },
    
    /// Anthropic Claude API provider
    Anthropic {
        api_key: String,
        model: String,
    },
    
    /// Ollama local model provider
    Ollama {
        host: String,
        model: String,
    },
}

/// Convert graph data to a text representation for AI providers
pub fn graph_to_prompt(graph: &GraphData) -> String {
    let mut prompt = String::new();
    
    // Graph metadata
    prompt.push_str(&format!("Graph ID: {}\n", graph.graph_id));
    if !graph.metadata.is_empty() {
        prompt.push_str("Metadata:\n");
        for (key, value) in &graph.metadata {
            prompt.push_str(&format!("  {key}: {value}\n"));
        }
    }
    
    // Nodes
    prompt.push_str(&format!("\nNodes ({}):\n", graph.nodes.len()));
    for node in &graph.nodes {
        prompt.push_str(&format!("- {} [{}]: {}\n", node.id, node.node_type, node.label));
        if !node.properties.is_empty() {
            for (key, value) in &node.properties {
                prompt.push_str(&format!("    {key}: {value}\n"));
            }
        }
    }
    
    // Edges
    prompt.push_str(&format!("\nEdges ({}):\n", graph.edges.len()));
    for edge in &graph.edges {
        prompt.push_str(&format!("- {} -> {} [{}]\n", edge.source, edge.target, edge.edge_type));
        if !edge.properties.is_empty() {
            for (key, value) in &edge.properties {
                prompt.push_str(&format!("    {key}: {value}\n"));
            }
        }
    }
    
    prompt
} 