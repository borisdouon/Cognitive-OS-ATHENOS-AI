/// Phase: D | Step: 10 | Source: Athenos_AI_Strategy.md#L141
/// Public Launch Preparation
/// Prepare for public launch: marketing narrative, onboarding playbook, support ops

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Marketing narrative
/// Source: Athenos_AI_Strategy.md#L141
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketingNarrative {
    pub tagline: String,
    pub value_proposition: String,
    pub key_features: Vec<String>,
    pub target_audience: Vec<String>,
}

/// Onboarding step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingStep {
    pub step_number: usize,
    pub title: String,
    pub description: String,
    pub required: bool,
}

/// Onboarding playbook
/// Source: Athenos_AI_Strategy.md#L141
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingPlaybook {
    pub steps: Vec<OnboardingStep>,
    pub estimated_duration_min: usize,
}

/// Support ticket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportTicket {
    pub id: String,
    pub user_id: String,
    pub category: SupportCategory,
    pub description: String,
    pub status: TicketStatus,
    pub created_at: i64,
}

/// Support category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SupportCategory {
    Technical,
    Billing,
    FeatureRequest,
    BugReport,
    General,
}

/// Ticket status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TicketStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

/// Public launch manager
/// Source: Athenos_AI_Strategy.md#L141
pub struct PublicLaunchManager {
    marketing_narrative: Option<MarketingNarrative>,
    onboarding_playbook: Option<OnboardingPlaybook>,
    support_tickets: HashMap<String, SupportTicket>,
}

impl PublicLaunchManager {
    /// Create new public launch manager
    pub fn new() -> Self {
        info!("PublicLaunchManager::new: Creating public launch manager");
        Self {
            marketing_narrative: None,
            onboarding_playbook: None,
            support_tickets: HashMap::new(),
        }
    }

    /// Set marketing narrative
    /// Source: Athenos_AI_Strategy.md#L141
    pub fn set_marketing_narrative(&mut self, narrative: MarketingNarrative) {
        info!("PublicLaunchManager::set_marketing_narrative: Setting marketing narrative");
        self.marketing_narrative = Some(narrative);
    }

    /// Set onboarding playbook
    /// Source: Athenos_AI_Strategy.md#L141
    pub fn set_onboarding_playbook(&mut self, playbook: OnboardingPlaybook) {
        info!("PublicLaunchManager::set_onboarding_playbook: Setting onboarding playbook");
        self.onboarding_playbook = Some(playbook);
    }

    /// Create support ticket
    pub fn create_support_ticket(&mut self, user_id: String, category: SupportCategory, description: String) -> SupportTicket {
        info!("PublicLaunchManager::create_support_ticket: Creating support ticket");
        
        let ticket = SupportTicket {
            id: format!("ticket_{}", chrono::Utc::now().timestamp()),
            user_id,
            category,
            description,
            status: TicketStatus::Open,
            created_at: chrono::Utc::now().timestamp(),
        };
        
        self.support_tickets.insert(ticket.id.clone(), ticket.clone());
        ticket
    }

    /// Get launch readiness checklist
    pub fn get_readiness_checklist(&self) -> LaunchReadiness {
        LaunchReadiness {
            marketing_narrative_ready: self.marketing_narrative.is_some(),
            onboarding_playbook_ready: self.onboarding_playbook.is_some(),
            support_ops_ready: true, // Stub
            overall_ready: self.marketing_narrative.is_some() && self.onboarding_playbook.is_some(),
        }
    }
}

/// Launch readiness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchReadiness {
    pub marketing_narrative_ready: bool,
    pub onboarding_playbook_ready: bool,
    pub support_ops_ready: bool,
    pub overall_ready: bool,
}

impl Default for PublicLaunchManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launch_manager_creation() {
        let manager = PublicLaunchManager::new();
        assert!(manager.marketing_narrative.is_none());
    }

    #[test]
    fn test_set_marketing_narrative() {
        let mut manager = PublicLaunchManager::new();
        let narrative = MarketingNarrative {
            tagline: "Upgrade your mind. Not just your machine.".to_string(),
            value_proposition: "Cognitive operating system".to_string(),
            key_features: vec!["Pattern detection".to_string()],
            target_audience: vec!["Developers".to_string()],
        };
        
        manager.set_marketing_narrative(narrative);
        assert!(manager.marketing_narrative.is_some());
    }

    #[test]
    fn test_create_support_ticket() {
        let mut manager = PublicLaunchManager::new();
        let ticket = manager.create_support_ticket(
            "user_001".to_string(),
            SupportCategory::Technical,
            "Need help".to_string(),
        );
        
        assert_eq!(ticket.status, TicketStatus::Open);
        assert_eq!(manager.support_tickets.len(), 1);
    }

    #[test]
    fn test_readiness_checklist() {
        let mut manager = PublicLaunchManager::new();
        let checklist = manager.get_readiness_checklist();
        assert!(!checklist.overall_ready);
        
        manager.set_marketing_narrative(MarketingNarrative {
            tagline: "Test".to_string(),
            value_proposition: "Test".to_string(),
            key_features: vec![],
            target_audience: vec![],
        });
        
        manager.set_onboarding_playbook(OnboardingPlaybook {
            steps: vec![],
            estimated_duration_min: 5,
        });
        
        let checklist = manager.get_readiness_checklist();
        assert!(checklist.overall_ready);
    }
}

