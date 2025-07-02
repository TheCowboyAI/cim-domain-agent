//! Mapping between Graph domain and Conceptual Spaces
//!
//! This module provides functionality to map graph structures to conceptual space
//! representations, enabling semantic analysis of graphs.

use cim_domain_graph::GraphId;
use cim_domain_graph::components::NodeContent;
use cim_domain_conceptualspaces::{
    ConceptualPoint, QualityDimension, DimensionType,
    ConceptualSpaceId, DimensionId
};
use std::collections::HashMap;
use tracing::debug;

/// Mapper for converting graph structures to conceptual representations
pub struct GraphConceptualMapper {
    /// Dimension mappings for graph properties
    dimension_mappings: HashMap<String, DimensionId>,
    
    /// Weights for different graph features
    feature_weights: GraphFeatureWeights,
    
    /// Semantic analyzer for content
    semantic_analyzer: SemanticAnalyzer,
}

/// Weights for different graph features in conceptual mapping
#[derive(Debug, Clone)]
pub struct GraphFeatureWeights {
    pub node_count: f64,
    pub edge_count: f64,
    pub connectivity: f64,
    pub clustering: f64,
    pub centrality: f64,
    pub modularity: f64,
}

impl Default for GraphFeatureWeights {
    fn default() -> Self {
        Self {
            node_count: 1.0,
            edge_count: 1.0,
            connectivity: 2.0,
            clustering: 1.5,
            centrality: 1.5,
            modularity: 1.0,
        }
    }
}

/// Simple semantic analyzer for extracting meaning from content
#[derive(Debug, Clone)]
pub struct SemanticAnalyzer {
    /// Keywords for different semantic categories
    keyword_categories: HashMap<String, Vec<String>>,
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        let mut keyword_categories = HashMap::new();
        
        // Define semantic categories and their keywords
        keyword_categories.insert("workflow".to_string(), vec![
            "process".to_string(), "flow".to_string(), "step".to_string(),
            "task".to_string(), "action".to_string(), "sequence".to_string(),
        ]);
        
        keyword_categories.insert("data".to_string(), vec![
            "data".to_string(), "information".to_string(), "record".to_string(),
            "entity".to_string(), "attribute".to_string(), "value".to_string(),
        ]);
        
        keyword_categories.insert("decision".to_string(), vec![
            "if".to_string(), "then".to_string(), "else".to_string(),
            "condition".to_string(), "branch".to_string(), "choice".to_string(),
        ]);
        
        keyword_categories.insert("integration".to_string(), vec![
            "api".to_string(), "service".to_string(), "connect".to_string(),
            "interface".to_string(), "endpoint".to_string(), "integration".to_string(),
        ]);
        
        Self { keyword_categories }
    }
}

impl SemanticAnalyzer {
    /// Analyze text content and return semantic scores
    pub fn analyze_content(&self, content: &str) -> HashMap<String, f64> {
        let mut scores = HashMap::new();
        let content_lower = content.to_lowercase();
        let word_count = content_lower.split_whitespace().count() as f64;
        
        if word_count == 0.0 {
            return scores;
        }
        
        // Calculate scores for each category
        for (category, keywords) in &self.keyword_categories {
            let mut category_score = 0.0;
            
            for keyword in keywords {
                let occurrences = content_lower.matches(keyword).count() as f64;
                category_score += occurrences / word_count;
            }
            
            // Normalize to 0-1 range
            scores.insert(category.clone(), (category_score * 10.0).min(1.0));
        }
        
        scores
    }
    
    /// Calculate semantic coherence based on category distribution
    pub fn calculate_coherence(&self, scores: &HashMap<String, f64>) -> f64 {
        if scores.is_empty() {
            return 0.5; // Default middle value
        }
        
        // Calculate entropy of the distribution
        let total: f64 = scores.values().sum();
        if total == 0.0 {
            return 0.5;
        }
        
        let mut entropy = 0.0;
        for score in scores.values() {
            if *score > 0.0 {
                let p = score / total;
                entropy -= p * p.ln();
            }
        }
        
        // Normalize entropy to 0-1 (lower entropy = higher coherence)
        let max_entropy = (scores.len() as f64).ln();
        if max_entropy > 0.0 {
            1.0 - (entropy / max_entropy)
        } else {
            1.0
        }
    }
}

