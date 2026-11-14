@history_item n=1 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (budget.json read by the gate, not ratified) — introduced the very surfaces this chunk removes: `viola_state::budget::read_budget` (check_path on home and file, shared lock on `budget.json.lock`, `take(MAX_FRAME + 1)`, `v` = 1, every failure no reading, gate fails open), the `budget-paused` refusal of a driver's `send`, the `five_hour` / `seven_day` thresholds (defaults 90 / 85, `Percent`, `parse-rejected{config-json, malformed}` per bad key, no value switching the gate off, read in `src/run/budget.rs`), and the ordering that the string-`from` release refusal comes before `budget` is acted on. Matters: the inventory of what leaves (rows decision/budget-gate-fails-open, decision/failed-budget-read-no-reading, decision/config-cannot-disable-controls, contract/config-json/five-hour, contract/config-json/seven-day) and the release guard's ordering clause that must be rewritten without `budget` while the guard stays.

@history_item n=2 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** (budget.json ratified) — the founder ratified as built the boundary widening that a hook-written `budget.json`, outside the trusted-file list, decides a wrapper's refusal, read behind the strict-modes check and failing open; the trusted-file list stayed unchanged. Matters: removing the gate retires a founder-ratified widening (a narrowing), so the row decision/budget-gate-fails-open and its ratification record leave together; `budget.json` was never on the trusted-file list, so nothing there changes.

@history_item n=3 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** (the Epoch 6 delegate marks are his) — the founder's acceptance of leftover items also covered "the checked `budget.json` read". Matters: one more ratification record that rests on the budget read; the removal leaves it with no subject.

