/// Phase: D | Step: 7 | Source: Athenos_AI_Strategy.md#L138
/// Multi-Region Scale
/// Scale infrastructure multi-region with latency-aware orchestration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Region configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub latency_ms: u64,
    pub active: bool,
}

/// Latency-aware orchestrator
/// Source: Athenos_AI_Strategy.md#L138
pub struct MultiRegionOrchestrator {
    regions: HashMap<String, Region>,
    user_regions: HashMap<String, String>, // user_id -> region_id
}

impl MultiRegionOrchestrator {
    /// Create new multi-region orchestrator
    pub fn new() -> Self {
        info!("MultiRegionOrchestrator::new: Creating multi-region orchestrator");
        Self {
            regions: HashMap::new(),
            user_regions: HashMap::new(),
        }
    }

    /// Add region
    /// Source: Athenos_AI_Strategy.md#L138
    pub fn add_region(&mut self, region: Region) {
        info!("MultiRegionOrchestrator::add_region: Adding region {}", region.id);
        self.regions.insert(region.id.clone(), region);
    }

    /// Select best region for user based on latency
    /// Source: Athenos_AI_Strategy.md#L138
    pub fn select_best_region(&self, user_id: &str) -> Option<&Region> {
        info!("MultiRegionOrchestrator::select_best_region: Selecting region for user {}", user_id);
        
        // Check if user already has assigned region
        if let Some(region_id) = self.user_regions.get(user_id) {
            return self.regions.get(region_id);
        }
        
        // Select region with lowest latency
        self.regions
            .values()
            .filter(|r| r.active)
            .min_by_key(|r| r.latency_ms)
    }

    /// Assign user to region
    pub fn assign_user_to_region(&mut self, user_id: String, region_id: String) {
        info!("MultiRegionOrchestrator::assign_user_to_region: Assigning user {} to {}", user_id, region_id);
        self.user_regions.insert(user_id, region_id);
    }

    /// Get user's region
    pub fn get_user_region(&self, user_id: &str) -> Option<&Region> {
        self.user_regions
            .get(user_id)
            .and_then(|rid| self.regions.get(rid))
    }

    /// Get all active regions
    pub fn get_active_regions(&self) -> Vec<&Region> {
        self.regions.values().filter(|r| r.active).collect()
    }
}

impl Default for MultiRegionOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrator_creation() {
        let orchestrator = MultiRegionOrchestrator::new();
        assert_eq!(orchestrator.regions.len(), 0);
    }

    #[test]
    fn test_add_region_and_select() {
        let mut orchestrator = MultiRegionOrchestrator::new();
        
        orchestrator.add_region(Region {
            id: "us-east".to_string(),
            name: "US East".to_string(),
            endpoint: "https://us-east.athenos.ai".to_string(),
            latency_ms: 50,
            active: true,
        });
        
        orchestrator.add_region(Region {
            id: "eu-west".to_string(),
            name: "EU West".to_string(),
            endpoint: "https://eu-west.athenos.ai".to_string(),
            latency_ms: 100,
            active: true,
        });
        
        let best = orchestrator.select_best_region("user_001");
        assert!(best.is_some());
        assert_eq!(best.unwrap().id, "us-east"); // Lower latency
    }

    #[test]
    fn test_assign_user_to_region() {
        let mut orchestrator = MultiRegionOrchestrator::new();
        orchestrator.add_region(Region {
            id: "us-east".to_string(),
            name: "US East".to_string(),
            endpoint: "https://us-east.athenos.ai".to_string(),
            latency_ms: 50,
            active: true,
        });
        
        orchestrator.assign_user_to_region("user_001".to_string(), "us-east".to_string());
        let region = orchestrator.get_user_region("user_001");
        assert!(region.is_some());
        assert_eq!(region.unwrap().id, "us-east");
    }
}

