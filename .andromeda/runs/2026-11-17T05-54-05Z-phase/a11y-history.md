@history_item n=1 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** (Windows zero-viola-bytes oracle is literal absence) — fixed the zero-viola-bytes clause's oracle: viola's own literals absent, plus "no SGR or cursor control the child did not emit" compared byte for byte with an unwrapped run; the Windows arm is retired since, but the byte-for-byte oracle is what decision/zero-viola-bytes-on-the-terminal still means. A rig that starts viola wrapping `claude` in its own pty must not be mistaken for, or weaken, that boundary oracle.

@history_item n=2 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — first put a deadline on `viola answer` and ruled it bounds the driver, never the human: at expiry the dialog renders for the human in the `claude` TUI. Origin of decision/dialog-hold-bounds-the-driver; the rig reads modal dialogs live and must leave this fail-open-to-the-human behaviour untouched.

@history_item n=3 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — widened the non-editing set from focus/mouse/resize to the closed list (focus reports, mouse reports, terminal replies DA1, DA2, CPR, DECRPM, kitty flags, OSC, DCS), every other byte editing (F-W2), proven by the classifier's unit table; case (3) sequenced step-wise. Origin of decision/non-editing-sequences, decision/stdin-classifier-non-editing-list and contract/tui-passthrough/focus-mouse-resize-sequences: any key or reply the rig types into a live wrapped session moves the wheel unless it is on that list.

@history_item n=4 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the readiness gate's 8.5 s bound falls on the driver only (`input-not-ready`, nothing typed); a human key during the wait takes the wheel and the send is refused `human-typing`; the outer-PTY case is sequenced on log records, never a timer. Origin of decision/human-key-takes-the-wheel, decision/boundary-tests-sequenced-on-records and contract/send-refusal/input-not-ready: rig readings that send while keys are typed meet this rule, and its tests must sequence on records, not sleeps.

@history_item n=5 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — extended the closed reply list by exact grammar (seven more CSI replies, proven on foot 1.28.0) and recorded the first live reading of the takeover clause on Linux (a `wtype` key into a session on a compositor the test started), "supplemental and never gating". Precedent for decision/live-readings-never-gate: whatever the standing rig reads live is evidence beside the gating nextest cases, never a gate.

@history_item n=6 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the same TUI passthrough boundary as `viola run`; a passed revive writes no line of its own while the child holds the terminal (`tests/chaos_revive.rs`); the zero-SGR list names it. Origin of contract/tui-passthrough/entry and the revive half of contract/tui-passthrough/own-bytes: a rig wrapping via revive is under the same boundary.

@history_item n=7 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — added a statusline-bearing start's own zero-viola-bytes reading and the 5 s statusline bound on the user's command; both retired by 2026-11-14-status-line-interception-removed. Context for the scope's "nothing of the founder's status line … in a record": viola no longer runs the user's status line, so the rig's isolation must not re-introduce it.

@history_item n=8 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `verify` stopped by Ctrl-C/TERM prints only the `unable: verify was stopped before it finished` / `hint:` pair, exit 1, no step line; its kill and pump join end at once. Named in contract/cli-refusal/unable-line; the rig's carried live `viola verify --record` (record-home) and its kill-on-every-exit-path hygiene meet this path.

@history_item n=9 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the web surfaces leave; the terminal surfaces stand (CLI output discipline, the TUI boundary, the wheel case, the manual terminal screen-reader passes); the tier `Standard (1)` kept, resting on the terminal surfaces alone. Basis of decision/a11y-tier-standard and the boundary-only rows this chunk binds.

@history_item n=10 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — cli/tui a11y cases run as nextest cases in the `coverage` suite under `gate --require coverage,doctest`, with no a11y artifact, no stage and no second driver of their own; §11 bans a separate a11y driver outside `scripts/agent-run.* run`. Constraint for the rig's home: a standing live rig must not become a parallel a11y driver or gate.

@history_item n=11 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the one `claude agents --json` read with a 5 s kill, fail open, the bound falling on viola's own child. The "agents read" of decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver.

@history_item n=12 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `wait` cap (`WAIT_CAP` 50 000 ms) is a driver-only bound. One of the bounds listed in decision/time-bounds-fall-on-the-driver; the hook-latency and scheduled-wake readings the rig carries are driver-side waits under the same rule.

@history_item n=13 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the confirm windows: 10 s fallback, 40 s for `/compact` on a verified CLI, unconfirmed `not-delivered` / `no-prompt-submitted`, the bound on the driver only. Origin of contract/send-refusal/no-prompt-submitted; the rig's carried whitespace-trim reading on submit bears on what counts as confirmed inside this window.

@history_item n=14 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the hold became `dialog_hold_s` 60–3600 s, default 60, under hooks.json timeout 3615 s; at its end the wrapper replies `null` and appends one `dialog-expired` line; any human key hands a held dialog back at once. Origin of contract/config-json/dialog-hold-s and contract/event-line/dialog-expired; the rig reading modal dialogs live must respect the hold and its human hand-back.

