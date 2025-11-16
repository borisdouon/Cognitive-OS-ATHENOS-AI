/// Phase: D | Step: 8 | Source: Athenos_AI_Strategy.md#L139
/// Knowledge Expansion Loop
/// Build knowledge expansion loop ingesting new research automatically

use crate::rag_expanded::ExpandedRAGIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Research document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchDocument {
    pub id: String,
    pub title: String,
    pub content: String,
    pub source: String,
    pub published_at: i64,
    pub tags: Vec<String>,
}

/// Knowledge expansion loop
/// Source: Athenos_AI_Strategy.md#L139
pub struct KnowledgeExpansionLoop {
    rag_index: ExpandedRAGIndex,
    ingested_documents: HashMap<String, ResearchDocument>,
    ingestion_schedule: Vec<i64>, // Timestamps for scheduled ingestions
}

impl KnowledgeExpansionLoop {
    /// Create new knowledge expansion loop
    pub fn new() -> Self {
        info!("KnowledgeExpansionLoop::new: Creating knowledge expansion loop");
        Self {
            rag_index: ExpandedRAGIndex::new(),
            ingested_documents: HashMap::new(),
            ingestion_schedule: Vec::new(),
        }
    }

    /// Ingest research document automatically
    /// Source: Athenos_AI_Strategy.md#L139
    pub fn ingest_research(&mut self, document: ResearchDocument) {
        info!("KnowledgeExpansionLoop::ingest_research: Ingesting research document {}", document.id);
        
        // Index document in RAG
        self.rag_index.base_index.load_documentation(&document.source, &document.content);
        
        // Store document
        self.ingested_documents.insert(document.id.clone(), document);
    }

    /// Schedule automatic ingestion
    pub fn schedule_ingestion(&mut self, timestamp: i64) {
        info!("KnowledgeExpansionLoop::schedule_ingestion: Scheduling ingestion at {}", timestamp);
        self.ingestion_schedule.push(timestamp);
    }

    /// Process scheduled ingestions (stub for cron-like execution)
    pub fn process_scheduled(&mut self, current_time: i64) -> Vec<String> {
        let mut processed = Vec::new();
        
        self.ingestion_schedule.retain(|&scheduled_time| {
            if scheduled_time <= current_time {
                processed.push(format!("Processing scheduled ingestion at {}", scheduled_time));
                false // Remove from schedule
            } else {
                true // Keep in schedule
            }
        });
        
        processed
    }

    /// Search knowledge base
    pub fn search_knowledge(&self, query: &str, limit: usize) -> Vec<String> {
        self.rag_index.base_index.search(query, limit)
            .iter()
            .map(|c| c.content.clone())
            .collect()
    }

    /// Get ingestion statistics
    pub fn get_statistics(&self) -> KnowledgeStatistics {
        KnowledgeStatistics {
            total_documents: self.ingested_documents.len(),
            scheduled_ingestions: self.ingestion_schedule.len(),
        }
    }
}

/// Knowledge statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeStatistics {
    pub total_documents: usize,
    pub scheduled_ingestions: usize,
}

impl Default for KnowledgeExpansionLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_loop_creation() {
        let loop_ref = KnowledgeExpansionLoop::new();
        assert_eq!(loop_ref.ingested_documents.len(), 0);
    }

    #[test]
    fn test_ingest_research() {
        let mut loop_ref = KnowledgeExpansionLoop::new();
        let document = ResearchDocument {
            id: "doc_001".to_string(),
            title: "Cognitive Science Research".to_string(),
            content: "Research content here".to_string(),
            source: "journal.md".to_string(),
            published_at: 1234567890,
            tags: vec!["cognitive".to_string(), "science".to_string()],
        };
        
        loop_ref.ingest_research(document);
        assert_eq!(loop_ref.ingested_documents.len(), 1);
    }

    #[test]
    fn test_process_scheduled() {
        let mut loop_ref = KnowledgeExpansionLoop::new();
        loop_ref.schedule_ingestion(1000);
        loop_ref.schedule_ingestion(2000);
        
        let processed = loop_ref.process_scheduled(1500);
        assert_eq!(processed.len(), 1); // Only first one should be processed
        assert_eq!(loop_ref.ingestion_schedule.len(), 1); // One remaining
    }
}

