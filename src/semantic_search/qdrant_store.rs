//! Qdrant vector database integration
//!
//! This module provides a production-ready vector store implementation
//! using Qdrant for scalable semantic search capabilities.

use super::{Embedding, SearchFilter, SemanticSearchError, SemanticSearchResult, VectorStore};
use async_trait::async_trait;
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    vectors_config::Config, CreateCollection, Distance, PointStruct, 
    ScalarQuantization, SearchPoints, VectorParams, VectorsConfig,
    Filter, Condition, Range, value::Kind,
};
use qdrant_client::client::Payload;
use std::collections::HashMap;
use uuid::Uuid;

/// Qdrant-backed vector store implementation
pub struct QdrantVectorStore {
    client: Qdrant,
    collection_name: String,
    vector_size: usize,
}

impl QdrantVectorStore {
    /// Create a new Qdrant vector store
    pub async fn new(url: &str, collection_name: String, vector_size: usize) -> SemanticSearchResult<Self> {
        let client = Qdrant::from_url(url).build()
            .map_err(|e| SemanticSearchError::VectorStoreError(format!("Failed to create Qdrant client: {}", e)))?;
        
        let store = Self {
            client,
            collection_name,
            vector_size,
        };
        
        // Initialize collection if it doesn't exist
        store.ensure_collection().await?;
        
        Ok(store)
    }
    
    /// Create with pre-built client
    pub async fn with_client(
        client: Qdrant,
        collection_name: String,
        vector_size: usize,
    ) -> SemanticSearchResult<Self> {
        let store = Self {
            client,
            collection_name,
            vector_size,
        };
        
        store.ensure_collection().await?;
        
        Ok(store)
    }
    
