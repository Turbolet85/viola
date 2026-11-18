@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the six landed ledger rows, `viola verify`'s probe, the stamps envelope) — landed the closed ledger in `viola_agent_claude::ledger`, `viola verify`, the stamps envelope `{"v":1,…,"writer":"verify"}` with per-version `rows` + `measured`, merge keeping unknown versions; a version verified only when every row reads pass. The base the 24th row and its stamp extend.

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the version gate placed and split by crate) — Claude parsing, rows, stamp verdict in the pure `viola-agent-claude` (no `viola-state` edge), the spawn in `run::version_gate`, stamps I/O in `viola_state::stamps`. The new literal and row belong in the agent crate under this split (decision/agent-claude-pure, decision/claude-shapes-in-one-crate).

@history_item n=3 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify's spawn pairs, verify-stamped homes) — harness homes are stamped by `viola verify` against the fake agent and root homes by `stamped_home` at a recorded version; a row-count change moves every stamped test home, so the fake agent must pass the new row on CI.

@history_item n=4 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (StampError folded into AgentError) — a non-`v:1` or wrong-shaped stamps envelope is `AgentError::StampsMalformed`, read as `cli_verified:false` with one `parse-rejected{parser:"ledger-stamps"}` (decision/malformed-stamps-unverified).

@history_item n=5 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (the readiness gate as landed) — the screen model is the pure `viola_agent_claude::screen::Screen` fed on `run`'s feed thread (`src/run/gate.rs`); the signature format is `Signatures { input_box, modals }`, two compiled literal lists, row-contains matching; a poisoned model reads `input-not-ready`. The browser-tools literal joins the `modals` list in this format.

@history_item n=6 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (confirmed send: kinds, refusals, exits) — the partial gate on an unverified CLI (poisoned or not quiet refuses, quiet is ready with no row read) and the `send` refusal order ending at the gate's `input-not-ready`; a modal on an unverified CLI is not refused by the gate, which bounds what this chunk's refusal covers (decision/partial-gate-on-unverified-cli, decision/unverified-cli-transport-only).

@history_item n=7 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (decisions on the six-row stamp, the strict stamps read) — `run`'s version gate reads stamps through `read_stamps_strict`; a strict-modes refusal is `cli_verified:false` with a `parse-rejected` line (decision/version-gate-strict-stamps-read). The gate that decides whether the new modal literal is read.

@history_item n=8 marker=2026-10-05-t00-09-13-wrap
**2026-10-05-t00-09-13-wrap** (the typed-input probe ratified) — the founder's R-S2 widening ratified a PTY-driven `viola verify` probe against the live installed claude, local only; screen-signature rows ride it. The ground on which a screen-literal row is probed by verify's interactive runs (decision/real-cli-probe-local-only).

