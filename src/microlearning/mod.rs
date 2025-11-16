/// Phase: C | Step: 2 | Source: Athenos_AI_Strategy.md#L121
/// Contextual Microlearning Nudges
/// Add contextual microlearning nudges driven by error/misuse detection

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Error/misuse pattern detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPattern {
    pub error_type: String,
    pub frequency: usize,
    pub context: String,
    pub detected_at: i64,
}

/// Microlearning nudge
/// Source: Athenos_AI_Strategy.md#L121
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrolearningNudge {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tip: String,
    pub apply_action: Option<String>,
    pub error_pattern: Option<String>,
    pub created_at: i64,
}

/// Microlearning nudge generator
/// Source: Athenos_AI_Strategy.md#L121
pub struct MicrolearningNudgeGenerator {
    error_patterns: HashMap<String, ErrorPattern>,
    nudge_templates: HashMap<String, String>,
}

impl MicrolearningNudgeGenerator {
    /// Create new microlearning nudge generator
    pub fn new() -> Self {
        info!("MicrolearningNudgeGenerator::new: Creating microlearning nudge generator");
        
        let mut nudge_templates = HashMap::new();
        nudge_templates.insert("repeated_error".to_string(), 
            "You've repeated this error {} times. Try: {}");
        nudge_templates.insert("inefficient_pattern".to_string(),
            "This pattern could be optimized. Consider: {}");
        nudge_templates.insert("misuse_detected".to_string(),
            "There's a better way to do this. Tip: {}");
        
        Self {
            error_patterns: HashMap::new(),
            nudge_templates,
        }
    }

    /// Detect error/misuse pattern
    /// Source: Athenos_AI_Strategy.md#L121
    pub fn detect_error_pattern(&mut self, error_type: String, context: String) {
        info!("MicrolearningNudgeGenerator::detect_error_pattern: Detecting error {}", error_type);
        
        let pattern = self.error_patterns
            .entry(error_type.clone())
            .and_modify(|p| {
                p.frequency += 1;
                p.context = context.clone();
                p.detected_at = chrono::Utc::now().timestamp();
            })
            .or_insert_with(|| ErrorPattern {
                error_type: error_type.clone(),
                frequency: 1,
                context,
                detected_at: chrono::Utc::now().timestamp(),
            });
        
        // Generate nudge if frequency threshold reached
        if pattern.frequency >= 3 {
            info!("Error pattern frequency threshold reached: {}", pattern.frequency);
        }
    }

    /// Generate contextual nudge for error pattern
    /// Source: Athenos_AI_Strategy.md#L121
    pub fn generate_nudge(&self, error_type: &str, tip: &str) -> Option<MicrolearningNudge> {
        info!("MicrolearningNudgeGenerator::generate_nudge: Generating nudge for {}", error_type);
        
        if let Some(pattern) = self.error_patterns.get(error_type) {
            if pattern.frequency >= 3 {
                let template = self.nudge_templates.get("repeated_error")
                    .unwrap_or(&"Try this: {}".to_string());
                let content = template.replace("{}", &format!("{} times", pattern.frequency));
                
                Some(MicrolearningNudge {
                    id: format!("nudge_{}", chrono::Utc::now().timestamp()),
                    title: format!("Improve your workflow: {}", error_type),
                    content,
                    tip: tip.to_string(),
                    apply_action: Some(format!("Apply tip: {}", tip)),
                    error_pattern: Some(error_type.to_string()),
                    created_at: chrono::Utc::now().timestamp(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Generate nudge for inefficient pattern
    pub fn generate_inefficiency_nudge(&self, pattern_desc: &str, suggestion: &str) -> MicrolearningNudge {
        info!("MicrolearningNudgeGenerator::generate_inefficiency_nudge: Generating nudge for pattern");
        
        MicrolearningNudge {
            id: format!("nudge_{}", chrono::Utc::now().timestamp()),
            title: "Optimization opportunity".to_string(),
            content: format!("Pattern detected: {}. Suggestion: {}", pattern_desc, suggestion),
            tip: suggestion.to_string(),
            apply_action: Some(format!("Apply: {}", suggestion)),
            error_pattern: None,
            created_at: chrono::Utc::now().timestamp(),
        }
    }

    /// Get all active nudges
    pub fn get_active_nudges(&self) -> Vec<MicrolearningNudge> {
        self.error_patterns
            .iter()
            .filter_map(|(error_type, pattern)| {
                if pattern.frequency >= 3 {
                    self.generate_nudge(error_type, "Review best practices")
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for MicrolearningNudgeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microlearning_generator_creation() {
        let generator = MicrolearningNudgeGenerator::new();
        assert_eq!(generator.error_patterns.len(), 0);
    }

    #[test]
    fn test_detect_error_pattern() {
        let mut generator = MicrolearningNudgeGenerator::new();
        
        generator.detect_error_pattern("wrong_git_command".to_string(), "git push origin".to_string());
        generator.detect_error_pattern("wrong_git_command".to_string(), "git push origin".to_string());
        generator.detect_error_pattern("wrong_git_command".to_string(), "git push origin".to_string());
        
        let pattern = generator.error_patterns.get("wrong_git_command").unwrap();
        assert_eq!(pattern.frequency, 3);
    }

    #[test]
    fn test_generate_nudge_after_threshold() {
        let mut generator = MicrolearningNudgeGenerator::new();
        
        for _ in 0..3 {
            generator.detect_error_pattern("repeated_mistake".to_string(), "context".to_string());
        }
        
        let nudge = generator.generate_nudge("repeated_mistake", "Use the correct command");
        assert!(nudge.is_some());
        let nudge = nudge.unwrap();
        assert!(nudge.content.contains("3 times"));
        assert_eq!(nudge.error_pattern, Some("repeated_mistake".to_string()));
    }

    #[test]
    fn test_generate_inefficiency_nudge() {
        let generator = MicrolearningNudgeGenerator::new();
        let nudge = generator.generate_inefficiency_nudge(
            "Repeated 10-step workflow",
            "Use 3-step shortcut"
        );
        
        assert!(nudge.content.contains("Repeated 10-step workflow"));
        assert_eq!(nudge.tip, "Use 3-step shortcut");
    }
}

