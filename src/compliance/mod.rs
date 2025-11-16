/// Phase: D | Step: 6 | Source: Athenos_AI_Strategy.md#L137
/// SOC2 Readiness + Differential Privacy
/// Achieve SOC2 readiness; implement differential privacy for aggregated metrics

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// SOC2 control
/// Source: Athenos_AI_Strategy.md#L137
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOC2Control {
    pub id: String,
    pub name: String,
    pub description: String,
    pub control_type: ControlType,
    pub implemented: bool,
    pub tested: bool,
    pub evidence: Vec<String>,
}

/// Control type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ControlType {
    AccessControl,
    Encryption,
    Monitoring,
    IncidentResponse,
    DataRetention,
}

/// SOC2 readiness tracker
/// Source: Athenos_AI_Strategy.md#L137
pub struct SOC2ReadinessTracker {
    controls: HashMap<String, SOC2Control>,
    readiness_score: f64,
}

impl SOC2ReadinessTracker {
    /// Create new SOC2 readiness tracker
    pub fn new() -> Self {
        info!("SOC2ReadinessTracker::new: Creating SOC2 readiness tracker");
        Self {
            controls: HashMap::new(),
            readiness_score: 0.0,
        }
    }

    /// Add SOC2 control
    /// Source: Athenos_AI_Strategy.md#L137
    pub fn add_control(&mut self, control: SOC2Control) {
        info!("SOC2ReadinessTracker::add_control: Adding control {}", control.id);
        self.controls.insert(control.id.clone(), control);
        self.update_readiness_score();
    }

    /// Mark control as implemented
    pub fn mark_implemented(&mut self, control_id: &str) {
        if let Some(control) = self.controls.get_mut(control_id) {
            control.implemented = true;
            self.update_readiness_score();
        }
    }

    /// Mark control as tested
    pub fn mark_tested(&mut self, control_id: &str) {
        if let Some(control) = self.controls.get_mut(control_id) {
            control.tested = true;
            self.update_readiness_score();
        }
    }

    fn update_readiness_score(&mut self) {
        let total = self.controls.len();
        if total == 0 {
            self.readiness_score = 0.0;
            return;
        }
        
        let implemented = self.controls.values().filter(|c| c.implemented).count();
        let tested = self.controls.values().filter(|c| c.tested).count();
        
        // Score: 50% implementation + 50% testing
        self.readiness_score = (implemented as f64 / total as f64 * 0.5) + 
                               (tested as f64 / total as f64 * 0.5);
    }

    /// Get readiness score
    pub fn get_readiness_score(&self) -> f64 {
        self.readiness_score
    }
}

/// Differential privacy noise generator
/// Source: Athenos_AI_Strategy.md#L137, Strategic_Reinforcements_Gap_Closures.md#L7
pub struct DifferentialPrivacy {
    epsilon: f64, // Privacy parameter (lower = more private)
}

impl DifferentialPrivacy {
    /// Create new differential privacy manager
    pub fn new(epsilon: f64) -> Self {
        info!("DifferentialPrivacy::new: Creating differential privacy manager (ε={})", epsilon);
        Self { epsilon }
    }

    /// Add noise to aggregated metric
    /// Source: Athenos_AI_Strategy.md#L137
    pub fn add_noise(&self, value: f64) -> f64 {
        // Simplified Laplace mechanism: add noise ~ Lap(Δ/ε)
        // For Phase D stub, we add small random noise
        use rand::Rng;
        let sensitivity = 1.0; // Maximum change from one user
        let scale = sensitivity / self.epsilon;
        let noise = (rand::thread_rng().gen::<f64>() - 0.5) * scale * 2.0;
        value + noise
    }

    /// Aggregate metrics with differential privacy
    pub fn aggregate_with_privacy(&self, values: &[f64]) -> f64 {
        let sum: f64 = values.iter().sum();
        let count = values.len() as f64;
        let avg = sum / count;
        self.add_noise(avg)
    }
}

impl Default for SOC2ReadinessTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soc2_tracker_creation() {
        let tracker = SOC2ReadinessTracker::new();
        assert_eq!(tracker.controls.len(), 0);
    }

    #[test]
    fn test_add_and_mark_control() {
        let mut tracker = SOC2ReadinessTracker::new();
        let control = SOC2Control {
            id: "CC1.1".to_string(),
            name: "Access Control".to_string(),
            description: "Control access".to_string(),
            control_type: ControlType::AccessControl,
            implemented: false,
            tested: false,
            evidence: Vec::new(),
        };
        
        tracker.add_control(control);
        tracker.mark_implemented("CC1.1");
        tracker.mark_tested("CC1.1");
        
        assert!(tracker.get_readiness_score() > 0.0);
    }

    #[test]
    fn test_differential_privacy() {
        let dp = DifferentialPrivacy::new(1.0);
        let values = vec![10.0, 20.0, 30.0];
        let aggregated = dp.aggregate_with_privacy(&values);
        
        // Should be close to average (20.0) but with noise
        assert!((aggregated - 20.0).abs() < 10.0); // Allow reasonable noise
    }
}

