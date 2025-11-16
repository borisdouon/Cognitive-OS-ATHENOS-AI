/// Phase: B | Step: 1 | Source: Athenos_AI_Strategy.md#L108
/// Supervised Models - Pattern Detection + Recommendation Ranking
/// Train supervised models for pattern detection and recommendation ranking

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Pattern detection model (simplified for Phase B)
/// Uses heuristic-based approach with feature weights
pub struct PatternDetector {
    weights: HashMap<String, f64>,
}

impl PatternDetector {
    /// Create new pattern detector
    pub fn new() -> Self {
        info!("PatternDetector::new: Creating pattern detector");
        let mut weights = HashMap::new();
        weights.insert("repeat_count".to_string(), 0.3);
        weights.insert("time_to_first_code_min".to_string(), 0.2);
        weights.insert("context_switch_count".to_string(), 0.25);
        weights.insert("focus_fragmentation_pct".to_string(), 0.25);
        Self { weights }
    }

    /// Train on observations (Phase B: simple weight adjustment)
    /// Source: Athenos_AI_Strategy.md#L108
    pub fn train(&mut self, observations: &[Observation]) {
        info!("PatternDetector::train: Training on {} observations", observations.len());
        // Phase B: Simple heuristic training
        // In production, would use proper ML training
        for obs in observations {
            if obs.metrics.get("repeat_count").copied().unwrap_or(0.0) > 5.0 {
                *self.weights.get_mut("repeat_count").unwrap() *= 1.1;
            }
        }
    }

    /// Detect pattern from observation
    pub fn detect_pattern(&self, observation: &Observation) -> PatternType {
        info!("PatternDetector::detect_pattern: Detecting pattern for {}", observation.id);
        
        let repeat_count = observation.metrics.get("repeat_count").copied().unwrap_or(0.0);
        let context_switches = observation.metrics.get("context_switch_count").copied().unwrap_or(0.0);
        let fragmentation = observation.metrics.get("focus_fragmentation_pct").copied().unwrap_or(0.0);
        
        if repeat_count > 5.0 && observation.observation.len() >= 3 {
            PatternType::WorkflowSequence
        } else if observation.observation.contains(&"copy_error".to_string()) {
            PatternType::DebuggingLoop
        } else if context_switches > 5.0 || fragmentation > 50.0 {
            PatternType::ContextSwitching
        } else {
            PatternType::TimingVariance
        }
    }

    /// Score pattern confidence (0.0 to 1.0)
    pub fn score_confidence(&self, observation: &Observation) -> f64 {
        let mut score = 0.0;
        for (key, weight) in &self.weights {
            if let Some(value) = observation.metrics.get(key) {
                score += value * weight;
            }
        }
        (score / 100.0).min(1.0).max(0.0)
    }
}

impl Default for PatternDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Recommendation ranker
/// Source: Athenos_AI_Strategy.md#L108
pub struct RecommendationRanker {
    pattern_detector: PatternDetector,
}

impl RecommendationRanker {
    /// Create new ranker
    pub fn new() -> Self {
        info!("RecommendationRanker::new: Creating recommendation ranker");
        Self {
            pattern_detector: PatternDetector::new(),
        }
    }

    /// Rank actions by expected value
    /// Source: Athenos_AI_Strategy.md#L108
    pub fn rank_actions(&self, observations: &[Observation]) -> Vec<(Observation, f64)> {
        info!("RecommendationRanker::rank_actions: Ranking {} observations", observations.len());
        let mut ranked: Vec<(Observation, f64)> = observations
            .iter()
            .map(|obs| {
                let pattern_score = self.pattern_detector.score_confidence(obs);
                let time_saved = obs.expected_outcome.get("time_saved_min").copied().unwrap_or(0.0);
                let confidence_multiplier = match obs.action.confidence {
                    Confidence::High => 1.0,
                    Confidence::Medium => 0.7,
                    Confidence::Low => 0.4,
                };
                let risk_penalty = match obs.action.risk {
                    RiskCategory::None => 1.0,
                    RiskCategory::Low => 0.8,
                    RiskCategory::High => 0.3,
                };
                
                let score = (pattern_score * 0.4 + time_saved / 100.0 * 0.6) * confidence_multiplier * risk_penalty;
                (obs.clone(), score)
            })
            .collect();
        
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        ranked
    }

