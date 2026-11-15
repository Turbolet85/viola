@history_item n=1 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — added the statusline-bearing start's own zero-viola-bytes reading (`passthrough_with_a_statusline_source_adds_no_viola_bytes` in `tests/tui_passthrough.rs`: the start writes the settings override and passes `--settings`; `viola hook statusline` hands the user's output to the child byte for byte), and put `STATUSLINE_DEADLINE` (5 s, PROVISIONAL) into §8 Timeout extensions as a bound on the user's own command. This entry is where `decision/statusline-output-passed-byte-for-byte`, `contract/viola-hook-statusline/exit` and the status-line clause of the time-bounds rows came from. This chunk retires all of them, and its guard test leaves or is rewritten.

@history_item n=2 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (8. Cognitive Accessibility: 6 rows; and the `~ decision/time-bounds-never-on-human` entry) — the closest precedent. It took the budget pause out of `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` by rewriting each row's list and leaving the rest of the rule word for word. It retired the removed feature's rows with `−` and a reason for each. This chunk follows the same form: «statusline deadline» / «status-line deadline» leaves both lists, and `contract/viola-hook-statusline/exit` and `decision/statusline-output-passed-byte-for-byte` are retired. Exit 11 shows that a retired code or word is never reused.

@history_item n=3 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — added the budget pause to §8 as a driver-only bound and gave keyboard-test-harness Tooling its closed-gate reading. It is the same kind of addition as the statusline entry: both added a reading to the keyboard harness Tooling and a bound to §8. Its removal (n=2) is the pattern for taking out the statusline reading from the `keyboard-test-harness` key.

@history_item n=4 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone) — the zero-viola-bytes case now compares the outer-PTY stream byte for byte with an unwrapped fake-agent run, on `ubuntu-latest`. keyboard-test-harness says «the statusline start writes the settings override and passes `--settings` with no Windows arm». That clause leaves with this chunk. `decision/zero-viola-bytes-on-the-terminal` stands on the unwrapped comparison alone.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the three-runner greens re-worded as dated readings) — reworded keyboard-test-harness's revive, statusline and budget-gate readings to «green in CI», keeping their chunks and runs. The statusline reading is one of these and goes now, as the budget reading went. The revive reading stays.

@history_item n=6 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary through the same start as `run`, and it writes no line of its own while the child holds the terminal (`contract/tui-passthrough/own-bytes`). That shared start is where the statusline override and `--settings` are removed. The revive clause has to hold after the removal.

@history_item n=7 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — fixed the tui wheel boundary: focus, mouse and resize sequences and terminal replies are non-editing, and the three boundary cases are on the outer PTY (`decision/wheel-ignores-terminal-events`). The statusline entry tied its bound to «the wheel untouched» here. The removal must keep these boundary cases unchanged.

@history_item n=8 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — the first bound in §8 that falls on the driver and not the human (the dialog deadline). This is how the time-bounds list began. The list entries in n=9 to n=13 add the other bounds, and the statusline deadline was one of them. The list after this chunk is: wait timeout and cap, dialog hold, readiness gate, confirm windows, agents read.

@history_item n=9 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — added the readiness gate's `GATE_MAX_WAIT` bound on the driver to §8, with its outer-PTY case. This bound stays on the time-bounds rows.

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the 5 s `claude agents --json` read to §8, a bound on viola's own child. It stays on the time-bounds rows, and it is the «viola's own child» bound beside the departing «user's command» one. With the statusline deadline gone, `decision/time-bounds-never-on-human` names no bound on «the user's command», so its wording should be checked.

@history_item n=11 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — added the MCP `WAIT_CAP` (50 s) to §8 as a driver-only bound. It stays on the time-bounds rows.

@history_item n=12 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added the confirm windows (10 s fallback, 40 s compact) to §8 as driver-only bounds. They stay on the time-bounds rows.

@history_item n=13 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — replaced the dialog deadline with the configurable dialog hold, which bounds the driver and never the human. It stays on the time-bounds rows. `decision/no-unwarned-timeouts` (SC 2.2.1) rests on all of these falling off the human.

@history_item n=14 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour decision emits no SGR under `--json` or a passthrough role, and none on `viola run` or a passed revive. This is the basis of `decision/no-sgr-off-terminal`. The rule stays after the status-line arm leaves, because the arm was one of the passthrough-role writers.

@history_item n=15 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — fixed `viola verify`'s output form: step lines and the summary, and the interrupted pair. `contract/viola-verify/step-line` prints `[NN/NN]` over the ledger rows. Run B loses its status-line probe, and `relied()` goes from 23 to 21, so the step count and the literal pins that spell it move with this chunk.

@history_item n=16 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — CLI output-discipline invariants (P4, P6) that live in crate tests are covered by the epoch-boundary `run --mutants` audit, not by a per-chunk gate (`decision/mutation-audit-covers-crate-invariants`). This is consistent with the directive: no mutation run in this chunk, and only a guard's one-off remove-the-guard control.

@history_item n=17 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the outer-PTY driver on portable-pty =0.9.0) — the tui boundary driver is portable-pty `=0.9.0`, and a11y inherits it as-is and never re-pins it (`decision/portable-pty-outer-pty-driver`). The removal's tui cases keep running on this driver.

@history_item n=18 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli output-discipline checks on std::process::Command and literal pins) — every fixed CLI string is pinned by a literal over `std::process::Command`, with no trycmd. A case that leaves or is rewritten here, including any that spell `hook statusline`, follows this literal-pin form.

@history_item n=19 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — changed `decision/no-screen-content-parsing` so the parsed signatures include the input box's rule and prompt rows. This is the source of this chunk's box-only fixture CARRY (cry-hqyx2rou). The fixture replays those signatures, and its recording waits on a live `claude` start.

@history_item n=20 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the tui passthrough boundary after a contained vt100 feed panic) — extended the `viola run` TUI passthrough entity: no viola literal reaches the outer terminal after a contained feed panic. It is a sibling boundary clause of `contract/tui-passthrough/viola-terminal-bytes`, and the removal must keep it.

@history_item n=21 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master's text into rows: decisions, contracts and absences with guards. It is the origin of every picked row handle (`decision/statusline-output-passed-byte-for-byte` with its guard `passthrough_with_a_statusline_source_adds_no_viola_bytes`, and `contract/viola-hook-statusline/exit`). A guard that names a deleted test must be re-pointed or dropped, as this conversion did («guards: 6 re-pointed · 2 left out»).
