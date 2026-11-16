@history_item n=1 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters as rows: turned the a11y plan's text into the rows this chunk's picked set is drawn from, each row verified against the code. A master keeps only what a planner must respect, and the code holds the rest. If the scheduled-wake count adds a fact a planner must respect, such as the count reaching `wait --json` or MCP `wait`, that fact needs a row. A detail the code already shows needs none.

@history_item n=2 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (8. Cognitive Accessibility, 6 rows; 8. Cognitive Accessibility, 1 row; 6. Visual Design Verification, 1 row) — `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` dropped the budget pause from their lists of time bounds. Exit 11 is retired and never reused. `budget-paused` left `decision/state-carried-by-printed-word`. These are the bound lists this chunk rests on. Leaving `wait`'s wake rule unchanged adds no bound. If the count were ever read to hold or wake a `wait`, it would be a new driver-side bound, and these rows would have to name it.

@history_item n=3 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (8. Cognitive Accessibility; 3. A11y Assertion Harness Contract) — `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` dropped the statusline deadline, leaving the lists as they stand. `− decision/statusline-output-passed-byte-for-byte` and `− contract/viola-hook-statusline/exit` retired a hook arm. This is the latest edit to the time-bound rows this chunk must leave unchanged. It is also a precedent for the hook-payload-to-event path being trimmed rather than widened.

