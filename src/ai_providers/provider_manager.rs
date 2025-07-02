//! AI Provider Manager for managing multiple AI provider connections
//!
//! This module provides a centralized manager for AI providers that handles
//! connection pooling, rate limiting, and provider selection.

use super::*;
use std::sync::{Arc, RwLock};
use tokio::sync::Semaphore;
use std::time::{SystemTime, Duration};
use tracing::{info, warn, error};

/// Manager for AI providers
pub struct AIProviderManager {
    /// Available providers
    providers: Arc<RwLock<HashMap<String, Arc<Box<dyn GraphAnalysisProvider>>>>>,
    
    /// Rate limiters per provider
    rate_limiters: Arc<RwLock<HashMap<String, RateLimiter>>>,
    
    /// Provider selection strategy
    selection_strategy: SelectionStrategy,
    
    /// Default provider ID
    default_provider: Option<String>,
}

/// Rate limiter for a provider
struct RateLimiter {
    /// Semaphore for concurrent requests
    concurrent_limit: Arc<Semaphore>,
    
    /// Last request times for rate limiting
    request_times: Arc<RwLock<Vec<SystemTime>>>,
    
    /// Requests per minute limit
    rpm_limit: u32,
}

/// Strategy for selecting providers
#[derive(Debug, Clone)]
pub enum SelectionStrategy {
    /// Always use the default provider
    Default,
    
    /// Round-robin between available providers
    RoundRobin,
    
    /// Select based on capability support
    CapabilityBased,
    
    /// Select based on lowest latency
    LowestLatency,
}

