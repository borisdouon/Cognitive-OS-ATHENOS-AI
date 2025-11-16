/// Phase: B | Step: 4 | Source: Athenos_AI_Strategy.md#L111
/// Predictive Shortcut Generator with Manual Approval
/// Launch predictive shortcut generator with manual approval workflow

use crate::types::*;
use crate::models::RecommendationRanker;
use crate::pattern_miner::PatternMiner;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Shortcut proposal awaiting approval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutProposal {
    pub id: String,
    pub description: String,
    pub sequence: Vec<String>,
    pub expected_time_saved_min: f64,
    pub confidence: Confidence,
    pub risk: RiskCategory,
    pub requires_approval: bool,
    pub created_at: i64,
}

/// Approval status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    Modified,
}

/// Shortcut generator with manual approval workflow
/// Source: Athenos_AI_Strategy.md#L111
pub struct ShortcutGenerator {
    ranker: RecommendationRanker,
    pattern_miner: PatternMiner,
    proposals: HashMap<String, ShortcutProposal>,
    approvals: HashMap<String, ApprovalStatus>,
}

impl ShortcutGenerator {
    /// Create new shortcut generator
    pub fn new() -> Self {
        info!("ShortcutGenerator::new: Creating shortcut generator");
        Self {
            ranker: RecommendationRanker::new(),
            pattern_miner: PatternMiner::new(),
            proposals: HashMap::new(),
            approvals: HashMap::new(),
        }
    }

    /// Generate predictive shortcut from observation
    /// Source: Athenos_AI_Strategy.md#L111
    pub fn generate_shortcut(&mut self, observation: &Observation) -> Option<ShortcutProposal> {
        info!("ShortcutGenerator::generate_shortcut: Generating shortcut for {}", observation.id);
        
        // Check if pattern suggests shortcut creation
        if observation.observation.len() < 3 {
            return None;
        }
        
        let repeat_count = observation.metrics.get("repeat_count").copied().unwrap_or(0.0);
        if repeat_count < 5.0 {
            return None; // Not enough repetition
        }
        
        let expected_saved = observation.expected_outcome.get("time_saved_min").copied().unwrap_or(0.0);
        
        let proposal = ShortcutProposal {
            id: format!("shortcut_{}", observation.id),
            description: format!("Automate sequence: {}", observation.observation.join(" → ")),
            sequence: observation.observation.clone(),
            expected_time_saved_min: expected_saved,
            confidence: observation.action.confidence.clone(),
            risk: observation.action.risk.clone(),
            requires_approval: observation.action.risk != RiskCategory::None || observation.action.confidence < Confidence::High,
            created_at: chrono::Utc::now().timestamp(),
        };
        
        self.proposals.insert(proposal.id.clone(), proposal.clone());
        self.approvals.insert(proposal.id.clone(), ApprovalStatus::Pending);
        
        Some(proposal)
    }

    /// Approve shortcut proposal
    pub fn approve_shortcut(&mut self, shortcut_id: &str) -> Result<(), String> {
        info!("ShortcutGenerator::approve_shortcut: Approving {}", shortcut_id);
        if let Some(status) = self.approvals.get_mut(shortcut_id) {
            *status = ApprovalStatus::Approved;
            Ok(())
        } else {
            Err("Shortcut not found".to_string())
        }
    }

    /// Reject shortcut proposal
    pub fn reject_shortcut(&mut self, shortcut_id: &str) -> Result<(), String> {
        info!("ShortcutGenerator::reject_shortcut: Rejecting {}", shortcut_id);
        if let Some(status) = self.approvals.get_mut(shortcut_id) {
            *status = ApprovalStatus::Rejected;
            Ok(())
        } else {
            Err("Shortcut not found".to_string())
        }
    }

    /// Get pending proposals requiring approval
    pub fn get_pending_proposals(&self) -> Vec<&ShortcutProposal> {
        self.proposals
            .values()
            .filter(|p| {
                self.approvals.get(&p.id) == Some(&ApprovalStatus::Pending) && p.requires_approval
            })
            .collect()
    }

    /// Get approved shortcuts ready for execution
    pub fn get_approved_shortcuts(&self) -> Vec<&ShortcutProposal> {
        self.proposals
            .values()
            .filter(|p| self.approvals.get(&p.id) == Some(&ApprovalStatus::Approved))
            .collect()
    }
}

impl Default for ShortcutGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_shortcut_generator_creation() {
        let generator = ShortcutGenerator::new();
        assert_eq!(generator.proposals.len(), 0);
    }

    #[test]
    fn test_generate_shortcut_sufficient_repetition() {
        let mut generator = ShortcutGenerator::new();
        let mut metrics = HashMap::new();
        metrics.insert("repeat_count".to_string(), 8.0);
        let mut expected = HashMap::new();
        expected.insert("time_saved_min".to_string(), 11.0);
        
        let observation = Observation {
            id: "test_001".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["Teams".to_string(), "Gmail".to_string(), "IDE".to_string()],
            metrics,
            intent: Intent::SuggestShortcut,
            action: Action {
                action_type: ActionType::AutomationMacro,
                description: "Test macro".to_string(),
                confidence: Confidence::High,
                risk: RiskCategory::None,
            },
            expected_outcome: expected,
            source: "test".to_string(),
            timestamp: 1234567890,
        };
        
        let proposal = generator.generate_shortcut(&observation);
        assert!(proposal.is_some());
        let proposal = proposal.unwrap();
        assert_eq!(proposal.sequence.len(), 3);
        assert_eq!(proposal.expected_time_saved_min, 11.0);
    }

    #[test]
    fn test_generate_shortcut_insufficient_repetition() {
        let mut generator = ShortcutGenerator::new();
        let mut metrics = HashMap::new();
        metrics.insert("repeat_count".to_string(), 2.0);
        
        let observation = Observation {
            id: "test_002".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["Teams".to_string(), "Gmail".to_string()],
            metrics,
            intent: Intent::SuggestShortcut,
            action: Action {
                action_type: ActionType::AutomationMacro,
                description: "Test".to_string(),
                confidence: Confidence::High,
                risk: RiskCategory::None,
            },
            expected_outcome: HashMap::new(),
            source: "test".to_string(),
            timestamp: 1234567890,
        };
        
        let proposal = generator.generate_shortcut(&observation);
        assert!(proposal.is_none());
    }

    #[test]
    fn test_approval_workflow() {
        let mut generator = ShortcutGenerator::new();
        let mut metrics = HashMap::new();
        metrics.insert("repeat_count".to_string(), 8.0);
        let mut expected = HashMap::new();
        expected.insert("time_saved_min".to_string(), 11.0);
        
        let observation = Observation {
            id: "test_003".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["Teams".to_string(), "Gmail".to_string(), "IDE".to_string()],
            metrics,
            intent: Intent::SuggestShortcut,
            action: Action {
                action_type: ActionType::AutomationMacro,
                description: "Test".to_string(),
                confidence: Confidence::Medium,
                risk: RiskCategory::Low,
            },
            expected_outcome: expected,
            source: "test".to_string(),
            timestamp: 1234567890,
        };
        
        let proposal = generator.generate_shortcut(&observation).unwrap();
        assert_eq!(generator.approvals.get(&proposal.id), Some(&ApprovalStatus::Pending));
        
        generator.approve_shortcut(&proposal.id).unwrap();
        assert_eq!(generator.approvals.get(&proposal.id), Some(&ApprovalStatus::Approved));
        
        let approved = generator.get_approved_shortcuts();
        assert_eq!(approved.len(), 1);
    }
}

