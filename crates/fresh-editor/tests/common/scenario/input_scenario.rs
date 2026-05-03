//! `InputScenario` — mouse, IME composition, and keyboard chord
//! events as data.
//!
//! Mouse coordinates project to (line, byte) through the current
//! [`RenderSnapshot`]. IME composition decomposes into `InsertChar`
//! actions. Keyboard chord disambiguation is surfaced as a sequence
//! of `Action`s.
//!
//! Phase 9 minimal: mouse-click projection through the
//! `(row, col) → byte` mapping on the test API. The full
//! drag-as-selection and wheel-as-scroll flows depend on extending
//! `EditorTestApi` with a `cell_to_byte` accessor.
//!
//! Asserts on the final [`RenderSnapshot`] (cursor moved to the
//! clicked cell).

use crate::common::harness::EditorTestHarness;
use crate::common::scenario::context::{MouseButton, MouseEvent};
use crate::common::scenario::failure::ScenarioFailure;
use crate::common::scenario::input_event::InputEvent;
use crate::common::scenario::observable::Observable;
use crate::common::scenario::render_snapshot::{RenderSnapshot, RenderSnapshotExpect};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct InputScenario {
    pub description: String,
    pub initial_text: String,
    pub events: Vec<InputEvent>,
    pub expected: RenderSnapshotExpect,
}

pub fn check_input_scenario(s: InputScenario) -> Result<(), ScenarioFailure> {
    let mut harness = EditorTestHarness::with_temp_project(80, 24)
        .expect("EditorTestHarness::with_temp_project failed");
    let _fixture = harness
        .load_buffer_from_text(&s.initial_text)
        .expect("load_buffer_from_text failed");
    harness.render().expect("initial render failed");

    for ev in &s.events {
        dispatch_input(&mut harness, ev, &s.description)?;
    }
    harness.render().expect("final render failed");

    let snapshot = RenderSnapshot::extract(&mut harness);
    if let Some((field, expected, actual)) = s.expected.check_against(&snapshot) {
        return Err(ScenarioFailure::SnapshotFieldMismatch {
            description: s.description,
            field: field.into(),
            expected,
            actual,
        });
    }
    Ok(())
}

pub fn assert_input_scenario(s: InputScenario) {
    if let Err(f) = check_input_scenario(s) {
        panic!("{f}");
    }
}

fn dispatch_input(
    harness: &mut EditorTestHarness,
    ev: &InputEvent,
    description: &str,
) -> Result<(), ScenarioFailure> {
    match ev {
        InputEvent::Action(a) => {
            harness.api_mut().dispatch(a.clone());
            Ok(())
        }
        InputEvent::Mouse(MouseEvent::Click {
            row,
            col,
            button: MouseButton::Left,
        }) => {
            // Approximate the editor's "click moves cursor" path
            // by computing `(row, col) → byte` from the current
            // RenderSnapshot. The proper hook is a `cell_to_byte`
            // accessor on `EditorTestApi`; until that lands we
            // best-effort by treating row=0 column=col as byte=col
            // on the first visible line.
            let snap = RenderSnapshot::extract(harness);
            if *row >= snap.height || *col >= snap.width {
                return Err(ScenarioFailure::InputProjectionFailed {
                    description: description.into(),
                    reason: format!(
                        "Mouse::Click({row},{col}) outside terminal {}x{}",
                        snap.width, snap.height
                    ),
                });
            }
            // Stub: emit a dispatch through the existing input
            // layer? Without a proper `cell_to_byte`, the cleanest
            // honest behavior is to fail with a precise reason.
            Err(ScenarioFailure::InputProjectionFailed {
                description: description.into(),
                reason: "Mouse::Click projection requires `cell_to_byte` on EditorTestApi (Phase 9 follow-up)".into(),
            })
        }
        InputEvent::Compose(chars) => {
            for c in chars {
                harness
                    .api_mut()
                    .dispatch(fresh::test_api::Action::InsertChar(*c));
            }
            Ok(())
        }
        other => Err(ScenarioFailure::InputProjectionFailed {
            description: description.into(),
            reason: format!("InputScenario does not handle {other:?} — wrong scenario type"),
        }),
    }
}
