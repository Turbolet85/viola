@history_item n=1 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the second input-box literal and the three-screen input-box-signature row) — `SIGNATURES.input_box` became `["for agents", "(shift+tab to cycle)"]` because 2.1.302 hides `for agents` after its attacher-caps timer; the `input-box-signature` row now reads the ready, turn and post-long-paste screens; Run B pastes only into rows holding *an* input-box literal. This is the footer-literal keying this chunk replaces (both literals are footer hints, V178); the literal crossing was the founder's own Fork 2 answer, so changing what the row/literal reads is a ledger-row change with a verify probe and a re-stamp.

@history_item n=2 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set and the long-turn screen switch) — added `fixtures/claude/2.1.302/` (29 files then; `Screen.ready`/`Screen.turn` keep one row holding both literals) and the fake agent's `--long-turn-screen <phase>` (13 options; the count's rule is the list). The fixtures/fake-agent screens that must replay a transient footer notice extend this set and option register; `contract_ledger_probes`' `STAMPED` list stamps it.

@history_item n=3 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the split modules) — `src/cmd/verify/typed.rs` split into `typed/runs.rs`, `turns.rs` (`poll_until`, `wait_for`, `turn`, `agents_read`, `framing_turns`, `probe_until`, `wait_capture`), `pty.rs` (`settled`, `box_wait`), `run.rs` (`Run`, `ctrl_c_ending`, `ended_by`). This is the exact module map of the chunk's in-boundary code (`input_box_up`, `framing_turns`, `turn`'s settle, `Run`'s `box_wait`); `relied_here` folded into `relied()`.

