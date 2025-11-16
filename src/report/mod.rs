/// Phase: A | Step: 7 | Source: Athenos_AI_Strategy.md#L102
/// Daily Cognitive Report - Rule-based insights
/// Prototype daily cognitive report using rule-based insights

use crate::types::*;
use crate::local_stack::FeatureStore;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Daily cognitive report
/// Source: Athenos_AI_Strategy.md#L102
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReport {
    pub date: String,
    pub metrics: CognitiveMetrics,
    pub patterns_detected: Vec<PatternInsight>,
    pub suggestions: Vec<ActionSuggestion>,
    pub time_saved_minutes: f64,
    pub focus_stability_pct: f64,
}

/// Pattern insight from rule-based analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternInsight {
    pub pattern_type: PatternType,
    pub description: String,
    pub frequency: usize,
    pub impact_score: f64,
}

/// Action suggestion for user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSuggestion {
    pub action: Action,
    pub expected_benefit: String,
    pub confidence: Confidence,
}

/// Report generator using rule-based logic
/// Source: Athenos_AI_Strategy.md#L102
pub struct ReportGenerator {
    feature_store: FeatureStore,
}

impl ReportGenerator {
    /// Create new report generator
    pub fn new(feature_store: FeatureStore) -> Self {
        info!("ReportGenerator::new: Creating report generator");
        Self { feature_store }
    }

    /// Generate daily report from observations
    /// Source: Athenos_AI_Strategy.md#L102
    pub fn generate_daily_report(&self, observations: &[Observation]) -> DailyReport {
        info!("ReportGenerator::generate_daily_report: Generating report for {} observations", observations.len());
        
        let mut time_saved = 0.0;
        let mut patterns = Vec::new();
        let mut suggestions = Vec::new();
        
        // Rule-based pattern detection
        for obs in observations {
            // Detect workflow sequence pattern
            if obs.observation.len() >= 3 {
                patterns.push(PatternInsight {
                    pattern_type: PatternType::WorkflowSequence,
                    description: format!("Repeated sequence: {}", obs.observation.join(" → ")),
                    frequency: obs.metrics.get("repeat_count").map(|v| *v as usize).unwrap_or(1),
                    impact_score: obs.metrics.get("time_to_first_code_min").copied().unwrap_or(0.0),
                });
            }
            
            // Generate suggestions based on confidence and risk
            if obs.action.confidence >= Confidence::Medium && obs.action.risk <= RiskCategory::Low {
                if let Some(saved) = obs.expected_outcome.get("time_saved_min") {
                    time_saved += saved;
                }
                
                suggestions.push(ActionSuggestion {
                    action: obs.action.clone(),
                    expected_benefit: format!("Expected to save {} minutes", 
                        obs.expected_outcome.get("time_saved_min").copied().unwrap_or(0.0)),
                    confidence: obs.action.confidence.clone(),
                });
            }
        }
        
        // Compute focus stability
        let obs_ids: Vec<String> = observations.iter().map(|o| o.id.clone()).collect();
        let focus_stability = self.feature_store.compute_focus_stability(&obs_ids);
        
        // Compute cognitive metrics (rule-based estimates)
        let metrics = CognitiveMetrics {
            cognitive_clarity_index: focus_stability / 100.0 * 0.8, // Simplified
            emotional_resilience_score: 0.7, // Placeholder
            habit_evolution_rate: 0.1, // Placeholder
            focus_stability_pct: focus_stability,
            time_saved_minutes: time_saved,
        };
        
        DailyReport {
            date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            metrics,
            patterns_detected: patterns,
            suggestions,
            time_saved_minutes: time_saved,
            focus_stability_pct: focus_stability,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_report_generation() {
        let feature_store = FeatureStore::new();
        let generator = ReportGenerator::new(feature_store);
        
        let mut metrics = HashMap::new();
        metrics.insert("repeat_count".to_string(), 8.0);
        metrics.insert("time_to_first_code_min".to_string(), 12.0);
        
        let mut expected = HashMap::new();
        expected.insert("time_saved_min".to_string(), 11.0);
        
        let observations = vec![Observation {
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
        }];
        
        let report = generator.generate_daily_report(&observations);
        assert_eq!(report.suggestions.len(), 1);
        assert_eq!(report.time_saved_minutes, 11.0);
        assert!(!report.patterns_detected.is_empty());
    }
}

