@history_item n=1 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (send and human split by phase) — split `src/run/send.rs` into `src/run/send/` (`slot.rs` holds the one-in-flight `SendSlot`, `append_hook_event` with the driver relabel and the session-start post-condition claim) as a pure move; item 1's `slot.rs:188`/`:285` sit in that module, and the precedent is that a corrective epoch chunk moves no output, refusal, span or event.

@history_item n=2 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (the mutation scratch re-created owner-only) — the previous Epoch 4 cleanup's audit-driven mend of a red mutation baseline whose cause was the scratch dir, not the product check; precedent for reading a mutation-run red as an environment fact before touching product code (items 2, 3, 6).

@history_item n=3 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — killed epoch-boundary mutation survivors by passing the replace loop's retry decision in from the caller (testable on every host) and made the viola-pty watch report per-pid after a survivor read missed only because two jobs shared a report file; direct precedent for items 2 and 3 (killing tests, not product changes) and for checking whether a "survivor" is a shared-file test artefact.

@history_item n=4 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — made tempfile `persist` through ONE shared helper (`replace_private`, mode before write) the snapshot writer, reversing atomic-write-file; item 1 measures `SendSlot::confirm` under a failing snapshot write, so the failure mode comes from this helper (row `decision/tempfile-snapshot-writer`, `decision/atomic-replaced-state-files`).

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** ([Snapshot writer]: `replace_private` is one attempt) — removed the Windows retry loop: a failed `persist` returns at once with the target as it was; this is exactly the error the `?` before `slot.settle` propagates in item 1, so a forced snapshot failure is a single-attempt error with no retry to lean on.

@history_item n=6 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** ([Snapshot writer] no-clobber placement) — widened the snapshot-writer decision to `persist` and `persist_noclobber` with a forced-window test parameter (`place_private_new_with`), "a test parameter, not a seam (no env read, no feature)"; the pattern item 1's failing-snapshot measurement must follow, since the row forbids a seam that reads env.

@history_item n=7 marker=2026-10-10-session-links
**2026-10-10-session-links** (the link set in the replay, the snapshot and the start) — fixed the rule that a failed append leaves the set as it was while "a line that landed is in the set even when the snapshot write then fails"; the nearest standing precedent for item 1's case (event line landed, snapshot update fails, settle skipped): the log stays the source of truth (`decision/ndjson-logs-and-atomic-snapshots`).

@history_item n=8 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — moved `pending_dialog` to be written before its event line and written back on a failed append (a CI red fixed by ordering); precedent for ordering snapshot vs log writes so a failure leaves no dangling in-memory state, as item 1 needs for the claimed send.

@history_item n=9 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — introduced `send-issued` / `send-confirmed` / `send-refused` (log-only), the in-flight claim that relabels a prompt `driver` then settles the send, and "a paste failing after send-issued is input-not-ready with its cursor"; the contract item 1's claimed-but-unsettled line must end inside (rows `contract/wrapper-event-appends/send-records`, `contract/event-data/send-refused`, `contract/send/paste-failure-after-issue`).

@history_item n=10 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (send consumes the local-command list) — added the session-start post-condition claim (`/clear` by a new id), the remembered session id under the in-flight lock, `SendSlot::new` carrying `cli_verified`, and the window closing as `not-delivered`/`no-prompt-submitted` with cursor; a second claim path in `SendSlot` whose settle item 1's end must also cover.

@history_item n=11 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (/compact confirmed by its same-session SessionStart) — made `SendSlot::window()` the one bound `confirm`'s deadline reads (40 s compact, 10 s fallback) and added the compact claim arm; item 1's new end for a claim whose settle never comes sits on that deadline (rows `decision/confirm-window-compiled`, `decision/compact-confirm-window`, `decision/compact-confirmed-by-same-session`).

@history_item n=12 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — replaced exact match with the submitted-form match and added `decision/send-flight-lock` (post-gate reads, link, send-issued and paste under one flight lock shared with the hook.event claim and turn/wheel moves); item 1 touches the claim/settle that runs under this lock, and item 8's look at `send_through`'s guarded section is this lock's section.

