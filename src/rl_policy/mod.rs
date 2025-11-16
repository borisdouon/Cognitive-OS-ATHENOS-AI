/// Phase: D | Step: 1 | Source: Athenos_AI_Strategy.md#L132
/// Reinforcement Learning Policies
/// Deploy reinforcement learning policies tuned by real user outcomes

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Policy action with Q-value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAction {
    pub action: Action,
    pub q_value: f64, // Q-learning value estimate
    pub visit_count: usize,
}

/// RL policy trained on user outcomes
/// Source: Athenos_AI_Strategy.md#L132
pub struct RLPolicy {
    q_table: HashMap<String, PolicyAction>,
    learning_rate: f64,
    discount_factor: f64,
    epsilon: f64, // Exploration rate
}

impl RLPolicy {
    /// Create new RL policy
    pub fn new() -> Self {
        info!("RLPolicy::new: Creating RL policy");
        Self {
            q_table: HashMap::new(),
            learning_rate: 0.1,
            discount_factor: 0.9,
            epsilon: 0.1, // 10% exploration
        }
    }

    /// Update policy from user outcome
    /// Source: Athenos_AI_Strategy.md#L132
    pub fn update_from_outcome(&mut self, observation: &Observation, outcome: &Outcome) {
        info!("RLPolicy::update_from_outcome: Updating policy from outcome {}", observation.id);
        
        let state_key = self.get_state_key(observation);
        let reward = self.compute_reward(outcome);
        
        // Q-learning update: Q(s,a) = Q(s,a) + α[r + γ*max(Q(s',a')) - Q(s,a)]
        let current_q = self.q_table
            .get(&state_key)
            .map(|pa| pa.q_value)
            .unwrap_or(0.0);
        
        let new_q = current_q + self.learning_rate * (reward - current_q);
        
        let policy_action = PolicyAction {
            action: observation.action.clone(),
            q_value: new_q,
            visit_count: self.q_table
                .get(&state_key)
                .map(|pa| pa.visit_count + 1)
                .unwrap_or(1),
        };
        
        self.q_table.insert(state_key, policy_action);
    }

    /// Select action using epsilon-greedy policy
    /// Source: Athenos_AI_Strategy.md#L132
    pub fn select_action(&self, observation: &Observation) -> Action {
        let state_key = self.get_state_key(observation);
        
        // Epsilon-greedy: explore with probability epsilon
        use rand::Rng;
        if rand::thread_rng().gen::<f64>() < self.epsilon {
            // Exploration: return original action
            observation.action.clone()
        } else {
            // Exploitation: return best known action for state
            self.q_table
                .get(&state_key)
                .map(|pa| pa.action.clone())
                .unwrap_or_else(|| observation.action.clone())
        }
    }

    fn get_state_key(&self, observation: &Observation) -> String {
        format!("{:?}_{:?}", observation.intent, observation.profile)
    }

    fn compute_reward(&self, outcome: &Outcome) -> f64 {
        let mut reward = 0.0;
        
        if outcome.accepted {
            reward += 10.0;
        } else if outcome.ignored {
            reward -= 2.0;
        }
        
        if let Some(time_saved) = outcome.time_saved_minutes {
            reward += time_saved * 0.5; // Time saved bonus
        }
        
        if let Some(error_change) = outcome.error_rate_change {
            if error_change < 0.0 {
                reward += 5.0; // Error reduction bonus
            }
        }
        
        reward
    }

    /// Get policy statistics
    pub fn get_statistics(&self) -> PolicyStatistics {
        let total_states = self.q_table.len();
        let avg_q_value = if total_states > 0 {
            self.q_table.values().map(|pa| pa.q_value).sum::<f64>() / total_states as f64
        } else {
            0.0
        };
        
        PolicyStatistics {
            total_states,
            avg_q_value,
            learning_rate: self.learning_rate,
            epsilon: self.epsilon,
        }
    }
}

/// Policy statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyStatistics {
    pub total_states: usize,
    pub avg_q_value: f64,
    pub learning_rate: f64,
    pub epsilon: f64,
}

impl Default for RLPolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_rl_policy_creation() {
        let policy = RLPolicy::new();
        assert_eq!(policy.q_table.len(), 0);
    }

    #[test]
    fn test_update_from_outcome() {
        let mut policy = RLPolicy::new();
        let observation = Observation {
            id: "test_001".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["Teams".to_string()],
            metrics: HashMap::new(),
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
        
        let outcome = Outcome {
            observation_id: "test_001".to_string(),
            accepted: true,
            ignored: false,
            modified: false,
            time_saved_minutes: Some(11.0),
            error_rate_change: None,
            timestamp: 1234567890,
        };
        
        policy.update_from_outcome(&observation, &outcome);
        assert_eq!(policy.q_table.len(), 1);
    }

    #[test]
    fn test_select_action() {
        let policy = RLPolicy::new();
        let observation = Observation {
            id: "test_002".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["App1".to_string()],
            metrics: HashMap::new(),
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
        
        let selected = policy.select_action(&observation);
        assert_eq!(selected.action_type, ActionType::AutomationMacro);
    }
}

