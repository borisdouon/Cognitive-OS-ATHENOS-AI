/// Phase: A | Step: 5 | Source: Athenos_AI_Strategy.md#L100
/// Edge Observation Agent - OS event logger
/// Captures OS events, app telemetry, optional sensors

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// OS event types captured
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OSEventType {
    AppLaunch,
    AppSwitch,
    AppClose,
    WindowFocus,
    WindowUnfocus,
    KeyPress,
    MouseClick,
    SystemSleep,
    SystemWake,
}

/// OS event with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSEvent {
    pub event_type: OSEventType,
    pub app_name: String,
    pub window_title: Option<String>,
    pub timestamp: i64,
    pub metadata: HashMap<String, String>,
}

/// Edge observation agent
/// Source: Athenos_AI_Strategy.md#L19-21
pub struct EdgeObserver {
    events: Vec<OSEvent>,
    max_events: usize,
}

impl EdgeObserver {
    /// Create new edge observer
    pub fn new(max_events: usize) -> Self {
        info!("EdgeObserver::new: Creating edge observer with max_events={}", max_events);
        Self {
            events: Vec::with_capacity(max_events),
            max_events,
        }
    }

    /// Record an OS event
    /// Source: Athenos_AI_Strategy.md#L100
    pub fn record_event(&mut self, event: OSEvent) {
        info!("EdgeObserver::record_event: Recording {:?} from {}", event.event_type, event.app_name);
        self.events.push(event);
        
        // Rotate if exceeds max
        if self.events.len() > self.max_events {
            self.events.remove(0);
        }
    }

    /// Get recent events
    pub fn get_recent_events(&self, limit: usize) -> Vec<OSEvent> {
        let start = self.events.len().saturating_sub(limit);
        self.events[start..].to_vec()
    }

    /// Get app sequence pattern (last N apps)
    pub fn get_app_sequence(&self, n: usize) -> Vec<String> {
        self.get_recent_events(n)
            .iter()
            .filter_map(|e| {
                match e.event_type {
                    OSEventType::AppLaunch | OSEventType::AppSwitch | OSEventType::WindowFocus => {
                        Some(e.app_name.clone())
                    }
                    _ => None,
                }
            })
            .collect()
    }

    /// Clear all events
    pub fn clear(&mut self) {
        info!("EdgeObserver::clear: Clearing {} events", self.events.len());
        self.events.clear();
    }
}

impl Default for EdgeObserver {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_observer_creation() {
        let observer = EdgeObserver::new(100);
        assert_eq!(observer.events.len(), 0);
    }

    #[test]
    fn test_record_event() {
        let mut observer = EdgeObserver::new(10);
        let event = OSEvent {
            event_type: OSEventType::AppLaunch,
            app_name: "Teams".to_string(),
            window_title: None,
            timestamp: 1234567890,
            metadata: HashMap::new(),
        };
        
        observer.record_event(event);
        assert_eq!(observer.events.len(), 1);
        assert_eq!(observer.events[0].app_name, "Teams");
    }

    #[test]
    fn test_app_sequence() {
        let mut observer = EdgeObserver::new(10);
        
        observer.record_event(OSEvent {
            event_type: OSEventType::AppLaunch,
            app_name: "Teams".to_string(),
            window_title: None,
            timestamp: 1,
            metadata: HashMap::new(),
        });
        
        observer.record_event(OSEvent {
            event_type: OSEventType::AppSwitch,
            app_name: "Gmail".to_string(),
            window_title: None,
            timestamp: 2,
            metadata: HashMap::new(),
        });
        
        observer.record_event(OSEvent {
            event_type: OSEventType::WindowFocus,
            app_name: "IDE".to_string(),
            window_title: None,
            timestamp: 3,
            metadata: HashMap::new(),
        });
        
        let sequence = observer.get_app_sequence(10);
        assert_eq!(sequence, vec!["Teams", "Gmail", "IDE"]);
    }

    #[test]
    fn test_event_rotation() {
        let mut observer = EdgeObserver::new(2);
        
        for i in 0..5 {
            observer.record_event(OSEvent {
                event_type: OSEventType::AppLaunch,
                app_name: format!("App{}", i),
                window_title: None,
                timestamp: i,
                metadata: HashMap::new(),
            });
        }
        
        assert_eq!(observer.events.len(), 2);
        assert_eq!(observer.events[0].app_name, "App3");
        assert_eq!(observer.events[1].app_name, "App4");
    }
}