@history_item n=13 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — added the stdout-echo confirmation (transcript tag count, `submitted_at` the read instant, no preceding line) and widened `unconfirmable`; another settle path of the same slot item 1 must keep consistent with `contract/events-ndjson/send-confirmed-order` and `decision/every-send-confirmed`.

@history_item n=14 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** (Established Decisions / Conventions / Design Philosophy) — landed `decision/paste-bound` (5 s, chunks of 4096, cut writes the closing marker with no Enter), `decision/paste-cut-mark`, the `paste-cut` detail and refusal rungs, and the wheel rule "past the current paste's in-flight chunk and closing marker"; item 3's three survivors are in this chunk's paste writer (`crates/viola-pty/src/pump.rs`), so killing tests must pin these rows, not change them.

@history_item n=15 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Epoch 4 delegate confirms ratified) — the founder's «Ратифицировать все» ended every PROVISIONAL mark of Epoch 4 (paste-bound, cut mark, refusal order, trust-record, verify child set, automatic verify, identity floor, cross-session row, MCP call deadline); this chunk opens Epoch 5 on rows that now stand ratified, so a mend that would move any of them is a new fork, not a cleanup.

@history_item n=16 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — added the identity-floor ledger row (pass/fail on unknown name or unread/off-shape env-names file, kept names dropped first), the `Env.<k>.json` env-names file of the capture arm, `measured.env_probe`, and grew the floor past eleven; item 2's first survivors (`env_names_fault`, `ledger.rs:2023` ×3) are in this check, whose readings `decision/identity-floor-ledger-row` fixes.

@history_item n=17 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the identity floor's Linux reading) — recorded the four known non-identity hook names (`CLAUDE_ENV_FILE`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR`) beside the floor; the name classes `env_names_fault` sorts, so killing tests for item 2 should pin known-floor, known-non-identity and unknown names.

@history_item n=18 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — moved the readiness refusal from any modal-literal row to a standing modal, with the except that a quote only counts when not option-shaped and the box's rows stand below it; item 2's `screen.rs:126`/`:135` survivors are in this predicate (rows `decision/screen-signatures-compiled`, `decision/vt100-readiness-gate`, `contract/send-results/input-not-ready`).

@history_item n=19 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — made a screen ready on a footer literal or the box's own rows (rule row of the rule char alone, at least 40, above a prompt-char row), changed verify's box wait and the screen-fixture rule; the boundary values (40, rule char, prompt char) are where `screen.rs` mutants survive, so they are the cases item 2's tests must pin (rows `decision/verify-box-wait-for-literal`, `decision/screen-fixtures-signature-rows-only`).

@history_item n=20 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `GATE_MAX_WAIT` 8.5 s, the wait living in `Screen::verdict`, with the recorded trap: "any case that reads a verified literal-less screen through a clock that never advances now waits forever"; a trap for item 2's new `screen.rs` killing tests and for the send-time bound (`decision/send-time-bound`, `decision/send-waits-out-paste-hint`).

@history_item n=21 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Standard Contracts / CLI Version Compatibility) — built `run::agents` (`read_logged`: R8 strip, `take(MAX_FRAME+1)`, 5 s kill, one `agents-probe` pair) for `list`, revive's preflight and verify's Run B (`typed::agents_read` with `VERSION_DEADLINE`), and the `agents-join` relied row; item 4's `src/run/agents.rs:39` bound is this read, shared by every spawner, and a bound of its own for Run B must not move `list`/`revive`'s fail-open read (rows `decision/agents-listing-never-errors`, `decision/agents-join-read-ungated`).

@history_item n=22 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (revive refuses a session id live elsewhere) — revive's `session-live` reading from the same agents read, `--fork` skips it, a failed read passes (fail open); any bound change in item 4 changes this preflight's outcome under a stall (row `decision/revive-refuses-live-session`).