@history_item n=15 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading: the founder's own key on a live `claude` 2.1.296 wrote `wheel{human, human-input}`, the next `send` exited 10, `viola release` returned the wheel; supplemental, never gating, no agent typing the key. Precedent for decision/critical-path-human-takes-wheel live readings and for keys in live starts being the founder's (the scope's "the live cap and the keys are the founder's").

@history_item n=16 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the plugin against an impostor wrapper writes no human-facing byte) — the first not-assertable plugin entity: hook and MCP calls write no human-facing byte. Basis of decision/no-assertions-for-plugin-and-ipc; the rig's hook-payload readings (scheduled wake `source`) are ipc-internal and add no a11y assertion.

@history_item n=17 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the tui passthrough boundary after a contained vt100 feed panic) — added the feed-panic boundary: `send` refused `input-not-ready` while a human key reaches the child, none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` on the outer terminal, the child's bytes (`e4 b8 ad`) reach it (`tests/chaos_feed_panic.rs`). Origin of contract/tui-passthrough/vt100-feed-panic and after-feed-panic; relevant if the rig's screen model reuses the product's vt100 `=0.16.2`.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start) — a `pump_child` error prints no document, the child having held the terminal, viola writing nothing past it. Origin of contract/viola-revive-json/pump-child-error and contract/tui-passthrough/pump-child-error.

@history_item n=19 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder's word keeps the a11y tier `Standard (1)`. Confirms decision/a11y-tier-standard as standing.

@history_item n=20 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the tui boundary and the wrapper's drop of a refused peer) — a peer the wrapper refuses is dropped before its worker thread and any frame, writes nothing to the outer terminal and moves no wheel. Origin of contract/tui-passthrough/refused-peer.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone; the Linux-only platform line; the a11y cases' CI home; Orca on Linux) — the TUI hosted in openpty only, the three boundary clauses each a nextest case on `ubuntu-latest`, the zero-viola-bytes case compared byte for byte with an unwrapped fake-agent run; Linux only, 0.2.0 window features on Hyprland; the tui screen-reader pairing Orca + foot on Hyprland, founder-owned, nothing gated. Basis of decision/linux-only, decision/wrapped-tui-reads-as-unwrapped and the rig's host assumptions.

@history_item n=22 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the three-runner greens re-worded as dated readings) — a dated measurement is re-worded by its owner, never left as current. Precedent for how the rig's live readings (trim, hook latency, scheduled wake) must be recorded: as dated readings on a named CLI version.

@history_item n=23 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli checks on std::process::Command and literal pins; the outer-PTY driver on portable-pty =0.9.0; the cli tooling in the a11y testing contracts) — cli output discipline checked by tests driving the `viola` binary through `std::process::Command` with literal pins; the tui boundary driver on the workspace's exact portable-pty `=0.9.0`, which a11y never re-pins. Origin of decision/cli-a11y-by-literal-tests, decision/tui-boundary-driver-pin and decision/portable-pty-outer-pty-driver: a rig owning its own pty must not re-pin or fork that driver.

@history_item n=24 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the dialog hold bounds owned by 0.2.0's first entry) — `DIALOG_HOLD_MIN` / `DIALOG_HOLD_MAX` PROVISIONAL, owner 0.2.0's first entry; the done-check's live round did not read the hold, an unverified CLI replying `null` at once. Trap for live dialog readings: an unverified CLI never holds a dialog.

@history_item n=25 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the frontend stubs retired: no web or graphical frontend, §5–§7 and the web keys N/A, the console's keyboard reach owned by «Fleet a11y verification». Origin of the absences and N/A sections in the picked rows; the rig adds no surface.

@history_item n=26 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (decision · contract · absence), each verified against the code. Why the picked rows read as handles; later amendments name rows by handle.

@history_item n=27 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/no-screen-content-parsing: the rendered TUI is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char, never the text after it; screenshots never a11y evidence (R7). Key constraint: the rig's screen model reading trust and modal dialogs by drawn literals and selector rows, and its kept screens, are test evidence, not product parsing and not a11y evidence; any product-side widening is a boundary widening needing the founder's word.

@history_item n=28 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — ~ decision/time-bounds-fall-on-the-driver and ~ decision/time-bounds-never-on-human drop the budget pause; exit 11 retired. The current list of time bounds the rig's readings run under.

@history_item n=29 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver drop the statusline deadline; − decision/statusline-output-passed-byte-for-byte. viola no longer runs the user's status line, which shapes the rig's isolation (no status line of the founder's in a record).

@history_item n=30 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — ~ decision/dialog-hold-bounds-the-driver: provisional → standing, the ceiling measured live and the dialog read on screen after expiry. Precedent: an earlier live round's evidence scripts read dialogs on screen; the standing rig replaces those scripts and must reproduce that reading.
