/// Phase: A | Source: Athenos_AI_Strategy.md#L95-105
/// Athenos AI - Cognitive Operating System
/// Main entry point

mod types;
mod privacy;
mod edge;
mod local_stack;
mod report;
mod sandbox;

use tracing::info;
use types::*;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Athenos AI starting - Phase A");
    info!("Source: Athenos_AI_Strategy.md#L95-105");
    
    // Initialize privacy kernel
    let consent_ledger = privacy::ConsentLedger::new();
    info!("Privacy kernel initialized - all opt-out by default");
    
    // Initialize edge observer
    let mut edge_observer = edge::EdgeObserver::new(1000);
    info!("Edge observer initialized");
    
    // Initialize feature store
    let feature_store = local_stack::FeatureStore::new();
    info!("Feature store initialized");
    
    // Initialize sandbox
    let sandbox_runner = sandbox::SandboxRunner::default();
    info!("Sandbox runner initialized");
    
    // Initialize report generator
    let report_generator = report::ReportGenerator::new(feature_store);
    info!("Report generator initialized");
    
    info!("Phase A initialization complete");
    info!("Ready for observation and reporting");
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_phase_a_integration() {
        // Test full Phase A pipeline
        let consent_ledger = privacy::ConsentLedger::new();
        assert!(!consent_ledger.can_sync_to_cloud());
        
        let mut observer = edge::EdgeObserver::new(10);
        observer.record_event(edge::OSEvent {
            event_type: edge::OSEventType::AppLaunch,
            app_name: "Teams".to_string(),
            window_title: None,
            timestamp: 1234567890,
            metadata: HashMap::new(),
        });
        
        let sequence = observer.get_app_sequence(10);
        assert_eq!(sequence, vec!["Teams"]);
    }
}

