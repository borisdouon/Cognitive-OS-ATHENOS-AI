/// Phase: D | Step: 5 | Source: Athenos_AI_Strategy.md#L136
/// Enterprise Admin Console
/// Ship enterprise admin console (compliance, team insights, policy controls)

use crate::types::*;
use crate::analytics::AnalyticsAggregator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Team member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub user_id: String,
    pub name: String,
    pub role: String,
    pub joined_at: i64,
}

/// Compliance policy
/// Source: Athenos_AI_Strategy.md#L136
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompliancePolicy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub rules: Vec<String>,
}

/// Enterprise admin console
/// Source: Athenos_AI_Strategy.md#L136
pub struct EnterpriseAdminConsole {
    teams: HashMap<String, Vec<TeamMember>>,
    compliance_policies: HashMap<String, CompliancePolicy>,
    analytics: AnalyticsAggregator,
    policy_controls: HashMap<String, bool>, // policy_id -> enabled
}

impl EnterpriseAdminConsole {
    /// Create new enterprise admin console
    pub fn new() -> Self {
        info!("EnterpriseAdminConsole::new: Creating enterprise admin console");
        Self {
            teams: HashMap::new(),
            compliance_policies: HashMap::new(),
            analytics: AnalyticsAggregator::new(),
            policy_controls: HashMap::new(),
        }
    }

    /// Add team member
    pub fn add_team_member(&mut self, team_id: String, member: TeamMember) {
        info!("EnterpriseAdminConsole::add_team_member: Adding member to team {}", team_id);
        self.teams
            .entry(team_id)
            .or_insert_with(Vec::new)
            .push(member);
    }

    /// Get team insights
    /// Source: Athenos_AI_Strategy.md#L136
    pub fn get_team_insights(&self, team_id: &str) -> TeamInsights {
        let members = self.teams.get(team_id).map(|v| v.len()).unwrap_or(0);
        
        TeamInsights {
            team_id: team_id.to_string(),
            total_members: members,
            avg_productivity: 0.75, // Placeholder
            compliance_score: 0.95, // Placeholder
        }
    }

    /// Add compliance policy
    /// Source: Athenos_AI_Strategy.md#L136
    pub fn add_compliance_policy(&mut self, policy: CompliancePolicy) {
        info!("EnterpriseAdminConsole::add_compliance_policy: Adding policy {}", policy.id);
        let policy_id = policy.id.clone();
        self.compliance_policies.insert(policy_id.clone(), policy.clone());
        self.policy_controls.insert(policy_id, policy.enabled);
    }

    /// Enable/disable policy control
    pub fn set_policy_control(&mut self, policy_id: &str, enabled: bool) {
        info!("EnterpriseAdminConsole::set_policy_control: Setting policy {} to {}", policy_id, enabled);
        self.policy_controls.insert(policy_id.to_string(), enabled);
        if let Some(policy) = self.compliance_policies.get_mut(policy_id) {
            policy.enabled = enabled;
        }
    }

    /// Get compliance report
    pub fn get_compliance_report(&self) -> ComplianceReport {
        let total_policies = self.compliance_policies.len();
        let enabled_policies = self.policy_controls.values().filter(|&&v| v).count();
        
        ComplianceReport {
            total_policies,
            enabled_policies,
            compliance_score: if total_policies > 0 {
                enabled_policies as f64 / total_policies as f64
            } else {
                1.0
            },
        }
    }
}

/// Team insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamInsights {
    pub team_id: String,
    pub total_members: usize,
    pub avg_productivity: f64,
    pub compliance_score: f64,
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub total_policies: usize,
    pub enabled_policies: usize,
    pub compliance_score: f64,
}

impl Default for EnterpriseAdminConsole {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enterprise_console_creation() {
        let console = EnterpriseAdminConsole::new();
        assert_eq!(console.teams.len(), 0);
    }

    #[test]
    fn test_add_team_member() {
        let mut console = EnterpriseAdminConsole::new();
        let member = TeamMember {
            user_id: "user_001".to_string(),
            name: "John Doe".to_string(),
            role: "Developer".to_string(),
            joined_at: 1234567890,
        };
        
        console.add_team_member("team_alpha".to_string(), member);
        assert_eq!(console.teams.get("team_alpha").unwrap().len(), 1);
    }

    #[test]
    fn test_compliance_policy() {
        let mut console = EnterpriseAdminConsole::new();
        let policy = CompliancePolicy {
            id: "policy_001".to_string(),
            name: "Data Retention".to_string(),
            description: "Retain data for 90 days".to_string(),
            enabled: true,
            rules: vec!["90_day_retention".to_string()],
        };
        
        console.add_compliance_policy(policy);
        assert_eq!(console.compliance_policies.len(), 1);
        
        let report = console.get_compliance_report();
        assert_eq!(report.total_policies, 1);
        assert_eq!(report.enabled_policies, 1);
    }
}

