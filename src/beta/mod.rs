/// Phase: C | Step: 10 | Source: Athenos_AI_Strategy.md#L129
/// Beta User Onboarding
/// Onboard 500 beta users, gather structured feedback, iterate

use crate::types::*;
use crate::cohort::{CohortManager, CohortMember};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Beta user feedback
/// Source: Athenos_AI_Strategy.md#L129
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaFeedback {
    pub user_id: String,
    pub feedback_type: FeedbackType,
    pub content: String,
    pub rating: Option<u8>, // 1-10
    pub timestamp: i64,
}

/// Feedback type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeedbackType {
    FeatureRequest,
    BugReport,
    UsabilityIssue,
    PositiveFeedback,
    General,
}

/// Beta onboarding manager
/// Source: Athenos_AI_Strategy.md#L129
pub struct BetaOnboardingManager {
    cohort_manager: CohortManager,
    feedback: Vec<BetaFeedback>,
    onboarding_complete: HashMap<String, bool>,
}

impl BetaOnboardingManager {
    /// Create new beta onboarding manager
    pub fn new() -> Self {
        info!("BetaOnboardingManager::new: Creating beta onboarding manager");
        Self {
            cohort_manager: CohortManager::new(500),
            feedback: Vec::new(),
            onboarding_complete: HashMap::new(),
        }
    }

    /// Onboard beta user
    /// Source: Athenos_AI_Strategy.md#L129
    pub fn onboard_user(&mut self, user_id: String, profile: UserProfile) {
        info!("BetaOnboardingManager::onboard_user: Onboarding user {}", user_id);
        self.cohort_manager.add_member(user_id.clone(), profile);
        self.onboarding_complete.insert(user_id, true);
    }

    /// Simulate onboarding 500 beta users
    /// Source: Athenos_AI_Strategy.md#L129
    pub fn simulate_beta_onboarding(&mut self) {
        info!("BetaOnboardingManager::simulate_beta_onboarding: Simulating 500 beta users");
        let profiles = vec![UserProfile::Developer, UserProfile::Accountant, UserProfile::Designer];
        self.cohort_manager.simulate_expansion(&profiles);
    }

    /// Collect feedback from beta user
    /// Source: Athenos_AI_Strategy.md#L129
    pub fn collect_feedback(&mut self, user_id: String, feedback_type: FeedbackType, content: String, rating: Option<u8>) {
        info!("BetaOnboardingManager::collect_feedback: Collecting feedback from {}", user_id);
        
        let feedback = BetaFeedback {
            user_id,
            feedback_type,
            content,
            rating,
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        self.feedback.push(feedback);
    }

    /// Get feedback summary
    pub fn get_feedback_summary(&self) -> FeedbackSummary {
        let total_feedback = self.feedback.len();
        let avg_rating = if total_feedback > 0 {
            self.feedback
                .iter()
                .filter_map(|f| f.rating)
                .sum::<u8>() as f64 / total_feedback as f64
        } else {
            0.0
        };
        
        let feedback_by_type: HashMap<String, usize> = self.feedback
            .iter()
            .map(|f| (format!("{:?}", f.feedback_type), 1))
            .fold(HashMap::new(), |mut acc, (k, v)| {
                *acc.entry(k).or_insert(0) += v;
                acc
            });
        
        FeedbackSummary {
            total_feedback,
            avg_rating,
            feedback_by_type,
            total_beta_users: self.cohort_manager.get_statistics().total_members,
        }
    }

    /// Get cohort statistics
    pub fn get_cohort_stats(&self) -> crate::cohort::CohortStatistics {
        self.cohort_manager.get_statistics()
    }
}

/// Feedback summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackSummary {
    pub total_feedback: usize,
    pub avg_rating: f64,
    pub feedback_by_type: HashMap<String, usize>,
    pub total_beta_users: usize,
}

impl Default for BetaOnboardingManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beta_onboarding_manager_creation() {
        let manager = BetaOnboardingManager::new();
        assert_eq!(manager.feedback.len(), 0);
    }

    #[test]
    fn test_onboard_user() {
        let mut manager = BetaOnboardingManager::new();
        manager.onboard_user("beta_001".to_string(), UserProfile::Developer);
        
        let stats = manager.get_cohort_stats();
        assert_eq!(stats.total_members, 1);
    }

    #[test]
    fn test_simulate_beta_onboarding() {
        let mut manager = BetaOnboardingManager::new();
        manager.simulate_beta_onboarding();
        
        let stats = manager.get_cohort_stats();
        assert_eq!(stats.total_members, 500);
    }

    #[test]
    fn test_collect_feedback() {
        let mut manager = BetaOnboardingManager::new();
        manager.collect_feedback(
            "beta_001".to_string(),
            FeedbackType::PositiveFeedback,
            "Great product!".to_string(),
            Some(9),
        );
        
        assert_eq!(manager.feedback.len(), 1);
        let summary = manager.get_feedback_summary();
        assert_eq!(summary.total_feedback, 1);
        assert_eq!(summary.avg_rating, 9.0);
    }
}