@history_item n=23 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (MCP list builds the board and reads agents) — made `viola mcp`'s `list` tool a fourth spawner of the same `claude agents --json` read through `board_document`; item 4's bound reaches it too (row `decision/list-is-caller-side-read`).

@history_item n=24 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (bounded readers, a cross-cutting pattern held by a census) — every product reader of external bytes bounded in its own statement or named, held by `tests/contract_bounded_readers.rs` (`BOUNDARIES`, census); item 4's agents read is one of its boundaries, so a changed bound must keep its `Boundary` row and witness standing (row `decision/bounded-readers`).

@history_item n=25 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — set the 55 s MCP call deadline and rewrote `decision/send-time-bound` after measuring (the old clause was false), with readings vs measurements kept apart; precedent for item 4's rule: a new bound only from a measurement, never a retry or loosened bound.

@history_item n=26 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — moved test homes onto a tmpfs link after a start stalled behind other builders' writes, and set the standing rule "a stalled-start red is a finding about the backing, reported and never re-run for green"; governs items 4 and 5, both red only under a neighbour's IO stall (row `decision/dev-host-e2e-home-on-tmpfs`, `decision/test-side-removal-outside-workdir`).

@history_item n=27 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the keepers refuse a linked base below the 9 GiB headroom floor and the census's `--case` form named a truncated profile's killer; prior cleanup precedent for measuring a host-condition red by a witness before mending, relevant to items 4–6.

@history_item n=28 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — raised the shared root `WITHIN` bound 7 s → 12 s and moved test sockets under a short `/tmp` root; the one recent precedent of a timeout raise, which this chunk's boundary forbids without measurement (items 5 and 6 keep `ABORT_DEADLINE` and the 15 s kill unraised unless measured).

@history_item n=29 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — registered the test-only self-spawn markers `ROOT_TIED_TEST_CHILD` and `HARNESS_TIED_TEST_CHILD`; item 5 reworks the boot supervisor test (`crates/viola-e2e/tests/harness_lifecycle.rs`, `crates/viola-e2e/src/harness/boot.rs`) whose tied-child machinery these names occupy.

@history_item n=30 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify's spawn pairs, the harness CI read, verify-stamped homes) — harness session homes are stamped at `boot` step 4 by `viola verify` against the fake agent, and `run --local-live` refuses under `CI`; explains part of what item 5's boot body times (copies plus a verify) and binds the no-live-start boundary (row `decision/no-live-claude-in-ci`).

@history_item n=31 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — the harness session guard's `Drop` keeps homes under `AGENT_RUN_KEEP_HOMES`/`AGENT_RUN_KEEP_FAILED`, and `boot` writes `bin/agents-mode` beside the session's `bin/claude` copy; item 5 moves binary copies out of the boot test's timed body and must keep these files and keep rules (rows `contract/harness-environment/agent-run-keep-homes`, `-keep-failed`).

@history_item n=32 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — test-home removal through `remove_owned`, deleting `owner.json` last so an interrupted removal keeps the record; item 5's supervisor test aborting under its deadline must leave homes the sweep can still take (row `decision/test-home-owner-sweep`).

@history_item n=33 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the shared Frozen guard) — `tests/support/frozen.rs` became the one SIGSTOP guard, polling `ps` state `T` under `WITHIN`; a timed wait that a host stall can push past its bound like item 5's (row `decision/root-tests-freeze-wrappers-with-one-guard`).

@history_item n=34 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — mutation left CI and pre-push and runs only through `agent-run run --mutants` for the epoch-boundary audit; frames items 2, 3 and 6 (survivors and the `mutants` profile read only at the audit, and this chunk runs no mutation run) (row `decision/mutation-at-epoch-boundary`).

@history_item n=35 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (Infrastructure Patterns) — `.config/nextest.toml` holds only the `ci` and `mutants` profiles; item 6 edits the `mutants` profile's kill for `contract_fake_agent_drift`, so the profile set it changes is these two.

@history_item n=36 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the pattern and its contract binary) — two root binaries were added to the `ci` profile's kill-line filter in `.config/nextest.toml`; precedent for how per-binary kill lines are named there, relevant to item 6's per-test override.

