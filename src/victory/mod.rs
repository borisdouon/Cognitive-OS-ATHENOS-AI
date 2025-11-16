/// Phase: C | Step: 6 | Source: Athenos_AI_Strategy.md#L125
/// Victory Stream
/// Establish victory stream (quantified daily wins) to drive retention

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Daily victory/win
/// Source: Athenos_AI_Strategy.md#L125
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Victory {
    pub id: String,
    pub title: String,
    pub description: String,
    pub metric: VictoryMetric,
    pub value: f64,
    pub timestamp: i64,
    pub category: VictoryCategory,
}

/// Victory metric type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VictoryMetric {
    TimeSaved,
    FocusIncrease,
    PatternOptimized,
    ErrorReduced,
    HabitFormed,
}

/// Victory category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VictoryCategory {
    Productivity,
    Focus,
    Automation,
    Learning,
    Wellbeing,
}

/// Victory stream manager
/// Source: Athenos_AI_Strategy.md#L125
pub struct VictoryStream {
    victories: Vec<Victory>,
    daily_victories: HashMap<String, Vec<Victory>>, // date -> victories
}

impl VictoryStream {
    /// Create new victory stream
    pub fn new() -> Self {
        info!("VictoryStream::new: Creating victory stream");
        Self {
            victories: Vec::new(),
            daily_victories: HashMap::new(),
        }
    }

    /// Record a victory
    /// Source: Athenos_AI_Strategy.md#L125
    pub fn record_victory(&mut self, title: String, description: String, metric: VictoryMetric, value: f64, category: VictoryCategory) {
        info!("VictoryStream::record_victory: Recording victory: {}", title);
        
        let victory = Victory {
            id: format!("victory_{}", chrono::Utc::now().timestamp()),
            title,
            description,
            metric: metric.clone(),
            value,
            timestamp: chrono::Utc::now().timestamp(),
            category,
        };
        
        let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
        self.victories.push(victory.clone());
        self.daily_victories
            .entry(date)
            .or_insert_with(Vec::new)
            .push(victory);
    }

    /// Record victory from observation outcome
    pub fn record_from_outcome(&mut self, outcome: &Outcome, observation: &Observation) {
        if let Some(time_saved) = outcome.time_saved_minutes {
            if time_saved > 5.0 {
                self.record_victory(
                    format!("Saved {} minutes!", time_saved as i64),
                    format!("Optimized workflow: {}", observation.action.description),
                    VictoryMetric::TimeSaved,
                    time_saved,
                    VictoryCategory::Productivity,
                );
            }
        }
    }

    /// Get today's victories
    pub fn get_today_victories(&self) -> Vec<&Victory> {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        self.daily_victories
            .get(&today)
            .map(|v| v.iter().collect())
            .unwrap_or_else(Vec::new)
    }

    /// Get victory summary for date
    pub fn get_daily_summary(&self, date: &str) -> VictorySummary {
        let victories = self.daily_victories
            .get(date)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        
        let total_time_saved: f64 = victories
            .iter()
            .filter(|v| v.metric == VictoryMetric::TimeSaved)
            .map(|v| v.value)
            .sum();
        
        let total_victories = victories.len();
        
        VictorySummary {
            date: date.to_string(),
            total_victories,
            total_time_saved_min: total_time_saved,
            categories: victories.iter().map(|v| v.category.clone()).collect(),
        }
    }

    /// Get recent victories
    pub fn get_recent_victories(&self, limit: usize) -> Vec<&Victory> {
        let start = self.victories.len().saturating_sub(limit);
        self.victories[start..].iter().collect()
    }
}

/// Victory summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictorySummary {
    pub date: String,
    pub total_victories: usize,
    pub total_time_saved_min: f64,
    pub categories: Vec<VictoryCategory>,
}

impl Default for VictoryStream {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_victory_stream_creation() {
        let stream = VictoryStream::new();
        assert_eq!(stream.victories.len(), 0);
    }

    #[test]
    fn test_record_victory() {
        let mut stream = VictoryStream::new();
        stream.record_victory(
            "Saved 11 minutes!".to_string(),
            "Optimized workflow".to_string(),
            VictoryMetric::TimeSaved,
            11.0,
            VictoryCategory::Productivity,
        );
        
        assert_eq!(stream.victories.len(), 1);
        assert!(!stream.get_today_victories().is_empty());
    }

    #[test]
    fn test_record_from_outcome() {
        let mut stream = VictoryStream::new();
        let outcome = Outcome {
            observation_id: "test".to_string(),
            accepted: true,
            ignored: false,
            modified: false,
            time_saved_minutes: Some(11.0),
            error_rate_change: None,
            timestamp: 1234567890,
        };
        
        let observation = Observation {
            id: "test".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["Teams".to_string()],
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
        
        stream.record_from_outcome(&outcome, &observation);
        assert!(!stream.get_today_victories().is_empty());
    }

    #[test]
    fn test_daily_summary() {
        let mut stream = VictoryStream::new();
        stream.record_victory(
            "Saved time".to_string(),
            "Test".to_string(),
            VictoryMetric::TimeSaved,
            11.0,
            VictoryCategory::Productivity,
        );
        
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let summary = stream.get_daily_summary(&today);
        assert_eq!(summary.total_victories, 1);
        assert_eq!(summary.total_time_saved_min, 11.0);
    }
}