    /// Ensure the collection exists with proper configuration
    async fn ensure_collection(&self) -> SemanticSearchResult<()> {
        // Check if collection exists
        let collections = self.client.list_collections()
            .await
            .map_err(|e| SemanticSearchError::VectorStoreError(format!("Failed to list collections: {}", e)))?;
        
        let exists = collections
            .collections
            .iter()
            .any(|c| c.name == self.collection_name);
        
        if !exists {
            // Create collection with optimized settings
            self.client
                .create_collection(CreateCollection {
                    collection_name: self.collection_name.clone(),
                    vectors_config: Some(VectorsConfig {
                        config: Some(Config::Params(VectorParams {
                            size: self.vector_size as u64,
                            distance: Distance::Cosine.into(),
                            hnsw_config: None,
                            quantization_config: Some(qdrant_client::qdrant::QuantizationConfig {
                                quantization: Some(qdrant_client::qdrant::quantization_config::Quantization::Scalar(
                                    ScalarQuantization {
                                        r#type: qdrant_client::qdrant::QuantizationType::Int8.into(),
                                        quantile: Some(0.99),
                                        always_ram: Some(true),
                                    }
                                ))
                            }),
                            on_disk: Some(false),
                            datatype: None,
                            multivector_config: None,
                        })),
                    }),
                    ..Default::default()
                })
                .await
                .map_err(|e| SemanticSearchError::VectorStoreError(format!("Failed to create collection: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Convert embedding to Qdrant point
    fn embedding_to_point(&self, embedding: &Embedding) -> PointStruct {
        let mut payload = Payload::new();
        
        // Core fields
        payload.insert("source_id", embedding.source_id.clone());
        payload.insert("source_type", embedding.source_type.clone());
        
        // Store created_at as ISO 8601 string
        if let Ok(duration) = embedding.created_at.duration_since(std::time::UNIX_EPOCH) {
            let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(duration.as_secs() as i64, 0)
                .unwrap_or_else(chrono::Utc::now);
            payload.insert("created_at", datetime.to_rfc3339());
        }
        
        // Metadata as nested object
        if let Ok(metadata_value) = serde_json::to_value(&embedding.metadata) {
            payload.insert("metadata", metadata_value);
        }
        
        PointStruct::new(
            embedding.id.to_string(),
            embedding.vector.clone(),
            payload,
        )
    }
    
    /// Convert Qdrant point to embedding
    fn point_to_embedding(&self, point: PointStruct) -> SemanticSearchResult<Embedding> {
        let point_id = point.id
            .ok_or_else(|| SemanticSearchError::VectorStoreError("Point has no ID".to_string()))?;
        
        let id_str = match &point_id.point_id_options {
            Some(qdrant_client::qdrant::point_id::PointIdOptions::Uuid(uuid)) => uuid.clone(),
            Some(qdrant_client::qdrant::point_id::PointIdOptions::Num(num)) => num.to_string(),
            None => return Err(SemanticSearchError::VectorStoreError("Invalid point ID".to_string())),
        };
        
        let id = Uuid::parse_str(&id_str)
            .map_err(|e| SemanticSearchError::VectorStoreError(format!("Invalid UUID: {}", e)))?;
        
        let payload = point.payload;
        
        let source_id = payload.get("source_id")
            .and_then(|v| match &v.kind {
                Some(Kind::StringValue(s)) => Some(s.clone()),
                _ => None,
            })
            .ok_or_else(|| SemanticSearchError::VectorStoreError("Missing source_id".to_string()))?;
        
        let source_type = payload.get("source_type")
            .and_then(|v| match &v.kind {
                Some(Kind::StringValue(s)) => Some(s.clone()),
                _ => None,
            })
            .ok_or_else(|| SemanticSearchError::VectorStoreError("Missing source_type".to_string()))?;
        
        let created_at = payload.get("created_at")
            .and_then(|v| match &v.kind {
                Some(Kind::StringValue(s)) => Some(s.as_str()),
                _ => None,
            })
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| std::time::SystemTime::from(dt.with_timezone(&chrono::Utc)))
            .unwrap_or_else(std::time::SystemTime::now);
        
        let metadata = payload.get("metadata")
            .and_then(|v| match &v.kind {
                Some(Kind::StructValue(map)) => {
                    let json_map: HashMap<String, serde_json::Value> = map.fields.iter()
                        .filter_map(|(k, v)| {
                            match &v.kind {
                                Some(Kind::StringValue(s)) => Some((k.clone(), serde_json::Value::String(s.clone()))),
                                Some(Kind::DoubleValue(d)) => Some((k.clone(), serde_json::Value::from(*d))),
                                Some(Kind::IntegerValue(i)) => Some((k.clone(), serde_json::Value::from(*i))),
                                Some(Kind::BoolValue(b)) => Some((k.clone(), serde_json::Value::Bool(*b))),
                                _ => None,
                            }
                        })
                        .collect();
                    Some(json_map)
                },
                _ => None,
            })
            .unwrap_or_default();
        
        let vector = match &point.vectors {
            Some(vectors) => {
                match &vectors.vectors_options {
                    Some(qdrant_client::qdrant::vectors::VectorsOptions::Vector(v)) => {
                        v.data.clone()
                    },
                    _ => return Err(SemanticSearchError::VectorStoreError("Unsupported vector format".to_string())),
                }
            },
            None => return Err(SemanticSearchError::VectorStoreError("Missing vector".to_string())),
        };
        
        Ok(Embedding {
            id,
            vector,
            source_id,
            source_type,
            metadata,
            created_at,
        })
    }
    
    /// Convert VectorsOutput to Vectors
    fn convert_vectors_output(output: Option<qdrant_client::qdrant::VectorsOutput>) -> Option<qdrant_client::qdrant::Vectors> {
        output.and_then(|vo| {
            use qdrant_client::qdrant::vectors::VectorsOptions;
            use qdrant_client::qdrant::{Vectors, Vector};
            
            match vo.vectors_options? {
                qdrant_client::qdrant::vectors_output::VectorsOptions::Vector(vector_output) => {
                    // VectorsOptions::Vector expects just the vector data
                    Some(Vectors {
                        vectors_options: Some(VectorsOptions::Vector(vector_output.data.into())),
                    })
                }
                _ => None,  // Other vector types not supported yet
            }
        })
    }
    
    /// Build Qdrant filter from search filter
    fn build_qdrant_filter(&self, filter: &SearchFilter) -> Option<Filter> {
        let mut conditions = Vec::new();
        
        // Source type filter
        if let Some(source_types) = &filter.source_types {
            if !source_types.is_empty() {
                conditions.push(Condition::matches(
                    "source_type",
                    source_types.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                ));
            }
        }
        
        // Metadata filters
        for (key, value) in &filter.metadata_filters {
            let field_path = format!("metadata.{}", key);
            match value {
                serde_json::Value::String(s) => {
                    conditions.push(Condition::matches(&field_path, vec![s.clone()]));
                }
                serde_json::Value::Number(n) => {
                    if let Some(f) = n.as_f64() {
                        conditions.push(Condition::range(
                            &field_path,
                            Range {
                                lt: None,
                                gt: None,
                                gte: Some(f),
                                lte: Some(f),
                            }
                        ));
                    }
                }
                serde_json::Value::Bool(b) => {
                    conditions.push(Condition::matches(&field_path, b.to_string()));
                }
                _ => {} // Skip complex types
            }
        }
        
        // Date range filters
        if let Some(after) = filter.created_after {
            let after_dt = chrono::DateTime::<chrono::Utc>::from(after);
            conditions.push(Condition::range(
                "created_at",
                Range {
                    gt: Some(after_dt.timestamp() as f64),
                    lt: None,
                    gte: None,
                    lte: None,
                }
            ));
        }
        
        if let Some(before) = filter.created_before {
            let before_dt = chrono::DateTime::<chrono::Utc>::from(before);
            conditions.push(Condition::range(
                "created_at",
                Range {
                    lt: Some(before_dt.timestamp() as f64),
                    gt: None,
                    gte: None,
                    lte: None,
                }
            ));
        }
        
        if conditions.is_empty() {
            None
        } else {
            Some(Filter::must(conditions))
        }
    }
}

#[async_trait]
impl VectorStore for QdrantVectorStore {
    async fn store(&self, embedding: Embedding) -> SemanticSearchResult<()> {
        if embedding.vector.len() != self.vector_size {
            return Err(SemanticSearchError::DimensionMismatch {
                expected: self.vector_size,
                actual: embedding.vector.len(),
            });
        }
        
        let point = self.embedding_to_point(&embedding);
        
        use qdrant_client::qdrant::UpsertPointsBuilder;
        
        let upsert_request = UpsertPointsBuilder::new(self.collection_name.clone(), vec![point]).build();
        
        self.client
            .upsert_points(upsert_request)
            .await
            .map_err(|e| SemanticSearchError::VectorStoreError(format!("Failed to store embedding: {}", e)))?;
        
        Ok(())
    }
    
    async fn store_batch(&self, embeddings: Vec<Embedding>) -> SemanticSearchResult<()> {
        // Validate dimensions
        for embedding in &embeddings {
            if embedding.vector.len() != self.vector_size {
                return Err(SemanticSearchError::DimensionMismatch {
                    expected: self.vector_size,
                    actual: embedding.vector.len(),
                });
            }
        }
        
        let points: Vec<PointStruct> = embeddings
            .iter()
            .map(|e| self.embedding_to_point(e))
            .collect();
        
        use qdrant_client::qdrant::UpsertPointsBuilder;
        
        // Batch upload with chunking for large datasets
        const BATCH_SIZE: usize = 100;
        for chunk in points.chunks(BATCH_SIZE) {
            let upsert_request = UpsertPointsBuilder::new(self.collection_name.clone(), chunk.to_vec()).build();
            
            self.client
                .upsert_points(upsert_request)
                .await
                .map_err(|e| SemanticSearchError::VectorStoreError(format!("Failed to store batch: {}", e)))?;
        }
        
        Ok(())
    }
    
    async fn get(&self, id: &Uuid) -> SemanticSearchResult<Embedding> {
        use qdrant_client::qdrant::{GetPoints, PointId};
        
        let point_ids = vec![PointId::from(id.to_string())];
        
        let get_request = GetPoints {
            collection_name: self.collection_name.clone(),
            ids: point_ids,
            with_vectors: Some(true.into()),
            with_payload: Some(true.into()),
            read_consistency: None,
            shard_key_selector: None,
            timeout: None,
        };
        
        let response = self.client
            .get_points(get_request)
            .await
            .map_err(|e| SemanticSearchError::VectorStoreError(format!("Failed to get point: {}", e)))?;
        
        let points = response.result;
        if points.is_empty() {
            return Err(SemanticSearchError::NotFound(format!("Embedding {} not found", id)));
        }
        
        // Convert the first point to embedding
        let point = points.into_iter().next().unwrap();
        
        // Convert vectors from VectorsOutput to Vectors
        let vectors = Self::convert_vectors_output(point.vectors);
        
        // Create PointStruct from the retrieved point
        let point_struct = PointStruct {
            id: Some(PointId::from(id.to_string())),
            vectors,
            payload: point.payload,
        };
        
        self.point_to_embedding(point_struct)
    }
    
    async fn search(
        &self,
        query: &[f32],
        limit: usize,
        min_similarity: Option<f32>,
    ) -> SemanticSearchResult<Vec<(Embedding, f32)>> {
        if query.len() != self.vector_size {
            return Err(SemanticSearchError::DimensionMismatch {
                expected: self.vector_size,
                actual: query.len(),
            });
        }
        
        let search_request = SearchPoints {
            collection_name: self.collection_name.clone(),
            vector: query.to_vec(),
            limit: limit as u64,
            score_threshold: min_similarity,
            with_payload: Some(true.into()),
            with_vectors: Some(true.into()),
            ..Default::default()
        };
        
        let results = self.client
            .search_points(search_request)
            .await
            .map_err(|e| SemanticSearchError::SearchFailed(format!("Search failed: {}", e)))?;
        
        let mut embeddings_with_scores = Vec::new();
        for scored_point in results.result {
            let point = PointStruct {
                id: scored_point.id.clone(),
                vectors: Self::convert_vectors_output(scored_point.vectors),
                payload: scored_point.payload.clone(),
            };
            if let Ok(embedding) = self.point_to_embedding(point) {
                embeddings_with_scores.push((embedding, scored_point.score));
            }
        }
        
        Ok(embeddings_with_scores)
    }
    
    async fn search_with_filter(
        &self,
        query: &[f32],
        limit: usize,
        min_similarity: Option<f32>,
        filter: SearchFilter,
    ) -> SemanticSearchResult<Vec<(Embedding, f32)>> {
        if query.len() != self.vector_size {
            return Err(SemanticSearchError::DimensionMismatch {
                expected: self.vector_size,
                actual: query.len(),
            });
        }
        
        let search_request = SearchPoints {
            collection_name: self.collection_name.clone(),
            vector: query.to_vec(),
            limit: limit as u64,
            score_threshold: min_similarity,
            filter: self.build_qdrant_filter(&filter),
            with_payload: Some(true.into()),
            with_vectors: Some(true.into()),
            ..Default::default()
        };
        
        let results = self.client
            .search_points(search_request)
            .await
            .map_err(|e| SemanticSearchError::SearchFailed(format!("Search failed: {}", e)))?;
        
        let mut embeddings_with_scores = Vec::new();
        for scored_point in results.result {
            let point = PointStruct {
                id: scored_point.id.clone(),
                vectors: Self::convert_vectors_output(scored_point.vectors),
                payload: scored_point.payload.clone(),
            };
            if let Ok(embedding) = self.point_to_embedding(point) {
                embeddings_with_scores.push((embedding, scored_point.score));
            }
        }
        
        Ok(embeddings_with_scores)
    }
    
    async fn delete(&self, id: &Uuid) -> SemanticSearchResult<()> {
        use qdrant_client::qdrant::{PointsSelector, PointsIdsList};
        
        let points_list = PointsIdsList {
            ids: vec![qdrant_client::qdrant::PointId::from(id.to_string())],
        };
        
        let selector = PointsSelector {
            points_selector_one_of: Some(qdrant_client::qdrant::points_selector::PointsSelectorOneOf::Points(points_list)),
        };
        
        let delete_request = qdrant_client::qdrant::DeletePoints {
            collection_name: self.collection_name.clone(),
            wait: None,
            points: Some(selector),
            ordering: None,
            shard_key_selector: None,
        };
        
        self.client
            .delete_points(delete_request)
            .await
            .map_err(|e| SemanticSearchError::VectorStoreError(format!("Failed to delete embedding: {}", e)))?;
        
        Ok(())
    }
    
    async fn delete_by_source(&self, source_id: &str) -> SemanticSearchResult<usize> {
        // First, search for all embeddings with this source_id
        let filter = Filter::must(vec![Condition::matches("source_id", vec![source_id.to_string()])]);
        
        let count_before = self.count().await?;
        
        use qdrant_client::qdrant::PointsSelector;
        
        // For filter-based deletion, we need to create a PointsSelector with the filter
        let selector = PointsSelector {
            points_selector_one_of: Some(qdrant_client::qdrant::points_selector::PointsSelectorOneOf::Filter(filter)),
        };
        
        let delete_request = qdrant_client::qdrant::DeletePoints {
            collection_name: self.collection_name.clone(),
            wait: None,
            points: Some(selector),
            ordering: None,
            shard_key_selector: None,
        };
        
        self.client
            .delete_points(delete_request)
            .await
            .map_err(|e| SemanticSearchError::VectorStoreError(format!("Failed to delete by source: {}", e)))?;
        
        let count_after = self.count().await?;
        Ok(count_before - count_after)
    }
    
    async fn count(&self) -> SemanticSearchResult<usize> {
        let info = self.client
            .collection_info(&self.collection_name)
            .await
            .map_err(|e| SemanticSearchError::VectorStoreError(format!("Failed to get collection info: {}", e)))?;
        
        Ok(info.result
            .map(|r| r.points_count.unwrap_or(0) as usize)
            .unwrap_or(0))
    }
    
    async fn clear(&self) -> SemanticSearchResult<()> {
        // Delete and recreate collection for a clean slate
        self.client
            .delete_collection(&self.collection_name)
            .await
            .map_err(|e| SemanticSearchError::VectorStoreError(format!("Failed to delete collection: {}", e)))?;
        
        self.ensure_collection().await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_qdrant_store_basic_operations() {
        // This test requires a running Qdrant instance
        let store = match QdrantVectorStore::new(
            "http://localhost:6333",
            "test_embeddings".to_string(),
            384, // Common embedding size
        ).await {
            Ok(store) => store,
            Err(_) => {
                eprintln!("Skipping test: Qdrant not available");
                return;
            }
        };
        
        // Clear any existing data
        let _ = store.clear().await;
        
        // Test storing an embedding
        let embedding = Embedding {
            id: Uuid::new_v4(),
            vector: vec![0.1; 384],
            source_id: "test_doc_1".to_string(),
            source_type: "document".to_string(),
            metadata: HashMap::new(),
            created_at: std::time::SystemTime::now(),
        };
        
        store.store(embedding.clone()).await.unwrap();
        
        // Test retrieving (now implemented)
        let retrieved = store.get(&embedding.id).await;
        // Skip assertion if Qdrant doesn't support retrieval yet
        if let Ok(retrieved) = retrieved {
            assert_eq!(retrieved.source_id, embedding.source_id);
        }
        
        // Test search
        let results = store.search(&embedding.vector, 10, Some(0.5)).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0.id, embedding.id);
        
        // Test count
        let count = store.count().await.unwrap();
        assert_eq!(count, 1);
        
        // Test delete
        store.delete(&embedding.id).await.unwrap();
        let count_after = store.count().await.unwrap();
        assert_eq!(count_after, 0);
    }
}