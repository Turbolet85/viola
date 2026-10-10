# Report — 2026-10-10-statusline-pass-through

**Chunk:** Statusline pass-through — named read-only source, per-home redirect for tests, settings.json rewritten
each start with absolute pinned path, user output unchanged, readings to budget.json
**Date:** 2026-10-10
**Commits:** `58f8720` chore(…): operator pre-CI commit, for the run this chunk's verdict reads · `67ab367`
fix(…): operator fix after CI run 38080061631, the every-OS hook_statusline cases start from a stamped home and
two unix-only helpers are cfg(unix) (basis: `git log --format='%h %s' 4d77eac..HEAD`, the parent of the oldest
pre-CI commit)

## Changes (structured — detectors read this)

- **Files:** 36 changed against `4d77eac` (basis: `gate.py scope`, `changed 36 · listed 35 · recorded 1`).
  - New (7): `crates/viola-agent-claude/src/statusline.rs`,
    `crates/viola-agent-claude/proptest-regressions/statusline.txt`, `crates/viola-state/src/budget.rs`,
    `src/cmd/hook/statusline.rs`, `tests/hook_statusline.rs`, `fuzz/corpus/hook_stdin/statusline-epoch`,
    `fuzz/corpus/hook_stdin/statusline-absent`.
  - Product, modified: `crates/viola-core/src/lib.rs`, `crates/viola-agent-claude/src/lib.rs`,
    `crates/viola-agent-claude/Cargo.toml`, `crates/viola-state/src/{lib,snapshot,replay,liveness,strict}.rs`,
    `src/cmd/{mod,run,hook,revive}.rs`, `src/run/{mod,dialog,wheel,snapshot}.rs`.
  - Test-only, modified: `src/bin/viola-fake-agent.rs`, `tests/{cli_fake_agent,cli_instance_state,
    tui_passthrough,hook_fail_open}.rs`, `tests/support/{home,fake}.rs`, `crates/viola-state/tests/state_replay.rs`,
    `fuzz/fuzz_targets/hook_stdin.rs`.
  - Build and config: `Cargo.toml` (one `[[test]]` entry), `Cargo.lock` (one line), `fuzz/Cargo.lock` (24
    packages), `.config/nextest.toml` (one binary added to a list).
- **Symbols / APIs:**
  - `viola-core`: `Reading<T>` (a value, or the word `"unknown"`), `UnknownWord`, `BudgetWindow
    {used_percentage: Reading<f64>, resets_at: Reading<String>}`, `BudgetReading {five_hour, seven_day}`. Serde
    both ways. No timestamp of its own, no Claude name.
  - `viola-agent-claude::statusline` (new module, pure: no file, clock or environment read): `reading(stdin) ->
    Result<Option<BudgetReading>, AgentError>` (not one JSON object → `AgentError::Malformed`; no `rate_limits`
    object → `None`), `user_command(settings) -> Option<String>`, `override_document(pinned_bin_fwd) ->
    Option<String>`, `shell_argv(command) -> Option<[&str; 3]>` (`/bin/sh`, `-c`, the command as one argument),
    `SETTINGS_FLAG = "--settings"`, `USER_SETTINGS = [".claude", "settings.json"]`. On Windows
    `override_document` and `shell_argv` return `None` for every input. No new error enum, no new `AgentError`
    variant.
  - `viola-state`: `budget::write_budget(home, &BudgetReading, read_at)` and `budget::BUDGET`;
    `InstanceSnapshot.statusline_command: Option<String>` (omitted when absent, `v` stays 1).
    **`SnapshotRead::Present` and `replay::Recovered::Snapshot` now hold `Box<InstanceSnapshot>`** (clippy
    `large_enum_variant` once the snapshot reached 216 bytes). Remaining callers: `read_snapshot` (unboxes; its
    signature is unchanged), `read_snapshot_or_replay`, `src/cmd/revive.rs` `recorded_dir` (reads through the
    box), and tests.
  - Root bin: `viola hook statusline` (a new arm of the hidden `hook` verb, dispatched on the word before
    `HookEvent::from_arg`; `HookEvent` still holds nine events); `STATUSLINE_DEADLINE = 5 s`, PROVISIONAL, in
    `src/cmd/hook.rs`; `cmd::default_home(user_home)`, shared by `resolve_home` and the source rule; in
    `src/cmd/run.rs` `statusline_source`, `statusline_command`, `write_override`, the `Recorded` argument of
    `start_state`; `run::child_launch` takes `settings: Option<&Path>` (three callers: `start` and two unit tests).
  - The child's arguments are now `--plugin-dir <dir>`, then `--settings <instance dir>/settings.json` when an
    override was written, then the caller's arguments.
  - No new CLI verb or flag of `viola`, no channel method, no event kind, no listener, no port, no socket.
- **Crates / modules:** added `viola_agent_claude::statusline`, `viola_state::budget`, `cmd::hook::statusline`
  (root bin). No crate added or removed. No edge between `viola-agent-claude` and `viola-state`.
- **Dependencies:** `viola-agent-claude` now names the workspace's `chrono` (`=0.4.45`, features `clock`, `std`),
  already in the root graph through `viola-state` and the root bin: `Cargo.lock` gains one line and no package.
  `fuzz/Cargo.lock` gains 24 packages (chrono and what its `clock` feature names per target), each at the root
  lockfile's version (basis: a comparison script over both lockfiles, 0 differing, taken before the pre-CI
  commit). CI `supply-chain` passed on both pushes.
