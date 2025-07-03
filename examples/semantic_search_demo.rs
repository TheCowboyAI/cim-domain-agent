//! Semantic Search Demo
//!
//! This example demonstrates the semantic search capabilities including:
//! - Indexing documents, graphs, and concepts
//! - Searching with similarity thresholds
//! - Filtering by metadata
//! - Cross-domain search

use cim_domain_agent::{
    semantic_search::{
        search_engine::DefaultSemanticSearchEngine, 
        EmbeddingRequest,
        embedding_service::MockEmbeddingService,
        vector_store::InMemoryVectorStore,
        SearchQuery, SemanticSearchConfig,
        SemanticSearchEngine,
        vector_store::SearchFilter,
    },
    ai_providers::{GraphData, NodeData, EdgeData},
};
use colored::*;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{"=== Semantic Search Demo ===".blue(}").bold());
    println!();
    
    // Create semantic search engine
    let engine = create_search_engine().await?;
    
    // Index various types of content
    index_sample_content(&engine).await?;
    
    // Demonstrate different search capabilities
    demo_basic_search(&engine).await?;
    demo_filtered_search(&engine).await?;
    demo_graph_search(&engine).await?;
    demo_cross_domain_search(&engine).await?;
    
    Ok(())
}

async fn create_search_engine() -> Result<Arc<dyn SemanticSearchEngine>, Box<dyn std::error::Error>> {
    println!("{"Creating semantic search engine...".green(}"));
    
    // Use mock embedding service for demo (would use real AI provider in production)
    let embedding_service = Arc::new(MockEmbeddingService::new(384));
    let vector_store = Arc::new(InMemoryVectorStore::new());
    let config = SemanticSearchConfig {
        default_model: "mock-embedding".to_string(),
        dimensions: 384,
        max_results: 10,
        min_similarity: 0.0,
        enable_cache: true,
        cache_ttl_seconds: 3600,
    };
    
    let engine = Arc::new(DefaultSemanticSearchEngine::new(
        embedding_service,
        vector_store,
        config,
    ));
    
    println!("{"✓ Search engine created".green(}"));
    println!();
    
    Ok(engine)
}

async fn index_sample_content(engine: &Arc<dyn SemanticSearchEngine>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{"Indexing sample content...".yellow(}"));
    
    // Index documents
    let documents = vec![
        EmbeddingRequest {
            text: "Graph-based workflow optimization using parallel processing techniques for improved performance".to_string(),
            source_id: "doc_workflow_1".to_string(),
            source_type: "document".to_string(),
            metadata: HashMap::from([
                ("category".to_string(), json!("workflow")),
                ("topic".to_string(), json!("optimization")),
                ("difficulty".to_string(), json!("advanced")),
            ]),
            model: None,
        },
        EmbeddingRequest {
            text: "Sequential task execution patterns in business process management systems".to_string(),
            source_id: "doc_bpm_1".to_string(),
            source_type: "document".to_string(),
            metadata: HashMap::from([
                ("category".to_string(), json!("workflow")),
                ("topic".to_string(), json!("bpm")),
                ("difficulty".to_string(), json!("intermediate")),
            ]),
            model: None,
        },
        EmbeddingRequest {
            text: "Knowledge graph construction techniques for semantic reasoning and inference".to_string(),
            source_id: "doc_kg_1".to_string(),
            source_type: "document".to_string(),
            metadata: HashMap::from([
                ("category".to_string(), json!("knowledge")),
                ("topic".to_string(), json!("semantics")),
                ("difficulty".to_string(), json!("advanced")),
            ]),
            model: None,
        },
        EmbeddingRequest {
            text: "Machine learning models for natural language understanding and processing".to_string(),
            source_id: "doc_ml_1".to_string(),
            source_type: "document".to_string(),
            metadata: HashMap::from([
                ("category".to_string(), json!("ai")),
                ("topic".to_string(), json!("nlp")),
                ("difficulty".to_string(), json!("advanced")),
            ]),
            model: None,
        },
    ];
    
    let doc_ids = engine.index_batch(documents).await?;
    println!("  {"✓".green(} Indexed {} documents"), doc_ids.len());
    
    // Index graph nodes
    let graph_nodes = vec![
        EmbeddingRequest {
            text: "Start node: Initialize workflow with input validation and parameter checking".to_string(),
            source_id: "node_start_1".to_string(),
            source_type: "graph_node".to_string(),
            metadata: HashMap::from([
                ("node_type".to_string(), json!("start")),
                ("graph_id".to_string(), json!("workflow_1")),
            ]),
            model: None,
        },
        EmbeddingRequest {
            text: "Process node: Execute parallel data transformation with map-reduce pattern".to_string(),
            source_id: "node_process_1".to_string(),
            source_type: "graph_node".to_string(),
            metadata: HashMap::from([
                ("node_type".to_string(), json!("process")),
                ("graph_id".to_string(), json!("workflow_1")),
                ("parallel".to_string(), json!(true)),
            ]),
            model: None,
        },
        EmbeddingRequest {
            text: "Decision node: Evaluate business rules and route to appropriate branch".to_string(),
            source_id: "node_decision_1".to_string(),
            source_type: "graph_node".to_string(),
            metadata: HashMap::from([
                ("node_type".to_string(), json!("decision")),
                ("graph_id".to_string(), json!("workflow_1")),
            ]),
            model: None,
        },
    ];
    
    let node_ids = engine.index_batch(graph_nodes).await?;
    println!("  {"✓".green(} Indexed {} graph nodes"), node_ids.len());
    
    // Index concepts
    let concepts = vec![
        EmbeddingRequest {
            text: "Parallel Processing: The simultaneous execution of multiple computational tasks to improve performance and reduce processing time".to_string(),
            source_id: "concept_parallel".to_string(),
            source_type: "concept".to_string(),
            metadata: HashMap::from([
                ("domain".to_string(), json!("computing")),
                ("related_to".to_string(), json!(["optimization", "performance"])),
            ]),
            model: None,
        },
        EmbeddingRequest {
            text: "Workflow Optimization: The process of improving workflow efficiency by eliminating bottlenecks and redundant steps".to_string(),
            source_id: "concept_optimization".to_string(),
            source_type: "concept".to_string(),
            metadata: HashMap::from([
                ("domain".to_string(), json!("business")),
                ("related_to".to_string(), json!(["efficiency", "performance"])),
            ]),
            model: None,
        },
    ];
    
    let concept_ids = engine.index_batch(concepts).await?;
    println!("  {"✓".green(} Indexed {} concepts"), concept_ids.len());
    
    let stats = engine.stats().await?;
    println!();
    println!("Total indexed items: {stats.total_items.to_string(}").cyan());
    println!();
    
    Ok(())
}

