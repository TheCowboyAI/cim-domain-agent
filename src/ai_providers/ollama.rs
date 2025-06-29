//! Ollama local AI provider for AI capabilities

use super::*;
use crate::value_objects::analysis_result::{
    Finding, Recommendation, RecommendationType, EffortLevel, 
    RecommendedAction, AnalysisResult
};
use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};

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
            .timeout(std::time::Duration::from_secs(120)) // Longer timeout for local models
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
    
    /// Parse Ollama response into analysis result
    fn parse_analysis_response(&self, response: &str, analysis_type: AnalysisCapability) -> AIProviderResult<AnalysisResult> {
        // Try to extract JSON from the response
        let json_result = if let Some(json_start) = response.find('{') {
            if let Some(json_end) = response.rfind('}') {
                serde_json::from_str::<serde_json::Value>(&response[json_start..=json_end])
            } else {
                Err(serde_json::Error::custom("No closing brace found"))
            }
        } else {
            Err(serde_json::Error::custom("No JSON found in response"))
        };
        
        let (findings, recommendations) = if let Ok(json) = json_result {
            let findings = self.extract_findings(&json);
            let recommendations = self.extract_recommendations(&json);
            (findings, recommendations)
        } else {
            // Fallback: create basic finding from text response
            let findings = vec![Finding {
                id: uuid::Uuid::new_v4().to_string(),
                finding_type: "analysis".to_string(),
                description: response.to_string(),
                severity: 0.5,
                related_elements: vec![],
                evidence: HashMap::new(),
            }];
            (findings, vec![])
        };
        
        Ok(AnalysisResult {
            analysis_type,
            confidence: 0.75, // Local model confidence estimate
            findings,
            recommendations,
            raw_response: Some(json!(response)),
        })
    }
    
    /// Extract findings from JSON response
    fn extract_findings(&self, json: &serde_json::Value) -> Vec<Finding> {
        json.get("findings")
            .and_then(|f| f.as_array())
            .map(|findings| {
                findings.iter()
                    .enumerate()
                    .filter_map(|(i, f)| {
                        Some(Finding {
                            id: f.get("id").and_then(|id| id.as_str()).unwrap_or(&format!("F{:03}", i + 1)).to_string(),
                            finding_type: f.get("type").and_then(|t| t.as_str()).unwrap_or("general").to_string(),
                            description: f.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string(),
                            severity: f.get("severity").and_then(|s| s.as_f64()).unwrap_or(0.5) as f32,
                            related_elements: f.get("related_elements")
                                .and_then(|e| e.as_array())
                                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                                .unwrap_or_default(),
                            evidence: f.get("evidence")
                                .and_then(|e| e.as_object())
                                .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                                .unwrap_or_default(),
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
                    .filter_map(|(i, r)| {
                        Some(Recommendation {
                            id: r.get("id").and_then(|id| id.as_str()).unwrap_or(&format!("R{:03}", i + 1)).to_string(),
                            recommendation_type: self.parse_recommendation_type(
                                r.get("type").and_then(|t| t.as_str()).unwrap_or("workflow")
                            ),
                            description: r.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string(),
                            expected_impact: r.get("expected_impact").and_then(|i| i.as_str()).unwrap_or("Unknown").to_string(),
                            effort_level: self.parse_effort_level(
                                r.get("effort").and_then(|e| e.as_str()).unwrap_or("medium")
                            ),
                            actions: self.extract_actions(r.get("actions")),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    
    fn parse_recommendation_type(&self, type_str: &str) -> RecommendationType {
        match type_str.to_lowercase().as_str() {
            "workflow" | "workflow_optimization" => RecommendationType::WorkflowOptimization,
            "structure" | "structural_improvement" => RecommendationType::StructuralImprovement,
            "performance" | "performance_enhancement" => RecommendationType::PerformanceEnhancement,
            "semantic" | "semantic_enrichment" => RecommendationType::SemanticEnrichment,
            _ => RecommendationType::Custom(type_str.to_string()),
        }
    }
    
    fn parse_effort_level(&self, effort_str: &str) -> EffortLevel {
        match effort_str.to_lowercase().as_str() {
            "low" => EffortLevel::Low,
            "medium" => EffortLevel::Medium,
            "high" => EffortLevel::High,
            _ => EffortLevel::Medium,
        }
    }
    
    fn extract_actions(&self, actions_value: Option<&serde_json::Value>) -> Vec<RecommendedAction> {
        actions_value
            .and_then(|a| a.as_array())
            .map(|actions| {
                actions.iter()
                    .enumerate()
                    .filter_map(|(i, a)| {
                        Some(RecommendedAction {
                            id: a.get("id").and_then(|id| id.as_str()).unwrap_or(&format!("A{:03}", i + 1)).to_string(),
                            action_type: a.get("type").and_then(|t| t.as_str()).unwrap_or("transform").to_string(),
                            target_elements: a.get("targets")
                                .and_then(|t| t.as_array())
                                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                                .unwrap_or_default(),
                            parameters: a.get("parameters")
                                .and_then(|p| p.as_object())
                                .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                                .unwrap_or_default(),
                            execution_order: i as u32 + 1,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
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
        
        // Try to parse JSON from response
        let json_result = if let Some(json_start) = generate_response.response.find('{') {
            if let Some(json_end) = generate_response.response.rfind('}') {
                serde_json::from_str::<serde_json::Value>(&generate_response.response[json_start..=json_end])
            } else {
                Err(serde_json::Error::custom("No closing brace found"))
            }
        } else {
            Err(serde_json::Error::custom("No JSON found in response"))
        };
        
        let suggestions = if let Ok(json) = json_result {
            json.get("transformations")
                .and_then(|t| t.as_array())
                .map(|transformations| {
                    transformations.iter()
                        .enumerate()
                        .map(|(i, t)| TransformationSuggestion {
                            id: t.get("id").and_then(|id| id.as_str()).unwrap_or(&format!("T{:03}", i + 1)).to_string(),
                            suggestion_type: t.get("type").and_then(|t| t.as_str()).unwrap_or("optimization").to_string(),
                            description: t.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string(),
                            rationale: t.get("rationale").and_then(|r| r.as_str()).unwrap_or("").to_string(),
                            expected_benefit: t.get("expected_benefit").and_then(|b| b.as_str()).unwrap_or("").to_string(),
                            transformation_steps: t.get("steps").and_then(|s| s.as_array()).cloned().unwrap_or_default(),
                            risk_assessment: t.get("risk_assessment").cloned().unwrap_or(json!({})),
                        })
                        .collect()
                })
                .unwrap_or_default()
        } else {
            // Fallback: create a basic suggestion
            vec![TransformationSuggestion {
                id: "T001".to_string(),
                suggestion_type: "analysis".to_string(),
                description: "Analysis completed but structured suggestions could not be parsed".to_string(),
                rationale: generate_response.response,
                expected_benefit: "Unknown".to_string(),
                transformation_steps: vec![],
                risk_assessment: json!({}),
            }]
        };
        
        Ok(suggestions)
    }
    
    fn supports_capability(&self, capability: &AnalysisCapability) -> bool {
        // Ollama supports all capabilities through prompting
        true
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
    model: String,
    response: String,
    done: bool,
    #[serde(default)]
    context: Vec<i32>,
    total_duration: Option<u64>,
    load_duration: Option<u64>,
    prompt_eval_duration: Option<u64>,
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
    modified_at: String,
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