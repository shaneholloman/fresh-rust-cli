//! `PersistenceScenario` — virtual filesystem + session/recovery
//! state.
//!
//! **Production hook required (Phase 6):** the editor's filesystem
//! adapter trait must be replaceable by a [`super::context::VirtualFs`]
//! backend in tests. The adapter trait is invoked from
//! `Buffer::load`, `Buffer::save`, `Workspace::reload_external`,
//! and the hot-exit recovery path.
//!
//! Until the adapter is replaceable, this runner panics. Data
//! shape is real; FsExternalEdit events serialise into the corpus
//! immediately.

use crate::common::scenario::context::VirtualFs;
use crate::common::scenario::failure::ScenarioFailure;
use crate::common::scenario::input_event::InputEvent;
use crate::common::scenario::observable::FsState;
use crate::common::scenario::property::BufferState;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PersistenceScenario {
    pub description: String,
    pub initial_fs: VirtualFs,
    /// Path the editor opens at scenario start.
    pub initial_open: String,
    pub events: Vec<InputEvent>,
    pub expected_buffer: BufferState,
    pub expected_fs: FsState,
}

pub fn check_persistence_scenario(_s: PersistenceScenario) -> Result<(), ScenarioFailure> {
    Err(ScenarioFailure::InputProjectionFailed {
        description: "PersistenceScenario".into(),
        reason: "Phase 6 not yet implemented: needs the editor's filesystem \
            adapter to be a trait whose impl is selectable per-harness. \
            Then VirtualFs becomes the test impl and FsExternalEdit \
            events route to its mutators. See \
            docs/internal/e2e-test-migration-design.md §6.2."
            .into(),
    })
}

pub fn assert_persistence_scenario(s: PersistenceScenario) {
    if let Err(f) = check_persistence_scenario(s) {
        panic!("{f}");
    }
}