impl GraphConceptualMapper {
    /// Create a new mapper with default settings
    pub fn new() -> Self {
        let mut dimension_mappings = HashMap::new();
        
        // Map graph properties to conceptual dimensions
        dimension_mappings.insert("complexity".to_string(), DimensionId::new());
        dimension_mappings.insert("connectivity".to_string(), DimensionId::new());
        dimension_mappings.insert("hierarchy".to_string(), DimensionId::new());
        dimension_mappings.insert("modularity".to_string(), DimensionId::new());
        dimension_mappings.insert("semantic_coherence".to_string(), DimensionId::new());
        
        Self {
            dimension_mappings,
            feature_weights: GraphFeatureWeights::default(),
            semantic_analyzer: SemanticAnalyzer::default(),
        }
    }
    
    /// Map a graph to a conceptual point
    pub fn map_graph_to_point(
        &self,
        graph_metrics: &GraphMetrics,
        graph_content: Option<&GraphContentSummary>,
    ) -> ConceptualPoint {
        debug!("Mapping graph to conceptual point");
        
        // Create dimension map
        let mut dimension_map = HashMap::new();
        let mut coordinates = Vec::new();
        
        // Map complexity (based on node and edge count)
        let complexity = self.calculate_complexity(graph_metrics);
        if let Some(&dim_id) = self.dimension_mappings.get("complexity") {
            dimension_map.insert(dim_id, coordinates.len());
            coordinates.push(complexity);
        }
        
        // Map connectivity
        let connectivity = graph_metrics.average_degree / graph_metrics.max_possible_edges.max(1.0);
        if let Some(&dim_id) = self.dimension_mappings.get("connectivity") {
            dimension_map.insert(dim_id, coordinates.len());
            coordinates.push(connectivity);
        }
        
        // Map hierarchy (based on depth and branching)
        let hierarchy = self.calculate_hierarchy(graph_metrics);
        if let Some(&dim_id) = self.dimension_mappings.get("hierarchy") {
            dimension_map.insert(dim_id, coordinates.len());
            coordinates.push(hierarchy);
        }
        
        // Map modularity
        if let Some(&dim_id) = self.dimension_mappings.get("modularity") {
            dimension_map.insert(dim_id, coordinates.len());
            coordinates.push(graph_metrics.modularity);
        }
        
        // Map semantic coherence
        let semantic_coherence = if let Some(content) = graph_content {
            // Analyze all node content together
            let all_content = content.node_titles.join(" ") + " " + &content.node_descriptions.join(" ");
            let semantic_scores = self.semantic_analyzer.analyze_content(&all_content);
            self.semantic_analyzer.calculate_coherence(&semantic_scores)
        } else {
            0.5 // Default middle value
        };
        
        if let Some(&dim_id) = self.dimension_mappings.get("semantic_coherence") {
            dimension_map.insert(dim_id, coordinates.len());
            coordinates.push(semantic_coherence);
        }
        
        ConceptualPoint::new(coordinates, dimension_map)
    }
    
    /// Map node properties to conceptual dimensions
    pub fn map_node_to_point(
        &self,
        node_content: &NodeContent,
        node_metrics: &NodeMetrics,
    ) -> ConceptualPoint {
        let mut dimension_map = HashMap::new();
        let mut coordinates = Vec::new();
        
        // Map node-specific properties
        
        // Centrality dimension
        if let Some(&dim_id) = self.dimension_mappings.get("centrality") {
            dimension_map.insert(dim_id, coordinates.len());
            coordinates.push(node_metrics.centrality);
        }
        
        // Connectivity dimension
        if let Some(&dim_id) = self.dimension_mappings.get("connectivity") {
            dimension_map.insert(dim_id, coordinates.len());
            coordinates.push(node_metrics.degree as f64 / node_metrics.max_degree.max(1) as f64);
        }
        
        // Semantic analysis of content
        let full_content = format!("{} {}", node_content.title, node_content.description);
        let semantic_scores = self.semantic_analyzer.analyze_content(&full_content);
        let coherence = self.semantic_analyzer.calculate_coherence(&semantic_scores);
        
        // Map semantic coherence
        if let Some(&dim_id) = self.dimension_mappings.get("semantic_coherence") {
            dimension_map.insert(dim_id, coordinates.len());
            coordinates.push(coherence);
        }
        
        // Map complexity based on content and data structure
        let content_complexity = self.calculate_content_complexity(node_content);
        if let Some(&dim_id) = self.dimension_mappings.get("complexity") {
            dimension_map.insert(dim_id, coordinates.len());
            coordinates.push(content_complexity);
        }
        
        // Map modularity based on data structure
        let data_modularity = self.calculate_data_modularity(&node_content.data);
        if let Some(&dim_id) = self.dimension_mappings.get("modularity") {
            dimension_map.insert(dim_id, coordinates.len());
            coordinates.push(data_modularity);
        }
        
        ConceptualPoint::new(coordinates, dimension_map)
    }
    
