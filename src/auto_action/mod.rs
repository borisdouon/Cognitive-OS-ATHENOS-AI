/// Phase: C | Step: 1 | Source: Athenos_AI_Strategy.md#L120
/// Auto-Action Synthesizer with Sandboxed Execution + Rollback
/// Introduce auto-action synthesizer with sandboxed execution and rollback

use crate::types::*;
use crate::sandbox::{SandboxRunner, SandboxResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Action execution state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionState {
    Pending,
    Executing,
    Completed,
    RolledBack,
    Failed,
}

/// Executed action with rollback capability
/// Source: Athenos_AI_Strategy.md#L120
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutedAction {
    pub id: String,
    pub action: Action,
    pub state: ActionState,
    pub execution_result: Option<SandboxResult>,
    pub rollback_diff: Option<String>,
    pub executed_at: Option<i64>,
    pub rolled_back_at: Option<i64>,
}

/// Auto-action synthesizer
/// Source: Athenos_AI_Strategy.md#L120
pub struct AutoActionSynthesizer {
    sandbox_runner: SandboxRunner,
    executed_actions: HashMap<String, ExecutedAction>,
    rollback_stack: Vec<String>, // Action IDs in execution order
}

impl AutoActionSynthesizer {
    /// Create new auto-action synthesizer
    pub fn new() -> Self {
        info!("AutoActionSynthesizer::new: Creating auto-action synthesizer");
        Self {
            sandbox_runner: SandboxRunner::default(),
            executed_actions: HashMap::new(),
            rollback_stack: Vec::new(),
        }
    }

    /// Synthesize and execute action automatically
    /// Source: Athenos_AI_Strategy.md#L120
    pub fn synthesize_and_execute(&mut self, observation: &Observation) -> Result<ExecutedAction, String> {
        info!("AutoActionSynthesizer::synthesize_and_execute: Synthesizing action for {}", observation.id);
        
        // Check if safe to auto-execute
        if !self.sandbox_runner.is_safe_to_auto_execute(&observation.action) {
            return Err("Action not safe for auto-execution".to_string());
        }
        
        // Test in sandbox first
        let sandbox_result = self.sandbox_runner.test_automation(&observation.action);
        if !sandbox_result.success {
            return Err(format!("Sandbox test failed: {:?}", sandbox_result.error_message));
        }
        
        // Generate rollback diff
        let rollback_diff = self.sandbox_runner.generate_undo(&observation.action);
        
        // Execute action (Phase C: simulated execution)
        let executed_action = ExecutedAction {
            id: format!("action_{}", observation.id),
            action: observation.action.clone(),
            state: ActionState::Completed,
            execution_result: Some(sandbox_result),
            rollback_diff: Some(rollback_diff),
            executed_at: Some(chrono::Utc::now().timestamp()),
            rolled_back_at: None,
        };
        
        self.executed_actions.insert(executed_action.id.clone(), executed_action.clone());
        self.rollback_stack.push(executed_action.id.clone());
        
        Ok(executed_action)
    }

    /// Rollback last action
    /// Source: Athenos_AI_Strategy.md#L120
    pub fn rollback_last(&mut self) -> Result<(), String> {
        info!("AutoActionSynthesizer::rollback_last: Rolling back last action");
        
        if let Some(action_id) = self.rollback_stack.pop() {
            if let Some(action) = self.executed_actions.get_mut(&action_id) {
                if action.state == ActionState::Completed {
                    action.state = ActionState::RolledBack;
                    action.rolled_back_at = Some(chrono::Utc::now().timestamp());
                    Ok(())
                } else {
                    Err("Action not in completed state".to_string())
                }
            } else {
                Err("Action not found".to_string())
            }
        } else {
            Err("No actions to rollback".to_string())
        }
    }

    /// Rollback specific action by ID
    pub fn rollback_action(&mut self, action_id: &str) -> Result<(), String> {
        info!("AutoActionSynthesizer::rollback_action: Rolling back action {}", action_id);
        
        if let Some(action) = self.executed_actions.get_mut(action_id) {
            if action.state == ActionState::Completed {
                action.state = ActionState::RolledBack;
                action.rolled_back_at = Some(chrono::Utc::now().timestamp());
                Ok(())
            } else {
                Err("Action not in completed state".to_string())
            }
        } else {
            Err("Action not found".to_string())
        }
    }

    /// Get execution history
    pub fn get_execution_history(&self) -> Vec<&ExecutedAction> {
        self.rollback_stack
            .iter()
            .filter_map(|id| self.executed_actions.get(id))
            .collect()
    }
}

impl Default for AutoActionSynthesizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_auto_action_synthesizer_creation() {
        let synthesizer = AutoActionSynthesizer::new();
        assert_eq!(synthesizer.executed_actions.len(), 0);
    }

    #[test]
    fn test_synthesize_and_execute_safe_action() {
        let mut synthesizer = AutoActionSynthesizer::new();
        let observation = Observation {
            id: "test_001".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["Teams".to_string(), "Gmail".to_string()],
            metrics: HashMap::new(),
            intent: Intent::AutomateAction,
            action: Action {
                action_type: ActionType::AutomationMacro,
                description: "Safe macro".to_string(),
                confidence: Confidence::High,
                risk: RiskCategory::None,
            },
            expected_outcome: HashMap::new(),
            source: "test".to_string(),
            timestamp: 1234567890,
        };
        
        let result = synthesizer.synthesize_and_execute(&observation);
        assert!(result.is_ok());
        let executed = result.unwrap();
        assert_eq!(executed.state, ActionState::Completed);
        assert!(executed.rollback_diff.is_some());
    }

    #[test]
    fn test_rollback_last_action() {
        let mut synthesizer = AutoActionSynthesizer::new();
        let observation = Observation {
            id: "test_002".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["App1".to_string()],
            metrics: HashMap::new(),
            intent: Intent::AutomateAction,
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
        
        synthesizer.synthesize_and_execute(&observation).unwrap();
        assert_eq!(synthesizer.rollback_stack.len(), 1);
        
        synthesizer.rollback_last().unwrap();
        let action = synthesizer.executed_actions.get("action_test_002").unwrap();
        assert_eq!(action.state, ActionState::RolledBack);
    }

    #[test]
    fn test_unsafe_action_rejected() {
        let mut synthesizer = AutoActionSynthesizer::new();
        let observation = Observation {
            id: "test_003".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["App1".to_string()],
            metrics: HashMap::new(),
            intent: Intent::AutomateAction,
            action: Action {
                action_type: ActionType::AutomationMacro,
                description: "Risky".to_string(),
                confidence: Confidence::Low,
                risk: RiskCategory::High,
            },
            expected_outcome: HashMap::new(),
            source: "test".to_string(),
            timestamp: 1234567890,
        };
        
        let result = synthesizer.synthesize_and_execute(&observation);
        assert!(result.is_err());
    }
}