@history_item n=4 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (screen signatures compiled, ten ledger rows, verify's interactive runs) — compiled `SIGNATURES` (`input_box: ["for agents"]`, the two trust modals), added `modal-signature` · `input-box-signature` · `quiet-period` · `confirm-window` rows, Run A/Run B, no key into any CLI dialog, `Screen::rows()` verify-only, full gate only when `cli_verified`. Origin of the literal-based readiness and of the rows `screen-signatures-compiled`, `screen-rows-read-only-by-verify`, `verify-never-types-into-dialog`; the probe is a founder-ratified widening (R-S2), so new probe screens/runs need his word.

@history_item n=5 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (registry rows for verify's interactive runs and screen fixtures) — `schemas/claude-screen.v1.json`, signature-rows-only `Screen.<phase>.json` fixtures, the fake agent's `--screens`/`--turn-stop`/`--trusted-root` (argv, no env), `--record` refusal naming file + closed code. Constrains any new recorded footer-notice screen: only signature rows kept (`screen-fixtures-signature-rows-only`), no env seam on the fake agent.

@history_item n=6 marker=2026-10-05-t00-09-13-wrap
**2026-10-05-t00-09-13-wrap** — the typed-input `viola verify` probe against the live installed `claude` ratified (local only, no Claude credential on CI); signature/quiet-period/max-wait rows ride it. Precedent for the chunk's live-start constraint: real-CLI probes run only locally (`real-cli-probe-local-only`).

@history_item n=7 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (the readiness gate as landed) — the pure `viola_agent_claude::screen::Screen` fed on `run`'s feed thread, caught panic poisons until resize, `Signatures { input_box, modals }` two compiled literal lists with row-contains matching (PROVISIONAL format then). The format the chunk may move off; the run gate shares `SIGNATURES`, so the scope's "Out unless it shares the same literal" test lands here.

@history_item n=8 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (vt100 in viola-agent-claude) — vt100 `=0.16.2` enters only through the pure `screen` module of `viola-agent-claude`; `vt100_feed` fuzz target. Any new input-box reading stays in that pure crate (`claude-shapes-isolated`, `agent-claude-pure`).

@history_item n=9 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (seventeen rows, Run B at four pastes) — Run B's added pastes go only into rows holding the input-box literal and no modal; after the long and tag-like turns the rows come from a wait bounded by `PROBE_DEADLINE`, not `GATE_MAX_WAIT`; measured: 2.1.287's footer holds no input-box literal for 8.0 s after a long paste. Kept: `viola run`'s gate unchanged although it sees the same absence. The origin of the per-paste guard (`input_box_up`) and of `verify-box-wait-for-literal`; the founder ratified the 120 s wait with the guard kept.

@history_item n=10 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (registry rows for the framing variants) — fake agent's `--framing` and test-only `--paste-hint-ms`; `--record` writes Run B's framing captures; CI runs verify against `--screens --turn-stop --trusted-root --dialogs --framing`. The fake-agent replay path a footer-notice fixture must slot into.

@history_item n=11 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (the paste-hint reading, the fake agent's eighth option) — `--tag-turn-screen <phase>` draws `Screen.<phase>` after the tag-like turn only (the control for the guard before the local-command paste); a quiet screen with no literal under the hint read `input-not-ready`. Precedent for a turn-specific screen switch to replay a displaced footer.

@history_item n=12 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (send consumes the local-command list) — `LOCAL_COMMANDS` decided by exact equality, `/clear` confirmed by a new session id, unconfirmable on unverified CLIs. Background for rows `local-commands-by-ledger`, `local-command-post-conditions`, `clear-confirmed-by-new-session` that the verify proof stamps.

@history_item n=13 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the paste hint measured on the live CLI) — the hint is an 8.0 s timer from the last long paste; on a verified CLI a send under it ends `input-not-ready`; founder decided the gate waits for the input box, bound 8.5 s. Shows that a literal-less quiet screen after a turn is a known, measured CLI state the gate must wait out, not a refusal.

@history_item n=14 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the long-paste frame beside typed text) — the founder's ruled limit: a shape no `send` relies on needs no probe (`no-probe-for-unrelied-shapes`). Decides whether a footer-notice reading needs its own ledger row: only if a send/verify verdict relies on it.

@history_item n=15 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `GATE_MAX_WAIT` 8.5 s; on a verified CLI a quiet screen with no input-box row waits for one until the bound; "verify's `box_wait` and the run gate are two waits". Trap stated: any case reading a verified literal-less screen through a clock that never advances waits forever. Directly bears on rows `send-waits-out-paste-hint`, `gate-timings-compiled`, `contract/send-results/input-not-ready`, and on whether the run gate shares the footer failure.

@history_item n=16 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (the live readings on 2.1.287) — a `send` under the real paste hint confirmed (4 248 ms, `duration_ms` 4 081 vs 8 500); the `input-not-ready` hint line's founder wording landed (`contract/human-hint-lines/input-not-ready`). Baseline for the run gate behaviour the chunk must not silently widen.

@history_item n=17 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — eighteen rows; Run B five pastes, `PROBE_NEWLINE_PASTE` under the same per-paste guards, `box_wait` after each added text turn; `[NN/MM]` step lines with `MM` the relied count; older stamps read unverified. The guard/`box_wait` call sites this chunk rewrites; the newline paste is a founder-ratified widening of the verify child set.

@history_item n=18 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (three statusline ledger rows and the per-OS relied set) — `statusline-override` · `statusline-shell` · `statusline-rate-limits`; `rate-limits` checks integer `used_percentage`/`resets_at` on both windows. The row the chunk's proof exempts ("every row pass but the account-dependent `statusline-rate-limits`").

@history_item n=19 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (Run B's probe --settings) — Run B gets `--settings <probe dir>/typed/statusline.json` so its status line is the probe capture; fake agent `--framing` replays StatusLine variants. Run B's footer/status area is probe-controlled; relevant when reading which footer text a transient notice displaces.

@history_item n=20 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (the resume row, Run E) — Run E resumes Run B's first SessionStart id and runs last; a verify is six `claude` starts. Constrains the live proof's start budget and ordering (`verify-run-e-resumes-run-b-first-session`, `verify-child-set`).

@history_item n=21 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the agents-join row and Run B's agents read) — once Run B's first turn settles, before the framing pastes, verify makes one `claude agents --json` read (`typed::agents_read`); founder-ratified widening. A step inside Run B's sequence that sits between the settle and the first guarded paste.

@history_item n=22 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (the twenty-fourth ledger row, Run B's sixth paste) — `local-command-compact` row; `PROBE_COMPACT_COMMAND` pasted only into the settled input box, its answer waited up to `PROBE_DEADLINE`, then `box_wait` before `/clear`; `FRAMING_TURNS` `paste-1` · `paste-2` · `paste-3` · `compact-1` · `clear-1`; 2.1.287 stamped `24 pass 0 fail`. Another `box_wait`/guard site of the chunk; the sixth paste is the founder's own answer.

@history_item n=23 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (/compact confirmed by its same-session SessionStart, its own 40 s window) — `COMPACT_CONFIRM_WINDOW` 40 s PROVISIONAL, validated by the row against `compact_latency_ms`. Background for rows `compact-confirm-window`, `compact-confirmed-by-same-session`, `confirm-window-compiled`.

@history_item n=24 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set: all twenty-four rows on every build) — `LedgerRow::relied()` is all of `ALL`; step lines `[NN/24]`; an all-pass stamp of an older id set reads unverified. The proof's "every row pass" count and the stamp verdict rule (`version-verified-all-relied-rows`).

@history_item n=25 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits row reads the account's plan state) — on an account with `five_hour` `unknown`, verify fails `statusline-rate-limits` on an unchanged CLI (`23 pass 1 fail`); what the budget gate stands on is OPEN (F-06i, owner 0.2.0's intent). The reason the chunk's acceptance carves that row out; do not treat its fail as a readiness regression.

@history_item n=26 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — the partial gate on unverified builds (poisoned or not quiet refuses, quiet is ready with no row read); a dropped feed copy poisons like a panic. Origin of rows `partial-gate-on-unverified-cli` and `poisoned-screen-model`, which any change to the input-box reading must keep.

@history_item n=27 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** ([Screen Model] the poisoning episode's parse-rejected detail) — one `parse-rejected{parser:"vt100-feed"}` per poisoning episode, `detail` `panicked` or `oversize`. Keeps the poisoned-screen contract exact if the screen module moves.

@history_item n=28 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (decisions on the six-row stamp, the strict stamps read) — `run`'s version gate reads `ledger/stamps.json` through `read_stamps_strict`; a refusal is `cli_verified:false` with `parse-rejected{ledger-stamps, strict-modes-failed}`. Origin of `version-gate-strict-stamps-read`; the stamps the proof writes are read this way.

@history_item n=29 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (StampError folded into AgentError) — a non-`v:1` or wrong-shaped stamps envelope is `AgentError::StampsMalformed`, `run` recording `cli_verified:false` with one `parse-rejected`. Origin of `malformed-stamps-unverified`.

@history_item n=30 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the six landed ledger rows, `viola verify`'s probe, the stamps envelope) — the closed ledger, a failing row still stamps and exits 1, `--record` writes only at 0 fail, `--version` first line `X.Y.Z (Claude Code)`, stamps envelope `{v, written_at, writer:"verify", data}` merged per version. Origin of the verify output/exit contracts and `stamps-merge-own-version-only` the proof relies on.

@history_item n=31 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check before the start's and verify's log) — `viola verify` runs `check_home_tree` (no name, no key) before its `cli` log; refused → home pair, exit 1, no step line. Rows `verify-refuses-untrusted-home-or-stamps`, `strict-check-before-log-open`.

@history_item n=32 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the stamps writer's strict-modes check) — `update_stamps` runs `check_stamps` before creating `ledger/` and again under its lock; refusal reads and writes nothing. Rows `stamps-writer-checks-before-and-under-lock`, `stamps-single-writer`.

@history_item n=33 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes on its `process-exit`) — verify's `process-exit` carries its `Ended::Refused` code beside `interrupted`; a failing row is no refusal; verify takes no `--json`. Rows `contract/viola-verify-refusal-codes/*`, `contract/verify/no-json`.