@history_item n=4 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (the settings override as landed, the hook rows, the arm's check before its log) — the Hook stdin row's statusline payload: no `rate_limits` object is no write, absent window `"unknown"`, `used_percentage` kept 0–100, `resets_at` epoch seconds or RFC 3339 written RFC 3339 UTC with ms, `budget.json` written only by this arm; `hook statusline` runs `check_instance` before its log, a refusal leaving `budget.json` unchanged. Matters: these are the rows contract/budget-json/* and decision/budget-json-single-writer / decision/budget-file-single-writer that leave; the pass-through, the shell-out and the instance check stay for the next entry, so the arm's rows must be cut to the budget half only (decision/statusline-payload-fails-silent says "no budget file").

@history_item n=5 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the two instance checks ahead of any log) — `hook statusline` runs its check before anything under the home is read or opened, a refused instance exiting 0 with no trace (was "`budget.json` unchanged"); revive's check before the wrapper log. Matters: the statusline arm's check-first, no-trace shape stands after the removal (rows decision/hook-strict-check-first, decision/strict-refusal-leaves-no-trace); only its `budget.json` wording leaves.

@history_item n=6 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `budget.json` gained its one checked read `read_budget_stamped` with two product paths (the gate's `read_budget` and `list`, which shows the reading and decides nothing), and the thresholds gained `list` as a reader through `read_thresholds`; the board's whole-board refusal and strict-modes entry point. Matters: the board's budget reading, `list`'s threshold read and the `list` payload's `budget` field leave with this chunk, while the board's strict-modes rows (decision/whole-board-refusal, decision/list-scan-checks-before-open) stay untouched.

@history_item n=7 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the board's one rule on the home) — Kept: `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home while the board takes the read rule. Matters: row decision/home-write-rule-checkers names the budget read as one of three write-rule checkers; with `read_budget_stamped` gone the row must lose its budget clause, `mcp` start and `plugin install` staying.

@history_item n=8 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (the seventh dated gap: pause and release frames) — Driver-originated `release`: params became `{budget?, from?}`, a string `from` refused `-32602` with `data.reason` `release-from-driver` (`ProtocolError::ReleaseFromDriver`) and one line `from_trust:"self-reported"`, another type `-32602` `data: null`; CLI `viola release` forwards `VIOLA_NAME` (exit 20); `release`'s `budget` absent or a bool. Matters: the guard this chunk keeps; the `budget` param and the "before any budget is acted on" ordering leave, so contract/channel-release/params, contract/channel-frames/release-params, contract/channel-release/string-from ("a budget pause is not lifted") and decision/driver-release-refused / decision/release-from-driver-guard need their budget clauses cut without weakening the refusal.

@history_item n=9 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (channel-response's closed detail word; the clients' screen of a peer's reply) — `release-from-driver` is one of three closed `-32602` detail words on the server's and client's `channel-response` line, and `sanitised_fault` keeps `data` only for the body whose `data.reason` is exactly `release-from-driver`. Matters: the kept release guard's log and client-side reading must survive the removal intact (rows contract/channel-response-line/release-from-driver, decision/channel-faults-sanitised).

@history_item n=10 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (viola mcp's exit on a cut before initialize) — trap recorded: "`exit 11` alone also reads `budget-paused`'s exit 11, so key on `ServeError::Initialize` and `initialize`"; `ServeError::Initialize` keeps 11 only as the unit child's code, the `viola mcp` process exits 1. Matters: when retiring CLI exit 11 (never reused), a sweep for `11` must not touch `ServeError::Initialize`'s unit-child code, and vice versa.

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census) — `tests/contract_error_bodies.rs` enumerates every error body with its writer and witness (`CLI_DOCUMENTS`, `MCP_RESULTS`, `CHANNEL_FAULTS`, exclusions) and `product_error_writers_are_each_named` fails on an unnamed writer. Matters: the scope lists the error-bodies contract among tests that change; the `budget-paused` exit-11 document and any budget writer must leave the census in step with the code, or the census reds.

@history_item n=12 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census) — `tests/contract_bounded_readers.rs` names each external boundary's bounded read and witness in `BOUNDARIES`, and `product_readers_are_each_bounded_or_named` requires every product reader bounded or named. Matters: removing the `budget.json` read, the thresholds read and the statusline `rate_limits` parse changes the reader set the census walks (row decision/bounded-readers-census); its tables must follow.

@history_item n=13 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog_hold_s` is read once at the wrapper's start by `run::dialog::read_hold`, "right after the thresholds", through its own `take(MAX_FRAME)` read with `v` = 1, `DialogHold` named "beside `Percent`" in `viola-core`. Matters: `config.json` and `dialog_hold_s` stay while the threshold read and `Percent` leave; the hold read's ordering and the config row (decision/config-tolerant-parse, decision/config-bad-value-handling, decision/config-cannot-disable-controls's "dialog never zero or unbounded" half) must stay true.

@history_item n=14 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — the `config.json` row: tolerant parse, closed values falling back and reported `parse-rejected`, the `MAX_FRAME` cap. Matters: scope item 2 asks what a `config.json` still carrying `five_hour` / `seven_day` reads after the removal; the config reader's unknown-key handling (counted, not refused) is the precedent this measurement settles against.

@history_item n=15 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — own state files on read: per-read `Skipped{unknown_kinds, unknown_fields, torn_lines}`, not logged; an unknown kind is not returned; a known-kind line with an extra key is returned and counted; readers never write. Matters: the "readers of old disk state" claim — an `events.ndjson` holding `budget-gate` lines and a snapshot holding `budget_paused` / `budget_override_until` must keep reading once `EventKind::BudgetGate` and the snapshot fields leave (rows decision/events-reader-bounded-lines, decision/state-readers-never-write).

@history_item n=16 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the `statusline-rate-limits` ledger row was created (with `statusline-shell` and `statusline-override`), stamped on 2.1.287 with two fixtures; the `hook statusline --capture` arm reads no `budget.json`. Matters: the rate-limits row, its `--record` fixture and its relied count leave here (scope item 6) while `statusline-shell` / `statusline-override` and the capture arm stay with the next entry.

@history_item n=17 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (statusline-rate-limits reads the account's plan) — the `statusline-rate-limits` row reads the account's status-line plan state, not the CLI (a verify red `23 pass  1 fail` on unchanged 2.1.287), owner 0.2.0 finding F-06i. Matters: the reason the row leaves with the budget gate rather than the status-line entry; the row's master sentence retires here.

@history_item n=18 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the stamp gating non-`null` dialog decisions reads twenty-four rows (`/24`), an older stamp of fewer ids reading `cli_verified:false`. Matters: dropping `statusline-rate-limits` moves the relied count 24 → 23 (row decision/dialog-answers-need-verify-stamp, contract/cli-exit-codes/12); the stamp's older-stamp rule must be re-read for a stamp still holding the retired id.

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the statusline shell-out, the settings override and the relied set hold on every build) — `LedgerRow::relied()` is all twenty-four rows on every build, counter `/24`, `20 pass  4 fail` recording `cli_verified:false`; the statusline ledger rows relied on every build. Matters: the current statement of the relied set this chunk edits to 23.

@history_item n=20 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange recorded by the test-only tap) — `fixtures/claude/<cli-version>/Mcp.*` is recorded by `viola-mcp-tap` between one live CLI start and a direct spawn of `viola mcp`, scrubbed and refused by closed codes, a founder-ratified widening capped at one start. Matters: scope item 8 re-records `Mcp.tools-list-1.json` for the MCP `list` description that names the budget; a re-record needs a live `claude` start, which this phase's directive forbids without a named operator step (rows decision/mcp-fixtures-via-test-only-tap, decision/fixture-scrub-and-refusal).

@history_item n=21 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (send's input validation as landed) — `validate_paste_text` runs client-side before any frame (exit 13) and again first in the wrapper's `send`, authoritative and first in its refusal order. Matters: removing the `budget-paused` rung from `send`'s refusal order must leave the control-character check first and the client/wrapper double check intact (rows decision/paste-validation-on-both-sides, contract/send-refusal/control-character).

@history_item n=22 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (trailing CR and LF removed; an empty typed text refused) — the wrapper refuses `empty-text` directly after the typed text is taken and before the first wheel read. Matters: the remaining wrapper `send` order after the budget rung leaves (rows decision/empty-typed-text-refused, contract/send-refusal/empty-text, contract/send-refusal/human-typing).
