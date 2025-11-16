/// Phase: A | Step: 6 | Source: Athenos_AI_Strategy.md#L101
/// Local Cognitive Stack - Feature pipeline
/// Temporal metrics, embeddings, affect signals

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Temporal metrics extracted from events
/// Source: Athenos_AI_Strategy.md#L24
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalMetrics {
    pub time_to_first_action_min: f64,
    pub focus_duration_min: f64,
    pub context_switch_count: usize,
    pub repeat_count: usize,
    pub session_duration_min: f64,
}

/// Feature store for cognitive analysis
/// Source: Athenos_AI_Strategy.md#L24
pub struct FeatureStore {
    metrics: HashMap<String, TemporalMetrics>,
    embeddings: HashMap<String, Vec<f32>>, // Simple embedding storage
}

impl FeatureStore {
    /// Create new feature store
    pub fn new() -> Self {
        info!("FeatureStore::new: Creating feature store");
        Self {
            metrics: HashMap::new(),
            embeddings: HashMap::new(),
        }
    }

    /// Store temporal metrics for an observation
    pub fn store_metrics(&mut self, observation_id: String, metrics: TemporalMetrics) {
        info!("FeatureStore::store_metrics: Storing metrics for {}", observation_id);
        self.metrics.insert(observation_id, metrics);
    }

    /// Get metrics for an observation
    pub fn get_metrics(&self, observation_id: &str) -> Option<&TemporalMetrics> {
        self.metrics.get(observation_id)
    }

    /// Store embedding vector
    pub fn store_embedding(&mut self, observation_id: String, embedding: Vec<f32>) {
        info!("FeatureStore::store_embedding: Storing embedding for {} (dim={})", observation_id, embedding.len());
        self.embeddings.insert(observation_id, embedding);
    }

    /// Get embedding vector
    pub fn get_embedding(&self, observation_id: &str) -> Option<&Vec<f32>> {
        self.embeddings.get(observation_id)
    }

    /// Compute focus stability percentage from metrics
    /// Source: Strategic_Reinforcements_Gap_Closures.md#L25
    pub fn compute_focus_stability(&self, observation_ids: &[String]) -> f64 {
        if observation_ids.is_empty() {
            return 0.0;
        }

        let total_focus_time: f64 = observation_ids
            .iter()
            .filter_map(|id| self.metrics.get(id))
            .map(|m| m.focus_duration_min)
            .sum();

        let total_session_time: f64 = observation_ids
            .iter()
            .filter_map(|id| self.metrics.get(id))
            .map(|m| m.session_duration_min)
            .sum();

        if total_session_time > 0.0 {
            (total_focus_time / total_session_time) * 100.0
        } else {
            0.0
        }
    }
}

impl Default for FeatureStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_store_creation() {
        let store = FeatureStore::new();
        assert_eq!(store.metrics.len(), 0);
        assert_eq!(store.embeddings.len(), 0);
    }

    #[test]
    fn test_store_and_retrieve_metrics() {
        let mut store = FeatureStore::new();
        let metrics = TemporalMetrics {
            time_to_first_action_min: 12.0,
            focus_duration_min: 45.0,
            context_switch_count: 8,
            repeat_count: 5,
            session_duration_min: 120.0,
        };
        
        store.store_metrics("obs_001".to_string(), metrics.clone());
        let retrieved = store.get_metrics("obs_001").unwrap();
        
        assert_eq!(retrieved.time_to_first_action_min, 12.0);
        assert_eq!(retrieved.focus_duration_min, 45.0);
    }

    #[test]
    fn test_focus_stability_computation() {
        let mut store = FeatureStore::new();
        
        store.store_metrics("obs_001".to_string(), TemporalMetrics {
            time_to_first_action_min: 5.0,
            focus_duration_min: 60.0,
            context_switch_count: 2,
            repeat_count: 1,
            session_duration_min: 90.0,
        });
        
        store.store_metrics("obs_002".to_string(), TemporalMetrics {
            time_to_first_action_min: 3.0,
            focus_duration_min: 45.0,
            context_switch_count: 1,
            repeat_count: 1,
            session_duration_min: 60.0,
        });
        
        let stability = store.compute_focus_stability(&["obs_001".to_string(), "obs_002".to_string()]);
        // (60 + 45) / (90 + 60) * 100 = 70.0
        assert!((stability - 70.0).abs() < 0.1);
    }
}

