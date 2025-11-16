/// Phase: C | Source: Athenos_AI_Strategy.md#L119-129
/// Athenos AI - Cognitive Operating System
/// Main entry point

mod types;
mod privacy;
mod edge;
mod local_stack;
mod report;
mod sandbox;
mod models;
mod wisdom;
mod pattern_miner;
mod shortcut;
mod consent;
mod emotion;
mod rag;
mod replay;
mod federated;
mod cohort;
mod auto_action;
mod microlearning;
mod scheduling;
mod reflection;
mod emotional_copilot;
mod victory;
mod security;
mod analytics;
mod plugin;
mod beta;

use tracing::info;
use types::*;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Athenos AI starting - Phase B");
    info!("Source: Athenos_AI_Strategy.md#L107-117");
    
    // Phase A components
    let consent_ledger = privacy::ConsentLedger::new();
    info!("Privacy kernel initialized - all opt-out by default");
    
    let mut edge_observer = edge::EdgeObserver::new(1000);
    info!("Edge observer initialized");
    
    let feature_store = local_stack::FeatureStore::new();
    info!("Feature store initialized");
    
    let sandbox_runner = sandbox::SandboxRunner::default();
    info!("Sandbox runner initialized");
    
    let report_generator = report::ReportGenerator::new(feature_store);
    info!("Report generator initialized");
    
    // Phase B components
    let mut pattern_detector = models::PatternDetector::new();
    info!("Pattern detector initialized");
    
    let mut recommendation_ranker = models::RecommendationRanker::new();
    info!("Recommendation ranker initialized");
    
    let mut wisdom_engine = wisdom::WisdomEngine::new();
    info!("Wisdom Engine initialized");
    
    let mut pattern_miner = pattern_miner::PatternMiner::new();
    info!("Pattern miner initialized");
    
    let mut shortcut_generator = shortcut::ShortcutGenerator::new();
    info!("Shortcut generator initialized");
    
    let mut micro_consent_manager = consent::MicroConsentManager::new();
    info!("Micro-consent manager initialized");
    
    let mut mood_adaptive_focus = emotion::MoodAdaptiveFocusMode::new();
    info!("Mood-adaptive focus mode initialized");
    
    let mut rag_index = rag::RAGIndex::new();
    info!("RAG index initialized");
    
    let mut replay_simulator = replay::ReplaySimulator::new();
    info!("Replay simulator initialized");
    
    let federated_coordinator = federated::FederatedLearningCoordinator::new(consent_ledger.clone());
    info!("Federated learning coordinator initialized");
    
    let mut cohort_manager = cohort::CohortManager::new(200);
    info!("Cohort manager initialized (target: 200 users)");
    
    info!("Phase B initialization complete");
    
    // Phase C components
    let mut auto_action_synthesizer = auto_action::AutoActionSynthesizer::new();
    info!("Auto-action synthesizer initialized");
    
    let mut microlearning_generator = microlearning::MicrolearningNudgeGenerator::new();
    info!("Microlearning nudge generator initialized");
    
    let mut calendar_agent = scheduling::CalendarNegotiationAgent::new();
    info!("Calendar negotiation agent initialized");
    
    let mut reflective_loop = reflection::ReflectiveReasoningLoop::new();
    info!("Reflective reasoning loop initialized");
    
    let mut emotional_copilot = emotional_copilot::EmotionalCoPilot::new();
    info!("Emotional co-pilot initialized");
    
    let mut victory_stream = victory::VictoryStream::new();
    info!("Victory stream initialized");
    
    let tpm_storage = security::TPMKeyStorage::new();
    info!("TPM key storage initialized");
    
    let mut threat_monitor = security::ThreatMonitor::new();
    info!("Threat monitor initialized");
    
    let mut analytics_aggregator = analytics::AnalyticsAggregator::new();
    info!("Analytics aggregator initialized");
    
    let mut plugin_registry = plugin::PluginRegistry::new();
    info!("Plugin registry initialized");
    
    let mut beta_manager = beta::BetaOnboardingManager::new();
    info!("Beta onboarding manager initialized");
    
    info!("Phase C initialization complete");
    info!("Ready for autonomy & delight");
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

