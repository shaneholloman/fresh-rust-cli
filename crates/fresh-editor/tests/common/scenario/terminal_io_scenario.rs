//! `TerminalIoScenario` — ANSI bytes via vt100 round-trip.
//!
//! Asserts on the [`super::observable::RoundTripGrid`] produced by
//! piping the editor's emitted ANSI through a `vt100` parser. The
//! harness already does this through `render_real` /
//! `render_real_incremental`; this scenario type formalises the
//! flow so escape-emission bugs are catchable as data.
//!
//! **Production hook (Phase 8) — partial:** the harness exposes
//! `render_real()` already; what's missing is a `roundtrip_grid()`
//! accessor that returns a typed `RoundTripGrid` from the vt100
//! state. Until that one accessor lands, this runner panics with a
//! precise message. The hook is ~30 LOC over the existing vt100
//! integration.

use crate::common::scenario::failure::ScenarioFailure;
use crate::common::scenario::input_event::InputEvent;
use crate::common::scenario::observable::RoundTripGrid;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TerminalIoScenario {
    pub description: String,
    pub initial_text: String,
    pub width: u16,
    pub height: u16,
    pub events: Vec<InputEvent>,
    pub expected: RoundTripGrid,
}

pub fn check_terminal_io_scenario(_s: TerminalIoScenario) -> Result<(), ScenarioFailure> {
    Err(ScenarioFailure::InputProjectionFailed {
        description: "TerminalIoScenario".into(),
        reason: "Phase 8 not yet implemented: needs `roundtrip_grid()` on \
            the harness — returns a typed `RoundTripGrid` from the \
            existing vt100 parser state after `render_real()`. ~30 LOC. \
            See docs/internal/e2e-test-migration-design.md §7.1."
            .into(),
    })
}

pub fn assert_terminal_io_scenario(s: TerminalIoScenario) {
    if let Err(f) = check_terminal_io_scenario(s) {
        panic!("{f}");
    }
}
