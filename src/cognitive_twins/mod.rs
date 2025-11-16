/// Phase: D | Step: 3 | Source: Athenos_AI_Strategy.md#L134
/// Multi-Persona Cognitive Twins
/// Launch multi-persona cognitive twins (developer, manager, creative coaches)

use crate::types::*;
use crate::wisdom::WisdomEngine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Cognitive twin persona
/// Source: Athenos_AI_Strategy.md#L134
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveTwin {
    pub user_id: String,
    pub persona: UserProfile,
    pub wisdom_engine: WisdomEngine,
    pub behavioral_model: HashMap<String, f64>,
    pub created_at: i64,
}

/// Multi-persona cognitive twin manager
/// Source: Athenos_AI_Strategy.md#L134
pub struct CognitiveTwinManager {
    twins: HashMap<String, CognitiveTwin>,
    persona_coaches: HashMap<UserProfile, String>, // Persona -> coach description
}

impl CognitiveTwinManager {
    /// Create new cognitive twin manager
    pub fn new() -> Self {
        info!("CognitiveTwinManager::new: Creating cognitive twin manager");
        
        let mut persona_coaches = HashMap::new();
        persona_coaches.insert(UserProfile::Developer, 
            "Developer Coach: Focuses on code quality, debugging efficiency, and technical workflow optimization.".to_string());
        persona_coaches.insert(UserProfile::Manager,
            "Manager Coach: Emphasizes team coordination, decision-making clarity, and strategic focus.".to_string());
        persona_coaches.insert(UserProfile::Designer,
            "Creative Coach: Supports creative flow, design iteration, and visual workflow optimization.".to_string());
        
        Self {
            twins: HashMap::new(),
            persona_coaches,
        }
    }

    /// Create cognitive twin for user
    /// Source: Athenos_AI_Strategy.md#L134
    pub fn create_twin(&mut self, user_id: String, persona: UserProfile) -> CognitiveTwin {
        info!("CognitiveTwinManager::create_twin: Creating twin for user {} with persona {:?}", user_id, persona);
        
        let twin = CognitiveTwin {
            user_id: user_id.clone(),
            persona: persona.clone(),
            wisdom_engine: WisdomEngine::new(),
            behavioral_model: HashMap::new(),
            created_at: chrono::Utc::now().timestamp(),
        };
        
        self.twins.insert(user_id.clone(), twin.clone());
        twin
    }

    /// Get cognitive twin for user
    pub fn get_twin(&self, user_id: &str) -> Option<&CognitiveTwin> {
        self.twins.get(user_id)
    }

    /// Update behavioral model from observation
    pub fn update_behavioral_model(&mut self, user_id: &str, observation: &Observation) {
        if let Some(twin) = self.twins.get_mut(user_id) {
            // Update behavioral patterns
            for (key, value) in &observation.metrics {
                twin.behavioral_model.insert(key.clone(), *value);
            }
        }
    }

    /// Get personalized insight from twin
    /// Source: Athenos_AI_Strategy.md#L134
    pub fn get_persona_insight(&self, user_id: &str, observation: &Observation) -> Option<String> {
        if let Some(twin) = self.twins.get(user_id) {
            let coach_desc = self.persona_coaches.get(&twin.persona)
                .map(|s| s.as_str())
                .unwrap_or("General coach");
            
            let insight = twin.wisdom_engine.generate_insight(observation, coach_desc);
            Some(format!("[{}] {}", coach_desc, insight))
        } else {
            None
        }
    }

    /// List all cognitive twins
    pub fn list_twins(&self) -> Vec<&CognitiveTwin> {
        self.twins.values().collect()
    }
}

impl Default for CognitiveTwinManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_cognitive_twin_manager_creation() {
        let manager = CognitiveTwinManager::new();
        assert_eq!(manager.twins.len(), 0);
        assert_eq!(manager.persona_coaches.len(), 3);
    }

    #[test]
    fn test_create_twin() {
        let mut manager = CognitiveTwinManager::new();
        let twin = manager.create_twin("user_001".to_string(), UserProfile::Developer);
        
        assert_eq!(twin.user_id, "user_001");
        assert_eq!(twin.persona, UserProfile::Developer);
        assert!(manager.get_twin("user_001").is_some());
    }

    #[test]
    fn test_get_persona_insight() {
        let mut manager = CognitiveTwinManager::new();
        manager.create_twin("user_001".to_string(), UserProfile::Developer);
        
        let observation = Observation {
            id: "test".to_string(),
            profile: UserProfile::Developer,
            observation: vec!["IDE".to_string()],
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
        };
        
        let insight = manager.get_persona_insight("user_001", &observation);
        assert!(insight.is_some());
        let insight = insight.unwrap();
        assert!(insight.contains("Developer Coach"));
    }
}