@history_item n=34 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the `.cmd`/`.bat` refusal `batch-script-child` retired) — verify's refusal codes 6 → 5 (`program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`), pinned in `tests/cross_exit_causes.rs`. The current closed set of those contract rows.

@history_item n=35 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's Ctrl-C/TERM stop: one flag every wait reads, a stopped run kills its child with no key and starts no later run; probe dirs removed only by their own drop guards. Any reworked wait in `turns.rs`/`pty.rs` must keep reading the stop flag (`verify-probe-dirs-drop-guarded`, `verify-removes-only-own-dirs`).

@history_item n=36 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — Runs C and D, dialogs answered only by the probe hook, plan files in a 0700 `plans/` inside the run dir; verify never types into a dialog. Rows `verify-plan-files-in-run-dir`, `verify-types-nothing-into-dialogs`; the modal-literal half of `input_box_up` guards this.

@history_item n=37 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the statusline wrap and its probe on every build) — Run B's probe `--settings` passed for shell-plain paths on every build; its status line is the probe capture. Determines what Run B's footer shows besides the CLI's own notices.

@history_item n=38 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution) — the `shim-resolution` row now reads `run.version_answered` alone. One of the 24 rows the proof must read pass.

@history_item n=39 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the Tokio client and viola-channel's tree-form ban) — rmcp pinned `>=3.4.1, <3.5` (was `~3.4`); `viola-channel`'s tree-form tokio check and the `channel-tokio-edge` deny probe; `rmcp-features.sh` in lint. The ratified range a bump past 3.5 must reconcile (the scope's supply-chain choice); a bump must keep the tokio containment checks green.

@history_item n=40 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tools' contract as landed) — release graph rmcp exactly `server` + `transport-io` (`scripts/rmcp-features.sh`); `viola-e2e` takes rmcp `client` alone; `tools/list` needs `ttlMs` 0 and `cacheScope` `private`. Rows `rmcp-release-features`, `rmcp-features-exact`, `contract/mcp-list-results/*` that an rmcp upgrade must keep.

@history_item n=41 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** ([MCP] the viola mcp stdin line cap) — `CappedLines` is handed to rmcp as the reader of its `(reader, writer)` stdio transport because rmcp 3.4.1's `AsyncRwTransport::receive` reads unbounded and its codec bounds only the writer (read in rmcp 3.4.1 source). Trap: an rmcp bump to >= 3.9.0 must re-read that transport/codec so the cap still sits in viola's reader and a cut still reads as end of stream.

@history_item n=42 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the recorded live MCP exchange) — `viola-mcp-tap` and `Mcp.tools-list-1.json` replayed by `tests/mcp_recorded_exchange.rs` holding the served `tools/list` to the recorded shape. A guard an rmcp version bump must keep green (`mcp-tools-list-held-to-recorded-exchange`).

