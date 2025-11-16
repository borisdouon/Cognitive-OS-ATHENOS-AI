# Athenos AI - Cognitive Operating System

**Phase: A — Foundations** | Source: `Athenos_AI_Strategy.md#L95-105`

## Overview

Athenos AI is a Cognitive Operating System that observes, learns, and optimizes how humans interact with digital tools. Built with privacy-first principles, all processing happens on-device by default.

## Phase A Implementation

### ✅ Completed Components

1. **Data Preparation** (`data/athenos_seed.jsonl`)
   - Converted conversation transcripts from DATA 1/2/3.txt into structured JSONL
   - 15 seed observations covering Developer, Accountant, and Designer profiles
   - Source: `TRAINING CONCEPT.txt#L40-57`

2. **Cognitive Taxonomy** (`src/types.rs`)
   - Intent types: DetectPattern, SuggestShortcut, AutomateAction, MoodIntervention
   - Pattern types: WorkflowSequence, DebuggingLoop, ContextSwitching, etc.
   - Action types with confidence and risk categorization
   - Source: `Athenos_AI_Strategy.md#L97`

3. **Privacy Kernel** (`src/privacy/`)
   - ConsentLedger with granular opt-in/opt-out controls
   - EncryptionManager using sodiumoxide (local encryption)
   - Default: 100% on-device processing
   - Source: `athenos-rules.mdc#L12-15`, `Athenos_AI_Strategy.md#L99`

4. **Edge Observer** (`src/edge/`)
   - OS event logger capturing app launches, switches, window focus
   - App sequence pattern detection
   - Event rotation for memory management
   - Source: `Athenos_AI_Strategy.md#L100`

5. **Feature Pipeline** (`src/local_stack/`)
   - Temporal metrics extraction (focus duration, context switches)
   - Embedding storage for semantic features
   - Focus stability computation
   - Source: `Athenos_AI_Strategy.md#L101`

6. **Daily Report Generator** (`src/report/`)
   - Rule-based pattern detection
   - Cognitive metrics computation
   - Action suggestion generation
   - Source: `Athenos_AI_Strategy.md#L102`

7. **Sandbox Infrastructure** (`src/sandbox/`)
   - Automation testing before suggestion
   - Safety checks (confidence + risk)
   - Undo function generation
   - Source: `athenos-rules.mdc#L50-52`, `Athenos_AI_Strategy.md#L104`

8. **Report Dashboard** (`ui/src/ReportDashboard.tsx`)
   - React + Recharts visualization
   - Focus stability chart
   - Time saved breakdown
   - Pattern insights display
   - Source: `Athenos_AI_Strategy.md#L103`

9. **Metrics Directory** (`metrics/`)
   - Structure for alpha cohort tracking
   - Core metrics: Cognitive Clarity Index, Emotional Resilience Score, etc.
   - Source: `Athenos_AI_Strategy.md#L105`

## Tech Stack

- **Backend**: Rust + Tauri
- **Database**: rusqlite (local)
- **Encryption**: sodiumoxide
- **UI**: React + Tailwind + Recharts
- **AI**: candle (local inference, Phase B+)

## Testing

All modules include unit tests following TDD principles:

```bash
cargo test --lib
```

## Privacy & Sovereignty

- **Default**: 100% on-device processing
- **Cloud sync**: Opt-in only via `ConsentLedger`
- **Encryption**: All local data encrypted at rest
- **Deletion**: One-click data removal supported

## Next Steps (Phase B)

Phase A complete. Ready for Phase B when approved.

Phase B will include:
- Supervised model training
- LLM fine-tuning
- Pattern miner with causal inference
- Predictive shortcut generator

## Documentation

- `Documentation/Athenos_AI_Strategy.md` - Core strategy and roadmap
- `Documentation/TRAINING CONCEPT.txt` - Training data preparation
- `Documentation/Strategic_Reinforcements_Gap_Closures.md` - Gap analysis
- `.cursor/rules/athenos-rules.mdc` - Development rules

## License

Proprietary - Braincode Africa

