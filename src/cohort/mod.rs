/// Phase: B | Step: 10 | Source: Athenos_AI_Strategy.md#L117
/// Expand Cohort to 200 Users
/// Expand cohort to 200 users, capture intervention acceptance data

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// User cohort member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortMember {
    pub user_id: String,
    pub profile: UserProfile,
    pub joined_at: i64,
    pub observations_count: usize,
    pub interventions_accepted: usize,
    pub interventions_rejected: usize,
    pub total_time_saved_min: f64,
}

/// Cohort manager for alpha/beta testing
/// Source: Athenos_AI_Strategy.md#L117
pub struct CohortManager {
    members: HashMap<String, CohortMember>,
    target_size: usize,
}

impl CohortManager {
    /// Create new cohort manager
    pub fn new(target_size: usize) -> Self {
        info!("CohortManager::new: Creating cohort manager with target size {}", target_size);
        Self {
            members: HashMap::new(),
            target_size,
        }
    }

    /// Add user to cohort
    /// Source: Athenos_AI_Strategy.md#L117
    pub fn add_member(&mut self, user_id: String, profile: UserProfile) {
        info!("CohortManager::add_member: Adding user {} to cohort", user_id);
        let member = CohortMember {
            user_id: user_id.clone(),
            profile,
            joined_at: chrono::Utc::now().timestamp(),
            observations_count: 0,
            interventions_accepted: 0,
            interventions_rejected: 0,
            total_time_saved_min: 0.0,
        };
        self.members.insert(user_id, member);
    }

    /// Record intervention outcome
    pub fn record_intervention(&mut self, user_id: &str, accepted: bool, time_saved_min: f64) {
        if let Some(member) = self.members.get_mut(user_id) {
            if accepted {
                member.interventions_accepted += 1;
                member.total_time_saved_min += time_saved_min;
            } else {
                member.interventions_rejected += 1;
            }
        }
    }

    /// Record observation
    pub fn record_observation(&mut self, user_id: &str) {
        if let Some(member) = self.members.get_mut(user_id) {
            member.observations_count += 1;
        }
    }

    /// Get cohort statistics
    pub fn get_statistics(&self) -> CohortStatistics {
        let total_members = self.members.len();
        let total_observations: usize = self.members.values().map(|m| m.observations_count).sum();
        let total_accepted: usize = self.members.values().map(|m| m.interventions_accepted).sum();
        let total_rejected: usize = self.members.values().map(|m| m.interventions_rejected).sum();
        let total_time_saved: f64 = self.members.values().map(|m| m.total_time_saved_min).sum();
        
        let acceptance_rate = if total_accepted + total_rejected > 0 {
            total_accepted as f64 / (total_accepted + total_rejected) as f64
        } else {
            0.0
        };
        
        CohortStatistics {
            total_members,
            target_size: self.target_size,
            total_observations,
            total_interventions: total_accepted + total_rejected,
            acceptance_rate,
            total_time_saved_min: total_time_saved,
            avg_time_saved_per_user: if total_members > 0 { total_time_saved / total_members as f64 } else { 0.0 },
        }
    }

    /// Simulate cohort expansion to target size
    /// Source: Athenos_AI_Strategy.md#L117
    pub fn simulate_expansion(&mut self, profiles: &[UserProfile]) {
        info!("CohortManager::simulate_expansion: Simulating expansion to {} users", self.target_size);
        
        let mut user_id = self.members.len();
        while self.members.len() < self.target_size {
            let profile = profiles[user_id % profiles.len()].clone();
            self.add_member(format!("user_{:03}", user_id), profile);
            user_id += 1;
        }
    }
}

/// Cohort statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortStatistics {
    pub total_members: usize,
    pub target_size: usize,
    pub total_observations: usize,
    pub total_interventions: usize,
    pub acceptance_rate: f64,
    pub total_time_saved_min: f64,
    pub avg_time_saved_per_user: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cohort_manager_creation() {
        let manager = CohortManager::new(200);
        assert_eq!(manager.target_size, 200);
        assert_eq!(manager.members.len(), 0);
    }

    #[test]
    fn test_add_member() {
        let mut manager = CohortManager::new(200);
        manager.add_member("user_001".to_string(), UserProfile::Developer);
        
        assert_eq!(manager.members.len(), 1);
        assert!(manager.members.contains_key("user_001"));
    }

    #[test]
    fn test_record_intervention() {
        let mut manager = CohortManager::new(200);
        manager.add_member("user_001".to_string(), UserProfile::Developer);
        
        manager.record_intervention("user_001", true, 11.0);
        manager.record_intervention("user_001", false, 0.0);
        
        let member = manager.members.get("user_001").unwrap();
        assert_eq!(member.interventions_accepted, 1);
        assert_eq!(member.interventions_rejected, 1);
        assert_eq!(member.total_time_saved_min, 11.0);
    }

    #[test]
    fn test_simulate_expansion() {
        let mut manager = CohortManager::new(200);
        let profiles = vec![UserProfile::Developer, UserProfile::Accountant, UserProfile::Designer];
        
        manager.simulate_expansion(&profiles);
        
        assert_eq!(manager.members.len(), 200);
        let stats = manager.get_statistics();
        assert_eq!(stats.total_members, 200);
        assert_eq!(stats.target_size, 200);
    }

    #[test]
    fn test_cohort_statistics() {
        let mut manager = CohortManager::new(200);
        manager.add_member("user_001".to_string(), UserProfile::Developer);
        manager.add_member("user_002".to_string(), UserProfile::Accountant);
        
        manager.record_intervention("user_001", true, 11.0);
        manager.record_intervention("user_001", true, 5.0);
        manager.record_intervention("user_002", false, 0.0);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_members, 2);
        assert_eq!(stats.total_interventions, 3);
        assert_eq!(stats.acceptance_rate, 2.0 / 3.0);
        assert_eq!(stats.total_time_saved_min, 16.0);
    }
}

