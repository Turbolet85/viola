@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `revive` among the CLI verbs; the TUI passthrough is entered by `viola run` and by a passed `viola revive` (one shared start), a passed revive writing no line of its own while the child holds the terminal (`tests/chaos_revive.rs`); zero-SGR list and keystroke ban cover a revived start. Why it matters: the trust write lands in exactly that shared start, so every passthrough row (entry, own-bytes, human keystrokes, single vendor-content boundary) binds the revive arm too.

@history_item n=2 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — a start variant that wrote a file (the settings override) before the spawn got its own zero-viola-bytes reading (`passthrough_with_a_statusline_source_adds_no_viola_bytes`, `tests/tui_passthrough.rs`) and its time bound listed with whom it falls on. Why it matters: the direct precedent for a start that writes a CLI-facing file pre-spawn: the trust-writing start needs its own zero-viola-bytes / no-terminal-line reading, and any bound (lock wait, 3 attempts) must be named as falling on viola's start, never on a human key.

@history_item n=3 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver` dropped the statusline deadline; `− decision/statusline-output-passed-byte-for-byte` removed the statusline passthrough case. Why it matters: the time-bounds lists now read (MCP wait cap, dialog hold, readiness gate, confirm windows, agents read); a new pre-spawn bound from the trust write's lock/retry would extend these two rows by the same `~` form.

@history_item n=4 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/time-bounds-fall-on-the-driver`, `~ decision/time-bounds-never-on-human` (budget pause dropped from the lists). Why it matters: the last prior edits to the two time-bound rows this chunk rests on; the form for adding or not adding a bound to them.

@history_item n=5 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — §8: `revive` makes one `claude agents --json` read before its start (direct spawn, `take(MAX_FRAME + 1)`, 5 s kill); a failed, oversize, malformed or timed-out read passes `revive` (fail open), the bound falling on viola's own child, never on the human. Why it matters: precedent for a pre-spawn read of CLI state that fails open on an oversize/malformed input (`MAX_FRAME`) — the trust write's refusals (oversize, not the JSON expected) likewise write nothing and leave the dialog to the human; the "agents read" entry in the time-bound rows comes from here.

@history_item n=6 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — §8: on the driver's deadline the dialog renders for the human in the `claude` TUI (architecture's fail-open contract leaves the dialog to the human). Why it matters: the precedent for "a refused or lost write leaves the dialog to the human, viola typing nothing" — the trust dialog's fallback must keep that fail-open-to-the-human shape.

@history_item n=7 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — §8 / §3 keyboard harness: the readiness gate's bound falls on the driver only; a human key during the wait takes the wheel (`send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`), cases sequenced on records, never a timer. Why it matters: source of `decision/human-key-takes-the-wheel` and `decision/boundary-tests-sequenced-on-records`; the trust write adds no key and must not delay a human key past the spawn — any case for it is sequenced on the run-log closed word, not on time.

@history_item n=8 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — §3 keyboard harness: the first live reading of the takeover clause on Linux, "supplemental and never gating". Why it matters: origin of `decision/live-readings-never-gate`; the chunk's live `S1` / `S2` starts are supplemental readings in that sense — the gating proof stays on the fake agent.

