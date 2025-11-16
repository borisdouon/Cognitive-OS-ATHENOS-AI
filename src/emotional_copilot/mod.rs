/// Phase: C | Step: 5 | Source: Athenos_AI_Strategy.md#L124
/// Emotional Co-pilot
/// Launch emotional co-pilot (stress mitigation, motivational messaging)

use crate::types::*;
use crate::emotion::EmotionEstimator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Motivational message
/// Source: Athenos_AI_Strategy.md#L124
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotivationalMessage {
    pub id: String,
    pub message: String,
    pub message_type: MessageType,
    pub emotional_state: EmotionalState,
    pub created_at: i64,
}

/// Message type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    StressMitigation,
    Encouragement,
    AchievementCelebration,
    FocusReminder,
}

/// Stress mitigation intervention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressIntervention {
    pub intervention_type: String,
    pub description: String,
    pub breathing_exercise: Option<String>,
    pub break_suggestion: Option<String>,
}

/// Emotional co-pilot
/// Source: Athenos_AI_Strategy.md#L124
pub struct EmotionalCoPilot {
    emotion_estimator: EmotionEstimator,
    messages: Vec<MotivationalMessage>,
    stress_interventions: Vec<StressIntervention>,
}

impl EmotionalCoPilot {
    /// Create new emotional co-pilot
    pub fn new() -> Self {
        info!("EmotionalCoPilot::new: Creating emotional co-pilot");
        Self {
            emotion_estimator: EmotionEstimator::new(),
            messages: Vec::new(),
            stress_interventions: Vec::new(),
        }
    }

    /// Detect stress and provide mitigation
    /// Source: Athenos_AI_Strategy.md#L124
    pub fn mitigate_stress(&mut self, metrics: &HashMap<String, f64>) -> Option<StressIntervention> {
        info!("EmotionalCoPilot::mitigate_stress: Checking for stress");
        
        let emotion = self.emotion_estimator.estimate_emotion(metrics);
        
        if emotion.emotional_state == EmotionalState::Stressed {
            let intervention = StressIntervention {
                intervention_type: "breathing_exercise".to_string(),
                description: "Take a moment to reset. Try this breathing exercise:".to_string(),
                breathing_exercise: Some("Inhale for 4 counts, hold for 4, exhale for 4. Repeat 3 times.".to_string()),
                break_suggestion: Some("Consider a 5-minute break after this task.".to_string()),
            };
            
            self.stress_interventions.push(intervention.clone());
            Some(intervention)
        } else {
            None
        }
    }

    /// Generate motivational message
    /// Source: Athenos_AI_Strategy.md#L124
    pub fn generate_motivational_message(&mut self, emotional_state: EmotionalState, context: &str) -> MotivationalMessage {
        info!("EmotionalCoPilot::generate_motivational_message: Generating message for {:?}", emotional_state);
        
        let (message, message_type) = match emotional_state {
            EmotionalState::Stressed => (
                "You're doing great work. Remember to take breaks and breathe. Your well-being matters.".to_string(),
                MessageType::StressMitigation,
            ),
            EmotionalState::Fatigued => (
                "You've been working hard. Consider a short break to recharge. Your productivity will thank you.".to_string(),
                MessageType::Encouragement,
            ),
            EmotionalState::Focused => (
                "Excellent focus! You're in the flow. Keep this momentum going.".to_string(),
                MessageType::FocusReminder,
            ),
            EmotionalState::CreativeFlow => (
                "You're in a creative flow state. This is when magic happens. Trust your process.".to_string(),
                MessageType::AchievementCelebration,
            ),
            _ => (
                "Keep going. Every step forward counts.".to_string(),
                MessageType::Encouragement,
            ),
        };
        
        let motivational_msg = MotivationalMessage {
            id: format!("msg_{}", chrono::Utc::now().timestamp()),
            message,
            message_type,
            emotional_state,
            created_at: chrono::Utc::now().timestamp(),
        };
        
        self.messages.push(motivational_msg.clone());
        motivational_msg
    }

    /// Get recent messages
    pub fn get_recent_messages(&self, limit: usize) -> Vec<&MotivationalMessage> {
        let start = self.messages.len().saturating_sub(limit);
        self.messages[start..].iter().collect()
    }
}

impl Default for EmotionalCoPilot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotional_copilot_creation() {
        let copilot = EmotionalCoPilot::new();
        assert_eq!(copilot.messages.len(), 0);
    }

    #[test]
    fn test_stress_mitigation() {
        let mut copilot = EmotionalCoPilot::new();
        let mut metrics = HashMap::new();
        metrics.insert("typing_speed_decrease_pct".to_string(), 40.0);
        metrics.insert("error_rate".to_string(), 0.2);
        
        let intervention = copilot.mitigate_stress(&metrics);
        assert!(intervention.is_some());
        let intervention = intervention.unwrap();
        assert!(intervention.breathing_exercise.is_some());
    }

    #[test]
    fn test_motivational_message_generation() {
        let mut copilot = EmotionalCoPilot::new();
        let message = copilot.generate_motivational_message(EmotionalState::Focused, "coding");
        
        assert_eq!(message.message_type, MessageType::FocusReminder);
        assert_eq!(message.emotional_state, EmotionalState::Focused);
        assert!(!message.message.is_empty());
    }
}

