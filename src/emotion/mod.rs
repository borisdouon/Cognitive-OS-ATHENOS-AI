/// Phase: B | Step: 6 | Source: Athenos_AI_Strategy.md#L113
/// Mood-Adaptive Focus Mode
/// Enable mood-adaptive focus mode (emotion estimator + UI adjustments)

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Emotion estimate from behavioral signals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionEstimate {
    pub emotional_state: EmotionalState,
    pub confidence: f64, // 0.0 to 1.0
    pub signals: Vec<String>,
    pub timestamp: i64,
}

/// Emotion estimator (stub for Phase B)
/// Source: Athenos_AI_Strategy.md#L113
pub struct EmotionEstimator {
    signal_weights: HashMap<String, f64>,
}

impl EmotionEstimator {
    /// Create new emotion estimator
    pub fn new() -> Self {
        info!("EmotionEstimator::new: Creating emotion estimator");
        let mut signal_weights = HashMap::new();
        signal_weights.insert("typing_speed_decrease".to_string(), 0.3);
        signal_weights.insert("error_rate_increase".to_string(), 0.25);
        signal_weights.insert("context_switch_frequency".to_string(), 0.2);
        signal_weights.insert("session_duration".to_string(), 0.25);
        
        Self { signal_weights }
    }

    /// Estimate emotion from behavioral signals
    /// Source: Athenos_AI_Strategy.md#L113
    pub fn estimate_emotion(&self, metrics: &HashMap<String, f64>) -> EmotionEstimate {
        info!("EmotionEstimator::estimate_emotion: Estimating emotion from metrics");
        
        let mut signals = Vec::new();
        let mut stress_score = 0.0;
        
        // Check typing speed decrease
        if let Some(speed_decrease) = metrics.get("typing_speed_decrease_pct") {
            if *speed_decrease > 30.0 {
                signals.push("Slow typing detected".to_string());
                stress_score += 0.3;
            }
        }
        
        // Check error rate
        if let Some(error_rate) = metrics.get("error_rate") {
            if *error_rate > 0.15 {
                signals.push("High error rate".to_string());
                stress_score += 0.25;
            }
        }
        
        // Check context switching
        if let Some(context_switches) = metrics.get("context_switch_count") {
            if *context_switches > 10.0 {
                signals.push("Frequent context switching".to_string());
                stress_score += 0.2;
            }
        }
        
        // Check session duration
        if let Some(session_duration) = metrics.get("session_duration_min") {
            if *session_duration > 120.0 {
                signals.push("Long session detected".to_string());
                stress_score += 0.25;
            }
        }
        
        let emotional_state = if stress_score > 0.6 {
            EmotionalState::Stressed
        } else if stress_score > 0.3 {
            EmotionalState::Fatigued
        } else if metrics.get("focus_duration_min").copied().unwrap_or(0.0) > 60.0 {
            EmotionalState::Focused
        } else {
            EmotionalState::Calm
        };
        
        EmotionEstimate {
            emotional_state,
            confidence: stress_score.min(1.0),
            signals,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

/// Focus mode adjustments based on emotion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusModeAdjustments {
    pub reduce_notifications: bool,
    pub dim_screen: bool,
    pub enable_zen_mode: bool,
    pub suggest_break: bool,
    pub breathing_guidance: bool,
}

/// Mood-adaptive focus mode
/// Source: Athenos_AI_Strategy.md#L113
pub struct MoodAdaptiveFocusMode {
    emotion_estimator: EmotionEstimator,
    current_adjustments: Option<FocusModeAdjustments>,
}

impl MoodAdaptiveFocusMode {
    /// Create new mood-adaptive focus mode
    pub fn new() -> Self {
        info!("MoodAdaptiveFocusMode::new: Creating mood-adaptive focus mode");
        Self {
            emotion_estimator: EmotionEstimator::new(),
            current_adjustments: None,
        }
    }

    /// Update focus mode based on emotion estimate
    /// Source: Athenos_AI_Strategy.md#L113
    pub fn update_focus_mode(&mut self, metrics: &HashMap<String, f64>) -> FocusModeAdjustments {
        info!("MoodAdaptiveFocusMode::update_focus_mode: Updating focus mode");
        
        let emotion = self.emotion_estimator.estimate_emotion(metrics);
        
        let adjustments = match emotion.emotional_state {
            EmotionalState::Stressed => FocusModeAdjustments {
                reduce_notifications: true,
                dim_screen: true,
                enable_zen_mode: true,
                suggest_break: true,
                breathing_guidance: true,
            },
            EmotionalState::Fatigued => FocusModeAdjustments {
                reduce_notifications: true,
                dim_screen: false,
                enable_zen_mode: false,
                suggest_break: true,
                breathing_guidance: false,
            },
            EmotionalState::Focused => FocusModeAdjustments {
                reduce_notifications: true,
                dim_screen: false,
                enable_zen_mode: false,
                suggest_break: false,
                breathing_guidance: false,
            },
            _ => FocusModeAdjustments {
                reduce_notifications: false,
                dim_screen: false,
                enable_zen_mode: false,
                suggest_break: false,
                breathing_guidance: false,
            },
        };
        
        self.current_adjustments = Some(adjustments.clone());
        adjustments
    }
}

impl Default for MoodAdaptiveFocusMode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotion_estimator_creation() {
        let estimator = EmotionEstimator::new();
        assert_eq!(estimator.signal_weights.len(), 4);
    }

    #[test]
    fn test_estimate_stressed_emotion() {
        let estimator = EmotionEstimator::new();
        let mut metrics = HashMap::new();
        metrics.insert("typing_speed_decrease_pct".to_string(), 40.0);
        metrics.insert("error_rate".to_string(), 0.2);
        metrics.insert("context_switch_count".to_string(), 15.0);
        
        let estimate = estimator.estimate_emotion(&metrics);
        assert_eq!(estimate.emotional_state, EmotionalState::Stressed);
        assert!(!estimate.signals.is_empty());
    }

    #[test]
    fn test_mood_adaptive_focus_mode() {
        let mut focus_mode = MoodAdaptiveFocusMode::new();
        let mut metrics = HashMap::new();
        metrics.insert("typing_speed_decrease_pct".to_string(), 40.0);
        metrics.insert("error_rate".to_string(), 0.2);
        
        let adjustments = focus_mode.update_focus_mode(&metrics);
        assert!(adjustments.reduce_notifications);
        assert!(adjustments.enable_zen_mode);
        assert!(adjustments.suggest_break);
    }
}