    /// Train ranker on observations
    pub fn train(&mut self, observations: &[Observation]) {
        info!("RecommendationRanker::train: Training ranker on {} observations", observations.len());
        self.pattern_detector.train(observations);
    }
}

impl Default for RecommendationRanker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_pattern_detector_creation() {
        let detector = PatternDetector::new();
        assert_eq!(detector.weights.len(), 4);
    }

    #[test]
    fn test_pattern_detection_workflow_sequence() {
        let detector = PatternDetector::new();
        let mut metrics = HashMap::new();
        metrics.insert("repeat_count".to_string(), 8.0);
        
        let observation = Observation {
            id: "test".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["Teams".to_string(), "Gmail".to_string(), "IDE".to_string()],
            metrics,
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
        
        let pattern = detector.detect_pattern(&observation);
        assert_eq!(pattern, PatternType::WorkflowSequence);
    }

    #[test]
    fn test_pattern_detection_debugging_loop() {
        let detector = PatternDetector::new();
        let observation = Observation {
            id: "test".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["copy_error".to_string(), "paste_chatgpt".to_string()],
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
        
        let pattern = detector.detect_pattern(&observation);
        assert_eq!(pattern, PatternType::DebuggingLoop);
    }

    #[test]
    fn test_recommendation_ranking() {
        let ranker = RecommendationRanker::new();
        let mut metrics1 = HashMap::new();
        metrics1.insert("repeat_count".to_string(), 8.0);
        let mut expected1 = HashMap::new();
        expected1.insert("time_saved_min".to_string(), 20.0);
        
        let mut metrics2 = HashMap::new();
        metrics2.insert("repeat_count".to_string(), 3.0);
        let mut expected2 = HashMap::new();
        expected2.insert("time_saved_min".to_string(), 5.0);
        
        let obs1 = Observation {
            id: "obs1".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["Teams".to_string(), "Gmail".to_string()],
            metrics: metrics1,
            intent: Intent::SuggestShortcut,
            action: Action {
                action_type: ActionType::AutomationMacro,
                description: "High value".to_string(),
                confidence: Confidence::High,
                risk: RiskCategory::None,
            },
            expected_outcome: expected1,
            source: "test".to_string(),
            timestamp: 1234567890,
        };
        
        let obs2 = Observation {
            id: "obs2".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["App1".to_string()],
            metrics: metrics2,
            intent: Intent::SuggestShortcut,
            action: Action {
                action_type: ActionType::AutomationMacro,
                description: "Low value".to_string(),
                confidence: Confidence::Medium,
                risk: RiskCategory::Low,
            },
            expected_outcome: expected2,
            source: "test".to_string(),
            timestamp: 1234567890,
        };
        
        let ranked = ranker.rank_actions(&[obs1.clone(), obs2.clone()]);
        assert_eq!(ranked.len(), 2);
        assert!(ranked[0].1 >= ranked[1].1); // First should have higher score
    }

    #[test]
    fn test_training_updates_weights() {
        let mut detector = PatternDetector::new();
        let initial_weight = *detector.weights.get("repeat_count").unwrap();
        
        let mut metrics = HashMap::new();
        metrics.insert("repeat_count".to_string(), 10.0);
        let observation = Observation {
            id: "test".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["App1".to_string(), "App2".to_string()],
            metrics,
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
        
        detector.train(&[observation]);
        let new_weight = *detector.weights.get("repeat_count").unwrap();
        assert!(new_weight > initial_weight);
    }
}