@history_item n=4 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ decision/dialog-hold-bounds-the-driver`, state moved from provisional to standing after the hold ceiling was measured live. The adjacent fact this scope places (a scheduled wake's prompt logs `origin: human`, and the next dialog is not held for the driver, `dialog_pending: false`) falls under this standing row. A held dialog goes back to the human at once on a human key. P3's block on what a further step would be must not weaken that rule.

@history_item n=5 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — set `dialog_hold_s` (60–3600 s, default 60). At the hold's end one `dialog-expired` line is appended, and it wakes a parked `wait`. The hold bounds the driver and never the human. `contract/config-json/dialog-hold-s` and `contract/event-line/dialog-expired` are picked rows. They are among the events that wake `wait` today, which this chunk's "wait wakes exactly as before" acceptance must leave alone.

@history_item n=6 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — the first deadline on a dialog: it falls on the driver's answer path, and the dialog then renders for the human. This is the origin of the rule now held by `decision/dialog-hold-bounds-the-driver` and `decision/time-bounds-never-on-human`. It is a precedent: every bound viola adds falls on the driver, none on the human.

@history_item n=7 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `wait` tool clamps `timeout_ms` to `WAIT_CAP` (50 000 ms, also its default), a bound on the driver only. MCP `wait` is a hop the count must reach, while its wake rule and cap stay unchanged. The cap is one of the bounds `decision/time-bounds-fall-on-the-driver` lists, alongside `contract/viola-wait/timed-out` (only a caller-passed timeout gives `timed_out`).

@history_item n=8 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — widened the non-editing set to a closed list: focus reports, mouse reports and terminal replies, with every other byte counting as editing (F-W2, the error only ever favouring the human). This is the source of `decision/non-editing-sequences`, `decision/stdin-classifier-non-editing-list`, `decision/wheel-ignores-terminal-events` and `contract/tui-passthrough/focus-mouse-resize-sequences`. It bears on the placed adjacent fact: a scheduled wake's prompt takes the wheel as `human`. Any further step that tells a wake from a typed key must not move this closed list without a ratified widening.

@history_item n=9 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the closed non-editing list was extended by exact grammar, and neighbours one field or one prefix away stay typing. The first live reading of the takeover clause was recorded as supplemental and never gating. This is the source of the "ratified, exact grammars" reason on `decision/stdin-classifier-non-editing-list` and of `decision/live-readings-never-gate`. A probe of a scheduled wake's wheel effect (no live start is given in this phase) is supplemental, never a gate.

@history_item n=10 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the second live reading of the takeover clause: a key the founder typed wrote `wheel{holder:"human",cause:"human-input"}`, the next `send` exited 10 `human-typing`, and `release` returned the wheel. This is the measured form of `decision/critical-path-human-takes-wheel`, `decision/send-refused-while-human-typing` and `contract/cli-exit-codes/10`. The adjacent fact (after a wake, sends are refused `human-typing`) is the same chain, set off by a wake's prompt rather than a key.

@history_item n=11 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — a key during the readiness-gate wait takes the wheel, and the waiting send is refused `human-typing` ("the human always wins"). The case is sequenced on log records, never on a timer. This is the source of `decision/human-key-takes-the-wheel` and its guard. Any refinement of how a wake's prompt is attributed must keep this outcome.

@history_item n=12 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (11. A11y Anti-Patterns) — `~ decision/no-screen-content-parsing`: the vendor screen is parsed only for compiled signatures, and the prompt row only for its opening char, never the text after it. A further step that tells a wake's prompt from a typed one must not read the prompt text off the screen. This matches the chunk's own no-content boundary (`IgnoredAny` elements, a count only).

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the plugin against a strict-modes-refused instance, not assertable) — a refused hook call exits 0 at once with empty streams, under 1.0 s: no `config.json` read, no stdin read, no frame, no line. This is the source of `contract/hook-call/strict-modes-failed`. The Stop hook is the path this chunk reads `session_crons` on, and that refusal path must stay ahead of any payload read. The plugin stays not-assertable (`decision/no-assertions-for-plugin-and-ipc`).

@history_item n=14 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the tui passthrough boundary after a contained vt100 feed panic; the Windows byte reading) — added the after-feed-panic boundary entity, the source of `contract/tui-passthrough/after-feed-panic`. This is a precedent for bounded, contained input parsing at a boundary. A malformed or oversized `session_crons` should read as none or `null` and fail no turn end.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail; a peer's refusal screened; the body check) — no peer text reaches a document. Strings become code words or nothing, and `assert_error_body_clean` reports place and class, never bytes. This is the precedent for the chunk's "no wake content reaches an event, log, reply or fixture" boundary and a ready witness pattern for asserting it.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document) — under `--json`, a verb that would print `error: internal error` prints the `internal-error` document with a null `detail` instead, and `run`, `mcp` and `hook` stay silent there. This is the source of `contract/json-fault/internal-error`. If carrying the count fails at the `wait --json` hop, it must take this form, and the hook must not print.

@history_item n=17 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — pinned the board caption NAME · LIVE · STATUS · WHEEL · DIALOG · CLI by the `CAPTION` literal, with each state carried by its word. This is the source of `decision/critical-path-cli-board-output` and `decision/fixed-cli-strings-pinned`. If P3 finds the count reaching `list` or `last`, any new caption, slot or fixed string must be pinned by literal and the slot order kept.

@history_item n=18 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — added the board's colour spans, each state still carried by its word, with no SGR under `--json`. It also asserts that `viola wait` carries no ESC and no lone CR. Under `decision/output-discipline-a11y-fallback`, a count added to `wait`'s human or `--json` output stays plain ASCII under these checks.

@history_item n=19 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: no web or graphical frontend. The cli path became `viola list`'s fixed captions and slot order, which the terminal fleet console reuses. This is the source of the wording of `decision/critical-path-cli-board-output` and of the absences and N/A stubs in the rows file. Any board-facing count stays in the terminal-only scope.

@history_item n=20 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli output-discipline checks on `std::process::Command` and literal pins; the §8 verification lines; the cli tooling) — renamed the cli a11y checks from assert_cmd and trycmd to `std::process::Command` with literal pins. This is the source of `decision/cli-a11y-by-literal-tests`, `decision/cli-output-discipline-assertion-tools` and `decision/fixed-cli-strings-pinned-by-literal`. A test of the new count on `wait --json` follows this form.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone; the a11y cases' CI home, the one ubuntu-latest leg) — the non-editing list and the wheel cases lost their Windows arms. The a11y cases run in the `coverage` suite on `ubuntu-latest` alone. That is the CI home of any a11y-bearing case this chunk adds, under `decision/a11y-inside-existing-suites`.

@history_item n=22 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli and tui cases run as nextest cases in the `coverage` suite, and §11 bans a separate a11y driver or a new `suite` value. This is the source of `decision/no-separate-a11y-driver` and `decision/a11y-inside-existing-suites`. Any wake-count case goes into an existing suite.

@history_item n=23 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary, and the keystroke ban covers its start. This is the source of `decision/single-vendor-content-boundary` and `decision/human-keystrokes-never-blocked`. A revived session's Stop payload carries `session_crons` the same way, so the count path is shared by both starts.
