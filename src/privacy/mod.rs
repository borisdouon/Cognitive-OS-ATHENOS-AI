/// Phase: A | Step: 4 | Source: Athenos_AI_Strategy.md#L99
/// Privacy Kernel - Consent Ledger + Encryption
/// Default: 100% on-device processing (athenos-rules.mdc#L12-15)

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Consent ledger tracks granular user permissions
/// Source: athenos-rules.mdc#L13
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentLedger {
    pub opt_in_cloud_sync: bool,
    pub opt_in_behavioral_logging: bool,
    pub opt_in_emotion_detection: bool,
    pub opt_in_automation: bool,
    pub consent_timestamp: i64,
    pub revocation_history: Vec<ConsentRevocation>,
}

/// Record of consent revocation for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRevocation {
    pub capability: String,
    pub revoked_at: i64,
    pub reason: Option<String>,
}

impl ConsentLedger {
    /// Create default consent ledger (all opt-out)
    /// Source: athenos-rules.mdc#L12 - Default: 100% on-device
    pub fn new() -> Self {
        info!("ConsentLedger::new: Creating default consent ledger (all opt-out)");
        Self {
            opt_in_cloud_sync: false,
            opt_in_behavioral_logging: false,
            opt_in_emotion_detection: false,
            opt_in_automation: false,
            consent_timestamp: chrono::Utc::now().timestamp(),
            revocation_history: Vec::new(),
        }
    }

    /// Revoke consent for a capability
    /// Source: Strategic_Reinforcements_Gap_Closures.md#L14
    pub fn revoke_consent(&mut self, capability: String, reason: Option<String>) {
        info!("ConsentLedger::revoke_consent: Revoking {} - reason: {:?}", capability, reason);
        match capability.as_str() {
            "cloud_sync" => self.opt_in_cloud_sync = false,
            "behavioral_logging" => self.opt_in_behavioral_logging = false,
            "emotion_detection" => self.opt_in_emotion_detection = false,
            "automation" => self.opt_in_automation = false,
            _ => {}
        }
        self.revocation_history.push(ConsentRevocation {
            capability,
            revoked_at: chrono::Utc::now().timestamp(),
            reason,
        });
    }

    /// Check if cloud sync is allowed
    pub fn can_sync_to_cloud(&self) -> bool {
        self.opt_in_cloud_sync
    }
}

impl Default for ConsentLedger {
    fn default() -> Self {
        Self::new()
    }
}

/// Encryption manager using sodiumoxide
/// Source: athenos-rules.mdc#L14
pub struct EncryptionManager {
    key: Vec<u8>,
}

impl EncryptionManager {
    /// Initialize encryption (must call sodiumoxide::init first)
    pub fn new() -> Result<Self, String> {
        info!("EncryptionManager::new: Initializing encryption");
        sodiumoxide::init().map_err(|e| format!("Failed to init sodiumoxide: {:?}", e))?;
        let key = sodiumoxide::crypto::secretbox::gen_key();
        Ok(Self {
            key: key.as_ref().to_vec(),
        })
    }

    /// Encrypt data locally
    /// Source: athenos-rules.mdc#L14
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        info!("EncryptionManager::encrypt: Encrypting {} bytes", data.len());
        let nonce = sodiumoxide::crypto::secretbox::gen_nonce();
        let key = sodiumoxide::crypto::secretbox::Key::from_slice(&self.key)
            .ok_or("Invalid key")?;
        let ciphertext = sodiumoxide::crypto::secretbox::seal(data, &nonce, &key);
        
        // Prepend nonce to ciphertext
        let mut result = nonce.as_ref().to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    /// Decrypt data locally
    pub fn decrypt(&self, encrypted: &[u8]) -> Result<Vec<u8>, String> {
        info!("EncryptionManager::decrypt: Decrypting {} bytes", encrypted.len());
        if encrypted.len() < 24 {
            return Err("Encrypted data too short".to_string());
        }
        
        let nonce = sodiumoxide::crypto::secretbox::Nonce::from_slice(&encrypted[..24])
            .ok_or("Invalid nonce")?;
        let ciphertext = &encrypted[24..];
        let key = sodiumoxide::crypto::secretbox::Key::from_slice(&self.key)
            .ok_or("Invalid key")?;
        
        sodiumoxide::crypto::secretbox::open(ciphertext, &nonce, &key)
            .map_err(|e| format!("Decryption failed: {:?}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consent_ledger_default() {
        let ledger = ConsentLedger::new();
        assert!(!ledger.opt_in_cloud_sync);
        assert!(!ledger.opt_in_behavioral_logging);
        assert!(!ledger.can_sync_to_cloud());
    }

    #[test]
    fn test_consent_revocation() {
        let mut ledger = ConsentLedger::new();
        ledger.opt_in_cloud_sync = true;
        assert!(ledger.can_sync_to_cloud());
        
        ledger.revoke_consent("cloud_sync".to_string(), Some("Privacy concern".to_string()));
        assert!(!ledger.can_sync_to_cloud());
        assert_eq!(ledger.revocation_history.len(), 1);
    }

    #[test]
    fn test_encryption_roundtrip() {
        sodiumoxide::init().unwrap();
        let manager = EncryptionManager::new().unwrap();
        let data = b"test data";
        
        let encrypted = manager.encrypt(data).unwrap();
        let decrypted = manager.decrypt(&encrypted).unwrap();
        
        assert_eq!(data, decrypted.as_slice());
    }
}

