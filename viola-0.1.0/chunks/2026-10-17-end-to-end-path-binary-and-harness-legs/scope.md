# Scope — 2026-10-17-end-to-end-path-binary-and-harness-legs · End-to-end path binary and harness legs

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry at take-up, tenth under
`### Epoch 5 — Driver surface`, directly after "Unwrapped sessions on the board"): End-to-end path binary and harness
legs — first `path_` E2E binary, `run --e2e` selector, harness boot with the statusline echo, two-instance budget leg,
status `list` member — plus six CARRY blocks and one WATCH (folded below; completeness check `route.py pins --run-dir`
→ this run dir's trail: seven rows on the entry, CARRY 692 · 864 · 1436 · 296 · 764 · 431 chars and WATCH 569 chars,
the WATCH row printed `clause UNPARSED`; no `BLOCKED-ON`, no `PREREQ`, no `CONTEXT`).

**The operator's direction at take-up:** none beyond the invocation (`/andromeda-phase`, no flags). No external input
is snapshotted at P1: every fact below is read from the entry, from the masters (`test-plan.md` §3 and §6, its keyed
contracts `5-command-implementation.md` and `status-endpoint-shape.md`), or from a file of this repository. The
earlier chunks' `inputs#I…` citations inside the CARRY text are those chunks' own snapshots, cited as they stand.

**CI read at Setup 5a** (the last wrap's flip `6042af6` = HEAD, `ci.py conclusion`, this run dir's trail):
- `6042af635c3e`: **green**, checks 15/15, wall 547 s, `ci#38983110543` push completed/success. No red or `not green`
  to disposition.

**Premise closure (P3):** every `[inferred]` bullet below was read against `research.md`. A verified bullet has lost
its tag; a falsified one carries `[premise-corrected: …]`.

## What this chunk builds

The harness (`crates/viola-e2e`, `viola-harness`, `scripts/agent-run.*`) and the E2E layer as test-plan §3 and §6
spell them, where the product surfaces they drive already exist. Each bullet below carries what the entry or its
freight states; a bullet the entry did not state is tagged `[inferred]` for P3's premise closure.

### 1. The first `path_` E2E binary (entry; CARRY 1)
- The first E2E-prefixed `path_` test binary (matched by the E2E regex `^(path|tui|mcp|cross|chaos|contract)_`) lands
  for test-plan §6 Path 1, the `run` start sequence. At P4 (fork 1, `inputs#I1`, the overseer as the founder's
  delegate, provisional) it became a ROOT `tests/path1_run_start.rs` running steps 1–6 as one ordered flow over one
  home on the existing root chain, not a `viola-e2e` binary over a fixture copy. `crates/viola-e2e`'s first `path_`
  binary is Path 6's harness-session leg (§4). (Validation-1, intent-incomplete: the scope's "`crates/viola-e2e`
  lands" read the CARRY's wording before the fork.)
- CARRY 1, as it stood: chunk 2026-09-27-instance-state-and-start-order covers Path 1's start steps with
  `cli_instance_state`'s `path1_start_writes_state_before_the_spawn` only; this entry, where Path 1's later steps exist,
  lands the `viola-e2e` `path_` E2E binary for Path 1 (proposals T19/T20 rejected to this CARRY at that wrap).
- Path 1's Cleanup is "TempDir drop" with a test-owned outer PTY, so its fixture is the
  `home → fake_agent_path → stamped_home → booted_wrapper` chain from `viola_e2e::fixtures`, whose copy "lands with its
  first consumer among these scenarios" (5-command `run` step 2); only the sync root copy (`tests/support/home.rs`)
  exists today. Verified: `crates/viola-e2e/src/lib.rs` holds `pub mod harness;` alone, and `viola-e2e` carries no
  `[dev-dependencies]` (no rstest).
- Every Path 1 step is already held at the root tier, as separate cases over separate homes (research §Patterns
  detected: steps 1–6 → `path1_start_writes_state_before_the_spawn`,
  `path1_session_start_is_record_three_through_the_absolute_hook`, `run_refuses_a_live_name`,
  `run_refuses_a_stale_name`, `run_refuses_a_squatted_endpoint`, `run_refuses_a_tampered_pinned_copy` (its tail is
  step 6), `run_rewrites_the_plugin_folder_each_start`, `run_rewrites_the_settings_override_each_start`; step 2's
  `list --json` item in `tests/cli_list.rs`). What no case holds is the six steps as one ordered flow over one home.
  Where the binary lives and what it carries is P4 fork 1 (research §Open questions).
  `[premise-corrected: verified at the root tier step by step; the "carried in the E2E binary too" lean is a fork, not a decision]`
