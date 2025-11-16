/// Phase: B | Step: 2 | Source: Athenos_AI_Strategy.md#L109
/// Wisdom Engine - Fine-tuned LLM on curated corpus
/// Fine-tune Wisdom Engine LLM on curated corpus (insights, philosophy, tone)

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Wisdom Engine prompt template
/// Source: Athenos_AI_Strategy.md#L85-89
pub struct WisdomEngine {
    prompt_template: String,
}

impl WisdomEngine {
    /// Create new Wisdom Engine
    /// Source: Athenos_AI_Strategy.md#L109
    pub fn new() -> Self {
        info!("WisdomEngine::new: Creating Wisdom Engine");
        let prompt_template = r#"You are Athenos AI, a cognitive mentor that helps humans transcend their habitual patterns.

Philosophy: Humans run on cognitive loops. You reveal, reflect, and help transcend them through calm, sovereign guidance.

Tone: Calm, insightful, supportive. Speak like a wise mentor, not a cold machine.

Context: {context}

Observation: {observation}

Provide insight in 2-3 sentences that:
1. Reflects the pattern observed
2. Explains why it matters cognitively
3. Offers a gentle path forward

Insight:"#.to_string();
        
        Self { prompt_template }
    }

    /// Generate insight from observation
    /// Source: Athenos_AI_Strategy.md#L109
    pub fn generate_insight(&self, observation: &Observation, context: &str) -> String {
        info!("WisdomEngine::generate_insight: Generating insight for {}", observation.id);
        
        // Phase B: Template-based generation (stub for LLM fine-tuning)
        // In production, would use fine-tuned candle model
        let observation_desc = observation.observation.join(" → ");
        let pattern = self.detect_pattern_type(&observation);
        
        format!(
            "I've noticed you frequently follow the pattern: {}. This {} pattern suggests your mind is operating on autopilot. Consider pausing to reflect: could this workflow be streamlined? The suggested action ({}) aligns with your cognitive rhythm and may help you transcend this habitual loop.",
            observation_desc,
            pattern,
            observation.action.description
        )
    }

    /// Fine-tune on seed data
    /// Source: Athenos_AI_Strategy.md#L109
    pub fn fine_tune(&mut self, observations: &[Observation]) -> Result<(), String> {
        info!("WisdomEngine::fine_tune: Fine-tuning on {} observations", observations.len());
        // Phase B: Stub for fine-tuning
        // In production, would load athenos_seed.jsonl and fine-tune candle model
        // For now, we adjust prompt template based on patterns
        if observations.len() > 10 {
            self.prompt_template = format!("{}\n\nFine-tuned on {} examples.", self.prompt_template, observations.len());
        }
        Ok(())
    }

    fn detect_pattern_type(&self, observation: &Observation) -> &str {
        if observation.observation.len() >= 3 {
            "workflow sequence"
        } else if observation.observation.contains(&"error".to_string()) {
            "debugging loop"
        } else {
            "behavioral pattern"
        }
    }
}

impl Default for WisdomEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_wisdom_engine_creation() {
        let engine = WisdomEngine::new();
        assert!(!engine.prompt_template.is_empty());
    }

    #[test]
    fn test_generate_insight() {
        let engine = WisdomEngine::new();
        let mut metrics = HashMap::new();
        metrics.insert("repeat_count".to_string(), 8.0);
        
        let observation = Observation {
            id: "test".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["Teams".to_string(), "Gmail".to_string(), "IDE".to_string()],
            metrics,
            intent: Intent::SuggestShortcut,
            action: Action {
                action_type: ActionType::AutomationMacro,
                description: "Create startup macro".to_string(),
                confidence: Confidence::High,
                risk: RiskCategory::None,
            },
            expected_outcome: HashMap::new(),
            source: "test".to_string(),
            timestamp: 1234567890,
        };
        
        let insight = engine.generate_insight(&observation, "Morning routine");
        assert!(insight.contains("Teams"));
        assert!(insight.contains("workflow sequence"));
    }

    #[test]
    fn test_fine_tune() {
        let mut engine = WisdomEngine::new();
        let observations = vec![Observation {
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
        }; 15]; // 15 observations
        
        let result = engine.fine_tune(&observations);
        assert!(result.is_ok());
        assert!(engine.prompt_template.contains("Fine-tuned"));
    }
}

