/// Phase: B | Step: 3 | Source: Athenos_AI_Strategy.md#L110
/// On-device Pattern Miner with Causal Inference
/// Implement on-device pattern miner with causal inference heuristics

use crate::types::*;
use crate::edge::OSEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Causal relationship between events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalRelationship {
    pub cause: String,
    pub effect: String,
    pub strength: f64, // 0.0 to 1.0
    pub confidence: f64,
}

/// Pattern miner with causal inference
/// Source: Athenos_AI_Strategy.md#L110
pub struct PatternMiner {
    event_sequences: Vec<Vec<String>>,
    causal_graph: HashMap<String, Vec<CausalRelationship>>,
}

impl PatternMiner {
    /// Create new pattern miner
    pub fn new() -> Self {
        info!("PatternMiner::new: Creating pattern miner");
        Self {
            event_sequences: Vec::new(),
            causal_graph: HashMap::new(),
        }
    }

    /// Mine patterns from OS events
    /// Source: Athenos_AI_Strategy.md#L110
    pub fn mine_patterns(&mut self, events: &[OSEvent]) -> Vec<PatternType> {
        info!("PatternMiner::mine_patterns: Mining patterns from {} events", events.len());
        
        // Extract app sequences
        let sequence: Vec<String> = events
            .iter()
            .filter_map(|e| {
                match e.event_type {
                    crate::edge::OSEventType::AppLaunch | 
                    crate::edge::OSEventType::AppSwitch |
                    crate::edge::OSEventType::WindowFocus => Some(e.app_name.clone()),
                    _ => None,
                }
            })
            .collect();
        
        if sequence.len() >= 3 {
            self.event_sequences.push(sequence.clone());
            
            // Infer causal relationships
            for i in 0..sequence.len().saturating_sub(1) {
                let cause = sequence[i].clone();
                let effect = sequence[i + 1].clone();
                let strength = self.compute_causal_strength(&cause, &effect);
                
                if strength > 0.3 {
                    let relationship = CausalRelationship {
                        cause: cause.clone(),
                        effect: effect.clone(),
                        strength,
                        confidence: 0.7, // Phase B: heuristic confidence
                    };
                    
                    self.causal_graph
                        .entry(cause)
                        .or_insert_with(Vec::new)
                        .push(relationship);
                }
            }
        }
        
        // Detect pattern types
        self.detect_pattern_types()
    }

    /// Compute causal strength between two events
    /// Source: Athenos_AI_Strategy.md#L110
    fn compute_causal_strength(&self, cause: &str, effect: &str) -> f64 {
        // Count co-occurrences
        let mut co_occurrences = 0;
        let mut cause_count = 0;
        
        for seq in &self.event_sequences {
            for i in 0..seq.len().saturating_sub(1) {
                if seq[i] == cause {
                    cause_count += 1;
                    if i + 1 < seq.len() && seq[i + 1] == effect {
                        co_occurrences += 1;
                    }
                }
            }
        }
        
        if cause_count > 0 {
            co_occurrences as f64 / cause_count as f64
        } else {
            0.0
        }
    }

    /// Detect pattern types from sequences
    fn detect_pattern_types(&self) -> Vec<PatternType> {
        let mut patterns = Vec::new();
        
        // Check for workflow sequences
        if self.event_sequences.len() > 5 {
            patterns.push(PatternType::WorkflowSequence);
        }
        
        // Check for context switching
        let avg_sequence_len: f64 = self.event_sequences.iter()
            .map(|s| s.len() as f64)
            .sum::<f64>() / self.event_sequences.len() as f64;
        
        if avg_sequence_len > 5.0 {
            patterns.push(PatternType::ContextSwitching);
        }
        
        patterns
    }

    /// Get causal relationships for an app
    pub fn get_causal_relationships(&self, app: &str) -> Vec<&CausalRelationship> {
        self.causal_graph
            .get(app)
            .map(|rels| rels.iter().collect())
            .unwrap_or_else(Vec::new)
    }
}

impl Default for PatternMiner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edge::{OSEvent, OSEventType};

    #[test]
    fn test_pattern_miner_creation() {
        let miner = PatternMiner::new();
        assert_eq!(miner.event_sequences.len(), 0);
    }

    #[test]
    fn test_mine_patterns_workflow_sequence() {
        let mut miner = PatternMiner::new();
        
        let events = vec![
            OSEvent {
                event_type: OSEventType::AppLaunch,
                app_name: "Teams".to_string(),
                window_title: None,
                timestamp: 1,
                metadata: HashMap::new(),
            },
            OSEvent {
                event_type: OSEventType::AppSwitch,
                app_name: "Gmail".to_string(),
                window_title: None,
                timestamp: 2,
                metadata: HashMap::new(),
            },
            OSEvent {
                event_type: OSEventType::WindowFocus,
                app_name: "IDE".to_string(),
                window_title: None,
                timestamp: 3,
                metadata: HashMap::new(),
            },
        ];
        
        // Add multiple sequences to trigger pattern detection
        for _ in 0..6 {
            miner.mine_patterns(&events);
        }
        
        let patterns = miner.mine_patterns(&events);
        assert!(patterns.contains(&PatternType::WorkflowSequence));
    }

    #[test]
    fn test_causal_relationship_detection() {
        let mut miner = PatternMiner::new();
        
        let events = vec![
            OSEvent {
                event_type: OSEventType::AppLaunch,
                app_name: "Teams".to_string(),
                window_title: None,
                timestamp: 1,
                metadata: HashMap::new(),
            },
            OSEvent {
                event_type: OSEventType::AppSwitch,
                app_name: "Gmail".to_string(),
                window_title: None,
                timestamp: 2,
                metadata: HashMap::new(),
            },
        ];
        
        // Add multiple sequences to build causal graph
        for _ in 0..10 {
            miner.mine_patterns(&events);
        }
        
        let relationships = miner.get_causal_relationships("Teams");
        assert!(!relationships.is_empty());
        assert_eq!(relationships[0].effect, "Gmail");
    }
}

