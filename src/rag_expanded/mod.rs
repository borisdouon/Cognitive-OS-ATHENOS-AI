/// Phase: D | Step: 2 | Source: Athenos_AI_Strategy.md#L133
/// Expanded RAG Corpus
/// Expand RAG corpus with industry-specific workflows; enable personalization

use crate::rag::RAGIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Industry workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryWorkflow {
    pub industry: String,
    pub workflow_name: String,
    pub steps: Vec<String>,
    pub best_practices: Vec<String>,
    pub common_pitfalls: Vec<String>,
}

/// Personalized RAG index
/// Source: Athenos_AI_Strategy.md#L133
pub struct ExpandedRAGIndex {
    base_index: RAGIndex,
    industry_workflows: HashMap<String, Vec<IndustryWorkflow>>,
    user_preferences: HashMap<String, Vec<String>>, // user_id -> preferred industries
}

impl ExpandedRAGIndex {
    /// Create new expanded RAG index
    pub fn new() -> Self {
        info!("ExpandedRAGIndex::new: Creating expanded RAG index");
        Self {
            base_index: RAGIndex::new(),
            industry_workflows: HashMap::new(),
            user_preferences: HashMap::new(),
        }
    }

    /// Add industry workflow
    /// Source: Athenos_AI_Strategy.md#L133
    pub fn add_industry_workflow(&mut self, workflow: IndustryWorkflow) {
        info!("ExpandedRAGIndex::add_industry_workflow: Adding workflow for {}", workflow.industry);
        self.industry_workflows
            .entry(workflow.industry.clone())
            .or_insert_with(Vec::new)
            .push(workflow);
    }

    /// Personalize search for user
    /// Source: Athenos_AI_Strategy.md#L133
    pub fn personalized_search(&self, user_id: &str, query: &str, limit: usize) -> Vec<String> {
        info!("ExpandedRAGIndex::personalized_search: Personalized search for user {}", user_id);
        
        // Get user preferences
        let preferred_industries = self.user_preferences
            .get(user_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        
        // Search base index
        let base_results: Vec<String> = self.base_index
            .search(query, limit)
            .iter()
            .map(|c| c.content.clone())
            .collect();
        
        // Add industry-specific results if user has preferences
        let mut results = base_results;
        for industry in preferred_industries {
            if let Some(workflows) = self.industry_workflows.get(*industry) {
                for workflow in workflows {
                    if query.to_lowercase().contains(&workflow.workflow_name.to_lowercase()) {
                        results.push(format!("Industry workflow: {} - {}", workflow.workflow_name, workflow.steps.join(" → ")));
                    }
                }
            }
        }
        
        results.into_iter().take(limit).collect()
    }

    /// Set user preferences
    pub fn set_user_preferences(&mut self, user_id: String, industries: Vec<String>) {
        info!("ExpandedRAGIndex::set_user_preferences: Setting preferences for user {}", user_id);
        self.user_preferences.insert(user_id, industries);
    }

    /// Get workflows for industry
    pub fn get_industry_workflows(&self, industry: &str) -> Vec<&IndustryWorkflow> {
        self.industry_workflows
            .get(industry)
            .map(|v| v.iter().collect())
            .unwrap_or_else(Vec::new)
    }
}

impl Default for ExpandedRAGIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expanded_rag_creation() {
        let index = ExpandedRAGIndex::new();
        assert_eq!(index.industry_workflows.len(), 0);
    }

    #[test]
    fn test_add_industry_workflow() {
        let mut index = ExpandedRAGIndex::new();
        let workflow = IndustryWorkflow {
            industry: "software".to_string(),
            workflow_name: "Code Review".to_string(),
            steps: vec!["Review".to_string(), "Test".to_string(), "Merge".to_string()],
            best_practices: vec!["Check tests".to_string()],
            common_pitfalls: vec!["Skipping tests".to_string()],
        };
        
        index.add_industry_workflow(workflow);
        assert_eq!(index.industry_workflows.len(), 1);
    }

    #[test]
    fn test_personalized_search() {
        let mut index = ExpandedRAGIndex::new();
        index.set_user_preferences("user_001".to_string(), vec!["software".to_string()]);
        
        let workflow = IndustryWorkflow {
            industry: "software".to_string(),
            workflow_name: "Code Review".to_string(),
            steps: vec!["Review".to_string()],
            best_practices: vec![],
            common_pitfalls: vec![],
        };
        index.add_industry_workflow(workflow);
        
        let results = index.personalized_search("user_001", "code review", 5);
        assert!(!results.is_empty());
    }
}

