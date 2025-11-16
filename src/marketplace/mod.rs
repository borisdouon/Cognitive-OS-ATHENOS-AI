/// Phase: D | Step: 4 | Source: Athenos_AI_Strategy.md#L135
/// Automation Marketplace
/// Offer automation marketplace with curated third-party plugins

use crate::plugin::PluginMetadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Marketplace plugin listing
/// Source: Athenos_AI_Strategy.md#L135
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplacePlugin {
    pub metadata: PluginMetadata,
    pub price: f64,
    pub rating: f64, // 0.0 to 5.0
    pub download_count: usize,
    pub verified: bool,
    pub category: PluginCategory,
}

/// Plugin category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginCategory {
    Productivity,
    Focus,
    Automation,
    Learning,
    Wellbeing,
}

/// Automation marketplace
/// Source: Athenos_AI_Strategy.md#L135
pub struct AutomationMarketplace {
    plugins: HashMap<String, MarketplacePlugin>,
    curated_plugins: Vec<String>, // Plugin IDs that are curated/verified
}

impl AutomationMarketplace {
    /// Create new marketplace
    pub fn new() -> Self {
        info!("AutomationMarketplace::new: Creating automation marketplace");
        Self {
            plugins: HashMap::new(),
            curated_plugins: Vec::new(),
        }
    }

    /// Add plugin to marketplace
    /// Source: Athenos_AI_Strategy.md#L135
    pub fn add_plugin(&mut self, plugin: MarketplacePlugin) {
        info!("AutomationMarketplace::add_plugin: Adding plugin {}", plugin.metadata.id);
        let plugin_id = plugin.metadata.id.clone();
        
        if plugin.verified {
            self.curated_plugins.push(plugin_id.clone());
        }
        
        self.plugins.insert(plugin_id, plugin);
    }

    /// Get curated plugins
    /// Source: Athenos_AI_Strategy.md#L135
    pub fn get_curated_plugins(&self) -> Vec<&MarketplacePlugin> {
        self.curated_plugins
            .iter()
            .filter_map(|id| self.plugins.get(id))
            .collect()
    }

    /// Search plugins by category
    pub fn search_by_category(&self, category: PluginCategory) -> Vec<&MarketplacePlugin> {
        self.plugins
            .values()
            .filter(|p| p.category == category)
            .collect()
    }

    /// Get top-rated plugins
    pub fn get_top_rated(&self, limit: usize) -> Vec<&MarketplacePlugin> {
        let mut plugins: Vec<&MarketplacePlugin> = self.plugins.values().collect();
        plugins.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap());
        plugins.into_iter().take(limit).collect()
    }

    /// Install plugin (simulated)
    pub fn install_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.download_count += 1;
            Ok(())
        } else {
            Err("Plugin not found".to_string())
        }
    }
}

impl Default for AutomationMarketplace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::PluginCapability;

    #[test]
    fn test_marketplace_creation() {
        let marketplace = AutomationMarketplace::new();
        assert_eq!(marketplace.plugins.len(), 0);
    }

    #[test]
    fn test_add_and_get_curated_plugin() {
        let mut marketplace = AutomationMarketplace::new();
        let plugin = MarketplacePlugin {
            metadata: PluginMetadata {
                id: "plugin_001".to_string(),
                name: "Test Plugin".to_string(),
                version: "1.0.0".to_string(),
                author: "Test Author".to_string(),
                capabilities: vec![PluginCapability::Automation],
                description: "Test".to_string(),
            },
            price: 9.99,
            rating: 4.5,
            download_count: 0,
            verified: true,
            category: PluginCategory::Productivity,
        };
        
        marketplace.add_plugin(plugin);
        assert_eq!(marketplace.get_curated_plugins().len(), 1);
    }

    #[test]
    fn test_install_plugin() {
        let mut marketplace = AutomationMarketplace::new();
        let plugin = MarketplacePlugin {
            metadata: PluginMetadata {
                id: "plugin_002".to_string(),
                name: "Test".to_string(),
                version: "1.0.0".to_string(),
                author: "Author".to_string(),
                capabilities: vec![],
                description: "Test".to_string(),
            },
            price: 0.0,
            rating: 4.0,
            download_count: 0,
            verified: false,
            category: PluginCategory::Automation,
        };
        
        marketplace.add_plugin(plugin);
        marketplace.install_plugin("plugin_002").unwrap();
        
        let installed = marketplace.plugins.get("plugin_002").unwrap();
        assert_eq!(installed.download_count, 1);
    }
}