- The squat step reaches everything it needs from `viola-e2e`'s present dependencies: `viola_channel::endpoint_name`
  (`crates/viola-channel/src/endpoint.rs:31`) and `viola_channel::Server::bind` (already used at
  `crates/viola-e2e/src/harness/cleanup.rs:390`). No new dependency.

### 2. `run --e2e` and the integration exclusion (entry; CARRY 2)
- CARRY 2, as it stood: the harness has no `run --e2e` selector (clap exit 2 `unexpected argument '--e2e'`, the same on
  a clean worktree at `90aba7c`; chunk 2026-09-29-h2-conpty-resize-probe's `evidence/entry-6-e2e-plan-defect.md`)
  though test-plan §3's `run` command body lists it — owner: this entry, which lands the first `viola-e2e` `path_` E2E
  binary and with it builds `run --e2e` and the integration filterset's `!binary(…)` exclusion (test-plan §3;
  `crates/viola-e2e/src/harness/run/nextest.rs`); until then no plan lists `run --e2e` as a gate.
- So: `run --e2e` runs 5-command `run` step 2 (`cargo nextest run --workspace --features viola/fake-agent --profile ci
  -E 'binary(/^(path|tui|mcp|cross|chaos|contract)_/)'`) as suite `nextest-e2e`; `--all` and the default selection run
  it after step 1; the integration layer becomes `kind(test) & !binary(/^(path|tui|mcp|cross|chaos|contract)_/)`.
- The `binary()` hazard stands at the CI pin: on cargo-nextest 0.9.146 a regex matching no binary still exits 94
  `failed to parse filterset` (control `binary(/^nomatch_zz_/)`, research §Patterns detected). But the E2E regex
  already matches 18 root binaries (`chaos_*` ×5, `contract_*` ×7, `cross_list_rows`, `tui_*` ×5; 155 tests), so the
  exclusion and `--e2e` are buildable now, whatever the new binary: `kind(test) & !binary(…)` reads 27 binaries / 336
  tests, `kind(test)` 45 / 491, an exact partition with no overlap. The re-cut moves those 18 binaries out of
  `nextest-integration` into `nextest-e2e`, the split test-plan §3's regex states.
  `[premise-corrected: 18 root binaries already match the E2E regex (measured on 0.9.146); "no root file matches" and "the exclusion waits for the first binary" are false]`
- CI needs no edit for the E2E step: the test job runs `run --coverage`, whose instrumented run passes no `-E`, so
  every binary (the 18 included) already runs there under `gate --require coverage,doctest`. `gate::SUITES` already
  holds `nextest-e2e` (`crates/viola-e2e/src/harness/gate.rs:14`). The new suite is read locally (`run --e2e`,
  `run --all`, the default selection).
  `[premise-corrected: CI runs run --coverage, not run --all; it picks up the new binaries through coverage, and names no layer]`

### 3. `boot --statusline-echo` (entry; CARRY 3)
- CARRY 3, as it stood: `agent-run boot --statusline-echo` is not built, and neither is the harness-session form of
  test-plan §6 Path 6; both are owed here, with step 7's two-instance leg (a `release --budget` on one instance leaves
  the other paused; the unit case `gate_release_on_one_instance_leaves_another_paused` holds the rule, no booted pair
  does). A `statusline-source-unresolved` refusal is dropped, its premise gone. Path 6's cli and ipc signals landed as
  root cases of `tests/cli_budget.rs`; owed here with the board: `list --json`'s `budget` and each item's
  `budget_paused`, and the board's cell. The gate never polls, so a snapshot can read `budget_paused: true` after the
  reset until a `send` arrives or a turn ends (architecture [Budget Governor]); the board shows what the snapshot
  holds. The fake agent's `statusline-echo` mode and the root helpers `plant_statusline_source`, `statusline_marker`
  and `statusline_echo_command` (`tests/support/home.rs`) exist, and so does `tests/support/statusline.rs`. test-plan
  §6 Path 6 step 1 and the `boot` key name this entry.
- So (5-command `boot`): `--statusline-echo` runs before boot step 5 and writes the session home's
  `<home>/statusline-source.json` (the Claude settings shape, viola only reads it) naming
  `<bin dir>/viola-fake-agent[.exe] statusline-echo`, appending to `<home>/fake/statusline.marker`. With no
  `--instance` flag the instances stay `overseer` and `builder`.
- On Windows `run` writes no `settings.json` override and the statusline arm runs no command, so the flag writes the
  source on every OS but its marker effect is Unix's; the harness-session Path 6 leg asserts the marker only under
  `cfg(unix)` (verified: test-plan §6 Path 6 verification signal; arch and obs extracts; `override_document` and
  `shell_argv` return `None` on Windows).