@history_item n=9 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (screen signatures compiled, ten ledger rows, verify's interactive runs) — `SIGNATURES` compiled with `modals: ["Yes, I trust this folder", "Yes, allow external imports"]`; `modal-signature` / `input-box-signature` rows; `run` passes `Some(&SIGNATURES)` only when `cli_verified`; `Screen::rows()` verify-only; Run A killed, no key into a CLI dialog. The precedent for adding a modal literal and its row, and the never-type rule (decision/verify-never-types-into-dialog, decision/screen-rows-read-only-by-verify).

@history_item n=10 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (registry rows for verify's interactive runs and screen fixtures) — `schemas/claude-screen.v1.json`, `Screen.<phase>.json` signature-rows-only fixtures, the `--record` refusal naming file and code never content, the fake agent's `--screens` option (argv, no env). Where a recorded browser-tools screen and its fake-agent replay would land.

@history_item n=11 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** (fourteen ledger rows, Runs C and D) — added Runs C and D with their own dirs and settings, answered only by the probe hook; a run that raised no dialog fails its rows. Runs B, C, D and F are where S0R-3 saw the browser-tools prompt; the new probe must read it without typing in these runs.

@history_item n=12 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (seventeen ledger rows, Run B at four pastes) — Run B's pastes go only into rows holding the input-box literal and no modal; a stamp of the older id set reads unverified. A standing modal on Run B's screen therefore blocks every paste row, which is why the prompt failed 2.1.325 verifies.

@history_item n=13 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (the paste-hint reading, the fake agent's eighth option) — measured under the fake agent that a quiet screen with no compiled literal ends `input-not-ready` with nothing typed; added `--tag-turn-screen <phase>` drawing a recorded `Screen.<phase>`. Precedent for a fake-agent screen option replaying a recorded screen.

@history_item n=14 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the long-paste frame as measured beside typed text and between two pairs) — the founder's ruled limit: a shape no `send` relies on needs no probe (decision/no-probe-for-unrelied-shapes, decision/capability-ledger-gate's except). The test for whether the browser-tools literal needs its row: run's gate and verify rely on it, so it does.

@history_item n=15 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** (the gate waits for the input box, bound 8.5 s) — on a verified CLI a modal row refuses at once, while a quiet screen with no input-box row waits until `GATE_MAX_WAIT` (8.5 s); send blocks at most 18.5 s. An unrecognised browser-tools prompt is read as such a quiet literal-less screen and waits out the bound; this chunk turns it into an immediate modal refusal. Trap: a verified literal-less screen read through a clock that never advances waits forever.

@history_item n=16 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (the live readings on 2.1.287, the hint line's wording) — fixed the `input-not-ready` hint line «<name> was not ready for input; send again, and if it repeats a human must look at the session» (contract/human-hint-lines/input-not-ready), the line a standing browser-tools modal will print.

@history_item n=17 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (an empty typed text is refused `empty-text`) — `empty-text` added to the closed `not-delivered` details and the refusal order before `human-typing` (contract/send-refusal/order, contract/refusal-reason/not-delivered); the order the gate's modal refusal sits in.

@history_item n=18 marker=2026-10-05-permission-end-to-end
**2026-10-05-permission-end-to-end** (the question first raised by PermissionRequest stays null) — a dependence with no ledger row is not built; adding one costs a row, its probe and a re-stamp of every version (decision/permission-request-question-stays-null). The cost this chunk pays for its row.

@history_item n=19 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** (verify's Ctrl-C / TERM stop) — a stopped run kills its child with no key, removes only what it created, writes no stamp, exit 1 with `interrupted`. A probe run that meets the browser-tools modal must end the same no-key way (decision/verify-types-nothing-into-dialogs, decision/verify-removes-only-own-dirs).

@history_item n=20 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** (eighteen ledger rows, `paste-newline`) — precedent for adding a row: members of `LedgerRow::ALL`, `[NN/MM]` with `MM` the count, older all-pass stamps read unverified so every home reads `cli_verified:false` until re-verify, the 2.1.287 fixture set re-stamped. The 23→24 step's shape.

@history_item n=21 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (three statusline ledger rows and the per-OS relied set) — introduced `LedgerRow::relied()`; a version is verified only when every relied row reads pass; `MM` is the relied count (decision/version-verified-all-relied-rows, contract/viola-verify-output/step-line).

@history_item n=22 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (the resume row, Run E) — a widening of verify's child set (Run E) ratified by the founder's own answer, with its own guarded dir, `--record` variant and older stamps reading unverified. If the browser-tools probe needs a new run or start, it is the same kind of widening (decision/verify-child-set, decision/verify-run-e-resumes-run-b-first-session).

@history_item n=23 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (CLI Version Compatibility: the agents-join row and Run B's agents read) — 23rd row added inside an existing run with no live start added; the one-read narrowing; every home stamped before reads unverified until re-verify. Precedent for measuring a row inside the existing runs.

@history_item n=24 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (the twenty-fourth ledger row, local-command-compact) — the last prior «24th row» step: step lines `[NN/24]`, fixture set count and stamp moved, the fake agent's `--framing` replay extended, a stamp of the pre-chunk ids reading unverified.

@history_item n=25 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the second input-box literal and the three-screen input-box-signature row) — the one precedent for adding a compiled screen literal for a newer CLI: `(shift+tab to cycle)` measured live by one scratch footer reading (a crossing answered by the founder), the row's screens widened, 2.1.302 stamped 24/0; without it `run`'s gate would refuse `input-not-ready` on a verified CLI.

@history_item n=26 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set and the long-turn screen switch) — the fake agent's `--long-turn-screen <phase>` draws a recorded `Screen.<phase>` to reproduce a live stop without a live start; `contract_ledger_probes` stamps recorded sets (`STAMPED`). The model for teaching the fake agent the browser-tools screen.

@history_item n=27 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the split modules) — verify's typed runs live in `src/cmd/verify/typed/` (`runs.rs` Runs A–E, `turns.rs`, `pty.rs` with `settled` and `box_wait`, `run.rs` with `ended_by`); where the new probe reading goes.

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check before the start's and verify's log) — `viola verify` runs `check_home_tree` before its `cli` log; a refusal exits 1 with the home pair, no step line or summary (decision/verify-refuses-untrusted-home-or-stamps, contract/verify-refusals/home-tree).

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the stamps writer's strict-modes check) — `update_stamps` checks strict modes before creating `ledger/` and again under its lock; a refusal reads and writes nothing (decision/stamps-writer-checks-before-and-under-lock, decision/stamps-single-writer).

