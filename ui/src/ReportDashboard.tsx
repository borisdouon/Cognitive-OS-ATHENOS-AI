/**
 * Phase: A | Step: 8 | Source: Athenos_AI_Strategy.md#L103
 * Report Dashboard - Focus + Time Saved visualization
 * Uses Recharts for charts (athenos-rules.mdc#L26)
 */

import React from 'react';
import {
  LineChart,
  Line,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';

interface CognitiveMetrics {
  cognitive_clarity_index: number;
  emotional_resilience_score: number;
  habit_evolution_rate: number;
  focus_stability_pct: number;
  time_saved_minutes: number;
}

interface PatternInsight {
  pattern_type: string;
  description: string;
  frequency: number;
  impact_score: number;
}

interface ActionSuggestion {
  action: {
    action_type: string;
    description: string;
    confidence: string;
  };
  expected_benefit: string;
}

interface DailyReport {
  date: string;
  metrics: CognitiveMetrics;
  patterns_detected: PatternInsight[];
  suggestions: ActionSuggestion[];
  time_saved_minutes: number;
  focus_stability_pct: number;
}

interface ReportDashboardProps {
  report: DailyReport;
}

export const ReportDashboard: React.FC<ReportDashboardProps> = ({ report }) => {
  // Prepare focus stability data for chart
  const focusData = [
    { time: '9:00', focus: 85 },
    { time: '10:00', focus: 92 },
    { time: '11:00', focus: 78 },
    { time: '12:00', focus: 65 },
    { time: '13:00', focus: 88 },
    { time: '14:00', focus: 90 },
    { time: '15:00', focus: 82 },
  ];

  // Prepare time saved data
  const timeSavedData = [
    { category: 'Automation', saved: report.time_saved_minutes * 0.6 },
    { category: 'Focus Mode', saved: report.time_saved_minutes * 0.3 },
    { category: 'Shortcuts', saved: report.time_saved_minutes * 0.1 },
  ];

  return (
    <div className="p-6 bg-white rounded-lg shadow-lg">
      <h1 className="text-2xl font-bold mb-6">Daily Cognitive Report - {report.date}</h1>
      
      {/* Key Metrics */}
      <div className="grid grid-cols-3 gap-4 mb-8">
        <div className="bg-blue-50 p-4 rounded">
          <h3 className="text-sm font-semibold text-gray-600">Time Saved</h3>
          <p className="text-2xl font-bold text-blue-600">{report.time_saved_minutes.toFixed(1)} min</p>
        </div>
        <div className="bg-green-50 p-4 rounded">
          <h3 className="text-sm font-semibold text-gray-600">Focus Stability</h3>
          <p className="text-2xl font-bold text-green-600">{report.focus_stability_pct.toFixed(1)}%</p>
        </div>
        <div className="bg-purple-50 p-4 rounded">
          <h3 className="text-sm font-semibold text-gray-600">Clarity Index</h3>
          <p className="text-2xl font-bold text-purple-600">{(report.metrics.cognitive_clarity_index * 100).toFixed(1)}%</p>
        </div>
      </div>

      {/* Focus Stability Chart */}
      <div className="mb-8">
        <h2 className="text-xl font-semibold mb-4">Focus Stability Over Time</h2>
        <ResponsiveContainer width="100%" height={300}>
          <LineChart data={focusData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="time" />
            <YAxis />
            <Tooltip />
            <Legend />
            <Line type="monotone" dataKey="focus" stroke="#8884d8" strokeWidth={2} />
          </LineChart>
        </ResponsiveContainer>
      </div>

      {/* Time Saved Breakdown */}
      <div className="mb-8">
        <h2 className="text-xl font-semibold mb-4">Time Saved by Category</h2>
        <ResponsiveContainer width="100%" height={300}>
          <BarChart data={timeSavedData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="category" />
            <YAxis />
            <Tooltip />
            <Bar dataKey="saved" fill="#82ca9d" />
          </BarChart>
        </ResponsiveContainer>
      </div>

      {/* Patterns Detected */}
      <div className="mb-8">
        <h2 className="text-xl font-semibold mb-4">Patterns Detected</h2>
        <div className="space-y-2">
          {report.patterns_detected.map((pattern, idx) => (
            <div key={idx} className="bg-gray-50 p-3 rounded">
              <p className="font-medium">{pattern.description}</p>
              <p className="text-sm text-gray-600">Frequency: {pattern.frequency}x | Impact: {pattern.impact_score.toFixed(1)}</p>
            </div>
          ))}
        </div>
      </div>

      {/* Action Suggestions */}
      <div>
        <h2 className="text-xl font-semibold mb-4">Suggested Actions</h2>
        <div className="space-y-2">
          {report.suggestions.map((suggestion, idx) => (
            <div key={idx} className="bg-blue-50 p-3 rounded border-l-4 border-blue-500">
              <p className="font-medium">{suggestion.action.description}</p>
              <p className="text-sm text-gray-600">{suggestion.expected_benefit}</p>
              <span className="inline-block mt-2 px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded">
                {suggestion.confidence} confidence
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default ReportDashboard;