impl AIProviderManager {
    /// Create a new provider manager
    pub fn new(selection_strategy: SelectionStrategy) -> Self {
        Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
            rate_limiters: Arc::new(RwLock::new(HashMap::new())),
            selection_strategy,
            default_provider: None,
        }
    }
    
    /// Register a provider
    pub fn register_provider(
        &mut self,
        id: String,
        provider: Box<dyn GraphAnalysisProvider>,
        set_as_default: bool,
    ) -> Result<(), AIProviderError> {
        info!("Registering AI provider: {}", id);
        
        // Get provider metadata for rate limits
        let metadata = provider.get_metadata();
        
        // Create rate limiter
        let rate_limiter = if let Some(limits) = metadata.rate_limits {
            RateLimiter {
                concurrent_limit: Arc::new(Semaphore::new(limits.concurrent_requests as usize)),
                request_times: Arc::new(RwLock::new(Vec::new())),
                rpm_limit: limits.requests_per_minute,
            }
        } else {
            // Default limits
            RateLimiter {
                concurrent_limit: Arc::new(Semaphore::new(5)),
                request_times: Arc::new(RwLock::new(Vec::new())),
                rpm_limit: 60,
            }
        };
        
        // Store provider and rate limiter
        self.providers.write().unwrap().insert(id.clone(), Arc::new(provider));
        self.rate_limiters.write().unwrap().insert(id.clone(), rate_limiter);
        
        if set_as_default || self.default_provider.is_none() {
            self.default_provider = Some(id);
        }
        
        Ok(())
    }
    
    /// Initialize providers from environment variables
    pub async fn initialize_from_env(&mut self) -> Result<(), AIProviderError> {
        info!("Initializing AI providers from environment");
        
        // Always register mock provider
        self.register_provider(
            "mock".to_string(),
            Box::new(mock::MockAIProvider::new()),
            false,
        )?;
        
        // Try to initialize OpenAI
        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            info!("Found OpenAI API key, initializing provider");
            let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4".to_string());
            
            match openai::OpenAIProvider::new(api_key, model) {
                Ok(provider) => {
                    self.register_provider(
                        "openai".to_string(),
                        Box::new(provider),
                        true, // Set as default if available
                    )?;
                }
                Err(e) => {
                    warn!("Failed to initialize OpenAI provider: {}", e);
                }
            }
        }
        
        // Try to initialize Anthropic
        if let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") {
            info!("Found Anthropic API key, initializing provider");
            let model = std::env::var("ANTHROPIC_MODEL").unwrap_or_else(|_| "claude-3-opus-20240229".to_string());
            
            match anthropic::AnthropicProvider::new(api_key, model) {
                Ok(provider) => {
                    self.register_provider(
                        "anthropic".to_string(),
                        Box::new(provider),
                        self.default_provider.is_none(),
                    )?;
                }
                Err(e) => {
                    warn!("Failed to initialize Anthropic provider: {}", e);
                }
            }
        }
        
        // Try to initialize Ollama
        let ollama_host = std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string());
        let ollama_model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama2".to_string());
        
        match ollama::OllamaProvider::new(ollama_model, Some(ollama_host)) {
            Ok(provider) => {
                info!("Initialized Ollama provider");
                self.register_provider(
                    "ollama".to_string(),
                    Box::new(provider),
                    self.default_provider.is_none(),
                )?;
            }
            Err(e) => {
                warn!("Failed to initialize Ollama provider: {}", e);
            }
        }
        
        info!(
            "Initialized {} AI providers. Default: {:?}",
            self.providers.read().unwrap().len(),
            self.default_provider
        );
        
        Ok(())
    }
    
    /// Get a provider for analysis
    pub async fn get_provider_for_analysis(
        &self,
        capability: &AnalysisCapability,
    ) -> Result<(String, Arc<Box<dyn GraphAnalysisProvider>>), AIProviderError> {
        let providers = self.providers.read().unwrap();
        
        if providers.is_empty() {
            return Err(AIProviderError::ConfigurationError("No providers available".to_string()));
        }
        
        let provider_id = match &self.selection_strategy {
            SelectionStrategy::Default => {
                self.default_provider.as_ref()
                    .ok_or_else(|| AIProviderError::ConfigurationError("No default provider set".to_string()))?
                    .clone()
            }
            SelectionStrategy::CapabilityBased => {
                // Find a provider that supports the capability
                providers.iter()
                    .find(|(_, provider)| provider.supports_capability(capability))
                    .map(|(id, _)| id.clone())
                    .ok_or_else(|| AIProviderError::UnsupportedCapability(capability.clone()))?
            }
            _ => {
                // For now, fall back to default for other strategies
                self.default_provider.as_ref()
                    .ok_or_else(|| AIProviderError::ConfigurationError("No default provider set".to_string()))?
                    .clone()
            }
        };
        
        // Wait for rate limit
        self.wait_for_rate_limit(&provider_id).await?;
        
        // Get the Arc-wrapped provider
        let provider_arc = providers.get(&provider_id).unwrap().clone();
        
        Ok((provider_id, provider_arc))
    }
    
    /// Wait for rate limit
    async fn wait_for_rate_limit(&self, provider_id: &str) -> Result<(), AIProviderError> {
        let rate_limiters = self.rate_limiters.read().unwrap();
        
        if let Some(limiter) = rate_limiters.get(provider_id) {
            // Acquire concurrent request permit
            let _permit = limiter.concurrent_limit.acquire().await
                .map_err(|_| AIProviderError::RateLimitExceeded)?;
            
            // Check RPM limit
            let now = SystemTime::now();
            let one_minute_ago = now.checked_sub(Duration::from_secs(60))
                .unwrap_or(SystemTime::UNIX_EPOCH);
            
            let mut request_times = limiter.request_times.write().unwrap();
            
            // Remove old request times
            request_times.retain(|&time| time > one_minute_ago);
            
            // Check if we're at the limit
            if request_times.len() >= limiter.rpm_limit as usize {
                let oldest = request_times[0];
                let next_allowed = oldest.checked_add(Duration::from_secs(60))
                    .unwrap_or(oldest);
                
                if let Ok(wait_duration) = next_allowed.duration_since(now) {
                    warn!("Rate limit reached for {}, waiting {:?}", provider_id, wait_duration);
                    tokio::time::sleep(wait_duration).await;
                }
            }
            
            // Record this request
            request_times.push(now);
        }
        
        Ok(())
    }
    
    /// Analyze a graph using the selected provider
    pub async fn analyze_graph(
        &self,
        graph_data: GraphData,
        analysis_type: AnalysisCapability,
        parameters: HashMap<String, Value>,
    ) -> AIProviderResult<AnalysisResult> {
        let (provider_id, provider) = self.get_provider_for_analysis(&analysis_type).await?;
        
        info!("Using provider {} for {:?} analysis", provider_id, analysis_type);
        
        // Clone values needed for fallback
        let graph_data_clone = graph_data.clone();
        let analysis_type_clone = analysis_type.clone();
        let parameters_clone = parameters.clone();
        
        match provider.analyze_graph(graph_data, analysis_type, parameters).await {
            Ok(result) => Ok(result),
            Err(e) => {
                error!("Provider {} failed: {}", provider_id, e);
                
                // If not using mock, fall back to mock provider
                if provider_id != "mock" {
                    warn!("Falling back to mock provider");
                    let providers = self.providers.read().unwrap();
                    if let Some(mock_provider) = providers.get("mock") {
                        return mock_provider.analyze_graph(graph_data_clone, analysis_type_clone, parameters_clone).await;
                    }
                }
                
                Err(e)
            }
        }
    }
    
    /// Get available providers
    pub fn get_available_providers(&self) -> Vec<(String, ProviderMetadata)> {
        self.providers.read().unwrap()
            .iter()
            .map(|(id, provider)| (id.clone(), provider.get_metadata()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_provider_manager_initialization() {
        let mut manager = AIProviderManager::new(SelectionStrategy::Default);
        
        // Should always have mock provider
        manager.initialize_from_env().await.unwrap();
        
        let providers = manager.get_available_providers();
        assert!(!providers.is_empty());
        assert!(providers.iter().any(|(id, _)| id == "mock"));
    }
    
    #[tokio::test]
    async fn test_provider_selection() {
        let mut manager = AIProviderManager::new(SelectionStrategy::CapabilityBased);
        
        // Register mock provider
        manager.register_provider(
            "mock".to_string(),
            Box::new(mock::MockAIProvider::new()),
            true,
        ).unwrap();
        
        // Should select mock provider for any capability
        let (_id, provider) = manager.get_provider_for_analysis(&AnalysisCapability::GraphAnalysis).await.unwrap();
        assert!(provider.supports_capability(&AnalysisCapability::GraphAnalysis));
    }
} 