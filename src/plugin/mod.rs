/// Phase: C | Step: 9 | Source: Athenos_AI_Strategy.md#L128
/// Plugin SDK Prototype
/// Prepare plugin SDK for internal teams; prototype external partner integration

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Plugin capability
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginCapability {
    Observation,
    Intervention,
    Analysis,
    Visualization,
}

/// Plugin metadata
/// Source: Athenos_AI_Strategy.md#L128
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub capabilities: Vec<PluginCapability>,
    pub description: String,
}

/// Plugin interface trait (stub)
/// Note: In production, would use proper trait objects or enum dispatch
pub trait Plugin: Send + Sync {
    fn metadata(&self) -> &PluginMetadata;
    fn execute(&self, input: &str) -> Result<String, String>;
}

/// Plugin registry
/// Source: Athenos_AI_Strategy.md#L128
pub struct PluginRegistry {
    metadata: HashMap<String, PluginMetadata>,
}

impl PluginRegistry {
    /// Create new plugin registry
    pub fn new() -> Self {
        info!("PluginRegistry::new: Creating plugin registry");
        Self {
            metadata: HashMap::new(),
        }
    }

    /// Register plugin
    /// Source: Athenos_AI_Strategy.md#L128
    pub fn register_plugin(&mut self, metadata: PluginMetadata) {
        info!("PluginRegistry::register_plugin: Registering plugin {}", metadata.id);
        self.metadata.insert(metadata.id.clone(), metadata);
    }

    /// Get plugin metadata
    pub fn get_plugin_metadata(&self, plugin_id: &str) -> Option<&PluginMetadata> {
        self.metadata.get(plugin_id)
    }

    /// List all plugins
    pub fn list_plugins(&self) -> Vec<&PluginMetadata> {
        self.metadata.values().collect()
    }

    /// Execute plugin (stub)
    pub fn execute_plugin(&self, plugin_id: &str, input: &str) -> Result<String, String> {
        info!("PluginRegistry::execute_plugin: Executing plugin {}", plugin_id);
        
        if self.metadata.contains_key(plugin_id) {
            Ok(format!("Plugin {} executed with input: {}", plugin_id, input))
        } else {
            Err("Plugin not found".to_string())
        }
    }
}

/// Example internal plugin (stub)
pub struct InternalPlugin {
    metadata: PluginMetadata,
}

impl InternalPlugin {
    pub fn new(name: String, author: String) -> Self {
        Self {
            metadata: PluginMetadata {
                id: format!("internal_{}", name.to_lowercase().replace(" ", "_")),
                name,
                version: "1.0.0".to_string(),
                author,
                capabilities: vec![PluginCapability::Observation, PluginCapability::Analysis],
                description: "Internal plugin prototype".to_string(),
            },
        }
    }
}

impl Plugin for InternalPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(format!("Internal plugin processed: {}", input))
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_registry_creation() {
        let registry = PluginRegistry::new();
        assert_eq!(registry.metadata.len(), 0);
    }

    #[test]
    fn test_register_and_list_plugin() {
        let mut registry = PluginRegistry::new();
        let plugin = InternalPlugin::new("Test Plugin".to_string(), "Test Author".to_string());
        let metadata = plugin.metadata().clone();
        
        registry.register_plugin(metadata.clone());
        
        assert_eq!(registry.list_plugins().len(), 1);
        assert!(registry.get_plugin_metadata(&metadata.id).is_some());
    }

    #[test]
    fn test_execute_plugin() {
        let mut registry = PluginRegistry::new();
        let plugin = InternalPlugin::new("Test Plugin".to_string(), "Test Author".to_string());
        let metadata = plugin.metadata().clone();
        
        registry.register_plugin(metadata.clone());
        
        let result = registry.execute_plugin(&metadata.id, "test input");
        assert!(result.is_ok());
    }
}

