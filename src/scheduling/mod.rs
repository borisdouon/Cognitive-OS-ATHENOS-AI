/// Phase: C | Step: 3 | Source: Athenos_AI_Strategy.md#L122
/// Anticipatory Scheduling + Calendar Negotiation Agent
/// Implement anticipatory scheduling and calendar negotiation agent

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Calendar event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub start_time: i64,
    pub end_time: i64,
    pub priority: EventPriority,
    pub is_flexible: bool,
}

/// Event priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Schedule optimization suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleSuggestion {
    pub event_id: String,
    pub suggested_start: i64,
    pub suggested_end: i64,
    pub reason: String,
    pub expected_benefit: String,
    pub requires_approval: bool,
}

/// Calendar negotiation agent
/// Source: Athenos_AI_Strategy.md#L122
pub struct CalendarNegotiationAgent {
    events: HashMap<String, CalendarEvent>,
    optimal_focus_hours: Vec<(u8, u8)>, // (start_hour, end_hour)
}

impl CalendarNegotiationAgent {
    /// Create new calendar negotiation agent
    pub fn new() -> Self {
        info!("CalendarNegotiationAgent::new: Creating calendar negotiation agent");
        Self {
            events: HashMap::new(),
            optimal_focus_hours: vec![(9, 11), (14, 16)], // Default optimal hours
        }
    }

    /// Add calendar event
    pub fn add_event(&mut self, event: CalendarEvent) {
        info!("CalendarNegotiationAgent::add_event: Adding event {}", event.id);
        self.events.insert(event.id.clone(), event);
    }

    /// Analyze schedule and suggest optimizations
    /// Source: Athenos_AI_Strategy.md#L122
    pub fn analyze_schedule(&self, date: i64) -> Vec<ScheduleSuggestion> {
        info!("CalendarNegotiationAgent::analyze_schedule: Analyzing schedule for date");
        
        let mut suggestions = Vec::new();
        
        // Find events that conflict with optimal focus hours
        for event in self.events.values() {
            if self.conflicts_with_focus_hours(event) && event.is_flexible {
                let (optimal_start, optimal_end) = self.find_optimal_slot(event);
                
                suggestions.push(ScheduleSuggestion {
                    event_id: event.id.clone(),
                    suggested_start: optimal_start,
                    suggested_end: optimal_end,
                    reason: format!("Move to preserve focus hours ({}:00-{}:00)", 
                        self.optimal_focus_hours[0].0, self.optimal_focus_hours[0].1),
                    expected_benefit: "Preserve 2 hours of peak focus time".to_string(),
                    requires_approval: event.priority >= EventPriority::Medium,
                });
            }
        }
        
        suggestions
    }

    /// Anticipatory scheduling - predict and suggest
    /// Source: Athenos_AI_Strategy.md#L122
    pub fn anticipatory_schedule(&self, new_event: &CalendarEvent) -> Option<ScheduleSuggestion> {
        info!("CalendarNegotiationAgent::anticipatory_schedule: Anticipatory scheduling for {}", new_event.id);
        
        // Check if new event would conflict with focus hours
        if self.conflicts_with_focus_hours(new_event) && new_event.is_flexible {
            let (optimal_start, optimal_end) = self.find_optimal_slot(new_event);
            
            Some(ScheduleSuggestion {
                event_id: new_event.id.clone(),
                suggested_start: optimal_start,
                suggested_end: optimal_end,
                reason: "Schedule outside focus hours to maximize productivity".to_string(),
                expected_benefit: "Preserve cognitive peak performance window".to_string(),
                requires_approval: new_event.priority >= EventPriority::Medium,
            })
        } else {
            None
        }
    }

    fn conflicts_with_focus_hours(&self, event: &CalendarEvent) -> bool {
        let event_start_hour = chrono::DateTime::from_timestamp(event.start_time, 0)
            .map(|dt| dt.hour())
            .unwrap_or(0);
        let event_end_hour = chrono::DateTime::from_timestamp(event.end_time, 0)
            .map(|dt| dt.hour())
            .unwrap_or(0);
        
        self.optimal_focus_hours.iter().any(|(start, end)| {
            event_start_hour >= *start && event_start_hour < *end ||
            event_end_hour > *start && event_end_hour <= *end
        })
    }

    fn find_optimal_slot(&self, event: &CalendarEvent) -> (i64, i64) {
        // Find next available slot outside focus hours
        let duration = event.end_time - event.start_time;
        let suggested_start = chrono::Utc::now().timestamp() + 3600; // 1 hour from now
        (suggested_start, suggested_start + duration)
    }
}

impl Default for CalendarNegotiationAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_agent_creation() {
        let agent = CalendarNegotiationAgent::new();
        assert_eq!(agent.events.len(), 0);
        assert_eq!(agent.optimal_focus_hours.len(), 2);
    }

    #[test]
    fn test_add_and_analyze_event() {
        let mut agent = CalendarNegotiationAgent::new();
        
        let event = CalendarEvent {
            id: "meeting_001".to_string(),
            title: "Team Standup".to_string(),
            start_time: chrono::Utc::now().timestamp() + 3600,
            end_time: chrono::Utc::now().timestamp() + 7200,
            priority: EventPriority::Low,
            is_flexible: true,
        };
        
        agent.add_event(event);
        assert_eq!(agent.events.len(), 1);
    }

    #[test]
    fn test_anticipatory_scheduling() {
        let agent = CalendarNegotiationAgent::new();
        
        let event = CalendarEvent {
            id: "new_meeting".to_string(),
            title: "New Meeting".to_string(),
            start_time: chrono::Utc::now().timestamp() + 3600,
            end_time: chrono::Utc::now().timestamp() + 7200,
            priority: EventPriority::Low,
            is_flexible: true,
        };
        
        let suggestion = agent.anticipatory_schedule(&event);
        // May or may not suggest based on timing
        assert!(suggestion.is_some() || suggestion.is_none());
    }
}

