/// Phase: C | Step: 4 | Source: Athenos_AI_Strategy.md#L123
/// Reflective Reasoning Loop
/// Build reflective reasoning loop (self-critique of recommendations)

use crate::types::*;
use crate::models::RecommendationRanker;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Self-critique of a recommendation
/// Source: Athenos_AI_Strategy.md#L123
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfCritique {
    pub recommendation_id: String,
    pub critique_score: f64, // 0.0 to 1.0
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub alternative_approaches: Vec<String>,
    pub confidence_adjustment: f64, // Adjustment to original confidence
}

/// Reflective reasoning loop
/// Source: Athenos_AI_Strategy.md#L123
pub struct ReflectiveReasoningLoop {
    ranker: RecommendationRanker,
    critiques: HashMap<String, SelfCritique>,
}

impl ReflectiveReasoningLoop {
    /// Create new reflective reasoning loop
    pub fn new() -> Self {
        info!("ReflectiveReasoningLoop::new: Creating reflective reasoning loop");
        Self {
            ranker: RecommendationRanker::new(),
            critiques: HashMap::new(),
        }
    }

    /// Critique a recommendation
    /// Source: Athenos_AI_Strategy.md#L123
    pub fn critique_recommendation(&mut self, observation: &Observation) -> SelfCritique {
        info!("ReflectiveReasoningLoop::critique_recommendation: Critiquing recommendation {}", observation.id);
        
        let mut strengths = Vec::new();
        let mut weaknesses = Vec::new();
        let mut alternative_approaches = Vec::new();
        let mut critique_score = 0.5;
        
        // Analyze confidence
        match observation.action.confidence {
            Confidence::High => {
                strengths.push("High confidence action".to_string());
                critique_score += 0.2;
            }
            Confidence::Low => {
                weaknesses.push("Low confidence - may need more data".to_string());
                critique_score -= 0.2;
            }
            _ => {}
        }
        
        // Analyze risk
        match observation.action.risk {
            RiskCategory::None => {
                strengths.push("No risk - safe to execute".to_string());
                critique_score += 0.1;
            }
            RiskCategory::High => {
                weaknesses.push("High risk - requires careful consideration".to_string());
                alternative_approaches.push("Consider manual approval first".to_string());
                critique_score -= 0.3;
            }
            _ => {}
        }
        
        // Analyze expected outcome
        if let Some(time_saved) = observation.expected_outcome.get("time_saved_min") {
            if *time_saved > 10.0 {
                strengths.push(format!("Significant time savings: {} min", time_saved));
                critique_score += 0.1;
            } else if *time_saved < 2.0 {
                weaknesses.push("Minimal time savings expected".to_string());
                critique_score -= 0.1;
            }
        }
        
        // Check for alternative approaches
        if observation.observation.len() > 5 {
            alternative_approaches.push("Consider breaking into smaller steps".to_string());
        }
        
        critique_score = critique_score.min(1.0).max(0.0);
        
        let confidence_adjustment = if critique_score > 0.7 {
            0.1 // Increase confidence
        } else if critique_score < 0.4 {
            -0.2 // Decrease confidence
        } else {
            0.0
        };
        
        let critique = SelfCritique {
            recommendation_id: observation.id.clone(),
            critique_score,
            strengths,
            weaknesses,
            alternative_approaches,
            confidence_adjustment,
        };
        
        self.critiques.insert(observation.id.clone(), critique.clone());
        critique
    }

    /// Get adjusted recommendation based on critique
    pub fn get_adjusted_recommendation(&self, observation: &Observation) -> Option<Action> {
        if let Some(critique) = self.critiques.get(&observation.id) {
            if critique.critique_score < 0.4 {
                // Suggest alternative or require approval
                let mut adjusted_action = observation.action.clone();
                adjusted_action.confidence = Confidence::Medium; // Downgrade confidence
                Some(adjusted_action)
            } else {
                Some(observation.action.clone())
            }
        } else {
            None
        }
    }

    /// Reflect on outcomes and update reasoning
    pub fn reflect_on_outcome(&mut self, observation_id: &str, outcome: &Outcome) {
        info!("ReflectiveReasoningLoop::reflect_on_outcome: Reflecting on outcome for {}", observation_id);
        
        if let Some(critique) = self.critiques.get_mut(observation_id) {
            if outcome.accepted {
                critique.critique_score += 0.1;
                critique.strengths.push("User accepted recommendation".to_string());
            } else if outcome.ignored {
                critique.critique_score -= 0.1;
                critique.weaknesses.push("User ignored recommendation".to_string());
            }
            
            critique.critique_score = critique.critique_score.min(1.0).max(0.0);
        }
    }
}

impl Default for ReflectiveReasoningLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_reflective_loop_creation() {
        let loop_ref = ReflectiveReasoningLoop::new();
        assert_eq!(loop_ref.critiques.len(), 0);
    }

    #[test]
    fn test_critique_recommendation() {
        let mut loop_ref = ReflectiveReasoningLoop::new();
        let observation = Observation {
            id: "test_001".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["Teams".to_string(), "Gmail".to_string()],
            metrics: HashMap::new(),
            intent: Intent::SuggestShortcut,
            action: Action {
                action_type: ActionType::AutomationMacro,
                description: "Test macro".to_string(),
                confidence: Confidence::High,
                risk: RiskCategory::None,
            },
            expected_outcome: HashMap::new(),
            source: "test".to_string(),
            timestamp: 1234567890,
        };
        
        let critique = loop_ref.critique_recommendation(&observation);
        assert!(critique.critique_score > 0.0);
        assert!(!critique.strengths.is_empty());
    }

    #[test]
    fn test_reflect_on_outcome() {
        let mut loop_ref = ReflectiveReasoningLoop::new();
        let mut metrics = HashMap::new();
        metrics.insert("repeat_count".to_string(), 8.0);
        let mut expected = HashMap::new();
        expected.insert("time_saved_min".to_string(), 11.0);
        
        let observation = Observation {
            id: "test_002".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["App1".to_string()],
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
        
        loop_ref.critique_recommendation(&observation);
        let initial_score = loop_ref.critiques.get("test_002").unwrap().critique_score;
        
        let outcome = Outcome {
            observation_id: "test_002".to_string(),
            accepted: true,
            ignored: false,
            modified: false,
            time_saved_minutes: Some(11.0),
            error_rate_change: None,
            timestamp: 1234567890,
        };
        
        loop_ref.reflect_on_outcome("test_002", &outcome);
        let updated_score = loop_ref.critiques.get("test_002").unwrap().critique_score;
        assert!(updated_score >= initial_score);
    }
}