    /// Calculate content complexity based on various factors
    fn calculate_content_complexity(&self, content: &NodeContent) -> f64 {
        let title_complexity = (content.title.len() as f64 / 50.0).min(1.0);
        let desc_complexity = (content.description.len() as f64 / 200.0).min(1.0);
        
        let data_complexity = match &content.data {
            serde_json::Value::Null => 0.0,
            serde_json::Value::Bool(_) | serde_json::Value::Number(_) | serde_json::Value::String(_) => 0.2,
            serde_json::Value::Array(arr) => {
                let size_factor = (arr.len() as f64 / 10.0).min(1.0);
                let depth_factor = self.calculate_json_depth(&content.data) as f64 / 5.0;
                (size_factor + depth_factor) / 2.0
            }
            serde_json::Value::Object(map) => {
                let size_factor = (map.len() as f64 / 10.0).min(1.0);
                let depth_factor = self.calculate_json_depth(&content.data) as f64 / 5.0;
                (size_factor + depth_factor) / 2.0
            }
        };
        
        // Weighted average
        (title_complexity * 0.2 + desc_complexity * 0.3 + data_complexity * 0.5).min(1.0)
    }
    
    /// Calculate modularity of data structure
    fn calculate_data_modularity(&self, data: &serde_json::Value) -> f64 {
        match data {
            serde_json::Value::Object(map) => {
                if map.is_empty() {
                    return 0.0;
                }
                
                // Check for nested objects (indicates modularity)
                let nested_count = map.values()
                    .filter(|v| matches!(v, serde_json::Value::Object(_) | serde_json::Value::Array(_)))
                    .count();
                
                (nested_count as f64 / map.len() as f64).min(1.0)
            }
            serde_json::Value::Array(arr) => {
                if arr.is_empty() {
                    return 0.0;
                }
                
                // Check for consistent structure (indicates modularity)
                let first_type = std::mem::discriminant(&arr[0]);
                let consistent = arr.iter().all(|v| std::mem::discriminant(v) == first_type);
                
                if consistent { 0.8 } else { 0.3 }
            }
            _ => 0.1, // Simple values have low modularity
        }
    }
    
    /// Calculate the depth of a JSON structure
    fn calculate_json_depth(&self, value: &serde_json::Value) -> usize {
        match value {
            serde_json::Value::Object(map) => {
                map.values()
                    .map(|v| self.calculate_json_depth(v))
                    .max()
                    .unwrap_or(0) + 1
            }
            serde_json::Value::Array(arr) => {
                arr.iter()
                    .map(|v| self.calculate_json_depth(v))
                    .max()
                    .unwrap_or(0) + 1
            }
            _ => 0,
        }
    }
    
    /// Create quality dimensions for graph analysis
    pub fn create_graph_dimensions() -> Vec<QualityDimension> {
        vec![
            QualityDimension {
                id: DimensionId::new(),
                name: "complexity".to_string(),
                dimension_type: DimensionType::Continuous,
                range: 0.0..1.0,
                context: Some("graph_analysis".to_string()),
                description: Some("Graph structural complexity".to_string()),
            },
            QualityDimension {
                id: DimensionId::new(),
                name: "connectivity".to_string(),
                dimension_type: DimensionType::Continuous,
                range: 0.0..1.0,
                context: Some("graph_analysis".to_string()),
                description: Some("Degree of interconnection".to_string()),
            },
            QualityDimension {
                id: DimensionId::new(),
                name: "hierarchy".to_string(),
                dimension_type: DimensionType::Continuous,
                range: 0.0..1.0,
                context: Some("graph_analysis".to_string()),
                description: Some("Hierarchical structure strength".to_string()),
            },
            QualityDimension {
                id: DimensionId::new(),
                name: "modularity".to_string(),
                dimension_type: DimensionType::Continuous,
                range: 0.0..1.0,
                context: Some("graph_analysis".to_string()),
                description: Some("Community structure strength".to_string()),
            },
            QualityDimension {
                id: DimensionId::new(),
                name: "semantic_coherence".to_string(),
                dimension_type: DimensionType::Continuous,
                range: 0.0..1.0,
                context: Some("graph_analysis".to_string()),
                description: Some("Semantic consistency of content".to_string()),
            },
        ]
    }
    
