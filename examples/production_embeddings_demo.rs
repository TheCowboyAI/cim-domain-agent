//! Production Embeddings Demo
//!
//! This example demonstrates using real AI providers for embeddings:
//! - OpenAI text-embedding models
//! - Anthropic Claude (simulated embeddings)
//! - Ollama local models
//! - Integration with Qdrant vector database

use cim_domain_agent::semantic_search::{
    Embedding, EmbeddingRequest, EmbeddingServiceConfig, EmbeddingServiceFactory,
    VectorStoreConfig, VectorStoreFactory, SearchFilter,
};
use colored::*;
use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("{}", "🚀 Production Embeddings Demo".blue().bold());
    println!("{}", "================================\n".blue());

    // Select embedding provider based on environment
    let embedding_config = select_embedding_provider();
    println!("Selected embedding provider: {}", format!("{:?}", embedding_config).green());

    // Create embedding service
    let embedding_service = EmbeddingServiceFactory::create(&embedding_config).await?;
    println!("✅ Embedding service initialized (dimensions: {})\n", embedding_service.dimensions());

    // Configure vector store
    let vector_config = if std::env::var("QDRANT_URL").is_ok() {
        VectorStoreConfig::Qdrant {
            url: std::env::var("QDRANT_URL").unwrap(),
            collection_name: "production_demo".to_string(),
            vector_size: embedding_service.dimensions(),
            api_key: std::env::var("QDRANT_API_KEY").ok(),
            timeout_secs: 30,
        }
    } else {
        println!("{}", "ℹ️  Using in-memory vector store (set QDRANT_URL for production)".yellow());
        VectorStoreConfig::Memory
    };

    let vector_store = VectorStoreFactory::create(&vector_config).await?;
    let _ = vector_store.clear().await; // Clean slate for demo

    // Sample content from different domains
    let content = vec![
        (
            "workflow_optimization",
            "workflow",
            "Advanced workflow optimization techniques using parallel processing and distributed computing for enterprise-scale automation"
        ),
        (
            "ai_agent_capabilities",
            "agent",
            "Autonomous AI agents with semantic reasoning capabilities for intelligent task automation and decision support"
        ),
        (
            "graph_visualization",
            "graph",
            "Interactive 3D graph visualization with force-directed layouts and real-time collaboration features"
        ),
        (
            "policy_compliance",
            "policy",
            "Automated policy compliance monitoring with real-time violation detection and remediation workflows"
        ),
        (
            "conceptual_reasoning",
            "conceptual",
            "Geometric conceptual spaces for semantic similarity computation and knowledge representation"
        ),
    ];

    // Generate and store embeddings
    println!("{}", "📝 Generating embeddings for content...".cyan());
    let mut requests = Vec::new();
    
    for (id, content_type, text) in &content {
        let mut metadata = HashMap::new();
        metadata.insert("content_type".to_string(), json!(content_type));
        metadata.insert("length".to_string(), json!(text.len()));
        
        requests.push(EmbeddingRequest {
            text: text.to_string(),
            source_id: id.to_string(),
            source_type: "document".to_string(),
            metadata,
            model: None,
        });
    }

    let start = std::time::Instant::now();
    let embeddings = embedding_service.generate_embeddings(requests).await?;
    let duration = start.elapsed();
    
    println!("✅ Generated {} embeddings in {:.2}s", embeddings.len(), duration.as_secs_f32());

    // Store in vector database
    vector_store.store_batch(embeddings).await?;
    println!("✅ Stored embeddings in vector database\n");

    // Demo queries
    demo_semantic_search(&embedding_service, &vector_store, "parallel workflow automation").await?;
    demo_semantic_search(&embedding_service, &vector_store, "AI decision making").await?;
    demo_semantic_search(&embedding_service, &vector_store, "compliance monitoring").await?;

    // Demo filtered search
    println!("{}", "\n🔍 Filtered Search Demo".blue().bold());
    println!("Query: 'intelligent systems' (filter: content_type='agent')");
    
    let query_request = EmbeddingRequest {
        text: "intelligent systems".to_string(),
        source_id: "query".to_string(),
        source_type: "query".to_string(),
        metadata: HashMap::new(),
        model: None,
    };
    
    let query_embedding = embedding_service.generate_embedding(query_request).await?;
    
    let filter = SearchFilter {
        source_types: None,
        metadata_filters: HashMap::from([
            ("content_type".to_string(), json!("agent"))
        ]),
        created_after: None,
        created_before: None,
    };
    
    let filtered_results = vector_store.search_with_filter(
        &query_embedding.vector,
        3,
        Some(0.5),
        filter
    ).await?;
    
    println!("\nFiltered Results:");
    for (embedding, score) in filtered_results {
        println!("  - {} (score: {:.3})", 
            embedding.source_id.green(), 
            score
        );
    }

    // Show statistics
    println!("\n{}", "📊 Statistics".blue().bold());
    let count = vector_store.count().await?;
    println!("Total embeddings stored: {}", count);
    println!("Embedding dimensions: {}", embedding_service.dimensions());
    println!("Vector store type: {}", 
        if matches!(vector_config, VectorStoreConfig::Qdrant { .. }) { 
            "Qdrant" 
        } else { 
            "In-Memory" 
        }
    );

    println!("\n{}", "✨ Demo completed successfully!".green().bold());
    
    Ok(())
}

