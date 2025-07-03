//! OpenAI API provider for AI capabilities

use super::*;
use crate::value_objects::{
    AnalysisResult, Recommendation, RecommendedAction, 
    Insight, Impact, Priority, EffortLevel
};
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};

/// OpenAI API provider
pub struct OpenAIProvider {
    client: Client,
    #[allow(dead_code)]
    api_key: String,
    model: String,
    base_url: String,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider
    pub fn new(api_key: String, model: String) -> AIProviderResult<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {api_key}"))
                .map_err(|e| AIProviderError::ConfigurationError(e.to_string()))?,
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        
        let client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| AIProviderError::ConfigurationError(e.to_string()))?;
        
        Ok(Self {
            client,
            api_key,
            model,
            base_url: "https://api.openai.com/v1".to_string(),
        })
    }
    
    /// Create a system prompt for graph analysis
    fn create_analysis_prompt(&self, analysis_type: &AnalysisCapability) -> String {
        match analysis_type {
            AnalysisCapability::GraphAnalysis => {
                "You are a graph analysis expert. Analyze the provided graph structure and identify patterns, issues, and optimization opportunities. Provide your response in JSON format with 'findings' and 'recommendations' arrays.".to_string()
            }
            AnalysisCapability::WorkflowOptimization => {
                "You are a workflow optimization expert. Analyze the workflow graph and suggest improvements for efficiency, parallelization, and bottleneck removal. Provide your response in JSON format.".to_string()
            }
            AnalysisCapability::PatternDetection => {
                "You are a pattern detection expert. Identify recurring patterns, anti-patterns, and structural similarities in the graph. Provide your response in JSON format.".to_string()
            }
            AnalysisCapability::SemanticAnalysis => {
                "You are a semantic analysis expert. Analyze the meaning and relationships in the graph, identifying conceptual connections and semantic inconsistencies. Provide your response in JSON format.".to_string()
            }
            AnalysisCapability::TransformationSuggestion => {
                "You are a graph transformation expert. Suggest structural transformations that would improve the graph's effectiveness. Provide your response in JSON format.".to_string()
            }
            AnalysisCapability::Custom(prompt) => prompt.clone(),
        }
    }
    
    /// Parse OpenAI response into analysis result
    fn parse_analysis_response(&self, response: &ChatResponse, analysis_type: AnalysisCapability) -> AIProviderResult<AnalysisResult> {
        let content = response.choices.first()
            .and_then(|c| c.message.content.as_ref())
            .ok_or_else(|| AIProviderError::InvalidResponse("No content in response".to_string()))?;
        
        // Try to parse as JSON first
        let json_result: Result<serde_json::Value, _> = serde_json::from_str(content);
        
        let (recommendations, insights) = if let Ok(json) = json_result {
            // Extract recommendations and insights from JSON
            let recommendations = self.extract_recommendations(&json);
            let insights = self.extract_insights(&json);
            (recommendations, insights)
        } else {
            // Fallback: create basic insight from text response
            let insights = vec![Insight {
                id: uuid::Uuid::new_v4(),
                category: "analysis".to_string(),
                description: content.clone(),
                evidence: vec![],
                confidence: 0.5,
                impact: Impact::Medium,
            }];
            (vec![], insights)
        };
        
        Ok(AnalysisResult {
            id: uuid::Uuid::new_v4(),
            confidence_score: 0.8, // OpenAI confidence estimate
            summary: format!("{:?} analysis completed", analysis_type),
            recommendations,
            insights,
            metadata: HashMap::from([
                ("provider".to_string(), json!("openai")),
                ("model".to_string(), json!(self.model.clone())),
                ("analysis_type".to_string(), json!(format!("{:?}", analysis_type))),
            ]),
            timestamp: std::time::SystemTime::now(),
        })
    }
    
    /// Extract insights from JSON response
    fn extract_insights(&self, json: &serde_json::Value) -> Vec<Insight> {
        json.get("insights")
            .or_else(|| json.get("findings")) // Support both "insights" and "findings"
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
                    .filter_map(|r| {
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
    
    /// Extract actions from a recommendation
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
}

#[async_trait]
impl GraphAnalysisProvider for OpenAIProvider {
    async fn analyze_graph(
        &self,
        graph_data: GraphData,
        analysis_type: AnalysisCapability,
        parameters: HashMap<String, Value>,
    ) -> AIProviderResult<AnalysisResult> {
        let system_prompt = self.create_analysis_prompt(&analysis_type);
        let user_prompt = format!("Analyze this graph:\n\n{}\n\nParameters: {:?}",
            graph_to_prompt(&graph_data),
            parameters
        );
        
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: Some(system_prompt),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: Some(user_prompt),
                },
            ],
            temperature: Some(0.7),
            max_tokens: Some(2000),
            response_format: None,
        };
        
        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AIProviderError::ApiError(e.to_string()))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AIProviderError::ApiError(format!("OpenAI API error: {}", error_text)));
        }
        
        let chat_response: ChatResponse = response.json().await
            .map_err(|e| AIProviderError::InvalidResponse(e.to_string()))?;
        
        self.parse_analysis_response(&chat_response, analysis_type)
    }
    
    async fn suggest_transformations(
        &self,
        graph_data: GraphData,
        optimization_goals: Vec<String>,
        constraints: HashMap<String, Value>,
    ) -> AIProviderResult<Vec<TransformationSuggestion>> {
        let system_prompt = "You are a graph transformation expert. Suggest specific transformations to optimize the graph for the given goals while respecting constraints. Provide your response as a JSON array of transformation suggestions.";
        
        let user_prompt = format!("Graph:\n{}\n\nOptimization Goals:\n{}\n\nConstraints:\n{:?}",
            graph_to_prompt(&graph_data),
            optimization_goals.join("\n- "),
            constraints
        );
        
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: Some(system_prompt.to_string()),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: Some(user_prompt),
                },
            ],
            temperature: Some(0.7),
            max_tokens: Some(2000),
            response_format: None,
        };
        
        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AIProviderError::ApiError(e.to_string()))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AIProviderError::ApiError(format!("OpenAI API error: {}", error_text)));
        }
        
        let chat_response: ChatResponse = response.json().await
            .map_err(|e| AIProviderError::InvalidResponse(e.to_string()))?;
        
        let content = chat_response.choices.first()
            .and_then(|c| c.message.content.as_ref())
            .ok_or_else(|| AIProviderError::InvalidResponse("No content in response".to_string()))?;
        
        // Parse transformations from response
        let json: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| AIProviderError::InvalidResponse(e.to_string()))?;
        
        let suggestions = json.get("transformations")
            .and_then(|t| t.as_array())
            .or(json.as_array())
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
                        risk_assessment: t.get("risk_assessment").cloned(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(suggestions)
    }
    
    fn supports_capability(&self, capability: &AnalysisCapability) -> bool {
        // OpenAI supports all capabilities through prompting
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
            name: "OpenAI".to_string(),
            version: "v1".to_string(),
            model: self.model.clone(),
            capabilities: vec![
                AnalysisCapability::GraphAnalysis,
                AnalysisCapability::WorkflowOptimization,
                AnalysisCapability::PatternDetection,
                AnalysisCapability::SemanticAnalysis,
                AnalysisCapability::TransformationSuggestion,
            ],
            rate_limits: Some(RateLimits {
                requests_per_minute: 60,
                tokens_per_minute: 90000,
                concurrent_requests: 5,
            }),
        }
    }
}

/// OpenAI chat request structure
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    #[allow(dead_code)]
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
} 