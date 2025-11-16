/// Phase: C | Step: 8 | Source: Athenos_AI_Strategy.md#L127
/// Analytics Dashboard
/// Integrate analytics dashboard for ops, safety, and product teams

use crate::types::*;
use crate::cohort::CohortStatistics;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Analytics metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsMetric {
    pub name: String,
    pub value: f64,
    pub timestamp: i64,
    pub category: MetricCategory,
}

/// Metric category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetricCategory {
    Operations,
    Safety,
    Product,
    UserEngagement,
}

/// Analytics dashboard data
/// Source: Athenos_AI_Strategy.md#L127
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsDashboard {
    pub ops_metrics: Vec<AnalyticsMetric>,
    pub safety_metrics: Vec<AnalyticsMetric>,
    pub product_metrics: Vec<AnalyticsMetric>,
    pub cohort_stats: Option<CohortStatistics>,
}

/// Analytics aggregator
/// Source: Athenos_AI_Strategy.md#L127
pub struct AnalyticsAggregator {
    metrics: Vec<AnalyticsMetric>,
    dashboard: AnalyticsDashboard,
}

impl AnalyticsAggregator {
    /// Create new analytics aggregator
    pub fn new() -> Self {
        info!("AnalyticsAggregator::new: Creating analytics aggregator");
        Self {
            metrics: Vec::new(),
            dashboard: AnalyticsDashboard {
                ops_metrics: Vec::new(),
                safety_metrics: Vec::new(),
                product_metrics: Vec::new(),
                cohort_stats: None,
            },
        }
    }

    /// Record metric
    /// Source: Athenos_AI_Strategy.md#L127
    pub fn record_metric(&mut self, name: String, value: f64, category: MetricCategory) {
        info!("AnalyticsAggregator::record_metric: Recording {} = {} ({:?})", name, value, category);
        
        let metric = AnalyticsMetric {
            name: name.clone(),
            value,
            timestamp: chrono::Utc::now().timestamp(),
            category: category.clone(),
        };
        
        self.metrics.push(metric.clone());
        
        // Add to appropriate dashboard category
        match category {
            MetricCategory::Operations => self.dashboard.ops_metrics.push(metric),
            MetricCategory::Safety => self.dashboard.safety_metrics.push(metric),
            MetricCategory::Product => self.dashboard.product_metrics.push(metric),
            _ => {}
        }
    }

    /// Update cohort statistics
    pub fn update_cohort_stats(&mut self, stats: CohortStatistics) {
        info!("AnalyticsAggregator::update_cohort_stats: Updating cohort statistics");
        self.dashboard.cohort_stats = Some(stats);
    }

    /// Get dashboard data
    pub fn get_dashboard(&self) -> &AnalyticsDashboard {
        &self.dashboard
    }

    /// Get metrics by category
    pub fn get_metrics_by_category(&self, category: MetricCategory) -> Vec<&AnalyticsMetric> {
        self.metrics
            .iter()
            .filter(|m| m.category == category)
            .collect()
    }

    /// Get recent metrics
    pub fn get_recent_metrics(&self, limit: usize) -> Vec<&AnalyticsMetric> {
        let start = self.metrics.len().saturating_sub(limit);
        self.metrics[start..].iter().collect()
    }
}

impl Default for AnalyticsAggregator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytics_aggregator_creation() {
        let aggregator = AnalyticsAggregator::new();
        assert_eq!(aggregator.metrics.len(), 0);
    }

    #[test]
    fn test_record_metric() {
        let mut aggregator = AnalyticsAggregator::new();
        aggregator.record_metric("time_saved".to_string(), 11.0, MetricCategory::Product);
        
        assert_eq!(aggregator.metrics.len(), 1);
        assert_eq!(aggregator.dashboard.product_metrics.len(), 1);
    }

    #[test]
    fn test_get_metrics_by_category() {
        let mut aggregator = AnalyticsAggregator::new();
        aggregator.record_metric("ops_metric".to_string(), 1.0, MetricCategory::Operations);
        aggregator.record_metric("safety_metric".to_string(), 2.0, MetricCategory::Safety);
        
        let ops_metrics = aggregator.get_metrics_by_category(MetricCategory::Operations);
        assert_eq!(ops_metrics.len(), 1);
    }
}

