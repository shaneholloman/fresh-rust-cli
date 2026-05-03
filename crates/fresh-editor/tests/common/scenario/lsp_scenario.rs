//! `LspScenario` — scripted LSP exchange + buffer assertions.
//!
//! **Production hook required (Phase 5):** a fake LSP server
//! adapter that intercepts the editor's outgoing JSON-RPC, matches
//! it against the script's expected sequence, and injects scripted
//! replies. The hook plugs into the existing `LspManager` at the
//! transport layer.
//!
//! Until the fake adapter lands, scenarios constructed here panic
//! with a precise blocker message. Data shape is real; external
//! drivers can produce LspScenarios into the corpus that the
//! runner will pick up the moment the adapter ships.

use crate::common::scenario::context::LspScript;
use crate::common::scenario::failure::ScenarioFailure;
use crate::common::scenario::input_event::InputEvent;
use crate::common::scenario::observable::LspTraffic;
use crate::common::scenario::property::BufferState;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LspScenario {
    pub description: String,
    pub initial_text: String,
    pub language: String,
    pub script: LspScript,
    pub events: Vec<InputEvent>,
    pub expected_buffer: BufferState,
    pub expected_traffic: LspTraffic,
}

pub fn check_lsp_scenario(_s: LspScenario) -> Result<(), ScenarioFailure> {
    Err(ScenarioFailure::InputProjectionFailed {
        description: "LspScenario".into(),
        reason: "Phase 5 not yet implemented: needs a fake LSP adapter that \
            plugs into LspManager's transport, matches scripted client \
            messages, and injects scripted server replies. See \
            docs/internal/e2e-test-migration-design.md §6.2."
            .into(),
    })
}

pub fn assert_lsp_scenario(s: LspScenario) {
    if let Err(f) = check_lsp_scenario(s) {
        panic!("{f}");
    }
}