@history_item n=37 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (the mutation scratch dir on the Linux host) — every mutation run takes a NOCOW btrfs `TMPDIR` because cargo-mutants copies the tree and a reflink drops the fake agent's exec bit; host facts that bear on reading item 6's cold parallel baseline and the survivors of items 2–3.

@history_item n=38 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (mutation runs: no scratch arm, no Windows leg) — `run --mutants` has no scratch arm and the audit runs on Linux alone; the mutation set item 6's profile serves.

@history_item n=39 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's dirs removed on pass, fail, Ctrl-C or TERM through `cmd::verify::stop`, while "SIGKILL, a crash, a failed install … leave the dirs by hand"; item 7 measures exactly the SIGKILL residue this entry declared outside cleanup (row `decision/verify-probe-dirs-drop-guarded`).

@history_item n=40 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's stop: `SIGINT` / `SIGTERM` alone) — the leftovers are SIGKILL, a crash and a failed handler install; the current form of the residual item 7 may name.

@history_item n=41 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify writes a trust record for its own temp folder only into the config its child reads, never creating the file, and removes it after pass, fail and TERM; Run F added (six runs); `CLAUDE_CONFIG_DIR` read when kept; the closed trust-writer words; item 7's suspect writer of a project entry under a verify scratch root is this path, and a SIGKILL skips its record guard (rows `decision/verify-own-trust-record`, `decision/verify-removes-only-own-dirs`, `decision/trust-record-ledger-row`, `contract/trust-writer-words/*`).

@history_item n=42 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — run and revive now write a trust record for an untrusted spawn folder; `key-present` and `removed` are verify's alone, a start reads them as `lost`; item 7 must tell verify's leftover entry from a start's deliberate one (rows `decision/run-start-order`, `contract/trust-writer-words/key-present`, `-removed`).

@history_item n=43 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (verify's print-mode probe leaves an empty CLI project dir) — the CLI's project dirs under `~/.claude/projects/` that verify's runs leave were accepted by the founder as a residual class; precedent for item 7's "named residual" outcome (row `decision/verify-transcripts-accepted-residual`).

@history_item n=44 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — a start runs an automatic verify under `ledger/verify.lock`, never re-verifying a red version; a second way a verify (and its trust record) can be killed mid-run, which item 7's measurement should cover (rows `decision/no-automatic-reverify-of-red`, `decision/run-start-order`).

@history_item n=45 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (Run E's resources and a killed PTY child's exit status) — Run E resumes Run B's first session id and a child killed at a stop closes with a `child_exit_status`; the measured stop behaviour item 7's SIGKILL case differs from (row `decision/verify-run-e-resumes-run-b-first-session`).

@history_item n=46 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the split modules) — split `src/cmd/verify/typed.rs` into `typed/` with `turns.rs` holding `framing_turns`, `agents_read`, `probe_until`; item 8's one step function for `framing_turns`' per-paste probes lives there, and `relied_here` was folded into `relied()` (item 9's precedent of one source).

@history_item n=47 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's compiled pastes each added only into rows holding the input box, waits after the long and tag-like turns bounded by `PROBE_DEADLINE`, `FRAMING_TURNS`; the per-paste guard shape item 8 would fold into one step, a ratified widening of verify's child set it must not change (row `decision/verify-child-set`).

@history_item n=48 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — Run B's eighth paste `/context` and the `local-command-context` row (a failing read is fail, never absent); the latest per-paste probe item 8 must keep identical in outcome (row `decision/local-command-context-probe`).

@history_item n=49 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — the 29th relied row and a print-mode sender inside Run B's cross-session turn; the count item 9 wants from ONE source, and another step inside the framing turns item 8 touches (rows `decision/verify-child-set`, `decision/cross-session-prefix-ledger-row`).

@history_item n=50 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — step lines' `MM` = `LedgerRow::ALL.len()` and "counts carry their rule"; the first statement that the relied count has one source, which item 9 re-establishes (rows `contract/viola-verify-output/step-line`, `-summary-line`).

@history_item n=51 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set: all rows on every build) — `LedgerRow::relied()` became every row of `ALL`; with the per-OS split gone, a second literal count anywhere is now pure duplication, item 9's target.