/// Select embedding provider based on environment variables
fn select_embedding_provider() -> EmbeddingServiceConfig {
    // Check for API keys in order of preference
    if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
        println!("{}", "Found OpenAI API key, using OpenAI embeddings".green());
        EmbeddingServiceConfig::OpenAI {
            api_key: Some(api_key),
            model: std::env::var("OPENAI_EMBEDDING_MODEL")
                .unwrap_or_else(|_| "text-embedding-3-small".to_string()),
            dimensions: std::env::var("EMBEDDING_DIMENSIONS")
                .ok()
                .and_then(|s| s.parse().ok()),
            max_concurrent_requests: 10,
        }
    } else if let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") {
        println!("{}", "Found Anthropic API key, using Anthropic embeddings".green());
        EmbeddingServiceConfig::Anthropic {
            api_key: Some(api_key),
            model: std::env::var("ANTHROPIC_MODEL")
                .unwrap_or_else(|_| "claude-3-haiku-20240307".to_string()),
        }
    } else if std::env::var("OLLAMA_URL").is_ok() || 
              std::path::Path::new("/usr/local/bin/ollama").exists() {
        println!("{}", "Detected Ollama, using local embeddings".green());
        EmbeddingServiceConfig::Ollama {
            url: std::env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
            model: std::env::var("OLLAMA_EMBEDDING_MODEL")
                .unwrap_or_else(|_| "nomic-embed-text".to_string()),
            dimensions: 768,
        }
    } else {
        println!("{}", "No API keys found, using mock embeddings".yellow());
        println!("{}", "Set OPENAI_API_KEY, ANTHROPIC_API_KEY, or OLLAMA_URL for real embeddings".yellow());
        EmbeddingServiceConfig::Mock {
            dimensions: 384,
        }
    }
}

/// Perform a semantic search query
async fn demo_semantic_search(
    embedding_service: &dyn cim_domain_agent::semantic_search::EmbeddingService,
    vector_store: &dyn cim_domain_agent::semantic_search::VectorStore,
    query: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", format!("🔍 Query: '{}'", query).cyan());
    
    let query_request = EmbeddingRequest {
        text: query.to_string(),
        source_id: "query".to_string(),
        source_type: "query".to_string(),
        metadata: HashMap::new(),
        model: None,
    };
    
    let start = std::time::Instant::now();
    let query_embedding = embedding_service.generate_embedding(query_request).await?;
    let embed_time = start.elapsed();
    
    let results = vector_store.search(
        &query_embedding.vector,
        3,
        Some(0.5)
    ).await?;
    let search_time = start.elapsed() - embed_time;
    
    println!("Results (embed: {:.3}s, search: {:.3}s):", 
        embed_time.as_secs_f32(), 
        search_time.as_secs_f32()
    );
    
    for (embedding, score) in results {
        println!("  - {} (score: {:.3})", 
            embedding.source_id.green(), 
            score
        );
    }
    
    Ok(())
}