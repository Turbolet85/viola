
## 2026-10-10-viola-revive — `revive` among the CLI verbs; a passed revive enters the TUI passthrough boundary
**Section:** §1 A11y Scope Summary (CLI and terminal entities: CLI verbs, TUI passthrough; Boundary-only vendor zones; Surface cli, Notes; Surface tui) · §3 → Keyboard test harness · §6 Visual Design Verification (CLI equivalent) · §8 Cognitive Accessibility (Error recovery, CLI) · §11 A11y Anti-Patterns (Strategy; Keyboard; Visual)
**Change:**
- §1 CLI verbs: the list names `revive`. It takes no `--json`; `--list` prints static ASCII rows `<ts>  <cause>  <id>` with no header, colour or ESC byte; each exit-1 refusal is one `unable:` line and one `hint:` line last; a clap usage error exits 2.
- §1 TUI passthrough, vendor zones and Surface tui: the passthrough is entered by `viola run` and by a `viola revive` whose preflight passed. The boundary binds both; the one clause read by a revive case of its own is that a passed revive writes no line of its own while the child holds the terminal (`tests/chaos_revive.rs`).
- §1 cli Notes, §6 CLI equivalent, §11 Visual: the zero-SGR list names a passed `viola revive`; revive's refusal pairs and `--list` rows carry no ESC byte.
- §8 Error recovery: revive's exit-1 refusals take the fixed-message form of design-system cli pattern 2.
- §11 Strategy and Keyboard: the no-conformance-claim ban and the keystroke ban cover a start a passed revive made.
- Key file, Keyboard test harness: the zero-own-lines reading of a revived start is recorded; the keystroke clause and the focus / mouse / resize clause have no revive-specific case.
**Why:** the chunk landed a second verb that hosts the wrapped `claude` TUI through the same start as `run`.
**Kept:** the surface keeps its name, the `viola run` TUI. §8's general CLI form, `unable  <reason>  <detail>`, stands; the fixed-message exceptions live in design-system.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/
