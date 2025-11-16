/// Phase: B | Step: 9 | Source: Athenos_AI_Strategy.md#L116
/// Federated Learning Pilot
/// Start federated learning pilot to share anonymized pattern templates

use crate::types::*;
use crate::privacy::ConsentLedger;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Anonymized pattern template for federated learning
/// Source: Athenos_AI_Strategy.md#L116
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizedPatternTemplate {
    pub pattern_type: PatternType,
    pub sequence_length: usize,
    pub frequency: usize,
    pub avg_time_saved_min: f64,
    pub confidence_score: f64,
    // No user-specific data
}

/// Federated learning coordinator
/// Source: Athenos_AI_Strategy.md#L116
pub struct FederatedLearningCoordinator {
    consent_ledger: ConsentLedger,
    local_templates: Vec<AnonymizedPatternTemplate>,
    aggregated_templates: Vec<AnonymizedPatternTemplate>,
}

impl FederatedLearningCoordinator {
    /// Create new federated learning coordinator
    pub fn new(consent_ledger: ConsentLedger) -> Self {
        info!("FederatedLearningCoordinator::new: Creating federated learning coordinator");
        Self {
            consent_ledger,
            local_templates: Vec::new(),
            aggregated_templates: Vec::new(),
        }
    }

    /// Anonymize pattern from observation
    /// Source: Athenos_AI_Strategy.md#L116
    pub fn anonymize_pattern(&self, observation: &Observation) -> Option<AnonymizedPatternTemplate> {
        // Only proceed if user has opted in
        if !self.consent_ledger.opt_in_cloud_sync {
            return None;
        }
        
        info!("FederatedLearningCoordinator::anonymize_pattern: Anonymizing pattern for {}", observation.id);
        
        let time_saved = observation.expected_outcome.get("time_saved_min").copied().unwrap_or(0.0);
        let confidence = match observation.action.confidence {
            Confidence::High => 0.9,
            Confidence::Medium => 0.6,
            Confidence::Low => 0.3,
        };
        
        Some(AnonymizedPatternTemplate {
            pattern_type: PatternType::WorkflowSequence, // Simplified
            sequence_length: observation.observation.len(),
            frequency: observation.metrics.get("repeat_count").map(|v| *v as usize).unwrap_or(1),
            avg_time_saved_min: time_saved,
            confidence_score: confidence,
        })
    }

    /// Prepare templates for federated sharing
    /// Source: Athenos_AI_Strategy.md#L116
    pub fn prepare_for_sharing(&mut self, observations: &[Observation]) -> Vec<AnonymizedPatternTemplate> {
        info!("FederatedLearningCoordinator::prepare_for_sharing: Preparing {} observations", observations.len());
        
        if !self.consent_ledger.opt_in_cloud_sync {
            return Vec::new();
        }
        
        observations
            .iter()
            .filter_map(|obs| self.anonymize_pattern(obs))
            .collect()
    }

    /// Aggregate templates from federated learning
    pub fn aggregate_templates(&mut self, templates: Vec<AnonymizedPatternTemplate>) {
        info!("FederatedLearningCoordinator::aggregate_templates: Aggregating {} templates", templates.len());
        
        // Phase B: Simple aggregation (would use proper FL algorithms in production)
        for template in templates {
            // Find similar template or add new
            if let Some(existing) = self.aggregated_templates.iter_mut()
                .find(|t| t.pattern_type == template.pattern_type && t.sequence_length == template.sequence_length) {
                // Update averages
                let total_freq = existing.frequency + template.frequency;
                existing.avg_time_saved_min = 
                    (existing.avg_time_saved_min * existing.frequency as f64 + 
                     template.avg_time_saved_min * template.frequency as f64) / total_freq as f64;
                existing.frequency = total_freq;
            } else {
                self.aggregated_templates.push(template);
            }
        }
    }

    /// Get aggregated templates
    pub fn get_aggregated_templates(&self) -> &[AnonymizedPatternTemplate] {
        &self.aggregated_templates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::privacy::ConsentLedger;
    use std::collections::HashMap;

    #[test]
    fn test_federated_coordinator_creation() {
        let consent = ConsentLedger::new();
        let coordinator = FederatedLearningCoordinator::new(consent);
        assert_eq!(coordinator.local_templates.len(), 0);
    }

    #[test]
    fn test_anonymize_pattern_with_consent() {
        let mut consent = ConsentLedger::new();
        consent.opt_in_cloud_sync = true;
        
        let coordinator = FederatedLearningCoordinator::new(consent);
        let mut metrics = HashMap::new();
        metrics.insert("repeat_count".to_string(), 8.0);
        let mut expected = HashMap::new();
        expected.insert("time_saved_min".to_string(), 11.0);
        
        let observation = Observation {
            id: "test".to_string(),
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
            expected_outcome: expected,
            source: "test".to_string(),
            timestamp: 1234567890,
        };
        
        let template = coordinator.anonymize_pattern(&observation);
        assert!(template.is_some());
        let template = template.unwrap();
        assert_eq!(template.sequence_length, 2);
        assert_eq!(template.avg_time_saved_min, 11.0);
    }

    #[test]
    fn test_anonymize_pattern_without_consent() {
        let consent = ConsentLedger::new(); // Default: opt-out
        let coordinator = FederatedLearningCoordinator::new(consent);
        
        let observation = Observation {
            id: "test".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["App1".to_string()],
            metrics: HashMap::new(),
            intent: Intent::DetectPattern,
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
        
        let template = coordinator.anonymize_pattern(&observation);
        assert!(template.is_none()); // Should return None without consent
    }
}

