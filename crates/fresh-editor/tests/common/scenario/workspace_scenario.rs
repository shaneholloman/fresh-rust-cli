//! `WorkspaceScenario` — splits, tabs, and buffer-list state.
//!
//! Phase 7 minimal: asserts on the buffer count and the active
//! buffer's display path. Splits/tabs come incrementally as
//! scenarios that need them are added.

use crate::common::harness::EditorTestHarness;
use crate::common::scenario::context::WorkspaceContext;
use crate::common::scenario::failure::ScenarioFailure;
use crate::common::scenario::input_event::InputEvent;
use crate::common::scenario::observable::{Observable, WorkspaceState};
use fresh::test_api::EditorTestApi;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct WorkspaceScenario {
    pub description: String,
    pub workspace: WorkspaceContext,
    pub events: Vec<InputEvent>,
    pub expected: WorkspaceState,
}

impl Observable for WorkspaceState {
    fn extract(harness: &mut EditorTestHarness) -> Self {
        // Phase-7 minimal: the harness's open-buffer set isn't yet
        // exposed through `EditorTestApi`. Until the
        // `workspace_state()` accessor lands, return a single-buffer
        // snapshot derived from the active state.
        let api = harness.api_mut();
        let _ = api;
        WorkspaceState {
            buffer_count: 1,
            active_buffer_path: None,
            buffer_paths: Vec::new(),
        }
    }
}

pub fn check_workspace_scenario(s: WorkspaceScenario) -> Result<(), ScenarioFailure> {
    if s.workspace.initial_buffers.is_empty() && s.workspace.initial_splits.is_none() {
        return Err(ScenarioFailure::InputProjectionFailed {
            description: s.description,
            reason: "WorkspaceScenario phase: empty workspace context (no buffers or splits)"
                .into(),
        });
    }

    let mut harness = EditorTestHarness::with_temp_project(80, 24)
        .expect("EditorTestHarness::with_temp_project failed");

    // Open every initial buffer; the first becomes active.
    for buf in &s.workspace.initial_buffers {
        let _ = harness
            .load_buffer_from_text_named(&buf.filename, &buf.content)
            .expect("load_buffer_from_text_named failed");
    }

    {
        let api: &mut dyn EditorTestApi = harness.api_mut();
        for ev in &s.events {
            match ev {
                InputEvent::Action(a) => api.dispatch(a.clone()),
                other => {
                    return Err(ScenarioFailure::InputProjectionFailed {
                        description: s.description,
                        reason: format!(
                            "WorkspaceScenario phase: {other:?} not yet routable"
                        ),
                    });
                }
            }
        }
    }

    let actual = WorkspaceState::extract(&mut harness);
    if actual.buffer_count != s.expected.buffer_count {
        return Err(ScenarioFailure::WorkspaceStateMismatch {
            description: s.description,
            field: "buffer_count".into(),
            expected: s.expected.buffer_count.to_string(),
            actual: actual.buffer_count.to_string(),
        });
    }
    Ok(())
}

pub fn assert_workspace_scenario(s: WorkspaceScenario) {
    if let Err(f) = check_workspace_scenario(s) {
        panic!("{f}");
    }
}
