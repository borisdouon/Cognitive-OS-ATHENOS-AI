/// Phase: D | Step: 9 | Source: Athenos_AI_Strategy.md#L140
/// Developer API
/// Release developer API for custom observation hooks and interventions

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// API key for developer access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIKey {
    pub key: String,
    pub developer_id: String,
    pub permissions: Vec<APIPermission>,
    pub created_at: i64,
    pub expires_at: Option<i64>,
}

/// API permission
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum APIPermission {
    ReadObservations,
    WriteInterventions,
    ReadMetrics,
    WriteHooks,
}

/// Custom observation hook
/// Source: Athenos_AI_Strategy.md#L140
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationHook {
    pub id: String,
    pub developer_id: String,
    pub hook_type: HookType,
    pub callback_url: Option<String>,
    pub filter: HashMap<String, String>, // Filter criteria
    pub active: bool,
}

/// Hook type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HookType {
    OnPatternDetected,
    OnActionExecuted,
    OnOutcomeRecorded,
}

/// Custom intervention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomIntervention {
    pub id: String,
    pub developer_id: String,
    pub intervention_type: String,
    pub action: Action,
    pub conditions: HashMap<String, f64>, // Conditions for triggering
}

/// Developer API manager
/// Source: Athenos_AI_Strategy.md#L140
pub struct DeveloperAPIManager {
    api_keys: HashMap<String, APIKey>,
    hooks: HashMap<String, ObservationHook>,
    interventions: HashMap<String, CustomIntervention>,
}

impl DeveloperAPIManager {
    /// Create new developer API manager
    pub fn new() -> Self {
        info!("DeveloperAPIManager::new: Creating developer API manager");
        Self {
            api_keys: HashMap::new(),
            hooks: HashMap::new(),
            interventions: HashMap::new(),
        }
    }

    /// Register API key
    /// Source: Athenos_AI_Strategy.md#L140
    pub fn register_api_key(&mut self, developer_id: String, permissions: Vec<APIPermission>) -> APIKey {
        info!("DeveloperAPIManager::register_api_key: Registering API key for developer {}", developer_id);
        
        let api_key = APIKey {
            key: format!("athenos_{}", chrono::Utc::now().timestamp()),
            developer_id: developer_id.clone(),
            permissions,
            created_at: chrono::Utc::now().timestamp(),
            expires_at: None,
        };
        
        self.api_keys.insert(api_key.key.clone(), api_key.clone());
        api_key
    }

    /// Register observation hook
    /// Source: Athenos_AI_Strategy.md#L140
    pub fn register_hook(&mut self, hook: ObservationHook) {
        info!("DeveloperAPIManager::register_hook: Registering hook {}", hook.id);
        self.hooks.insert(hook.id.clone(), hook);
    }

    /// Register custom intervention
    pub fn register_intervention(&mut self, intervention: CustomIntervention) {
        info!("DeveloperAPIManager::register_intervention: Registering intervention {}", intervention.id);
        self.interventions.insert(intervention.id.clone(), intervention);
    }

    /// Validate API key
    pub fn validate_api_key(&self, key: &str) -> Option<&APIKey> {
        self.api_keys.get(key)
    }

    /// Get hooks for developer
    pub fn get_developer_hooks(&self, developer_id: &str) -> Vec<&ObservationHook> {
        self.hooks
            .values()
            .filter(|h| h.developer_id == developer_id && h.active)
            .collect()
    }
}

impl Default for DeveloperAPIManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_manager_creation() {
        let manager = DeveloperAPIManager::new();
        assert_eq!(manager.api_keys.len(), 0);
    }

    #[test]
    fn test_register_api_key() {
        let mut manager = DeveloperAPIManager::new();
        let api_key = manager.register_api_key(
            "dev_001".to_string(),
            vec![APIPermission::ReadObservations, APIPermission::WriteHooks],
        );
        
        assert_eq!(manager.api_keys.len(), 1);
        assert!(manager.validate_api_key(&api_key.key).is_some());
    }

    #[test]
    fn test_register_hook() {
        let mut manager = DeveloperAPIManager::new();
        let hook = ObservationHook {
            id: "hook_001".to_string(),
            developer_id: "dev_001".to_string(),
            hook_type: HookType::OnPatternDetected,
            callback_url: Some("https://example.com/webhook".to_string()),
            filter: HashMap::new(),
            active: true,
        };
        
        manager.register_hook(hook);
        assert_eq!(manager.hooks.len(), 1);
        assert_eq!(manager.get_developer_hooks("dev_001").len(), 1);
    }
}

