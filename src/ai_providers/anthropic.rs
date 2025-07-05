//! Anthropic Claude API provider for AI capabilities

use super::*;
use crate::value_objects::{
    AnalysisResult, Recommendation, RecommendedAction,
    Insight, Impact, Priority, EffortLevel
};
use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};

/// Anthropic Claude API provider
pub struct AnthropicProvider {
    #[allow(dead_code)]
    api_key: String,
    model: String,
    client: Client,
    base_url: String,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider
    pub fn new(api_key: String, model: String) -> AIProviderResult<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&api_key)
                .map_err(|e| AIProviderError::ConfigurationError(e.to_string()))?,
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "anthropic-version",
            HeaderValue::from_static("2023-06-01"),
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
            base_url: "https://api.anthropic.com/v1".to_string(),
        })
    }
    
    /// Create a system prompt for graph analysis
    fn create_analysis_prompt(&self, analysis_type: &AnalysisCapability) -> String {
        match analysis_type {
            AnalysisCapability::GraphAnalysis => {
                "You are a graph analysis expert. Analyze the provided graph structure and identify patterns, issues, and optimization opportunities. Structure your response as JSON with 'findings' and 'recommendations' arrays."
            }
            AnalysisCapability::WorkflowOptimization => {
                "You are a workflow optimization expert. Analyze the workflow graph and suggest improvements for efficiency, parallelization, and bottleneck removal. Structure your response as JSON."
            }
            AnalysisCapability::PatternDetection => {
                "You are a pattern detection expert. Identify recurring patterns, anti-patterns, and structural similarities in the graph. Structure your response as JSON."
            }
            AnalysisCapability::SemanticAnalysis => {
                "You are a semantic analysis expert. Analyze the meaning and relationships in the graph, identifying conceptual connections and semantic inconsistencies. Structure your response as JSON."
            }
            AnalysisCapability::TransformationSuggestion => {
                "You are a graph transformation expert. Suggest structural transformations that would improve the graph's effectiveness. Structure your response as JSON."
            }
            AnalysisCapability::Custom(prompt) => prompt.as_str(),
        }.to_string()
    }
    
    /// Parse Anthropic response into analysis result
    fn parse_analysis_response(&self, response: &MessageResponse, analysis_type: AnalysisCapability) -> AIProviderResult<AnalysisResult> {
        let content = response.content.first()
            .map(|c| match c {
                ContentBlock::Text { text } => text,
            })
            .ok_or_else(|| AIProviderError::InvalidResponse("No text content in response".to_string()))?;
        
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
            confidence_score: 0.85, // Claude confidence estimate
            summary: format!("{analysis_type:?} analysis completed"),
            recommendations,
            insights,
            metadata: HashMap::from([
                ("provider".to_string(), json!("anthropic")),
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
impl GraphAnalysisProvider for AnthropicProvider {
    async fn analyze_graph(
        &self,
        graph_data: GraphData,
        analysis_type: AnalysisCapability,
        parameters: HashMap<String, Value>,
    ) -> AIProviderResult<AnalysisResult> {
        let system_prompt = self.create_analysis_prompt(&analysis_type);
        let user_prompt = format!("Please analyze this graph:\n\n{}\n\nParameters: {:?}\n\nProvide your analysis in JSON format.",
            graph_to_prompt(&graph_data),
            parameters
        );
        
        let request = MessageRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "user".to_string(),
                    content: vec![ContentBlock::Text { text: user_prompt }],
                },
            ],
            system: Some(system_prompt),
            max_tokens: 2000,
            temperature: Some(0.7),
        };
        
        let response = self.client
            .post(format!("{}/messages", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AIProviderError::ApiError(e.to_string()))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AIProviderError::ApiError(format!("Anthropic API error: {error_text}")));
        }
        
        let message_response: MessageResponse = response.json().await
            .map_err(|e| AIProviderError::InvalidResponse(e.to_string()))?;
        
        self.parse_analysis_response(&message_response, analysis_type)
    }
    
    async fn suggest_transformations(
        &self,
        graph_data: GraphData,
        optimization_goals: Vec<String>,
        constraints: HashMap<String, Value>,
    ) -> AIProviderResult<Vec<TransformationSuggestion>> {
        let system_prompt = "You are a graph transformation expert. Suggest specific transformations to optimize the graph for the given goals while respecting constraints. Structure your response as JSON with a 'transformations' array.";
        
        let user_prompt = format!("Graph:\n{}\n\nOptimization Goals:\n- {}\n\nConstraints:\n{:?}\n\nProvide transformation suggestions in JSON format.",
            graph_to_prompt(&graph_data),
            optimization_goals.join("\n- "),
            constraints
        );
        
        let request = MessageRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "user".to_string(),
                    content: vec![ContentBlock::Text { text: user_prompt }],
                },
            ],
            system: Some(system_prompt.to_string()),
            max_tokens: 2000,
            temperature: Some(0.7),
        };
        
        let response = self.client
            .post(format!("{}/messages", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AIProviderError::ApiError(e.to_string()))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AIProviderError::ApiError(format!("Anthropic API error: {error_text}")));
        }
        
        let message_response: MessageResponse = response.json().await
            .map_err(|e| AIProviderError::InvalidResponse(e.to_string()))?;
        
        let content = message_response.content.first()
            .map(|c| match c {
                ContentBlock::Text { text } => text,
            })
            .ok_or_else(|| AIProviderError::InvalidResponse("No text content in response".to_string()))?;
        
        // Parse transformations from response
        let json: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| AIProviderError::InvalidResponse(e.to_string()))?;
        
        let suggestions = json.get("transformations")
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
                        risk_assessment: t.get("risk_assessment").cloned(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(suggestions)
    }
    
    fn supports_capability(&self, capability: &AnalysisCapability) -> bool {
        // Claude supports all capabilities through prompting
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
            name: "Anthropic Claude".to_string(),
            version: "2023-06-01".to_string(),
            model: self.model.clone(),
            capabilities: vec![
                AnalysisCapability::GraphAnalysis,
                AnalysisCapability::WorkflowOptimization,
                AnalysisCapability::PatternDetection,
                AnalysisCapability::SemanticAnalysis,
                AnalysisCapability::TransformationSuggestion,
            ],
            rate_limits: Some(RateLimits {
                requests_per_minute: 50,
                tokens_per_minute: 100000,
                concurrent_requests: 5,
            }),
        }
    }
}

/// Anthropic message request structure
#[derive(Debug, Serialize)]
struct MessageRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: Vec<ContentBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    #[allow(dead_code)]
    id: String,
    content: Vec<ContentBlock>,
    #[allow(dead_code)]
    model: String,
    #[allow(dead_code)]
    usage: Usage,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Usage {
    input_tokens: u32,
    output_tokens: u32,
} 