    // Helper methods
    
    fn calculate_complexity(&self, metrics: &GraphMetrics) -> f64 {
        // Normalize complexity based on graph size
        let size_factor = (metrics.node_count as f64).ln() / 10.0;
        let edge_factor = metrics.edge_count as f64 / metrics.max_possible_edges.max(1.0);
        
        (size_factor * self.feature_weights.node_count + 
         edge_factor * self.feature_weights.edge_count) / 
        (self.feature_weights.node_count + self.feature_weights.edge_count)
    }
    
    fn calculate_hierarchy(&self, metrics: &GraphMetrics) -> f64 {
        // Simple hierarchy measure based on depth and branching
        let depth_factor = (metrics.max_depth as f64 / 10.0).min(1.0);
        let branching_factor = if metrics.node_count > 1 {
            1.0 - (metrics.edge_count as f64 / (metrics.node_count as f64 - 1.0)).min(1.0)
        } else {
            0.0
        };
        
        (depth_factor + branching_factor) / 2.0
    }
}

/// Metrics for a graph
#[derive(Debug, Clone)]
pub struct GraphMetrics {
    pub node_count: usize,
    pub edge_count: usize,
    pub average_degree: f64,
    pub max_possible_edges: f64,
    pub clustering_coefficient: f64,
    pub modularity: f64,
    pub max_depth: usize,
    pub connected_components: usize,
}

impl Default for GraphMetrics {
    fn default() -> Self {
        Self {
            node_count: 0,
            edge_count: 0,
            average_degree: 0.0,
            max_possible_edges: 0.0,
            clustering_coefficient: 0.0,
            modularity: 0.0,
            max_depth: 0,
            connected_components: 0,
        }
    }
}

/// Metrics for a node
#[derive(Debug, Clone)]
pub struct NodeMetrics {
    pub degree: usize,
    pub max_degree: usize,
    pub centrality: f64,
    pub clustering_coefficient: f64,
    pub betweenness: f64,
}

impl Default for NodeMetrics {
    fn default() -> Self {
        Self {
            degree: 0,
            max_degree: 1,
            centrality: 0.0,
            clustering_coefficient: 0.0,
            betweenness: 0.0,
        }
    }
}

/// Summary of graph content for semantic analysis
#[derive(Debug, Clone)]
pub struct GraphContentSummary {
    pub node_titles: Vec<String>,
    pub node_descriptions: Vec<String>,
    pub edge_labels: Vec<String>,
}

impl GraphContentSummary {
    pub fn new() -> Self {
        Self {
            node_titles: Vec::new(),
            node_descriptions: Vec::new(),
            edge_labels: Vec::new(),
        }
    }
    
    pub fn add_node(&mut self, title: String, description: String) {
        self.node_titles.push(title);
        self.node_descriptions.push(description);
    }
    
    pub fn add_edge(&mut self, label: String) {
        self.edge_labels.push(label);
    }
}

/// Integration event: Graph mapped to conceptual space
#[derive(Debug, Clone)]
pub struct GraphMappedToConceptualSpace {
    pub graph_id: GraphId,
    pub space_id: ConceptualSpaceId,
    pub conceptual_point: ConceptualPoint,
    pub metrics: GraphMetrics,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_graph_to_conceptual_mapping() {
        let mapper = GraphConceptualMapper::new();
        
        let metrics = GraphMetrics {
            node_count: 10,
            edge_count: 15,
            average_degree: 3.0,
            max_possible_edges: 45.0,
            clustering_coefficient: 0.4,
            modularity: 0.6,
            max_depth: 3,
            connected_components: 1,
        };
        
        let point = mapper.map_graph_to_point(&metrics, None);
        
        // Check that point has expected dimensions
        assert_eq!(point.coordinates.len(), 5);
        
        // Check that all coordinates are in valid range
        for coord in &point.coordinates {
            assert!(*coord >= 0.0 && *coord <= 1.0);
        }
    }
    
    #[test]
    fn test_map_node_to_point() {
        let mapper = GraphConceptualMapper::new();
        let node_content = NodeContent {
            title: "Test Node".to_string(),
            description: "Test description".to_string(),
            data: serde_json::json!({"type": "test"}),
        };
        let metrics = NodeMetrics {
            centrality: 0.5,
            degree: 3,
            clustering_coefficient: 0.7,
            betweenness: 0.3,
            max_degree: 10,
        };
        
        let point = mapper.map_node_to_point(&node_content, &metrics);
        assert!(!point.coordinates.is_empty());
    }
} 