@history_item n=43 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — a RustSec advisory closed by moving the exact pin (tokio `=1.53.1` → `=1.53.3`) with no ignore widened, the containment checks re-run green. Direct precedent for the rmcp/rustix advisories: bump rather than `deny.toml` ignore.

@history_item n=44 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (RUSTSEC-2026-0441 red, owned by «Version done-check») — the CARRY's rule: move the crate to a release whose graph drops the advisory, "with no ignore-list widening". Precedent for the choice the scope leaves open.

@history_item n=45 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (RUSTSEC-2026-0441 closed and every CI job green; portable-pty pinned at =0.9.0) — closed by the pin move, `cargo deny check` advisories ok with `ignore = []`, CI 7 jobs green. Precedent: `deny.toml`'s `ignore = []` has been kept empty and the supply-chain job closed by version moves.

@history_item n=46 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (cargo-deny: one Linux triple, one 0BSD exception) — `deny.toml` / `deny-sync.toml` evaluated for `x86_64-unknown-linux-gnu` only; 0BSD exception `doctest-file` alone. Any new dependency an rmcp bump pulls must pass this licence allowlist and triple (`licence-allowlist`, `deny-single-target`).

@history_item n=47 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the serde_json unbounded_depth ban and the deny probe count) — `serde_json/unbounded_depth` banned; `deny-probes: 14/14 banned, control clean`, the count one per `probes=(` row plus `channel-tokio-edge`. An rmcp bump must not pull a banned feature; `deny-probes.sh` stays green (`deny-bans-proven`).

@history_item n=48 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (Infrastructure Patterns) — the feature bans are `veil/toggle`, rmcp `auth` / `transport-streamable-http-server`, `tracing-subscriber/env-filter`. The three rmcp advisories name OAuth and Streamable HTTP, already banned features: the reasoning base for a per-advisory ignore if a bump is not chosen (`banned-features`).

@history_item n=49 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — the four-family `deny.toml` (advisories, licences, sources, bans), the sole-root `deny-sync.toml` tokio ban per `scripts/sync-crates.txt`, the `supply-chain` CI job. The job this chunk turns green and the families a bump must pass (`cargo-deny-dependency-policy`, `sync-crates-tokio-free`).

@history_item n=50 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — `supply-chain` also audits `fuzz/Cargo.lock` (advisories + sources into `deny-fuzz.json`) and nightly advisories cover both lockfiles. The rustix/rmcp advisories may also hit the fuzz lockfile; `fuzz-lockfile-audited` needs it clean too.

@history_item n=51 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — tempfile `=3.27.0` `persist` through one shared helper replaced atomic-write-file (BSD-3-Clause outside `deny.toml`). rustix 1.1.5 arrives through `tempfile =3.27.0` and `viola-state`; the exact pin stays and rustix moves by a lockfile bump (`tempfile-snapshot-writer`, `workspace-dependency-pins`).

@history_item n=52 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** ([Snapshot writer]) — `place_private_new` via tempfile 3.27.0 `TempPath::persist_noclobber` for the pinned copy; tempfile already held both calls. A rustix/tempfile lock move must keep `persist` and `persist_noclobber` behaviour (`atomic-replaced-state-files`, `private-mode-before-write`).

@history_item n=53 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (1062 rows), each verified against the code at 8834195b0683, fates by the founder. The picked rows this chunk binds are these; duplicated rows across areas (e.g. the ledger, rmcp, tokio rows) are one fact stated per area.
