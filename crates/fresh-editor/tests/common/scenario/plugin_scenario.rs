//! `PluginScenario` — plugin source + expected message log.
//!
//! **Production hook required (Phase 11):** the plugin runtime
//! (`fresh-plugin-runtime`) needs a test-mode entry point that
//! loads a script string (instead of from disk), captures its
//! emitted messages, and exposes them through `EditorTestApi`.

use crate::common::scenario::context::PluginScript;
use crate::common::scenario::failure::ScenarioFailure;
use crate::common::scenario::input_event::InputEvent;
use crate::common::scenario::observable::PluginLog;
use crate::common::scenario::property::BufferState;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PluginScenario {
    pub description: String,
    pub initial_text: String,
    pub script: PluginScript,
    pub events: Vec<InputEvent>,
    pub expected_buffer: BufferState,
    pub expected_log: PluginLog,
}

pub fn check_plugin_scenario(_s: PluginScenario) -> Result<(), ScenarioFailure> {
    Err(ScenarioFailure::InputProjectionFailed {
        description: "PluginScenario".into(),
        reason: "Phase 11 not yet implemented: needs the plugin runtime to \
            accept a script-string load (vs file load) and expose its \
            emitted-message log through EditorTestApi."
            .into(),
    })
}

pub fn assert_plugin_scenario(s: PluginScenario) {
    if let Err(f) = check_plugin_scenario(s) {
        panic!("{f}");
    }
}