- **Schema / config:** none in `schemas/` (preservation guard green). New files on disk:
  - `<home>/budget.json`: `{"v":1,"five_hour":…,"seven_day":…,"read_at":"<RFC 3339 UTC ms>"}`, replaced whole
    through `replace_private` at 0600 under an exclusive lock on `budget.json.lock`; written only by `hook
    statusline`, and only when the payload holds a `rate_limits` object. A window is the word `"unknown"` or
    `{used_percentage, resets_at}`; either field may be `"unknown"`.
  - `<home>/statusline-source.json`: read-only to viola, in the Claude settings shape; the source whenever it
    exists.
  - `<home>/instances/<name>/settings.json`: `{"statusLine":{"type":"command","command":"<pinned path> hook
    statusline"}}`, rewritten at every start through `replace_private` at 0600; written only on Unix and only
    when every character of the pinned path is an ASCII letter, a digit or one of `_ - . / :`.
  - No `config.json` key.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - fake agent argv options: +2 (`--settings`, `--statusline-stdin`), and one mode word `statusline-echo`
    (the plan's count: ten → twelve, architecture's own list);
  - fake agent receipt kinds: +1, `statusline` (the `hook` receipt's fields without `event`);
  - `fuzz/corpus/hook_stdin`: 10 → 12 seeds (`ls | wc -l`);
  - `InstanceSnapshot` struct literals: 11 → 12 (the plan forecast 11; the twelfth is in the new arm's unit
    tests; basis `grep -rn -E 'InstanceSnapshot \{' src crates tests --include=*.rs`: 20 lines, 18 at the plan);
  - `.config/nextest.toml` verify-driven binary list: 12 → 13 (`binary(hook_statusline)`);
  - root `[[test]]` entries: +1;
  - root waits `Instant::now() + WITHIN` under `tests/`: 26 → 26 (`grep -rn -F … | wc -l`, and `git grep` at the
    base); the plan forecast a rise;
  - local unit suite 1624 → 1765; CI test totals `windows-2025` 2023 → 2169, `macos-latest` 1998 → 2159,
    `ubuntu-latest` 2002 → 2163 (the runs' own Summary lines);
  - unchanged, verified: `HookEvent` 9, `LedgerRow` 17, `ObsEvent` 19, `hook-decision` detail codes 7, the
    fail-open matrix 11 cases (its `unknown_event` case now names `status-line`).
- **Dev-tool versions:** none — `claude` re-read at 2.1.287 by path (19:22:41Z, dev host); the bare name still
  resolves to 2.1.289 there.
- **Harness / gate surface:** none under `scripts/`, `crates/viola-e2e`, `.github` (preservation guard green).
  Root test support gained `TestHome::default_of_user`, `Wrapper::boot_as_user` (sets `HOME`, or `USERPROFILE`
  on Windows, on that one child), `plant_statusline_source`, `statusline_marker`, `statusline_echo_command`,
  `fake::wait_statusline`. `boot --statusline-echo` and `statusline-source-unresolved` are not built (the plan's
  cut to "Budget governor").
- **Cross-project / external claims:**
  - CI, `Turbolet85/viola`: run `38080061631` measured `58f872077352`, conclusion `failure` (`lint` and `test` on
    `windows-2025`); run `38080855246` measured `67ab3677cacf`, conclusion `success`, 15/15, attempt 1. Basis:
    `evidence/operator-pass.md`.
  - The live CLI: three sessions on `claude` 2.1.287 on the Linux dev host (`evidence/live-statusline.md`).
  - `https://code.claude.com/docs/en/statusline` as fetched at planning (`research.md`).
  - The founder's `~/.claude/settings.json`: only the presence and type of `statusLine` were read (present,
    `command`), never snapped and never written.
  - Inputs (`inputs.py verify`, this wrap): `I1 · message · copy · n/a`, `I2 · memory file · copy ·
    unchanged`, `I3 · message · copy · n/a`, `I4 · message · copy · n/a` — `inputs#I4` is the implement
    invocation's relay of the founder's words on W1, W2, W3, R and the Windows cut, cited here and in
    Deviations. No entry drifted, vanished or broke; none unparsed.
  - Added at this wrap's resume (2026-10-10, Phase 2): `I5 · relay file · copy · unchanged` — `inputs#I5` is
    the operator's directions for this wrap, six numbered items written by the overseer at 20:03Z (the one card
    for the answered widenings, the founder's ratification of the revive session id, the revive check-order
    CARRY, the `--settings` sentence, the rows owed, the rest to first consumers). It changes no fact above.
    `I6 · message · copy · n/a` — `inputs#I6` is the operator's answers at the Phase 2 halt: the one card
    confirmed (W1, W2, W3 and the revive session id) with its playbook rule, the two start steps without a
    span recorded as by design, and two route placements.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **plan.md step 2 and acceptance 1: "the literal `--settings` is defined here and nowhere else in product
     code".** False on the tree before and after this chunk: `src/cmd/verify/typed.rs:132` and `:173` hold the
     literal (basis: `grep -rn -F -e '--settings' src crates --include=*.rs`: `statusline.rs` 2, of which one a
     test; `verify/typed.rs` 2; `src/run/mod.rs` 2, both test oracles; the fake agent 8). `src/cmd/verify` is
     under the plan's own preservation guard, so this chunk could not move those two. The diff adds no second
     product site.
  2. **plan.md step 7, items 3 and 4: obs init, then `check_instance`.** In that order a home at mode 0770
     passes the check: `viola_obs_init` → `open_role_file` → `create_private_dir(home)` sets an existing home to
     0700 first. Measured: case 5 red in the plan's order (`evidence/red-green.md` §3). The arm checks first.
  3. **plan.md step 9: a home is "one viola has already created" after "a first boot and stop".** False on
     Windows x64: `Wrapper::boot` calls `seed_conpty`, which makes the home with `create_dir_all`. CI run
     `38080061631`: three cases refused by the arm's check from such homes; the stamped-home case passed. The
     refused home's DACL itself was not read.
  4. **The fetched documentation: `rate_limits` "only after the first API response in the session".** Live start
     2 carried it on the second payload, 0.35 s after the first. Cause not known. No master states the sentence.
  5. **Read in source, not measured: `viola revive` opens its log before its instance check.** `revive` calls
     `open_wrapper_log` (which narrows the home the same way) and then `preflight` → `check_instance`. By the
     mechanism of item 2, a home the user owns at a group- or other-writable mode is 0700 before that check
     reads it. Stated in `.claude/rules/security.md` (the `viola revive` bullet) and in the masters as "run
     before any snapshot or log value is used", which stays true; what the check can refuse on the home's mode
     is what this questions. `tests/cli_revive.rs:285` widens the instance directory, not the home.