@history_item n=9 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading (the founder's own key on the product over a live `claude`), still supplemental and never gating. Why it matters: how a live reading on the real CLI is recorded in the keyboard harness; the `S1`/`S2` readings of the trust dialog on 2.1.325 follow the same supplemental form.

@history_item n=10 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `~ decision/no-screen-content-parsing` except: the test-only standing live rig reads the trust selector's place, the input box as empty / holds-text / absent, and modal dialogs, as closed words only, raw frames local and gitignored. Why it matters: `S1` RED "when the first screen reads `trust`" and `S2` RED when it does not read `trust` rest on exactly this exception; the product write itself must not read the screen for trust, and any widening of what the rig reads needs this row amended again.

@history_item n=11 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/no-screen-content-parsing` rule: compiled signatures now footer literals, the input box's rule and prompt rows, modal literals, the prompt row only for its opening char; a boundary widening ratified. Why it matters: the product-side bound on screen parsing this chunk stays inside — viola decides trust from the config file read the CLI's way, not from the rendered screen.

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — the `viola run` / revive start / `verify` strict-modes refusals as scope entities and their text: refused before any log opens, exit 1, stdout empty, home-state pair with `hint:` last, no child (`checked_exe`, `refuse_tree`, `run_refuses_a_widened_home_tree_before_its_log`). Why it matters: source of `contract/start-refusal/home-strict-modes` and `decision/strict-modes-refusal-before-any-write`; the trust write sits after the version gate and before the spawn, so it must come after these refusals and add no write on a refused start.

@history_item n=13 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start: exit 1, revive's eleventh code) — `viola run` and a revive start refuse a failing socket directory before any snapshot, bind or child; `socket-dir-refused` under `revive --json`; the fixed-message `unable:` / `hint:` form for run's socket-dir refusal. Why it matters: source of `contract/start-refusal/socket-dir`, `contract/revive-json-refusal/socket-dir-refused` and `decision/socket-dir-refusal-before-any-frame`; the trust write must land after these start refusals and change none of them.

@history_item n=14 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive` takes `--json` on both arms: each exit-1 refusal one `{"v":1,"error":"<code>","detail":null}` document, empty stderr; without `--json` the `unable:` / `hint:` pair byte-identical. Why it matters: origin of `contract/revive-json-refusal/document` and `contract/revive-human-refusal/pair`; the trust write's refusals are no start refusal (the start proceeds, dialog left to the human), so they must print no document and no pair.

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start; the `cli` catch site's `internal-error` document) — every refusal of the start revive shares with `run` prints one exit-1 document (`pinned-hash-mismatch`, `squatted-name`, `internal-error` for a `claude` not found); a `pump_child` error prints no document, viola writing nothing past the child holding the terminal; `run` / revive's start stay silent at the catch site. Why it matters: source of `contract/viola-revive-json/start-refusal`, `contract/revive-json-refusal/*` and `contract/json-fault/internal-error`; a trust-write failure that is not a refusal must not reach this closed code set or the catch site's document.

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (revive's start refusals without batch-script-child; the tui boundary on openpty alone; the a11y cases' CI home; the Linux-only platform line) — revive's exit-1 codes read ten; the zero-viola-bytes case compares the outer-PTY stream byte for byte with an unwrapped fake-agent run, each boundary clause its own nextest case on `ubuntu-latest`; Linux only. Why it matters: the current form of `decision/zero-viola-bytes-on-the-terminal`, `decision/linux-only` and the start-refusal code set — a trust-writing start's boundary case runs on the fake agent on `ubuntu-latest` against that oracle, and no new closed word joins revive's set.

@history_item n=17 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the outer-PTY driver on portable-pty =0.9.0; the cli output-discipline checks on std::process::Command and literal pins) — the tui driver reads portable-pty `=0.9.0`, inherited and never re-pinned; cli checks via `std::process::Command` with literal pins. Why it matters: source of `decision/tui-boundary-driver-pin`, `decision/portable-pty-outer-pty-driver` and `decision/cli-a11y-by-literal-tests`; the chunk's boundary cases use that driver as-is.

@history_item n=18 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (a peer's refusal screened to a code word; the census of error writers and the body check) — `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first); `thiserror_messages_interpolate_nothing` and the census `product_error_writers_are_each_named` in `tests/contract_error_bodies.rs`; error bodies hold no absolute path. Why it matters: source of `contract/code-word/format` and `decision/thiserror-messages-interpolate-nothing`; the "one closed word per start, nothing of the file anywhere" must be a code word, any new error type interpolate nothing (no config path), and a new writer is named in the census.

@history_item n=19 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the tui passthrough boundary after a contained vt100 feed panic) — the passthrough boundary held after a contained failure: no viola literal on the outer terminal, the child's own bytes pass. Why it matters: precedent for recording a contained start-side failure as a passthrough boundary reading — a refused or lost trust write likewise puts no viola literal on the terminal (`decision/run-diagnostics-off-terminal`).

@history_item n=20 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (decision / contract / absence), each verified against the code. Why it matters: every picked row's current wording dates from this conversion; amendments this chunk proposes take the `+` / `~` / `−` row form.
