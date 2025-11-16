/// Phase: A | Step: 2 | Source: Athenos_AI_Strategy.md#L97
/// Cognitive Taxonomy - Core types for Athenos AI
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Intent types for cognitive interventions
/// Source: TRAINING CONCEPT.txt#L26
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    DetectPattern,
    SuggestShortcut,
    AutomateAction,
    MoodIntervention,
}

/// Pattern archetypes observed in user behavior
/// Source: TRAINING CONCEPT.txt#L27
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PatternType {
    WorkflowSequence,
    DebuggingLoop,
    ContextSwitching,
    TimingVariance,
    RepetitiveGesture,
    AttentionFragmentation,
}

/// Action types for interventions
/// Source: TRAINING CONCEPT.txt#L28
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    AutomationMacro,
    MicroNudge,
    ScheduleChange,
    SandboxPatch,
    PreemptiveDebugAssistant,
    FocusMode,
    ZenMode,
    SystemHygiene,
}

/// Confidence levels for action execution
/// Source: TRAINING CONCEPT.txt#L29
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    Low,
    Medium,
    High,
}

/// Risk categories for safety guardrails
/// Source: TRAINING CONCEPT.txt#L30
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum RiskCategory {
    None,
    Low,
    High,
}

/// Emotional states detected from behavior
/// Source: Athenos_AI_Strategy.md#L98
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EmotionalState {
    Calm,
    Focused,
    Stressed,
    Fatigued,
    CreativeFlow,
    Fragmented,
}

/// User profile types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserProfile {
    Developer,
    Accountant,
    Designer,
    Manager,
    Student,
    Other,
}

/// Observation unit - captures a single behavioral pattern
/// Source: TRAINING CONCEPT.txt#L40-57
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: String,
    pub profile: UserProfile,
    pub observation: Vec<String>, // Sequence of actions/apps
    pub metrics: HashMap<String, f64>, // Temporal and behavioral metrics
    pub intent: Intent,
    pub action: Action,
    pub expected_outcome: HashMap<String, f64>,
    pub source: String,
    pub timestamp: i64,
}

/// Action definition for interventions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_type: ActionType,
    pub description: String,
    pub confidence: Confidence,
    pub risk: RiskCategory,
}

/// Outcome tracking
/// Source: TRAINING CONCEPT.txt#L31
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub observation_id: String,
    pub accepted: bool,
    pub ignored: bool,
    pub modified: bool,
    pub time_saved_minutes: Option<f64>,
    pub error_rate_change: Option<f64>,
    pub timestamp: i64,
}

/// Cognitive metrics for daily reports
/// Source: Strategic_Reinforcements_Gap_Closures.md#L25
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveMetrics {
    pub cognitive_clarity_index: f64,
    pub emotional_resilience_score: f64,
    pub habit_evolution_rate: f64,
    pub focus_stability_pct: f64,
    pub time_saved_minutes: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_intent_serialization() {
        let intent = Intent::DetectPattern;
        let json = serde_json::to_string(&intent).unwrap();
        assert_eq!(json, "\"detect_pattern\"");
    }

    #[test]
    fn test_observation_creation() {
        let mut metrics = HashMap::new();
        metrics.insert("repeat_count".to_string(), 8.0);
        
        let observation = Observation {
            id: "test_001".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["Teams".to_string(), "Gmail".to_string()],
            metrics,
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
        
        assert_eq!(observation.id, "test_001");
        assert_eq!(observation.profile, UserProfile::Developer);
    }

    #[test]
    fn test_confidence_ordering() {
        assert!(Confidence::High > Confidence::Medium);
        assert!(Confidence::Medium > Confidence::Low);
    }

    #[test]
    fn test_risk_category_ordering() {
        assert!(RiskCategory::High > RiskCategory::Low);
        assert!(RiskCategory::Low > RiskCategory::None);
    }
}

