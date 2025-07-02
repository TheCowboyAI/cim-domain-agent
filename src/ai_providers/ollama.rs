//! Ollama local AI provider for AI capabilities

use super::*;
use crate::value_objects::{
    AnalysisResult, Recommendation, RecommendedAction,
    Insight, Impact, Priority, EffortLevel
};
use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};
use uuid;

/// Ollama local AI provider
pub struct OllamaProvider {
    client: Client,
    model: String,
    base_url: String,
}

impl OllamaProvider {
    /// Create a new Ollama provider
    pub fn new(model: String, base_url: Option<String>) -> AIProviderResult<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        
        let client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(300)) // 5 minutes for local models
            .build()
            .map_err(|e| AIProviderError::ConfigurationError(e.to_string()))?;
        
        Ok(Self {
            client,
            model,
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".to_string()),
        })
    }
    
    /// Check if Ollama is running and model is available
    pub async fn check_health(&self) -> AIProviderResult<()> {
        // Check if Ollama is running
        let response = self.client
            .get(&format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map_err(|e| AIProviderError::ConnectionError(format!("Cannot connect to Ollama: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(AIProviderError::ConnectionError("Ollama is not running".to_string()));
        }
        
        // Check if model exists
        let models: ModelList = response.json().await
            .map_err(|e| AIProviderError::InvalidResponse(e.to_string()))?;
        
        if !models.models.iter().any(|m| m.name == self.model || m.name.starts_with(&format!("{}:", self.model))) {
            return Err(AIProviderError::ConfigurationError(
                format!("Model '{}' not found in Ollama. Available models: {}", 
                    self.model,
                    models.models.iter().map(|m| &m.name).cloned().collect::<Vec<_>>().join(", ")
                )
            ));
        }
        
        Ok(())
    }
    
    /// Create a prompt for graph analysis
    fn create_analysis_prompt(&self, graph_data: &GraphData, analysis_type: &AnalysisCapability) -> String {
        let analysis_instruction = match analysis_type {
            AnalysisCapability::GraphAnalysis => {
                "Analyze this graph structure and identify patterns, issues, and optimization opportunities."
            }
            AnalysisCapability::WorkflowOptimization => {
                "Analyze this workflow graph and suggest improvements for efficiency and parallelization."
            }
            AnalysisCapability::PatternDetection => {
                "Identify recurring patterns and anti-patterns in this graph structure."
            }
            AnalysisCapability::SemanticAnalysis => {
                "Analyze the semantic relationships and meaning in this graph."
            }
            AnalysisCapability::TransformationSuggestion => {
                "Suggest structural transformations to improve this graph."
            }
            AnalysisCapability::Custom(prompt) => prompt.as_str(),
        };
        
        format!(
            "{}\n\nGraph Structure:\n{}\n\nProvide your analysis in JSON format with 'findings' and 'recommendations' arrays. Each finding should have: id, type, description, severity (0-1), related_elements, and evidence. Each recommendation should have: id, type, description, expected_impact, effort (low/medium/high), and actions.",
            analysis_instruction,
            graph_to_prompt(graph_data)
        )
    }
    
    /// Extract JSON from Ollama response
    fn extract_json_from_response(&self, response: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        // Try to parse the entire response first
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(response) {
            return Ok(json);
        }
        
        // Look for JSON between braces
        if let Some(start) = response.find('{') {
            if let Some(end) = response.rfind('}') {
                let json_str = &response[start..=end];
                return Ok(serde_json::from_str(json_str)?);
            } else {
                return Err("No closing brace found".into());
            }
        }
        Err("No JSON found in response".into())
    }
    
    /// Parse Ollama response into analysis result
    fn parse_analysis_response(&self, response: &str, analysis_type: AnalysisCapability) -> AIProviderResult<AnalysisResult> {
        // Try to extract JSON, but fall back to plain text analysis if needed
        match self.extract_json_from_response(response) {
            Ok(json) => {
                let insights = self.extract_insights(&json);
                let recommendations = self.extract_recommendations(&json);
                
                Ok(AnalysisResult {
                    id: uuid::Uuid::new_v4(),
                    confidence_score: 0.8, // Default confidence for Ollama
                    summary: json.get("summary")
                        .and_then(|s| s.as_str())
                        .unwrap_or("Analysis completed")
                        .to_string(),
                    recommendations,
                    insights,
                    metadata: HashMap::from([
                        ("analysis_type".to_string(), json!(format!("{:?}", analysis_type))),
                        ("model".to_string(), json!(self.model.clone())),
                    ]),
                    timestamp: std::time::SystemTime::now(),
                })
            }
            Err(_) => {
                // Fallback: create a simple analysis from plain text
                let insight = Insight {
                    id: uuid::Uuid::new_v4(),
                    category: "general".to_string(),
                    description: response.lines()
                        .take(5)
                        .collect::<Vec<_>>()
                        .join(" ")
                        .chars()
                        .take(200)
                        .collect(),
                    evidence: vec![],
                    confidence: 0.6,
                    impact: Impact::Medium,
                };
                
                Ok(AnalysisResult {
                    id: uuid::Uuid::new_v4(),
                    confidence_score: 0.6,
                    summary: "Analysis completed (plain text response)".to_string(),
                    recommendations: vec![],
                    insights: vec![insight],
                    metadata: HashMap::from([
                        ("analysis_type".to_string(), json!(format!("{:?}", analysis_type))),
                        ("model".to_string(), json!(self.model.clone())),
                        ("response_type".to_string(), json!("plain_text")),
                        ("raw_response".to_string(), json!(response.chars().take(500).collect::<String>())),
                    ]),
                    timestamp: std::time::SystemTime::now(),
                })
            }
        }
    }
    
    /// Extract insights from JSON response
    fn extract_insights(&self, json: &serde_json::Value) -> Vec<Insight> {
        json.get("insights")
            .or_else(|| json.get("findings"))
            .and_then(|f| f.as_array())
            .map(|insights| {
                insights.iter()
                    .filter_map(|i| {
                        Some(Insight {
                            id: uuid::Uuid::new_v4(),
                            category: i.get("category")
                                .or_else(|| i.get("type"))
                                .and_then(|t| t.as_str())
                                .unwrap_or("general")
                                .to_string(),
                            description: i.get("description")?.as_str()?.to_string(),
                            evidence: i.get("evidence")
                                .and_then(|e| e.as_array())
                                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                                .unwrap_or_default(),
                            confidence: i.get("confidence")
                                .or_else(|| i.get("severity"))
                                .and_then(|s| s.as_f64())
                                .unwrap_or(0.5) as f32,
                            impact: match i.get("impact").and_then(|imp| imp.as_str()).unwrap_or("medium") {
                                "high" => Impact::High,
                                "low" => Impact::Low,
                                _ => Impact::Medium,
                            },
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Extract recommendations from JSON response
    fn extract_recommendations(&self, json: &serde_json::Value) -> Vec<Recommendation> {
        json.get("recommendations")
            .and_then(|r| r.as_array())
            .map(|recommendations| {
                recommendations.iter()
                    .enumerate()
                    .filter_map(|(_i, r)| {
                        Some(Recommendation {
                            id: uuid::Uuid::new_v4(),
                            title: r.get("title")
                                .or_else(|| r.get("description"))
                                .and_then(|d| d.as_str())
                                .unwrap_or("Recommendation")
                                .to_string(),
                            description: r.get("description")
                                .or_else(|| r.get("details"))
                                .and_then(|d| d.as_str())
                                .unwrap_or("")
                                .to_string(),
                            priority: match r.get("priority").and_then(|p| p.as_str()).unwrap_or("medium") {
                                "critical" => Priority::Critical,
                                "high" => Priority::High,
                                "low" => Priority::Low,
                                _ => Priority::Medium,
                            },
                            expected_impact: r.get("expected_impact")
                                .and_then(|i| i.as_str())
                                .unwrap_or("Unknown")
                                .to_string(),
                            effort_level: match r.get("effort").and_then(|e| e.as_str()).unwrap_or("medium") {
                                "high" => EffortLevel::High,
                                "low" => EffortLevel::Low,
                                _ => EffortLevel::Medium,
                            },
                            actions: self.extract_actions(r),
                            metadata: HashMap::new(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    

    
    fn extract_actions(&self, recommendation: &serde_json::Value) -> Vec<RecommendedAction> {
        recommendation.get("actions")
            .or_else(|| recommendation.get("steps"))
            .and_then(|a| a.as_array())
            .map(|actions| {
                actions.iter()
                    .enumerate()
                    .map(|(i, action)| {
                        RecommendedAction {
                            id: uuid::Uuid::new_v4(),
                            action_type: action.get("type")
                                .and_then(|t| t.as_str())
                                .unwrap_or("step")
                                .to_string(),
                            target: action.get("target")
                                .and_then(|t| t.as_str())
                                .unwrap_or("")
                                .to_string(),
                            description: action.get("description")
                                .and_then(|d| d.as_str())
                                .or_else(|| action.as_str())
                                .map(|d| d.to_string())
                                .unwrap_or_else(|| format!("Step {}", i + 1)),
                            estimated_duration: std::time::Duration::from_secs(
                                action.get("duration_seconds")
                                    .and_then(|d| d.as_u64())
                                    .unwrap_or(300)
                            ),
                            parameters: action.get("parameters")
                                .and_then(|p| p.as_object())
                                .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                                .unwrap_or_default(),
                            dependencies: Vec::new(),
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Parse transformation suggestions from Ollama response
    fn parse_transformation_response(&self, response: &str) -> AIProviderResult<Vec<TransformationSuggestion>> {
        let json = self.extract_json_from_response(response)
            .map_err(|e| AIProviderError::InvalidResponse(e.to_string()))?;
        
        let suggestions = if let Some(suggestions_array) = json.get("suggestions").and_then(|s| s.as_array()) {
            suggestions_array.iter()
                .filter_map(|suggestion| {
                    Some(TransformationSuggestion {
                        id: uuid::Uuid::new_v4().to_string(),
                        suggestion_type: suggestion.get("type")
                            .and_then(|t| t.as_str())
                            .unwrap_or("optimization")
                            .to_string(),
                        description: suggestion.get("description")
                            .and_then(|d| d.as_str())
                            .unwrap_or("")
                            .to_string(),
                        rationale: suggestion.get("rationale")
                            .and_then(|r| r.as_str())
                            .unwrap_or("")
                            .to_string(),
                        expected_benefit: suggestion.get("expected_benefit")
                            .and_then(|b| b.as_str())
                            .unwrap_or("")
                            .to_string(),
                        transformation_steps: suggestion.get("steps")
                            .and_then(|s| s.as_array())
                            .cloned()
                            .unwrap_or_default(),
                        risk_assessment: suggestion.get("risk_assessment").cloned(),
                    })
                })
                .collect()
        } else {
            // Fallback: create a single suggestion from the response
            vec![TransformationSuggestion {
                id: uuid::Uuid::new_v4().to_string(),
                suggestion_type: "general".to_string(),
                description: "Potential improvement identified".to_string(),
                rationale: "Analysis suggests optimization opportunities".to_string(),
                expected_benefit: "Improved efficiency".to_string(),
                transformation_steps: vec![json!({
                    "action": "review",
                    "target": "graph",
                })],
                risk_assessment: Some(json!({
                    "risk_level": "low",
                    "mitigation": "Manual review recommended",
                })),
            }]
        };
        
        Ok(suggestions)
    }
}

#[async_trait]
impl GraphAnalysisProvider for OllamaProvider {
    async fn analyze_graph(
        &self,
        graph_data: GraphData,
        analysis_type: AnalysisCapability,
        parameters: HashMap<String, Value>,
    ) -> AIProviderResult<AnalysisResult> {
        // Check health first
        self.check_health().await?;
        
        let prompt = self.create_analysis_prompt(&graph_data, &analysis_type);
        
        let request = GenerateRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
            options: Some(GenerateOptions {
                temperature: parameters.get("temperature")
                    .and_then(|t| t.as_f64())
                    .unwrap_or(0.7) as f32,
                num_predict: parameters.get("max_tokens")
                    .and_then(|t| t.as_u64())
                    .unwrap_or(2000) as i32,
                ..Default::default()
            }),
        };
        
        let response = self.client
            .post(&format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AIProviderError::ApiError(e.to_string()))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AIProviderError::ApiError(format!("Ollama API error: {}", error_text)));
        }
        
        let generate_response: GenerateResponse = response.json().await
            .map_err(|e| AIProviderError::InvalidResponse(e.to_string()))?;
        
        self.parse_analysis_response(&generate_response.response, analysis_type)
    }
    
    async fn suggest_transformations(
        &self,
        graph_data: GraphData,
        optimization_goals: Vec<String>,
        constraints: HashMap<String, Value>,
    ) -> AIProviderResult<Vec<TransformationSuggestion>> {
        // Check health first
        self.check_health().await?;
        
        let prompt = format!(
            "Suggest specific transformations to optimize this graph:\n\n{}\n\nOptimization Goals:\n- {}\n\nConstraints:\n{:?}\n\nProvide transformation suggestions in JSON format with a 'transformations' array. Each transformation should have: id, type, description, rationale, expected_benefit, steps (array), and risk_assessment.",
            graph_to_prompt(&graph_data),
            optimization_goals.join("\n- "),
            constraints
        );
        
        let request = GenerateRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
            options: Some(GenerateOptions {
                temperature: 0.7,
                num_predict: 2000,
                ..Default::default()
            }),
        };
        
        let response = self.client
            .post(&format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AIProviderError::ApiError(e.to_string()))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AIProviderError::ApiError(format!("Ollama API error: {}", error_text)));
        }
        
        let generate_response: GenerateResponse = response.json().await
            .map_err(|e| AIProviderError::InvalidResponse(e.to_string()))?;
        
        self.parse_transformation_response(&generate_response.response)
    }
    
    fn supports_capability(&self, capability: &AnalysisCapability) -> bool {
        // Ollama supports all capabilities through prompting
        match capability {
            AnalysisCapability::GraphAnalysis => true,
            AnalysisCapability::WorkflowOptimization => true,
            AnalysisCapability::SemanticAnalysis => true,
            AnalysisCapability::PatternDetection => true,
            AnalysisCapability::TransformationSuggestion => true,
            AnalysisCapability::Custom(_) => true, // Support custom prompts
        }
    }
    
    fn get_metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            name: "Ollama Local".to_string(),
            version: "0.1.0".to_string(),
            model: self.model.clone(),
            capabilities: vec![
                AnalysisCapability::GraphAnalysis,
                AnalysisCapability::WorkflowOptimization,
                AnalysisCapability::PatternDetection,
                AnalysisCapability::SemanticAnalysis,
                AnalysisCapability::TransformationSuggestion,
            ],
            rate_limits: None, // No rate limits for local models
        }
    }
}

/// Ollama generate request
#[derive(Debug, Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GenerateOptions>,
}

#[derive(Debug, Serialize, Default)]
struct GenerateOptions {
    temperature: f32,
    num_predict: i32,
}

/// Ollama generate response
#[derive(Debug, Deserialize)]
struct GenerateResponse {
    #[allow(dead_code)]
    model: String,
    response: String,
    #[allow(dead_code)]
    done: bool,
    #[allow(dead_code)]
    #[serde(default)]
    context: Vec<i32>,
    #[allow(dead_code)]
    total_duration: Option<u64>,
    #[allow(dead_code)]
    load_duration: Option<u64>,
    #[allow(dead_code)]
    prompt_eval_duration: Option<u64>,
    #[allow(dead_code)]
    eval_duration: Option<u64>,
}

/// Ollama model list response
#[derive(Debug, Deserialize)]
struct ModelList {
    models: Vec<Model>,
}

#[derive(Debug, Deserialize)]
struct Model {
    name: String,
    #[allow(dead_code)]
    modified_at: String,
    #[allow(dead_code)]
    size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_ollama_provider_creation() {
        let provider = OllamaProvider::new(
            "llama2".to_string(),
            None,
        ).unwrap();
        
        assert_eq!(provider.model, "llama2");
    }
    
    #[test]
    fn test_capability_support() {
        let provider = OllamaProvider::new(
            "llama2".to_string(),
            None,
        ).unwrap();
        
        assert!(provider.supports_capability(&AnalysisCapability::GraphAnalysis));
        assert!(provider.supports_capability(&AnalysisCapability::WorkflowOptimization));
        assert!(provider.supports_capability(&AnalysisCapability::Custom("test".to_string())));
    }
} 