@history_item n=30 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes on its `process-exit`) — verify's closed refusal codes (`program-not-found`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, `interrupted`…) and that a failing row is no refusal and carries no code (contract/viola-verify-refusal-codes/*); a browser-tools-prompt failure is a row outcome, not a new code.

@history_item n=31 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set: all twenty-four rows on every build) — `LedgerRow::relied()` is every row of `ALL` on every build, `MM` equal to the ledger count; the new row is relied with no per-OS cut.

@history_item n=32 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Established Decisions: 2 row(s)) — set decision/screen-signatures-compiled's readiness rule (no modal literal, and a footer literal or the box's own rule/prompt rows) and decision/verify-box-wait-for-literal (verify's wait ends on the input box or a modal literal); the one shared predicate the browser-tools literal enters, modal read first. Card confirm PROVISIONAL.

@history_item n=33 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Standard Contracts / Occupied Resources) — `measured.typed_probe` gained `box_only_guards` (contract/ledger-stamps/measured: additions need no `v` bump); recorded screen fixtures keep footer or modal literal rows whole and box rows as their chars (decision/screen-fixtures-signature-rows-only). A recorded browser-tools modal row is kept whole under this rule.

@history_item n=34 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Cross-cutting Patterns: 1 row(s)) — decision/mixed-version-tolerance: a stamp holding a retired ledger row verifies on the relied rows alone; the converse (a 23-row stamp lacking a new relied row reads unverified) is the 23→24 reading P3 must state for a home stamped at 23.

@history_item n=35 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Established Decisions: 7 row(s) (1 of 2)) — the relied rows went 24 → 23 with `statusline-rate-limits` removed (the count in decision/permission-request-question-stays-null moved); history of the row count this chunk raises again.

@history_item n=36 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Conventions: 2 row(s)) — decision/protocol-version-bump-rule's except: removing the stamps' two status-line row ids kept every own-format `v` at 1 (ratified by the founder at 2026-11-15T18-01-44-wrap); adding a row is additive with no bump.

@history_item n=37 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Occupied Resources: 10 row(s) (1 of 2)) — decision/claude-flag-literals-in-agent-crate: the agent crate holds the program name and the `--settings` flag verify reads; Claude literals stay in that crate. Also rows 23 → 21 (Established Decisions entry), a further step of the count.

