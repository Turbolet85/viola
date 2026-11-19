@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (fake agent's print-ban exemption) — the fake agent is a `[[bin]]` of the root package (`src/bin/viola-fake-agent.rs`), exempt from the print-macro ban by a crate-level allow only. This chunk's size risk is the fake agent replaying a recorded set, so any version-keyed change to it stays under that one bin-level allow and adds no other lint carve-out.

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (verify's outcome on stdout, the uninstrumented `hook --capture` arm) — verify's agent-readable outcome is the exit code plus the stdout step and `stamped <version>  N pass  N fail` lines. The hidden capture arm that the recorder's probes run is uninstrumented by design. A `viola verify --record` on 2.1.328 is read by those stdout lines and not by any log line.

@history_item n=3 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — in CI, verify runs only against the fake agent at a recorded version (2.1.283 then), and the real-`claude` verify runs only through the local live run, which is refused under `CI`. Precedent and constraint: the 2.1.328 recordings are live/local only, and CI keeps replaying a stamped set through the fake agent (the rows `ci-verify-fake-agent-only` and `real-cli-verify-never-in-ci`).

@history_item n=4 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — CI's `viola verify` moved to the recorded 2.1.287 (was 2.1.283). It also added `parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}`. This is the direct precedent for re-pointing CI and the probes' `STAMPED` at a newer recorded set: a version move is recorded as a master change. The ledger-stamps line is unchanged by a re-stamp.

@history_item n=5 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added the `verify-pty-probe` subject. Each interactive PTY run of verify is one `process-start` / `process-exit` pair with no row text, prompt or path. In CI, verify drives the fake agent's interactive runs (`--screens --turn-stop --trusted-root`). A 2.1.328 recording runs these same pairs.

@history_item n=6 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the fake-agent verify flag list gained `--dialogs`, and the capture arm's `--answers` body was added. The fake agent's flags are bound to test-plan §3's boot step 4. Any fake-agent change for 2.1.328 moves that list in both masters.

@history_item n=7 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify flags gained `--framing` (boot step 4 and the stamped-home fixture stamp through the framing replay). This is a precedent for a fake-agent replay change moving obs §4's CI bullet together with test-plan.

@history_item n=8 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — an unconfirmable local command opens no `run.confirm_window`: it writes one `send-issued` and one `send-confirmed{confirmed:false}`, and no hook line is awaited. `/clear` confirms on `session-start`. This bears on C3: `/context`'s post-condition half is unread on 2.1.328, and the master's send outcome shapes do not move with this recording.

@history_item n=9 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — E2's identity-floor sentence: the strip always removes the 11 floor names. A per-version Linux reading (`claude` 2.1.287) stands beside it and moves nothing. Precedent: a new CLI version's environment reading is recorded in architecture, not obs, and the logged strip fields stay names only.

@history_item n=10 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `process-exit{detail:"interrupted"}` on a stopped verify, after every spawn pair it opened is closed. A stopped run writes no stamp, and the probe dir is removed on pass, fail and stop. Trap for the live `R4` start: an interrupted recording stamps nothing and must be re-run whole.

@history_item n=11 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify's stdout prints one `[NN/MM]` step line per relied row, `MM` being the relied count, and the summary sums to it. The second capture arm was uninstrumented (since retired). These are the step and summary shapes a clean 2.1.328 `verify --record` prints.

@history_item n=12 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E (`--resume`) was added, and a PTY child killed at a stop carries `child_exit_status`, as measured. This continues the verify spawn-pair count that the recording reproduces.

@history_item n=13 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify's Run B makes one `agents-probe` read (the eighth spawn pair). `parse-rejected{parser:"claude-agents-json"}` is written on an oversize or malformed answer, and agents rows are never logged. A 2.1.328 `claude agents --json` shape change would surface as that line and the value `unknown`, never as row text (rows `agents-probe-parse-failure-unknown`, `agent-rows-never-verbatim`).

@history_item n=14 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` confirms on `session-start` cause `compact` within `COMPACT_CONFIRM_WINDOW` (40 s, provisional), and `MM` became 24 with `local-command-compact`. The relied-row count and the compact window are among what the recorded set's drift suites replay.

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`viola verify`'s home-tree check) — verify checks the home tree before its log opens, and a refusal writes no line. `update_stamps` runs `check_stamps` before and under its lock, and a refusal ends verify's own `process-exit` `strict-modes-failed`. These are the constraints on the rig's private-config home for `R4`, and the source of the rows `strict-modes-failure-lines` and `verify-process-exit/strict-modes-failed`.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`verify` writes its refusal's code; the `process-exit` `detail` enum's count) — verify's `process-exit` under a parsed `VIOLA_NAME` carries `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture` or `interrupted`. A failing row carries no code, and the fixture's `<file>` enters no code. `unclean-fixture` is the recorder hygiene's refusal word: a payload the hygiene refuses is a red, never a hand scrub.

@history_item n=17 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows host arms of the panic capture, the env strip, verify and the statusline retired) — the relied count reads all 24 rows of `ALL`, a verify stop reads `SIGINT` / `SIGTERM`, and CI is the single `ubuntu-latest` leg. The 2.1.328 set is recorded and replayed on Linux alone, with no per-OS relied counts.

@history_item n=18 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, each checked against the code. The source of the picked rows `fixtures-not-telemetry`, `claude-fixtures-scrub-required` and `fixtures-not-a-telemetry-source`: recorded fixtures are scrub-required data and not a telemetry source, and observability adds no fixture content. This bounds the chunk's obs footprint: landing a fixture set adds no event, field or line.

@history_item n=19 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (8. PII Scrubbing & Compliance: `decision/secret-scan-before-upload`) — the secret scan's classes are `CLAUDE*` values, the content canary, mode 0600 and an unreadable file, and the cookie class is retired. Diagnostics from the live recording homes fall under these classes, and the recorder's own scrub stays the fixture-side control.

@history_item n=20 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (4. Span / Trace Coverage: `decision/verify-lines-carry-no-content`; the capture arm rows) — Run B's status-line probe and its captures were retired, leaving one hidden capture arm (`hook <event> --capture`). No verify line carries a status-line payload. A 2.1.328 set records no status-line captures, unlike older sets.

@history_item n=21 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — a live round on 2.1.325 measured that the folder-trust keys write `wheel{human-input}` before `session-start`, so the start order is read from that start's own lines in `events.ndjson`. This is the precedent for live starts under the rig: the recorded start order on 2.1.328 (with its new trust-dialog decline text) must match the amended rows `run-start-events/order` and `product-order-from-events-log`.

@history_item n=22 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — `hook_event` gained `stop-failure` (ten closed hook names). StopFailure stands with no ledger row and no verify probe, so the recording takes no StopFailure row. Its live reading belongs to the next route entry, «A live StopFailure payload», which this chunk sits directly ahead of (C1).

@history_item n=23 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (the StopFailure no-ledger-row answer ratified; the send trailing-only tolerance ratified) — the founder ratified «no ledger row» for StopFailure, with its live reading owned by the 2.1.325 recorded-fixture-set entry. Trap: a fixture-set chunk was named as the StopFailure reader before. C3 moved a carry from «A live StopFailure payload» into this entry, so the plan must say whether this recording reads StopFailure or leaves it to that entry. The ordinary-send trailing-whitespace tolerance (row `confirm-window-close/ordinary-send`) stands ratified.

@history_item n=24 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — a StopFailure error word outside the CLI's set reads cause `api-error` with one `parse-rejected{parser:"hook-stdin", detail:"unknown-value"}`. These are the exceptions in the picked rows `parse-failures-unknown-counter` and `parse-failures-surface-as-unknown`. A 2.1.328 error word outside the set would show as that line in the recording homes.

@history_item n=25 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — a local command's echo confirmation counts the stdout tag in the session transcript, keeping no byte. The echo read opens no span or line, refuses a symlinked, non-regular or foreign transcript silently, and expiry is `no-prompt-submitted`. This is the mechanism behind C3: `/context`'s stdout-tag half was never read on 2.1.328 because the transcript was absent. Reading it from a readable transcript beside this recording must stay within these rows (`claude-internals-not-a-contract`, `transcripts-not-a-contract`, `transcript-echo-read-uninstrumented`).

@history_item n=26 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** (the delegate's confirm ratified by the founder on the local-command-delivery rows) — the transcript-echo exceptions in `claude-internals-not-a-contract`, `refusal-telemetry`, `send-path-log-events/event-order`, `deterministic-event-emission`, `deterministic-emission` and `transcripts-not-a-contract` stand on the founder's own word. C3's transcript reading rests on these as settled, not provisional.

@history_item n=27 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added verify's Run F (six PTY runs, nine spawn pairs). Verify reads the kept `CLAUDE_CONFIG_DIR` value and writes and removes its temp folder's trust record, uninstrumented, with the outcome only on the stamps. This governs the `R4` run under the rig's private config, located by `CLAUDE_CONFIG_DIR` and never logged. A 2.1.328 trust-lookup change would surface only in the stamps.

@history_item n=28 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — a verified start sets its folder's trust flag only and writes one `trust-record` line (16 closed start words), and the run and revive start read `CLAUDE_CONFIG_DIR` too. On 2.1.328 a trust prompt appears only with `--add-dir`, and the trust dialog has a new decline text. A recording that meets a new trust shape must not add a start word or line outside this closed set.

@history_item n=29 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verify's step line takes `pass|absent|fail`, and the browser-tools-prompt row reads `absent` when no run met the prompt and still counts toward exit 0. The prompt reading is uninstrumented. A 2.1.328 recording may stamp that row absent and still be clean (the browser-tools key and confirm label are unchanged from 2.1.325).

@history_item n=30 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — verify's stdout gains one fixed line on a met browser-tools prompt, the only record that the prompt stood at start. This is the chunk that stamped the live home 24/24 on 2.1.328, and this fixture set follows directly from it. The recording's stdout may carry that line, and it adds no log line.