@history_item n=52 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (permission-request-question why) — reworded «needs a 22nd row» to «a further relied row» "with no count to go stale"; the masters' own rule for item 9 and for the wrap's drift-base detector over count literals.

@history_item n=53 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set) — `contract_ledger_probes` stamps every set on its `STAMPED` list; the stamped-set table item 9 would hold once.

@history_item n=54 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verified means every relied row counts (`pass`, or `absent` on browser-tools-prompt alone), the summary's N counting rows that count; item 9's single count source must keep this counting rule, not a raw pass count (rows `decision/version-verified-all-relied-rows`, `contract/ledger-stamps/verified-version`).

@history_item n=55 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Cross-cutting Patterns) — `decision/mixed-version-tolerance` gained "a stamp holding a retired ledger row verifies on the relied rows alone"; item 9's one-source count must not count retired ids in an old stamp.

@history_item n=56 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the stamps writer's strict-modes check) — `update_stamps` checks strict modes before creating `ledger/` and again under its lock; any item 9 change to the stamped-set handling stays behind it (rows `decision/stamps-writer-checks-before-and-under-lock`, `decision/stamps-single-writer`).

@history_item n=57 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (decisions on the six-row stamp, the strict stamps read) — `run`'s gate reads stamps through `read_stamps_strict`; a refusal is `cli_verified:false` with one parse-rejected line (row `decision/version-gate-strict-stamps-read`), bounding where item 9's count may be read.

@history_item n=58 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — closed an advisory by an exact-pin bump to a patched version, "with no ignore widened", the stack row carrying the version only; the precedent for the chrono advisory's "found → the ignore leaves with the upgrade" branch (rows `decision/chrono-timestamps`, `decision/workspace-dependency-pins`).

@history_item n=59 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (RUSTSEC-2026-0441 red, owned by «Version done-check») — an unmaintained-crate advisory carried as a route CARRY to a version move "with no ignore-list widening"; the shape of the chrono advisory's "none → re-pinned" branch.

@history_item n=60 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (portable-pty pinned at =0.9.0; RUSTSEC-2026-0441 closed) — moving the pin dropped the advisory's crate from the graph and `cargo deny check` read ok with `ignore = []`; precedent for measuring whether a safe chrono upgrade exists and how deny reads it.

@history_item n=61 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (cargo-deny: one Linux triple, one 0BSD exception) — `deny.toml` evaluated for `x86_64-unknown-linux-gnu` alone, and an exception naming a crate no longer in the graph was dropped after a `license-exception-not-encountered` warning; directly analogous to how cargo-deny may treat an ignored advisory id once an upgrade lands (row `decision/cargo-deny-dependency-policy`).

@history_item n=62 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the serde_json unbounded_depth ban and the deny probe count) — `deny.toml` feature ban and `deny-probes: 14/14`; any `deny.toml` edit for the chrono advisory must keep the probes green (row `decision/cargo-deny-bans`).

@history_item n=63 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (counts, crate edges and one order finding) — named `viola-agent-claude`'s `chrono` edge (workspace pin, `clock` + `std`); a chrono upgrade's reach includes the agent crate's edge besides rmcp's (rows `decision/chrono-utc-millisecond-timestamps`, `contract/timestamps/format`).

@history_item n=64 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** — retired an ordinal wait count for one counted by its pattern, with the trap that a number is true only with its rule beside it; precedent for item 9 and for the wrap freight's drift-base detector over count literals (row `decision/root-waits-share-one-bound`).

@history_item n=65 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers moved onto the replay) — every product snapshot reader goes through `read_snapshot_or_replay`, an unreadable snapshot meaning no endpoint; context for item 1's failing snapshot write (a later reader meets the log, not the in-memory slot) (row `decision/snapshots-read-through-replay`).