@history_item n=38 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Cross-cutting Patterns / Design Philosophy / Established Decisions) — StopFailure registered with no ledger row and no verify probe, the named exception to decision/capability-ledger, decision/cli-capability-ledger and decision/capability-ledger-gate; ratified at 2026-11-16T06-05-33-wrap. The only exception to the rule this chunk follows by adding a row.

@history_item n=39 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Established Decisions: 2 row(s) (2 of 2)) — decision/local-command-echo-probe: a relied row probed by Run B's seventh paste, failing when its input is unreadable; a widening of verify's pastes. Also decision/unconfirmed-delivery-not-delivered's except (Cross-cutting entry). Run B's paste chain is the place a standing modal blocks.

@history_item n=40 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Established Decisions / Stack and Technologies) — decision/vt100-readiness-gate gained the except that the test-only live rig uses vt100 to read its child's trust dialog, selector and input box; decision/vt100-screen-model names the rig's screen. The rig is where the live arm with the prompt standing runs.

@history_item n=41 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Established Decisions: 1 row(s)) — decision/claude-first-no-adapter-trait gained a test-only except for the rig reading its captured payload's key names; Claude shapes outside the agent crate are named exceptions only.

@history_item n=42 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions: 4 row(s)) — verify's child set became six interactive runs (Run F, a ruled widening); added decision/trust-record-ledger-row (23rd relied row) and decision/verify-own-trust-record; card confirm PROVISIONAL. The immediate precedent for a new relied row with its own probe and for widening verify's runs.

@history_item n=43 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Occupied Resources: 4 row(s) (1 of 2)) — the six runs execute A to F (decision/verify-run-e-resumes-run-b-first-session), verify removes its own trust record on every exit path (decision/verify-removes-only-own-dirs), and verify reads a kept `CLAUDE_CONFIG_DIR`. Bounds any new probe run's place and cleanup.

@history_item n=44 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Standard Contracts: 9 row(s) (1 of 2)) — contract/ledger-stamps/rows and /content: trust-record may hold a writer word instead of pass/fail; the trust-writer code words registered. The stamp shape the 24th row joins.

@history_item n=45 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions: 1 row(s), the trust-record rule) — the row reworded to ask per version whether a record opens the folder with no modal, Run F passing only on «no modal and the input box»; the first live reading on 2.1.325 read fail. A browser-tools modal at Run F's start would fail this row too.

@history_item n=46 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions: 1 row(s), permission-request) — decision/permission-request-question-stays-null now says «a further relied row», no count, so the 23→24 step leaves it standing.

@history_item n=47 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Established Decisions: 2 row(s)) — the hidden rig-only `--record-first-screens` writes a failed verify's whole-row first screens (decision/verify-record-only-clean except); the rig seeds its private Claude config (decision/claude-first-no-adapter-trait except) — the seed carrying `claudeInChromeDefaultEnabled: false` that hid the prompt in S0B. Card confirm PROVISIONAL.

@history_item n=48 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Occupied Resources / Stack and Technologies) — decision/recorded-fixtures-scrubbed and decision/screen-fixtures-signature-rows-only gained the first-screen except; contract/viola-cli-arguments/verify registers the hidden flag; decision/run-never-reads-screen-content names the rig-only record. The instrument that captured the `browser-tools-prompt` screens of runs B, C, D, F.

@history_item n=49 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Standard Contracts: 2 row(s)) — added contract/ledger-stamps/first-screen (closed words trust, imports, box, none, exited per run, no text) and /first-settle-ms. A host with the prompt reads `none` here; whether the new modal adds a word to this closed set is P3's to say.

@history_item n=50 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Established Decisions: 2 row(s)) — decision/run-start-order gained the trust-record step on a verified CLI after the version gate; decision/verify-own-trust-record no longer says run and revive write none. Card confirm PROVISIONAL. The prior chunk; a start's screen after that step is where the browser-tools prompt next stands for `run`.