- **Expected amendments (from plan):** site search for all lines: one script over the seven bodies and every
  file under `.andromeda/registries/contracts/<master>/`, lines matching each pattern, written `body+keys`.
  - architecture §Standard Contracts → Hook contract — carried: Symbols / APIs (the arm) and Schema / config;
    its facts: the named source rule, the arm's order (canonicalise, instance check, then the log, stdin, the
    write, the snapshot read, the shell-out), the 5 s PROVISIONAL bound, the command's environment and directory
    unchanged and stderr discarded, no output on oversize or malformed stdin, the write only with `rate_limits`.
    Sites: `hook statusline` architecture 6; `effective … settings` architecture 1; `only shell-out`
    architecture 1.
  - architecture §Occupied Resources — carried: Schema / config and Counts. Sites: `statusline-source`
    architecture 0 (a new name there); `settings\.json` architecture 2; `budget\.json` architecture 6;
    `--settings` architecture 2.
  - architecture §Standard Contracts → Instance snapshot, §Established Decisions [Session Liveness],
    §Cross-cutting Patterns (Config management; Capability ledger) — carried: Symbols / APIs
    (`statusline_command`; the override is written after the first snapshot and before the spawn; the per-home
    source). The Capability-ledger sentence follows the founder's word R (`inputs#I4`): relied on with no row,
    three live readings, no row and no fixture. Sites: `statusline_command` architecture 3.
  - architecture §Infrastructure Patterns → Crate dependency direction — carried: Dependencies. Sites: `chrono`
    architecture 5 + 2 key lines.
  - security-plan §Data Protection, §Authentication & Authorization, §Input Validation — carried: Schema /
    config, Symbols / APIs, Deviations (the check-first order). W1, W2 and W3 are written with the founder's
    word as recorded in `inputs#I4`. Sites: `settings\.json` security-plan 5; `statusline_command`
    security-plan 11; `hook statusline` security-plan 8; `resets_at` security-plan 2; `only shell-out`
    security-plan 2; `statusline-source` security-plan 0.
  - test-plan §3 → 5-command implementation, §6 Path 6, §6 Property suite, §7 Fake agent, §7 Seed strategies —
    carried: Harness / gate surface, Counts, Spec claims disproved 3 (the seed rule: on Windows only a stamped
    home is one viola created). Sites: `statusline-echo` test-plan 3 + 1 key line; `statusline-source`
    test-plan 0 + 1 key line; `statusline-source-unresolved` test-plan 0 + 1 key line; `resets_at` test-plan 8;
    `rate_limits` test-plan 6; `budget\.json` test-plan 13 + 2 key lines.
  - obs-plan §4 Scenario: Budget governor, §6 `detail` code catalog — carried: Coverage of new surfaces (the
    lines and spans as landed; details used: `strict-modes-failed`, `oversize-stdin`, `malformed-json`,
    `deadline`; `deadline_hit` is written only when true). Sites: `hook statusline` obs-plan 5; `budget\.json`
    obs-plan 4; `statusline_command` obs-plan 6.
  - a11y-plan §3 → Keyboard test harness, §8 Cognitive Accessibility — carried: Outcome (the
    statusline-bearing start; the 5 s bound falls on the user's own command, never on a keystroke). Sites:
    `hook statusline` a11y-plan 1.
  - design-system, layout-templates: none expected — holds: the chunk adds no line, hint, refusal or `verify`
    row. (`resets_at` design-system 4 are the board's fields, untouched.)
  - `matrix#v1-24 notes`, `matrix#v1-45 notes` — not the wrap's: phase P5 wrote both before implement (they ride
    `58f8720`).
- **Coverage of new surfaces:**
  - `viola hook statusline` stdin → validation ✓ (`take(MAX_FRAME + 1)`, one JSON object, every field tolerant)
    · instrumentation ✓ (span `hook.statusline`; `hook-invoked`, `parse-rejected`, `hook-decision` with
    `budget_written`) · PII ✓ (no payload byte in a process log; `hook_statusline` case 9) · tests unit + integ
    + property + fuzz · a11y n/a · tokens n/a
  - the statusline source read at start → validation ✓ (`take(MAX_FRAME)`; a command only from `statusLine`
    of type `command`, non-empty, no NUL) · instrumentation n/a (no line and no span by design) · PII ✓ (the
    command stands in `snapshot.json` only) · tests unit + integ · a11y n/a · tokens n/a
  - the shell-out of the user's command → validation ✓ (the instance check first; the command one argument;
    stdout through `take(MAX_FRAME)`; a 5 s bound) · instrumentation ✓ (span `statusline.shell_out`;
    `process-start` / `process-exit` with subject `statusline-shell`, `shell_exit_status`, `duration_ms`) · PII ✓
    (neither line nor span carries the command) · tests unit (Unix) + integ + three live readings · a11y n/a ·
    tokens n/a
  - `budget.json` write → validation n/a · instrumentation ✓ (span `state.budget_write` with `five_hour_pct`,
    `seven_day_pct`) · PII n/a · tests unit + integ · a11y n/a · tokens n/a
  - the settings override and `--settings` → validation ✓ (the plain-character rule on the pinned path) ·
    instrumentation n/a (no new span or line; the span-order test's list is unchanged) · PII n/a · tests unit +
    integ + live · a11y ✓ (the statusline-bearing start adds no viola byte to the terminal, three OSes) ·
    tokens n/a

## Deviations from intent

1. **The instance check runs before the hook's log opens** (plan step 7 lists the log first). Justification:
   Spec claims disproved 2. The lines logged and their order are the plan's.
2. **Step 12 item 1: the recorder was the rig directory's project-scope status line, not a user-scope one.** The
   user-scope file is the founder's own and feeds his open sessions; nothing covered editing it. The shell and
   the payload read are a project-scope status line's.
3. **The live rig used a new, unstamped home** (`cli_verified` false in every session), so the stamped live home
   got no source file and no `budget.json`. `live-start.sh` of the earlier chunk was not used: it passes a
   `--settings` of its own.
4. **`live-pty.py` gained a count of marker literals** in the drained bytes (said in its header); the rig keeps
   no screen, and the plan asks what the status line row shows. The row's own appearance stays unread.
5. **`.config/nextest.toml` changed**: five `hook_statusline` cases start from a stamped home (the plan's "if").
6. **`USER_SETTINGS` lives in `viola-agent-claude`**; the plan did not name it. It is a Claude-specific path.
7. **`state_replay.rs`'s literal carries a command**, not `None`, so the replay assertion beside `cwd` compares
   a present value with an absent one.
8. **The property's strategy gained whole seconds across the date range**: as first written it stayed green on
   the stub (`evidence/red-green.md` §1).
9. **After the first CI run** the three every-OS cases that run the arm's whole course take the `stamped_home`
   fixture, and the process-log scan requires its decision to read `budget_written` true with no `detail`
   (Spec claims disproved 3).
10. **Founder's words, recorded:** W1 confirmed, W2 confirmed, W3 the file in the viola home, R option B (a
    manual reading, three live starts, no row, no fixture), the Windows cut confirmed, so S3 is final
    (`inputs#I4`, relayed verbatim by the operator, 2026-10-10).

Scope record (`gate.py scope`, this wrap: `scope: clean — changed 36 · listed 35 · recorded 1`):
- mechanical: `fuzz/Cargo.lock` · serves `crates/viola-agent-claude/Cargo.toml` · self.

## Decisions & corrections

- The operator ran the close in two sessions: this wrap's Phase 1, then a clear, then Phase 2 on, because the
  implementing session's context read 79.8 %, past its 60 % line.
- Step 12 was run on a home made for it rather than the stamped live home; the stamped home stays free of a
  statusline source.
- Sweep and tool hazards met this chunk:
  - on Windows a helper called only by `cfg(unix)` cases is dead code, and no gate on the host lints the
    Windows target. A check-only `cargo clippy --target x86_64-pc-windows-msvc` under `target/wincheck`
    reproduced the runner's error in a control;
  - a count of nextest `PASS` lines split on a fixed column misreads rows whose duration is padded
    differently (93 read as 72): strip the bracket and the ordinal first;
  - the shell variable `TMPDIR` is unset on this host: a redirect written against it lands at the filesystem
    root;
  - the Bash guard refused a `cat` heredoc append to a record (the standing rule); it went through the Edit
    tool;
  - one gate call's output was read through a line filter; the verdict was then taken from the trail;
  - `gh run list --commit` was given a mistyped sha once and printed nothing; the full sha read the run;
  - `SnapshotRead::Present(` and `Recovered::Snapshot(` now take a box: a grep for the unboxed literal form
    finds nothing.

## Outcome

Acceptance criteria, each against the diff:

- (arch) shapes in `viola-agent-claude` only; files through `viola-state`'s replace helper; no
  `viola-agent-claude` → `viola-state` edge; no Tokio — **UNMET as written in one clause, met in the rest.**
  The clause "the `--settings` literal … nowhere else in product code" is contradicted by `src/cmd/verify/
  typed.rs:132`, `:173`, which predate the chunk and sit under its preservation guard (Spec claims disproved
  1). Unlinked to a capability: a P2 escalation. The payload reader, the settings reader, the override
  document and the shell are defined in the agent crate alone; this diff adds no other product site of the
  literal.
- (arch) `hook statusline` exits 0, empty stderr, prints the user's stdout byte for byte, nothing without a
  recorded command — met (`hook_statusline` cases 1 to 4).
- (arch) `budget.json` only by `hook statusline`, by replace under its lock, `v` 1, `"unknown"` forms — met.
- (arch) no environment variable, `config.json` key, flag of `viola`, channel method, event kind or listener;
  snapshot `v` 1; no ledger row, hook event or schema value — met (preservation guard exit 0). Beside it:
  `start` now also reads `std::env::home_dir()` for the source rule, the read `resolve_home` already made.
- (arch) the one source rule; viola writes neither file — met.
- (security) a home another user can write: no command, nothing printed, marker and `budget.json` unchanged,
  exit 0 — met (case 5; remove-the-guard red and green in `evidence/red-green.md` §2).
- (security) the override rewritten whole at 0600 with the absolute pinned path; `budget.json` 0600 — met.
- (security) oversize and non-object stdin: exit 0, empty stderr, no `budget.json`; a malformed `resets_at` is
  `"unknown"` — met.
- (security) the command, the echo's marker and the canary in no file under `<home>/diagnostics/` and no
  stderr — met (case 9, from a stamped home, with the arm's whole course asserted).
- (security) neither `snapshot.json` nor `ledger/stamps.json` changes — met (case 8).
- (tests) unit and both filters green; `pre-push` green with coverage lines 97.69, functions 97.63, regions
  97.46 (floors 85, 95, 80), ignore regex unchanged; no mutation gate — met.
- (tests) Path 6's signal under the fake agent — met (cases 1 and 5).
- (tests) the property at `cases: 512` with its regressions file in the tree — met.
- (tests) the `hook_stdin` target reaches the reader; corpus replays green — met.
- (obs) the four role lines, no `corr`; nothing without `VIOLA_NAME` — met (cases 1 and 7).
- (obs) G4 and G2 clean — met: locally over the smoke home and the live rig home (157 files, 2198 lines); on CI
  over the kept homes (292, 300 and 300 files).
- (design, layouts) no line of viola's, no byte added, no verb in `--help` — met.
- (a11y) the zero-viola-bytes cases green on three OS legs; the new start holds none of viola's literals — met
  (run `38080855246`).
- (ci) the final HEAD's run green on all three OSes — met for `67ab3677cacf`: run `38080855246`. This wrap's
  commit adds to that tree.
- No capability is claimed; `v1-24` advanced, not proven — holds (`matrix.py show --chunk`: claimed 0).

Gates (implement's block run of 19:15:48Z, then the operator pass; by `run` text):

- `cargo fmt --all --check` — green · exit 0
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green · exit 0 (red once
  before the block: `large_enum_variant`, fixed by the box)
- `bash scripts/agent-run.sh run --unit` — green · `"ok":true` · 1765 of 1765
- `bash scripts/agent-run.sh run --unit --filter 'test(/statusline|budget_write|budget_reading|spawn_settings|
  spawn_without_an_override/)'` — green · 140 of 140
- `bash scripts/agent-run.sh run --integration --filter 'binary(hook_statusline) | binary(cli_instance_state) |
  binary(tui_passthrough) | binary(hook_fail_open) | binary(cli_fake_agent) | binary(state_replay)'` — green ·
  107 of 107 (again after the fix)
- `git diff --quiet 4d77eacd14d6 -- …` (the preservation guard) — green · exit 0
- `bash scripts/agent-run.sh run --fuzz-replay` — green · 5 targets
- `bash scripts/agent-run.sh cleanup --session p-sl-smoke` — green · `cleaned:[]`
- `bash scripts/agent-run.sh boot --session p-sl-smoke --instance builder` — green · `"ok":true`
- `bash scripts/agent-run.sh status --session p-sl-smoke` — green · `state:"ready"`
- `bash scripts/g2-zero-panics.sh` — green · `g2: clean`
- `bash scripts/agent-run.sh schema-check` — green · `"ok":true`
- `bash scripts/agent-run.sh cleanup --session p-sl-smoke` — green · `processes_gone:true`,
  `endpoint_gone:true`
- `bash scripts/agent-run.sh pre-push` — green · `"ok":true`, `"stage":"linux-tests"` · three firings, the
  last on the fixed tree
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` — `leg =
  'operator'`, fired as written: `hygiene: clean`
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` — `leg = 'operator'`, fired twice
  through `--operator`: green both times (`4d77eacd→58f87207`, `58f87207→67ab3677`)
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait
  1800` — `leg = 'operator'`, fired as written twice: `verdict: red` on `58f872077352` (run `38080061631`: two
  Windows jobs, both this chunk's own test code, both fixed in `67ab367`), then `verdict: green` on
  `67ab3677cacf` (run `38080855246`, 15/15, attempt 1). Record: `evidence/operator-pass.md`.
- Smoke: the boot, status and cleanup entries above ran in the block; not driven again.

Watches: none — the entry folded no `watch:`.

Outcome basis: the operator pass's final state, `67ab367`, and run `38080855246` as recorded in
`evidence/operator-pass.md`; implement's P4 report as given in this conversation; one finding made at this
wrap's authoring and absent from that report (Spec claims disproved 1).

Process hygiene (implement's census, re-measured here at 19:57:02Z: 0 processes whose executable lies under
this repository's `target/`, 0 rig hosts, 0 of `claude` 2.1.287 by path):

| process | started by | final state |
|---|---|---|
| the smoke session `p-sl-smoke` | implement's gate block | terminated (`processes_gone:true`) |
| two rehearsal hosts, three live hosts | implement, plan step 12 | terminated (exit 0, not killed) |
| background builds, gate calls, the Windows-target lint | implement and the operator pass | terminated |
| the code-graph refresh | this wrap's Setup | terminated (rust 4979 nodes / 26188 edges, ts 7 / 1) |
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 4d77eacd (the parent of the oldest pre-CI commit 58f87207) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — added 1 line(s) in 1 range(s)
added: 32
### Cargo.lock — added 1 line(s) in 1 range(s)
added: 1625
### Cargo.toml — added 5 line(s) in 1 range(s)
added: 96-100
### crates/viola-agent-claude/Cargo.toml — added 2 line(s) in 1 range(s)
added: 16-17
### crates/viola-agent-claude/proptest-regressions/statusline.txt — new file · 7 line(s)
### crates/viola-agent-claude/src/lib.rs — added 1 line(s) in 1 range(s)
added: 8
### crates/viola-agent-claude/src/statusline.rs — new file · 458 line(s)
- 23-36 @25 «pub fn reading(stdin: &[u8]) -> Result<Option<BudgetReading>, AgentError> {»
  - 26-28 «let Ok(Value::Object(payload)) = serde_json::from_slice::<Value>(stdin) else {»
  - 29-31 «let Some(Value::Object(limits)) = payload.get("rate_limits") else {»
  - 32-35 «Ok(Some(BudgetReading {»
- 38-46 «fn window(value: Option<&Value>) -> Reading<BudgetWindow> {»
  - 39-41 «let Some(Value::Object(window)) = value else {»
  - 42-45 «Reading::Known(BudgetWindow {»
- 48-53 «fn used_percentage(value: Option<&Value>) -> Reading<f64> {»
  - 49-52 «match value.and_then(Value::as_f64) {»
- 55-73 @57 «fn resets_at(value: Option<&Value>) -> Reading<String> {»
  - 58-66 «let at = match value {»
  - 67-72 «match at {»
- 75-85 @77 «pub fn user_command(settings: &[u8]) -> Option<String> {»
  - 80-82 «if line.get("type")?.as_str()? != "command" {»
- 87-93 @91 «pub fn override_document(pinned_bin_fwd: &str) -> Option<String> {»
- 95-101 «fn override_on(pinned_bin_fwd: &str, wrapped: bool) -> Option<String> {»
  - 96-98 «if !wrapped || !pinned_bin_fwd.chars().all(is_shell_plain) {»
- 103-105 «fn is_shell_plain(c: char) -> bool {»
- 107-111 @109 «pub fn shell_argv(command: &str) -> Option<[&str; 3]> {»
- 113-115 «fn shell_argv_on(command: &str, wrapped: bool) -> Option<[&str; 3]> {»
- 117-458 @118 «mod tests {»
  - 124-129 «fn known(used_percentage: Reading<f64>, resets_at: Reading<String>) -> Reading<BudgetWindow> {»
  - 131-133 «fn at(text: &str) -> Reading<String> {»
  - 135-137 «fn read(payload: &Value) -> Option<BudgetReading> {»
  - 139-150 @148 «fn statusline_reading_of_a_payload_that_is_not_one_object_is_malformed(#[case] stdin: &[u8]) {»
  - 152-161 @159 «fn statusline_reading_without_a_rate_limits_object_is_none(#[case] payload: Value) {»
  - 163-181 @164 «fn statusline_reading_takes_both_windows_and_ignores_every_other_key() {»
  - 183-193 @189 «fn statusline_reading_a_window_that_is_absent_or_no_object_is_unknown(#[case] limits: Value) {»
  - 195-204 @196 «fn statusline_reading_each_window_stands_alone() {»
  - 206-228 @220 «fn statusline_reading_used_percentage_is_kept_only_from_0_to_100(»
  - 230-238 @231 «fn statusline_reading_a_window_without_used_percentage_has_it_unknown() {»
  - 240-271 @263 «fn statusline_reading_resets_at_is_epoch_seconds_or_rfc3339_as_utc_millis(»
  - 273-278 @274 «fn statusline_reading_a_window_without_resets_at_has_it_unknown() {»
  - 289-309 @301 «fn statusline_user_command_is_the_command_of_a_command_status_line(»
  - 311-317 @315 «fn statusline_user_command_of_bytes_that_are_no_document_is_none(#[case] settings: &[u8]) {»
  - 319-334 @320 «fn statusline_override_is_the_pinned_path_then_hook_statusline() {»
  - 336-344 @338 «fn statusline_override_reads_back_as_a_command_status_line() {»
  - 346-361 @359 «fn statusline_override_of_a_path_a_shell_could_misread_is_none(#[case] path: &str) {»
  - 363-367 @364 «fn statusline_override_where_the_status_line_is_not_wrapped_is_none() {»
  - 369-375 @370 «fn statusline_override_on_this_host_is_built_only_on_unix() {»
  - 377-386 @378 «fn statusline_shell_is_bin_sh_with_the_command_as_one_argument() {»
  - 388-396 «fn config() -> ProptestConfig {»
  - 398-421 @401 «fn arbitrary_json() -> impl Strategy<Value = Value> {»
  - 423-437 «fn round_trips(window: &Reading<BudgetWindow>) -> bool {»
  - 439-457 «proptest! {»
### crates/viola-core/src/lib.rs — added 121 line(s) in 2 range(s)
added: 223-258 · 621-705
- 223-228 @226 «pub enum UnknownWord {»
- 230-237 @234 «pub enum Reading<T> {»
- 239-241 «impl<T> Reading<T> {»
- 243-249 @246 «pub struct BudgetWindow {»
- 251-257 @254 «pub struct BudgetReading {»
  - 621-626 «fn window(used_percentage: Reading<f64>, resets_at: Reading<String>) -> Reading<BudgetWindow> {»
  - 628-634 @629 «fn written(reading: &BudgetReading) -> String {»
  - 636-652 @637 «fn budget_reading_both_windows_read_are_two_objects() {»
  - 654-664 @655 «fn budget_reading_an_omitted_window_is_the_word_unknown() {»
  - 666-679 @667 «fn budget_reading_an_unparseable_percentage_is_the_word_unknown() {»
  - 681-691 @682 «fn budget_reading_an_unparseable_reset_is_the_word_unknown() {»
  - 693-704 @694 «fn budget_reading_a_whole_number_percentage_reads_back_as_a_number() {»
### crates/viola-state/src/budget.rs — new file · 187 line(s)
- 18-24 @19 «struct Envelope<'a> {»
- 26-35 @27 «fn used(window: &Reading<BudgetWindow>) -> Option<f64> {»
  - 28-34 «match window {»
- 44-59 «pub fn write_budget(»
  - 49-54 «let envelope = Envelope {»
- 61-187 @62 «mod tests {»
  - 67-72 «fn read_at() -> DateTime<Utc> {»
  - 74-79 «fn window(used_percentage: Reading<f64>, resets_at: Reading<String>) -> Reading<BudgetWindow> {»
  - 81-83 «fn at(text: &str) -> Reading<String> {»
  - 85-87 «fn on_disk(home: &Path) -> String {»
  - 89-102 @90 «fn budget_write_is_the_v_1_envelope_with_both_windows_and_read_at() {»
  - 104-116 @105 «fn budget_write_keeps_each_unknown_form_as_the_word() {»
  - 118-136 @119 «fn budget_write_a_second_write_replaces_the_first_whole() {»
  - 138-159 @140 «fn budget_write_leaves_both_files_owner_only() {»
  - 161-172 @162 «fn budget_write_into_a_missing_home_fails_and_leaves_nothing() {»
  - 174-186 @175 «fn budget_write_percentages_are_the_ones_that_were_read() {»
### crates/viola-state/src/lib.rs — added 1 line(s) in 1 range(s)
added: 6
### crates/viola-state/src/liveness.rs — added 1 line(s) in 1 range(s)
added: 92
### crates/viola-state/src/replay.rs — added 6 line(s) in 3 range(s)
added: 39 · 188 · 434-437
### crates/viola-state/src/snapshot.rs — added 50 line(s) in 7 range(s)
added: 62-65 · 111 · 148 · 156 · 182 · 346-386 · 412
  - 346-365 @347 «fn snapshot_statusline_command_is_written_under_v_1_and_reads_back() {»
  - 376-385 «fn snapshot_statusline_command_reads_as_absent_without_its_key(»
### crates/viola-state/src/strict.rs — added 1 line(s) in 1 range(s)
added: 737
### crates/viola-state/tests/state_replay.rs — added 13 line(s) in 6 range(s)
added: 35 · 47-54 · 129 · 133 · 203 · 220
- 47-53 @48 «fn statusline_command_of(recovered: &Recovered) -> Option<&str> {»
  - 49-52 «match recovered {»
### fuzz/Cargo.lock — added 223 line(s) in 15 range(s)
added: 5-13 · 26-37 · 56-66 · 76-81 · 100-123 · 141-164 · 204-214 · 252-257 · 264-272 · 348-353 · 421-426 · 549
       613-657 · 664-698 · 705-722
- 10-12 «dependencies = [»
- 61-65 «dependencies = [»
- 117-122 «dependencies = [»
- 146-154 «dependencies = [»
- 161-163 «dependencies = [»
- 209-213 «dependencies = [»
- 269-271 «dependencies = [»
- 618-624 «dependencies = [»
- 631-634 «dependencies = [»
- 641-647 «dependencies = [»
- 654-656 «dependencies = [»
- 669-675 «dependencies = [»
- 682-686 «dependencies = [»
- 693-697 «dependencies = [»
- 710-712 «dependencies = [»
- 719-721 «dependencies = [»
### fuzz/corpus/hook_stdin/statusline-absent — new file · 1 line(s)
### fuzz/corpus/hook_stdin/statusline-epoch — new file · 1 line(s)
### fuzz/fuzz_targets/hook_stdin.rs — added 5 line(s) in 3 range(s)
added: 1-3 · 9 · 17
### src/bin/viola-fake-agent.rs — added 219 line(s) in 14 range(s)
added: 37-40 · 80-81 · 124-125 · 335-372 · 468-509 · 528-540 · 543 · 855-859 · 895 · 898 · 967-970 · 975-976 · 1044-1045
       1298-1399
- 335-345 @337 «fn statusline_words(settings: &[u8]) -> Option<(String, Vec<String>)> {»
  - 339-342 «let mut words = doc["statusLine"]["command"]»
- 347-363 @350 «fn statusline_echo(args: &[String]) -> ExitCode {»
  - 353-357 «if let Some(marker) = args.first() {»
  - 359-361 «let _ = out»
- 365-371 @366 «fn statusline_echo_exit(args: &[String]) -> u8 {»
  - 367-370 «match args {»
  - 471-489 @475 «fn run_statusline(&self) {»
  - 855-859 «if let Some((mode, rest)) = args.split_first()»
  - 1298-1318 @1299 «fn fake_statusline_words_are_the_command_split_on_ascii_white_space() {»
  - 1320-1331 @1321 «fn fake_statusline_echo_exit_is_the_code_after_the_marker_path() {»
  - 1333-1367 @1336 «fn fake_statusline_runs_only_with_both_options_and_only_an_absolute_command() {»
  - 1369-1398 @1372 «fn fake_statusline_receipts_the_run_of_an_absolute_command() {»
### src/cmd/hook.rs — added 14 line(s) in 5 range(s)
added: 33 · 55-58 · 96-98 · 562-566 · 744
  - 96-98 «if args.event == statusline::WORD && args.capture.is_none() {»
  - 562-565 @563 «fn statusline_deadline_is_five_seconds() {»
### src/cmd/hook/statusline.rs — new file · 720 line(s)
- 35-41 @37 «fn canonical(instance: &Instance) -> Option<Instance> {»
- 43-81 «pub(super) fn statusline(started: Instant) -> Result<ExitCode, Failure> {»
  - 44-47 «let instance = instance_of(»
  - 48-50 «let Some(instance) = instance.as_ref().and_then(canonical) else {»
  - 55-60 «let _ = obs::viola_obs_init(»
  - 65-69 «let io = Io {»
  - 70-80 «match handle(&instance, trusted, io, started) {»
- 83-88 @84 «struct Io<'a> {»
- 90-95 @92 «struct Outcome {»
- 97-105 «impl Outcome {»
  - 98-104 @99 «fn failed(detail: &'static str) -> Self {»
- 107-125 @110 «fn handle(instance: &Instance, trusted: bool, io: Io<'_>, started: Instant) -> anyhow::Result<()> {»
  - 112-117 «obs_event!(»
  - 118-122 «let outcome = if trusted {»
- 127-163 @128 «fn serve(instance: &Instance, io: Io<'_>) -> anyhow::Result<Outcome> {»
  - 131-134 «if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME {»
  - 135-138 «let Ok(reading) = statusline::reading(&bytes) else {»
  - 139-141 @140 «let budget_written =»
  - 143-146 «let shell = command»
  - 147-158 «let detail = match shell {»
  - 159-162 «Ok(Outcome {»
- 165-185 «fn decided(outcome: &Outcome, started: Instant) {»
  - 167-184 «match outcome.detail {»
- 187-198 @189 «enum Shell {»
  - 190-194 @191 «Exited {»
- 208-230 «fn shell_out([shell, flag, command]: [&str; 3], stdin: &[u8], bound: Duration) -> Shell {»
  - 212-215 «let code = match &ran {»
  - 217-223 «obs_event!(»
  - 225-227 «if let Some(code) = code {»
- 232-256 @234 «fn run_bounded(command: &mut Command, stdin: &[u8], bound: Duration) -> Shell {»
  - 235-239 «let spawned = command»
  - 240-242 «let Ok(mut child) = spawned else {»
  - 245-255 «match wait_bounded(&mut child, &stdout, bound) {»
- 258-266 @260 «fn feed(pipe: Option<impl Write + Send + 'static>, bytes: Vec<u8>) {»
  - 261-265 «if let Some(mut pipe) = pipe {»
- 268-279 @269 «fn drain(pipe: Option<impl Read + Send + 'static>) -> mpsc::Receiver<Vec<u8>> {»
  - 271-277 «if let Some(pipe) = pipe {»
- 281-298 @282 «fn wait_bounded(»
  - 289-297 «loop {»
- 300-720 @301 «mod tests {»
  - 315-328 @316 «fn instance_in(root: &Path) -> Instance {»
  - 330-350 @331 «fn instance_running(root: &Path, command: Option<&str>) -> Instance {»
  - 356-362 «impl<S: tracing::Subscriber> Layer<S> for Lines {»
  - 364-384 @365 «fn handled(»
  - 386-388 «fn events(lines: &[Value]) -> Vec<&str> {»
  - 390-393 «fn budget(instance: &Instance) -> Option<Value> {»
  - 398-402 «impl Read for NeverRead {»
  - 406-411 @407 «fn statusline_canonical_instance_keeps_a_plain_directory() {»
  - 413-421 @414 «fn statusline_canonical_instance_of_a_missing_directory_is_none() {»
  - 423-435 @427 «fn statusline_canonical_instance_of_a_link_to_another_place_is_none() {»
  - 437-452 @441 «fn statusline_canonical_instance_resolves_a_linked_home() {»
  - 454-483 @455 «fn statusline_without_a_recorded_command_prints_nothing_and_records_the_reading() {»
  - 485-498 @486 «fn statusline_without_rate_limits_writes_no_reading_and_keeps_the_last() {»
  - 500-512 @503 «fn statusline_a_reading_that_cannot_be_written_is_budget_written_false() {»
  - 514-525 @515 «fn statusline_a_refused_instance_reads_runs_and_writes_nothing() {»
  - 527-543 @528 «fn statusline_oversize_stdin_writes_nothing() {»
  - 545-555 @547 «fn statusline_stdin_of_exactly_the_frame_cap_is_taken() {»
  - 557-570 @558 «fn statusline_malformed_stdin_writes_nothing() {»
  - 572-579 @573 «fn statusline_shell_out_of_a_shell_that_cannot_start_is_not_started() {»
  - 581-719 @582 «mod shell {»
### src/cmd/mod.rs — added 19 line(s) in 3 range(s)
added: 156-160 · 164-165 · 167-178
- 156-159 @157 «pub(super) fn default_home(user_home: &Path) -> PathBuf {»
### src/cmd/revive.rs — added 3 line(s) in 2 range(s)
added: 382 · 395-396
### src/cmd/run.rs — added 250 line(s) in 14 range(s)
added: 2-3 · 13 · 16 · 19 · 237 · 283-286 · 288 · 294 · 441-499 · 507 · 523-524 · 1015-1024 · 1043 · 1076-1240
- 448-463 @451 «fn statusline_source(home: &Path, user_home: Option<&Path>) -> Option<PathBuf> {»
  - 453-455 «if own.exists() {»
  - 458-462 «(default == home).then(|| {»
- 465-477 @468 «fn statusline_command(home: &Path, user_home: Option<&Path>) -> Option<String> {»
  - 471-475 «File::open(source)»
- 479-489 @482 «fn write_override(instance_dir: &Path, pinned: &Pinned) -> anyhow::Result<Option<PathBuf>> {»
  - 483-485 «let Some(document) = statusline::override_document(&pinned.path_fwd) else {»
- 491-495 @492 «struct Recorded<'a> {»
  - 1076-1078 «fn settings_with(command: &str) -> String {»
  - 1080-1091 @1082 «fn homes(root: &Path, user_command: &str) -> (PathBuf, PathBuf) {»
  - 1093-1106 @1094 «fn statusline_source_the_home_s_own_file_wins_over_the_user_s_settings() {»
  - 1108-1121 @1109 «fn statusline_source_a_home_named_by_home_reads_no_user_settings() {»
  - 1123-1138 @1124 «fn statusline_source_the_default_home_falls_back_to_the_user_s_settings() {»
  - 1140-1155 @1141 «fn statusline_source_that_is_absent_unreadable_or_holds_no_command_is_no_command() {»
  - 1157-1172 @1159 «fn statusline_source_over_the_frame_cap_is_no_command() {»
  - 1174-1213 @1175 «fn statusline_override_of_a_start_is_rewritten_whole_at_owner_only() {»
  - 1215-1239 @1217 «fn snapshot_statusline_command_of_a_start_is_the_source_s_command() {»
### src/run/dialog.rs — added 1 line(s) in 1 range(s)
added: 457
### src/run/mod.rs — added 67 line(s) in 7 range(s)
added: 16 · 96-98 · 109 · 131-134 · 190 · 225 · 236-291
  - 131-134 «if let Some(settings) = settings {»
  - 237-250 «fn launch_args(settings: Option<&Path>, user_args: &[OsString]) -> Vec<OsString> {»
  - 252-277 @254 «fn spawn_settings_flag_follows_the_plugin_dir() {»
  - 279-291 @280 «fn spawn_without_an_override_passes_no_settings_flag() {»
### src/run/snapshot.rs — added 1 line(s) in 1 range(s)
added: 72
### src/run/wheel.rs — added 1 line(s) in 1 range(s)
added: 793
### tests/cli_fake_agent.rs — added 100 line(s) in 2 range(s)
added: 20-21 · 885-982
- 898-900 «fn hex_of(bytes: &[u8]) -> String {»
- 902-911 @904 «fn fake_statusline_settings_without_a_stdin_file_runs_nothing() {»
- 913-944 @916 «fn fake_statusline_stdin_runs_the_settings_command_with_the_file_s_bytes() {»
  - 920-929 «let agent = Direct::spawn(»
  - 931-937 «assert_eq!(»
  - 940-943 «assert_eq!(»
- 946-981 @949 «fn fake_statusline_echo_mode_prints_its_output_and_files_its_stdin() {»
  - 952-969 «let echo = |stdin: &[u8], extra: &[&str]| {»
  - 977-980 «assert_eq!(»
### tests/cli_instance_state.rs — added 171 line(s) in 3 range(s)
added: 18-19 · 21-22 · 354-520
- 354-359 @355 «fn started_once(stamped: StampedHome) -> StampedHome {»
- 361-363 «fn settings_with(command: &str) -> String {»
- 365-370 «fn bytes_and_mtime(path: &Path) -> (Vec<u8>, std::time::SystemTime) {»
  - 366-368 «let modified = fs::metadata(path)»
- 372-407 @376 «fn run_rewrites_the_settings_override_each_start() {»
  - 381-384 «if cfg!(windows) {»
  - 386-389 «let pinned = snapshot_data(&dir).expect("the snapshot")["pinned_bin"]»
  - 391-394 «assert_eq!(»
  - 397-400 «fs::write(»
- 409-429 @411 «fn run_records_the_statusline_command_of_the_home_s_source() {»
  - 414-417 «assert!(»
  - 422-425 «assert_eq!(»
- 431-438 @432 «fn user_home_with(user_home: &Path, command: &str) -> PathBuf {»
- 440-457 @443 «fn run_with_home_reads_no_user_settings() {»
  - 453-456 «assert!(»
- 459-474 @462 «fn run_in_the_default_home_reads_the_user_settings() {»
- 476-519 @481 «fn path6_wrapped_statusline_prints_the_user_output_unchanged() {»
  - 486-487 «let payload = json!({"session_id": "canary-chain-value-5c1e",»
  - 494-499 «let wrapper = Wrapper::boot(»
  - 501-507 «assert_eq!(»
  - 508-511 «assert_eq!(»
  - 512-514 «let budget: Value =»
### tests/hook_fail_open.rs — added 1 line(s) in 1 range(s)
added: 180
### tests/hook_statusline.rs — new file · 459 line(s)
- 21-24 «use support::home::{»
- 29-36 @30 «fn payload(rate_limits: Option<Value>) -> Vec<u8> {»
  - 32-34 «if let Some(rate_limits) = rate_limits {»
- 38-43 @39 «fn reading(used: u64) -> Vec<u8> {»
  - 40-42 «payload(Some(»
- 45-48 @46 «fn hex_of(bytes: &[u8]) -> String {»
- 50-73 @52 «fn hook_statusline(env: &[(&str, OsString)], stdin: Vec<u8>) -> Ran {»
  - 54-61 «command»
  - 62-64 «for (key, value) in env {»
  - 67-69 «let writer = std::thread::spawn(move || {»
- 75-82 @76 «struct Session {»
- 84-105 «impl Session {»
  - 85-91 «fn hook(&self, stdin: Vec<u8>) -> Ran {»
  - 93-95 «fn role_lines(&self) -> Vec<Value> {»
  - 97-100 «fn budget(&self) -> Option<Value> {»
  - 102-104 «fn marker(&self) -> Option<String> {»
- 107-113 @109 «fn started_once(stamped: StampedHome) -> StampedHome {»
- 115-148 @123 «fn session(stamped: StampedHome, echo: Option<&[&str]>) -> Session {»
  - 126-130 «let stamped = if home.is_dir() {»
  - 132-139 «let stamped = match &command {»
  - 142-147 «Session {»
- 150-153 @151 «fn unstamped() -> StampedHome {»
- 155-157 «fn events(lines: &[Value]) -> Vec<&str> {»
- 159-173 @160 «fn violations(lines: &[Value]) -> Vec<String> {»
  - 163-172 «lines»
- 175-179 «fn assert_silent_success(ran: &Ran, stdout: &str) {»
- 181-231 @185 «fn hook_statusline_passes_the_user_output_through_and_records_the_reading() {»
  - 202-210 «assert_eq!(»
  - 213-215 «for pair in &lines[1..3] {»
  - 225-229 «assert!(»
- 233-248 @236 «fn hook_statusline_without_rate_limits_writes_no_reading_and_still_passes_through() {»
- 250-269 @253 «fn hook_statusline_a_failing_command_prints_nothing() {»
  - 260-263 «let exit = lines»
- 271-289 @274 «fn hook_statusline_without_a_recorded_command_prints_nothing_and_records_the_reading(»
- 291-319 @295 «fn hook_statusline_a_home_another_user_can_write_runs_nothing() {»
  - 309-312 «assert_eq!(»
- 323-327 «{»
- 331-353 @332 «fn hook_statusline_oversize_and_malformed_stdin_write_nothing(»
  - 343-346 «assert_eq!(»
- 355-377 @360 «fn hook_statusline_outside_a_wrapped_session_writes_nothing(#[case] named: bool) {»
  - 364-366 «if named {»
  - 370-375 «let left: Vec<_> = fs::read_dir(tmp.scratch())»
- 379-384 «fn bytes_and_mtime(path: &Path) -> (Vec<u8>, SystemTime) {»
  - 380-382 «let modified = fs::metadata(path)»
- 386-405 @388 «fn hook_statusline_changes_neither_the_snapshot_nor_the_stamps(stamped_home: StampedHome) {»
  - 394-398 «let shown = if cfg!(unix) {»
  - 401-404 «assert_eq!(»
- 407-418 «fn files_under(dir: &Path) -> Vec<PathBuf> {»
  - 409-416 «for entry in fs::read_dir(dir).expect("read dir") {»
- 420-424 «fn holds(haystack: &[u8], needle: &str) -> bool {»
  - 421-423 «haystack»
- 426-459 @429 «fn hook_statusline_keeps_the_command_and_the_payload_out_of_the_process_logs(»
  - 437-441 «let decisions: Vec<Value> = session»
  - 449-458 «for file in files {»
### tests/support/fake.rs — added 9 line(s) in 1 range(s)
added: 78-86
- 78-85 @80 «pub fn wait_statusline(path: &Path) -> Value {»
  - 81-83 «let lines = wait_for(path, "the statusline receipt", |l| {»
### tests/support/home.rs — added 108 line(s) in 10 range(s)
added: 185-193 · 269-310 · 406-419 · 443 · 454-455 · 460-486 · 533 · 535-537 · 539-542 · 562-566
  - 185-192 @188 «pub fn default_of_user() -> Self {»
- 269-278 @272 «pub fn plant_statusline_source(home: &Path, command: &str) -> PathBuf {»
- 280-283 @281 «pub fn statusline_marker(home: &Path) -> PathBuf {»
- 289-309 @293 «pub fn statusline_echo_command(marker: &Path, extra: &[&str]) -> String {»
  - 295-299 «let mut words = vec![»
  - 300-306 «for path in [&words[0], &words[2]] {»
- 409-418 @411 «struct Host<'a> {»
### tests/tui_passthrough.rs — added 58 line(s) in 2 range(s)
added: 18-21 · 263-316
- 18-21 «use support::home::{»
- 263-315 @268 «fn passthrough_with_a_statusline_source_adds_no_viola_bytes(#[from(home)] wrapped: TestHome) {»
  - 269-271 @270 «let (stopped, stamped) =»
  - 277-278 «let payload = json!({"session_id": "canary-chain-value-5c1e",»
  - 283-291 «let mut pty = OuterPty::spawn(»
  - 293-301 «if cfg!(unix) {»
  - 305-309 «assert_eq!(»
  - 311-314 «assert!(»
