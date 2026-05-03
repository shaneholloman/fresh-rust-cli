//! `GuiScenario` — wgpu/winit observables.
//!
//! **Production hook required (Phase 12):** a GUI-side test API
//! analogous to `EditorTestApi` but driven through the wgpu
//! front-end's input pipeline. Most editor-level coverage of
//! `gui.rs` is already provided by `BufferScenario` /
//! `LayoutScenario`; what remains is the GUI-specific surface
//! (font fallback, sub-pixel positioning, IME interaction).
//! Lowest priority; `gui.rs` is one file.

use crate::common::scenario::failure::ScenarioFailure;
use crate::common::scenario::input_event::InputEvent;
use crate::common::scenario::observable::GuiSnapshot;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GuiScenario {
    pub description: String,
    pub initial_text: String,
    pub events: Vec<InputEvent>,
    pub expected: GuiSnapshot,
}

pub fn check_gui_scenario(_s: GuiScenario) -> Result<(), ScenarioFailure> {
    Err(ScenarioFailure::InputProjectionFailed {
        description: "GuiScenario".into(),
        reason: "Phase 12 not yet implemented: needs a wgpu/winit-side test \
            API. May stay imperative; gui.rs is one file and most of \
            its content is editor behavior covered by BufferScenario / \
            LayoutScenario."
            .into(),
    })
}

pub fn assert_gui_scenario(s: GuiScenario) {
    if let Err(f) = check_gui_scenario(s) {
        panic!("{f}");
    }
}
