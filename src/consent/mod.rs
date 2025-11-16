/// Phase: B | Step: 5 | Source: Athenos_AI_Strategy.md#L112
/// Micro-consent UX + Transparency Timeline
/// Integrate micro-consent UX and transparency timeline

use crate::privacy::ConsentLedger;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Micro-consent request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroConsent {
    pub capability: String,
    pub description: String,
    pub requested_at: i64,
    pub granted_at: Option<i64>,
    pub revoked_at: Option<i64>,
}

/// Transparency timeline entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub timestamp: i64,
    pub event_type: String,
    pub description: String,
    pub data_accessed: Vec<String>,
    pub action_taken: Option<String>,
}

/// Micro-consent manager with transparency timeline
/// Source: Athenos_AI_Strategy.md#L112
pub struct MicroConsentManager {
    consent_ledger: ConsentLedger,
    micro_consents: Vec<MicroConsent>,
    timeline: Vec<TimelineEntry>,
}

impl MicroConsentManager {
    /// Create new micro-consent manager
    pub fn new() -> Self {
        info!("MicroConsentManager::new: Creating micro-consent manager");
        Self {
            consent_ledger: ConsentLedger::new(),
            micro_consents: Vec::new(),
            timeline: Vec::new(),
        }
    }

    /// Request micro-consent for a capability
    /// Source: Athenos_AI_Strategy.md#L112
    pub fn request_consent(&mut self, capability: String, description: String) -> MicroConsent {
        info!("MicroConsentManager::request_consent: Requesting consent for {}", capability);
        
        let consent = MicroConsent {
            capability: capability.clone(),
            description: description.clone(),
            requested_at: chrono::Utc::now().timestamp(),
            granted_at: None,
            revoked_at: None,
        };
        
        self.micro_consents.push(consent.clone());
        
        self.add_timeline_entry(
            "consent_requested".to_string(),
            format!("Requested consent for: {}", description),
            vec![capability],
            None,
        );
        
        consent
    }

    /// Grant micro-consent
    pub fn grant_consent(&mut self, capability: &str) -> Result<(), String> {
        info!("MicroConsentManager::grant_consent: Granting consent for {}", capability);
        
        if let Some(consent) = self.micro_consents.iter_mut().find(|c| c.capability == capability && c.granted_at.is_none()) {
            consent.granted_at = Some(chrono::Utc::now().timestamp());
            
            // Update consent ledger
            match capability {
                "cloud_sync" => self.consent_ledger.opt_in_cloud_sync = true,
                "behavioral_logging" => self.consent_ledger.opt_in_behavioral_logging = true,
                "emotion_detection" => self.consent_ledger.opt_in_emotion_detection = true,
                "automation" => self.consent_ledger.opt_in_automation = true,
                _ => {}
            }
            
            self.add_timeline_entry(
                "consent_granted".to_string(),
                format!("Granted consent for: {}", capability),
                vec![capability.to_string()],
                None,
            );
            
            Ok(())
        } else {
            Err("Consent not found or already granted".to_string())
        }
    }

    /// Revoke micro-consent
    pub fn revoke_consent(&mut self, capability: &str, reason: Option<String>) -> Result<(), String> {
        info!("MicroConsentManager::revoke_consent: Revoking consent for {}", capability);
        
        if let Some(consent) = self.micro_consents.iter_mut().find(|c| c.capability == capability) {
            consent.revoked_at = Some(chrono::Utc::now().timestamp());
            self.consent_ledger.revoke_consent(capability.to_string(), reason.clone());
            
            self.add_timeline_entry(
                "consent_revoked".to_string(),
                format!("Revoked consent for: {} - reason: {:?}", capability, reason),
                vec![capability.to_string()],
                None,
            );
            
            Ok(())
        } else {
            Err("Consent not found".to_string())
        }
    }

    /// Add timeline entry for transparency
    /// Source: Strategic_Reinforcements_Gap_Closures.md#L14
    pub fn add_timeline_entry(&mut self, event_type: String, description: String, data_accessed: Vec<String>, action_taken: Option<String>) {
        let entry = TimelineEntry {
            timestamp: chrono::Utc::now().timestamp(),
            event_type,
            description,
            data_accessed,
            action_taken,
        };
        
        self.timeline.push(entry);
        
        // Keep only last 1000 entries
        if self.timeline.len() > 1000 {
            self.timeline.remove(0);
        }
    }

    /// Get transparency timeline
    pub fn get_timeline(&self, limit: Option<usize>) -> Vec<&TimelineEntry> {
        let limit = limit.unwrap_or(100);
        let start = self.timeline.len().saturating_sub(limit);
        self.timeline[start..].iter().collect()
    }

    /// Check if capability has consent
    pub fn has_consent(&self, capability: &str) -> bool {
        self.micro_consents
            .iter()
            .any(|c| c.capability == capability && c.granted_at.is_some() && c.revoked_at.is_none())
    }
}

impl Default for MicroConsentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_micro_consent_manager_creation() {
        let manager = MicroConsentManager::new();
        assert_eq!(manager.micro_consents.len(), 0);
    }

    #[test]
    fn test_request_and_grant_consent() {
        let mut manager = MicroConsentManager::new();
        
        let consent = manager.request_consent(
            "cloud_sync".to_string(),
            "Sync data to cloud for federated learning".to_string(),
        );
        
        assert_eq!(consent.capability, "cloud_sync");
        assert!(consent.granted_at.is_none());
        
        manager.grant_consent("cloud_sync").unwrap();
        assert!(manager.has_consent("cloud_sync"));
    }

    #[test]
    fn test_revoke_consent() {
        let mut manager = MicroConsentManager::new();
        
        manager.request_consent("cloud_sync".to_string(), "Test".to_string());
        manager.grant_consent("cloud_sync").unwrap();
        assert!(manager.has_consent("cloud_sync"));
        
        manager.revoke_consent("cloud_sync", Some("Privacy concern".to_string())).unwrap();
        assert!(!manager.has_consent("cloud_sync"));
    }

    #[test]
    fn test_timeline_entries() {
        let mut manager = MicroConsentManager::new();
        
        manager.request_consent("behavioral_logging".to_string(), "Test".to_string());
        manager.add_timeline_entry(
            "observation".to_string(),
            "Observed app sequence".to_string(),
            vec!["Teams".to_string(), "Gmail".to_string()],
            Some("Pattern detected".to_string()),
        );
        
        let timeline = manager.get_timeline(Some(10));
        assert!(timeline.len() >= 2);
    }
}