async fn demo_basic_search(engine: &Arc<dyn SemanticSearchEngine>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{"1. Basic Semantic Search".blue(}").bold());
    println!("   Searching for: {"workflow optimization techniques".italic(}"));
    
    let query = SearchQuery::new("workflow optimization techniques")
        .with_limit(5)
        .with_min_similarity(0.3);
    
    let results = engine.search(query).await?;
    
    println!("\n   Results:");
    for (i, result) in results.iter().enumerate() {
        println!("   {i + 1}. {result.source_id.green(} ({})"),
            format!("{:.2}%", result.similarity * 100.0).yellow()
        );
        if let Some(metadata) = &result.metadata {
            if let Some(category) = metadata.get("category") {
                println!("      Category: {category}");
            }
        }
    }
    
    println!();
    Ok(())
}

async fn demo_filtered_search(engine: &Arc<dyn SemanticSearchEngine>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{"2. Filtered Search".blue(}").bold());
    println!("   Searching for: {"process management".italic(} with filter: category=workflow"));
    
    let filter = SearchFilter {
        source_types: Some(vec!["document".to_string()]),
        metadata_filters: HashMap::from([
            ("category".to_string(), json!("workflow")),
        ]),
        created_after: None,
        created_before: None,
    };
    
    let query = SearchQuery::new("process management")
        .with_limit(5)
        .with_filter(filter);
    
    let results = engine.search(query).await?;
    
    println!("\n   Results (filtered):");
    for (i, result) in results.iter().enumerate() {
        println!("   {i + 1}. {result.source_id.green(} ({})"),
            format!("{:.2}%", result.similarity * 100.0).yellow()
        );
    }
    
    println!();
    Ok(())
}

async fn demo_graph_search(engine: &Arc<dyn SemanticSearchEngine>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{"3. Graph Node Search".blue(}").bold());
    println!("   Searching for: {"parallel execution nodes".italic(}"));
    
    let filter = SearchFilter {
        source_types: Some(vec!["graph_node".to_string()]),
        metadata_filters: HashMap::new(),
        created_after: None,
        created_before: None,
    };
    
    let query = SearchQuery::new("parallel execution data transformation")
        .with_limit(3)
        .with_filter(filter);
    
    let results = engine.search(query).await?;
    
    println!("\n   Graph nodes found:");
    for result in results {
        println!("   - {result.source_id.green(} ({})"),
            format!("{:.2}%", result.similarity * 100.0).yellow()
        );
        if let Some(metadata) = &result.metadata {
            if let Some(node_type) = metadata.get("node_type") {
                println!("     Type: {node_type.to_string(}").cyan());
            }
        }
    }
    
    println!();
    Ok(())
}

async fn demo_cross_domain_search(engine: &Arc<dyn SemanticSearchEngine>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{"4. Cross-Domain Search".blue(}").bold());
    println!("   Searching across all content types for: {"optimization performance".italic(}"));
    
    let query = SearchQuery::new("optimization and performance improvements")
        .with_limit(8)
        .with_min_similarity(0.2);
    
    let results = engine.search(query).await?;
    
    println!("\n   Results from all domains:");
    
    // Group by source type
    let mut by_type: HashMap<String, Vec<_>> = HashMap::new();
    for result in results {
        by_type.entry(result.source_type.clone())
            .or_insert_with(Vec::new)
            .push(result);
    }
    
    for (source_type, items) in by_type {
        println!("\n   {source_type.to_uppercase(}:").cyan());
        for item in items {
            println!("   - {item.source_id.green(} ({})"),
                format!("{:.2}%", item.similarity * 100.0).yellow()
            );
        }
    }
    
    println!();
    println!("{"Demo completed successfully!".green(}").bold());
    
    Ok(())
} 