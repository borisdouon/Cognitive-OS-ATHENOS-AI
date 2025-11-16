/// Phase: B | Step: 7 | Source: Athenos_AI_Strategy.md#L114
/// RAG Stack - Index docs + neuroscience excerpts
/// Deploy RAG stack with documentation, neuroscience excerpts, workflow playbooks

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Document chunk for RAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub id: String,
    pub content: String,
    pub source: String,
    pub embedding: Vec<f32>, // Simplified: would use proper embeddings in production
    pub metadata: HashMap<String, String>,
}

/// RAG index for retrieval-augmented generation
/// Source: Athenos_AI_Strategy.md#L114
pub struct RAGIndex {
    chunks: Vec<DocumentChunk>,
    source_index: HashMap<String, Vec<usize>>,
}

impl RAGIndex {
    /// Create new RAG index
    pub fn new() -> Self {
        info!("RAGIndex::new: Creating RAG index");
        Self {
            chunks: Vec::new(),
            source_index: HashMap::new(),
        }
    }

    /// Index a document chunk
    /// Source: Athenos_AI_Strategy.md#L114
    pub fn index_chunk(&mut self, chunk: DocumentChunk) {
        info!("RAGIndex::index_chunk: Indexing chunk {} from {}", chunk.id, chunk.source);
        let idx = self.chunks.len();
        self.chunks.push(chunk.clone());
        
        self.source_index
            .entry(chunk.source.clone())
            .or_insert_with(Vec::new)
            .push(idx);
    }

    /// Search for relevant chunks (simplified similarity)
    /// Source: Athenos_AI_Strategy.md#L114
    pub fn search(&self, query: &str, limit: usize) -> Vec<&DocumentChunk> {
        info!("RAGIndex::search: Searching for '{}' (limit: {})", query, limit);
        
        // Phase B: Simple keyword matching (would use vector similarity in production)
        let query_lower = query.to_lowercase();
        let mut scored: Vec<(&DocumentChunk, usize)> = self.chunks
            .iter()
            .map(|chunk| {
                let score = chunk.content.to_lowercase()
                    .split_whitespace()
                    .filter(|word| query_lower.contains(word))
                    .count();
                (chunk, score)
            })
            .filter(|(_, score)| *score > 0)
            .collect();
        
        scored.sort_by(|a, b| b.1.cmp(&a.1));
        scored.into_iter()
            .take(limit)
            .map(|(chunk, _)| chunk)
            .collect()
    }

    /// Get chunks by source
    pub fn get_by_source(&self, source: &str) -> Vec<&DocumentChunk> {
        self.source_index
            .get(source)
            .map(|indices| indices.iter().map(|&idx| &self.chunks[idx]).collect())
            .unwrap_or_else(Vec::new)
    }

    /// Load documentation into index
    /// Source: Athenos_AI_Strategy.md#L114
    pub fn load_documentation(&mut self, source: &str, content: &str) {
        info!("RAGIndex::load_documentation: Loading documentation from {}", source);
        
        // Split into chunks (simplified: 500 char chunks)
        let chunk_size = 500;
        let mut chunk_id = 0;
        
        for (i, chunk_text) in content.as_bytes().chunks(chunk_size).enumerate() {
            let chunk = DocumentChunk {
                id: format!("{}_{}", source, i),
                content: String::from_utf8_lossy(chunk_text).to_string(),
                source: source.to_string(),
                embedding: vec![0.0; 128], // Placeholder
                metadata: HashMap::new(),
            };
            
            self.index_chunk(chunk);
            chunk_id = i;
        }
    }
}

impl Default for RAGIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rag_index_creation() {
        let index = RAGIndex::new();
        assert_eq!(index.chunks.len(), 0);
    }

    #[test]
    fn test_index_and_search() {
        let mut index = RAGIndex::new();
        
        let chunk = DocumentChunk {
            id: "doc1".to_string(),
            content: "Humans run on cognitive loops. Athenos reveals patterns.".to_string(),
            source: "strategy.md".to_string(),
            embedding: vec![0.0; 128],
            metadata: HashMap::new(),
        };
        
        index.index_chunk(chunk);
        
        let results = index.search("cognitive loops", 5);
        assert_eq!(results.len(), 1);
        assert!(results[0].content.contains("cognitive loops"));
    }

    #[test]
    fn test_load_documentation() {
        let mut index = RAGIndex::new();
        let content = "This is a test document. ".repeat(50); // ~1000 chars
        
        index.load_documentation("test.md", &content);
        
        let chunks = index.get_by_source("test.md");
        assert!(chunks.len() >= 2); // Should be split into multiple chunks
    }
}

