/// Phase: B | Step: 8 | Source: Athenos_AI_Strategy.md#L115
/// Replay Simulations for Safety and Quality Gating
/// Conduct replay simulations for safety and quality gating

use crate::types::*;
use crate::sandbox::SandboxRunner;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Replay simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayResult {
    pub observation_id: String,
    pub action_safe: bool,
    pub quality_score: f64, // 0.0 to 1.0
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Replay simulator for safety gating
/// Source: Athenos_AI_Strategy.md#L115
pub struct ReplaySimulator {
    sandbox_runner: SandboxRunner,
    historical_outcomes: HashMap<String, Outcome>,
}

impl ReplaySimulator {
    /// Create new replay simulator
    pub fn new() -> Self {
        info!("ReplaySimulator::new: Creating replay simulator");
        Self {
            sandbox_runner: SandboxRunner::default(),
            historical_outcomes: HashMap::new(),
        }
    }

    /// Simulate action replay from historical data
    /// Source: Athenos_AI_Strategy.md#L115
    pub fn replay_action(&mut self, observation: &Observation) -> ReplayResult {
        info!("ReplaySimulator::replay_action: Replaying action for {}", observation.id);
        
        // Test in sandbox first
        let sandbox_result = self.sandbox_runner.test_automation(&observation.action);
        
        // Check historical outcomes for similar patterns
        let mut quality_score = 0.5; // Default
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        for (_, outcome) in &self.historical_outcomes {
            if outcome.accepted {
                quality_score += 0.1;
            } else if outcome.ignored {
                quality_score -= 0.05;
            }
        }
        
        quality_score = quality_score.min(1.0).max(0.0);
        
        if !sandbox_result.success {
            errors.push("Sandbox test failed".to_string());
        }
        
        if observation.action.risk > RiskCategory::Low {
            warnings.push("High risk action detected".to_string());
        }
        
        if observation.action.confidence < Confidence::Medium {
            warnings.push("Low confidence action".to_string());
        }
        
        ReplayResult {
            observation_id: observation.id.clone(),
            action_safe: sandbox_result.success && observation.action.risk <= RiskCategory::Low,
            quality_score,
            errors,
            warnings,
        }
    }

    /// Add historical outcome for learning
    pub fn add_outcome(&mut self, observation_id: String, outcome: Outcome) {
        info!("ReplaySimulator::add_outcome: Adding outcome for {}", observation_id);
        self.historical_outcomes.insert(observation_id, outcome);
    }

    /// Run batch replay simulation
    /// Source: Athenos_AI_Strategy.md#L115
    pub fn batch_replay(&mut self, observations: &[Observation]) -> Vec<ReplayResult> {
        info!("ReplaySimulator::batch_replay: Running batch replay on {} observations", observations.len());
        observations
            .iter()
            .map(|obs| self.replay_action(obs))
            .collect()
    }

    /// Gate actions based on replay results
    pub fn gate_action(&self, result: &ReplayResult) -> bool {
        result.action_safe && result.quality_score > 0.6 && result.errors.is_empty()
    }
}

impl Default for ReplaySimulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_replay_simulator_creation() {
        let simulator = ReplaySimulator::new();
        assert_eq!(simulator.historical_outcomes.len(), 0);
    }

    #[test]
    fn test_replay_action_safe() {
        let mut simulator = ReplaySimulator::new();
        let observation = Observation {
            id: "test_001".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["Teams".to_string(), "Gmail".to_string()],
            metrics: HashMap::new(),
            intent: Intent::SuggestShortcut,
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
        
        let result = simulator.replay_action(&observation);
        assert!(result.action_safe);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_gate_action() {
        let simulator = ReplaySimulator::new();
        let result = ReplayResult {
            observation_id: "test".to_string(),
            action_safe: true,
            quality_score: 0.8,
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        
        assert!(simulator.gate_action(&result));
        
        let bad_result = ReplayResult {
            observation_id: "test".to_string(),
            action_safe: false,
            quality_score: 0.3,
            errors: vec!["Error".to_string()],
            warnings: Vec::new(),
        };
        
        assert!(!simulator.gate_action(&bad_result));
    }
}