- The source can only be planted once the home exists, and `boot` creates no home itself: `viola verify` creates it
  at step 4 (`crates/viola-e2e/src/harness/boot.rs:134-138`). The plant goes after the stamp and before the
  `supervise.json` write, and under `--unstamped` there is no viola-made home to plant into (research §Files
  inspected, `boot.rs`).

### 4. Path 6 in harness-session form and its two-instance leg (entry; CARRY 3)
- A `crates/viola-e2e` `path_` scenario for Path 6 (Cleanup "harness `cleanup`") over the rstest `harness_session`
  fixture booted with `--statusline-echo`: step 1's harness boot, the statusline feed through `viola hook statusline`,
  and step 7's two-instance leg — re-feed `five_hour` 91, `viola release builder --budget --json` only, read
  `viola list --json`, `send` to both; signals: `.ok.budget.paused` stays `true`, `builder` item `budget_paused:false`
  and `overseer` `true`, `builder`'s `snapshot.json` `budget_override_until`, one `budget-gate{paused:false,
  override_until}` record for `builder` only, `send` to `builder` exit 0 and to `overseer` exit 11 `five-hour`,
  `override_until` later than that record's `ts` and equal to the snapshot's.
- No `harness_session` fixture exists (`harness_session` over `crates/ tests/ src/` hits two test names only), and
  `viola-e2e` has no `[dev-dependencies]`, so it cannot be an rstest `#[fixture]`: it is a plain function and guard
  (testing.md §Naming). Whether its supervisor is the spawned `supervise` process the library `boot` starts today
  (`harness_lifecycle.rs`'s form) or test-plan §3's fixture-owned thread is P4 fork 2.
  `[premise-corrected: not an rstest fixture — viola-e2e has no dev-dependencies]`
- Steps 2–6 and 8 of Path 6 already hold as root cases (`tests/cli_budget.rs`: `path6_a_reading_over_the_threshold_refuses_the_send`,
  `path6_the_boundaries_and_the_window_word`, `path6_release_budget_lifts_the_pause`,
  `path6_the_refusal_hint_never_names_release`, `path6_list_shows_the_reading_and_the_pause`; `tests/hook_statusline.rs`
  for step 6). The harness-session form re-asserts what the harness path adds: the boot flag's planted source and
  marker, and step 7's two-instance leg (verified; tests-history 2026-10-10-session-links supports the lean).
- `viola release` and `viola send` read `VIOLA_NAME` as `from`, and this builder's own shell carries 3 `VIOLA_*`
  variables (research §Patterns detected). So the leg's verbs run with `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`
  removed, as the root helpers do (`tests/support/cli.rs:113`, `tests/support/statusline.rs:56-58`); a `release`
  carrying a `from` exits 20 and lifts nothing (security extract). (New, read at P3.)

### 5. `status` runs `viola list` (entry; CARRY 4)
- CARRY 4, as it stood: the harness `status` document's `list` member stays `null` while `status` runs no
  `viola list` (test-plan §3 → Status endpoint shape, amended at that wrap); `viola list` itself is built.
- So (5-command `status`; Status endpoint shape): `status` runs `viola list --json --home <home>` with the harness
  `PATH` (the fake agent answers `claude agents --json`), nests the verbatim document as `list`, and derives `state`
  from it: `ready` when every booted instance has `liveness:"live"`, `degraded` when any item is `stale`, `down` when
  `list` exits non-zero (`last_error` a harness code, e.g. `list-exit-21`); `ok` true only for `ready`; exit 0 / 1 / 2
  as the key states. The interim paragraph (`list`, `pid`, `uptime_ms` null; `state` from wrapper liveness) retires.
- `pid` and `uptime_ms` stay `null` (verified: the shape's example and `.claude/rules/verification-harness.md` §status,
  "`pid` and `uptime_ms` are `null`").
- `status` reads the bin dir and the session `bin/` from `supervise.json` (`SuperviseSpec.bin_dir`, `.session_bin`;
  `session.json` records neither) to run `<bin dir>/viola` on the session `PATH`, with `VIOLA_*` removed so the read
  files no role line under the builder's own bridge name. (New, read at P3.)
- A booted instance whose wrapper has exited is absent from `list --json` (a `gone` item is omitted), and the shape's
  three rules leave that case undecided; `harness_session_with_a_dead_wrapper_reads_degraded` reads it `degraded`
  today. `ready` needs every booted instance `live`, so an absent one is not `ready` and reads `degraded` (a decisive
  lean on "`state` is `ready` when every booted instance has `liveness:"live"`", stated in the plan). (New, read at P3.)

### 6. `boot --agents-mode` (CARRY 6)
- CARRY 6, as it stood: `boot --agents-mode recorded|oversize|malformed` (test-plan §3, not built) lands here, with
  `status` running `viola list`; under the founder's own answer to chunk 2026-10-16-unwrapped-sessions-on-the-board's
  P4 fork 2 (`viola-0.1.0/chunks/2026-10-16-unwrapped-sessions-on-the-board/inputs/I1-relay-1.md.txt`) the flag
  writes the fake agent's `agents-mode` file beside the session's `bin/claude` copy, never an env var. (Origin: that
  chunk's P5 review card's row, the operator's own yes,
  `viola-0.1.0/chunks/2026-10-16-unwrapped-sessions-on-the-board/inputs/I2-relay-2.md.txt`.)
- So (5-command `boot`): default `recorded`; the file sits beside `target/agent-run/<session>/bin/claude[.exe]`; the
  value is recorded in `session.json` and `supervise.json`, so a spec can reproduce the E1 unwrapped row.
- The fake agent already reads `agents-mode` beside its exe: `recorded` · `oversize` · `malformed`, absent reads
  `recorded` (verified: `src/bin/viola-fake-agent.rs:55`, `:812`). A failed, oversize or malformed agents read leaves
  `list` at exit 0 with the board as read, so `--agents-mode oversize|malformed` leaves `status` `ready` (security
  extract).

### 7. The cleanup case over a dangling link (CARRY 5)
- CARRY 5, as it stood: the `viola-e2e` unit case `harness::cleanup::tests::remove_session_home_removes_a_boot_created_parent`
  failed `AlreadyExists` once after a host reboot, because it calls `fs::create_dir_all` on the link `target/e2e-home`
  while the link's tmpfs target is gone and std's `create_dir_all` over a dangling link returns `AlreadyExists` (as
  measured two-sided in a scratch program at chunk 2026-10-11-the-board-viola-list; its `report.md`, Decisions &
  corrections); a parallel case's `ensure_e2e_home` re-made the target mid-run and every later firing read green. The
  case, or the harness's home-base preparation it relies on, owes a form that survives a dangling link.
- The fix routes the case through `Workspace::ensure_e2e_home` instead of its bare `fs::create_dir_all(ws.e2e_home())`
  (verified at `crates/viola-e2e/src/harness/cleanup.rs:362`). The keeper's own case
  `e2e_home_backing_gone_target_is_made_again_owner_only` (`crates/viola-e2e/src/harness/mod.rs:394`) already proves it
  re-makes a gone link target. The fix's witness is a red-before reading of the case over a dangling link in a
  scratch workspace.

### Observation
- watch: the `viola::cli_send` case `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` red once (exit 0
  where 10 is asserted: the second send delivered where `human-typing` is asserted), green in 3 other local firings,
  in 20 two-sided reruns on that tree and on its parent, and on all three runners of ci#38971772946; cause not known
  (0/3; since chunk 2026-10-16-unwrapped-sessions-on-the-board; retires: a recurrence, or 3 green runs; placed on the
  entry that next touches the send / gate path by the operator's word at that chunk's wrap invocation). This chunk
  touches the send / gate path only as a caller (Path 6's `send` to both instances), not its code: verified, because
  research's two lists name no `src/` product file.

## Boundaries
- No product behaviour changes: the product verbs these legs drive (`run`, `list`, `hook statusline`, `release
  --budget`, `send`) exist, and research's lists name no product source; a defect a new leg finds is a red folded
  into this chunk. (Verified.)
- Not here: the MCP forms of any step (owed to "MCP server for drivers"); Paths 2–5, 7 and E1–E5 as E2E binaries
  (their root cases stand; this entry names Path 1 and Path 6's harness form only); override expiry (§4 unit tier, by
  test-plan §6 Path 6 step 7); the harness-prefix and R8 identity-floor rows of `v1-34` (owed, no entry minted).
- Windows: every new case runs on the three CI runners; a Unix-only assertion is `cfg(unix)` with its Windows
  branch stated, as test-plan §6 already writes them.

## Spec effects expected (wrap)
- test-plan §3: the `run --e2e` usage-error clause and "`--e2e` selects nothing yet" retire; the integration layer's
  `!binary(…)` form becomes the standing one; `boot`'s "not built" clauses for `--statusline-echo` and `--agents-mode`
  retire; the Status endpoint shape's interim paragraph retires.
- test-plan §6 Path 1 and Path 6: their as-landed lines name the `path_` binaries and the harness-session leg.
