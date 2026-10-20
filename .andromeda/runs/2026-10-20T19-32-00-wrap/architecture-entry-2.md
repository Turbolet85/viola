
## 2026-10-20-epoch-5-cleanup-ii — the split modules, `relied_here` and the `viola-mcp` serde edge retired
**Section:** §Established Decisions (the capability-ledger / `viola verify` decision: its interactive runs, `agents_read`, `trusted_args`) · §Infrastructure Patterns → `project-directory-structure` (`human/board.rs`, `verify/typed.rs`, the viola-agent-claude row) · §Infrastructure Patterns → `crate-dependency-direction` (`viola-agent-claude`'s ledger list, `viola-mcp`, the shared pins)
**Change:**
- `src/cmd/verify/typed.rs` is the module root (`RunDirs`, `Inputs`, `RunStep`, `measure`, `POLL`) over `src/cmd/verify/typed/`: `runs.rs` (Runs A–E, `trusted_args`, `TrustedDir`), `turns.rs` (`poll_until`, `wait_for`, `turn`, `agents_read`, `framing_turns`, `probe_until`, `wait_capture`), `pty.rs` (`KeyPipe`, `SharedPty`, `Feed`, `Sink`, `settled`, `box_wait`), `run.rs` (`Run`, `ctrl_c_ending`, `ended_by`) and the `#[cfg(test)]` `test_support.rs`; the decision cites `typed::turns::agents_read` and `typed::runs::trusted_args`. Was: one file, `typed::agents_read`, `typed::trusted_args`.
- `src/human/board.rs` is the module root re-exporting `BayBudget`, `home_state_pair` / `instance_state_pair` and `write_board` over `src/human/board/`: `pairs.rs`, `bay.rs`, `cells.rs`, `rows.rs` and the `#[cfg(test)]` `test_support.rs`. Was: one file.
- The viola-agent-claude row names `src/ledger/test_cases.rs`, the ledger's unit cases, declared `#[cfg(test)] #[path = "ledger/test_cases.rs"] mod tests`.
- The ledger's per-OS relied set is `relied()`, filtering on `relied_on(WRAPPED_HERE)`. Was: `relied()`, `relied_here`.
- `viola-mcp` takes no serde (was `serde` among its edges); the shared pins are taken by the crates that take them, `viola-pty` none and `viola-mcp` no serde. Was: "every workspace crate except `viola-pty`".
**Why:** the size split moved code with every caller's path kept; `relied_here` had one caller and was folded into `relied()`; the serde edge was unused (`cargo machete crates/viola-mcp` clean).
**Kept:** viola-e2e's new `viola-agent-claude` path edge takes no row: the key states viola-e2e's dependencies are not listed.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/
