//! Configuration for AI providers

use super::{ProviderConfig, AIProviderError, AIProviderResult};
use std::env;

/// Load provider configuration from environment variables
pub fn load_provider_config() -> AIProviderResult<ProviderConfig> {
    // Try to load .env file if it exists
    let _ = dotenvy::dotenv();
    
    // Check for default provider
    let provider_type = env::var("DEFAULT_AI_PROVIDER")
        .unwrap_or_else(|_| "mock".to_string());
    
    match provider_type.as_str() {
        "mock" => Ok(ProviderConfig::Mock),
        
        "openai" => {
            let api_key = env::var("OPENAI_API_KEY")
                .map_err(|_| AIProviderError::ConfigurationError(
                    "OPENAI_API_KEY environment variable not set".to_string()
                ))?;
            
            let model = env::var("OPENAI_MODEL")
                .unwrap_or_else(|_| "gpt-4-turbo".to_string());
            
            Ok(ProviderConfig::OpenAI { api_key, model })
        }
        
        "anthropic" => {
            let api_key = env::var("ANTHROPIC_API_KEY")
                .map_err(|_| AIProviderError::ConfigurationError(
                    "ANTHROPIC_API_KEY environment variable not set".to_string()
                ))?;
            
            let model = env::var("ANTHROPIC_MODEL")
                .unwrap_or_else(|_| "claude-3-5-sonnet-20241022".to_string());
            
            Ok(ProviderConfig::Anthropic { api_key, model })
        }
        
        "ollama" => {
            let host = env::var("OLLAMA_HOST")
                .unwrap_or_else(|_| "http://localhost:11434".to_string());
            
            let model = env::var("OLLAMA_MODEL")
                .unwrap_or_else(|_| "llama2".to_string());
            
            Ok(ProviderConfig::Ollama { host, model })
        }
        
        _ => Err(AIProviderError::ConfigurationError(
            format!("Unknown provider type: {}", provider_type)
        )),
    }
}

/// Provider type enum for simplified configuration
#[derive(Debug, Clone)]
pub enum ProviderType {
    Mock,
    OpenAI,
    Anthropic,
    Ollama,
}

/// Create a provider configuration from a provider type using environment variables
pub fn create_provider_config_from_type(provider_type: &ProviderType) -> ProviderConfig {
    match provider_type {
        ProviderType::Mock => ProviderConfig::Mock,
        ProviderType::OpenAI => {
            let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
            let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4-turbo".to_string());
            ProviderConfig::OpenAI { api_key, model }
        }
        ProviderType::Anthropic => {
            let api_key = env::var("ANTHROPIC_API_KEY").unwrap_or_default();
            let model = env::var("ANTHROPIC_MODEL")
                .unwrap_or_else(|_| "claude-3-5-sonnet-20241022".to_string());
            ProviderConfig::Anthropic { api_key, model }
        }
        ProviderType::Ollama => {
            let host = env::var("OLLAMA_HOST")
                .unwrap_or_else(|_| "http://localhost:11434".to_string());
            let model = env::var("OLLAMA_MODEL")
                .unwrap_or_else(|_| "llama2".to_string());
            ProviderConfig::Ollama { host, model }
        }
    }
}

/// Create a provider configuration from parameters
pub fn create_provider_config(
    provider_name: &str,
    api_key: Option<String>,
    model: Option<String>,
    host: Option<String>,
) -> AIProviderResult<ProviderConfig> {
    match provider_name.to_lowercase().as_str() {
        "mock" => Ok(ProviderConfig::Mock),
        
        "openai" => {
            let api_key = api_key.ok_or_else(|| {
                AIProviderError::ConfigurationError("API key required for OpenAI".to_string())
            })?;
            let model = model.unwrap_or_else(|| "gpt-4".to_string());
            Ok(ProviderConfig::OpenAI { api_key, model })
        }
        
        "anthropic" => {
            let api_key = api_key.ok_or_else(|| {
                AIProviderError::ConfigurationError("API key required for Anthropic".to_string())
            })?;
            let model = model.unwrap_or_else(|| "claude-3-opus-20240229".to_string());
            Ok(ProviderConfig::Anthropic { api_key, model })
        }
        
        "ollama" => {
            let host = host.unwrap_or_else(|| "http://localhost:11434".to_string());
            let model = model.unwrap_or_else(|| "llama2".to_string());
            Ok(ProviderConfig::Ollama { host, model })
        }
        
        _ => Err(AIProviderError::ConfigurationError(
            format!("Unknown provider: {}", provider_name)
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_mock_config() {
        let config = create_provider_config_from_type(&ProviderType::Mock);
        match config {
            ProviderConfig::Mock => (),
            _ => panic!("Expected mock config"),
        }
    }
    
    #[test]
    fn test_create_openai_config() {
        // Set env vars for test
        env::set_var("OPENAI_API_KEY", "test-key");
        env::set_var("OPENAI_MODEL", "gpt-4");
        
        let config = create_provider_config_from_type(&ProviderType::OpenAI);
        
        match config {
            ProviderConfig::OpenAI { api_key, model } => {
                assert_eq!(api_key, "test-key");
                assert_eq!(model, "gpt-4");
            }
            _ => panic!("Expected OpenAI config"),
        }
        
        // Clean up
        env::remove_var("OPENAI_API_KEY");
        env::remove_var("OPENAI_MODEL");
    }
} 