# Report — 2026-10-10-viola-revive

**Chunk:** viola revive — resume a dead instance in place by its newest logged session id in the recorded cwd, no
launch replay, four preflight refusals
**Date:** 2026-10-10
**Commits:** since the last wrap (`00c73fd`, 2026-10-10T11:23:19Z): `0fad11c chore(2026-10-10-viola-revive):
operator pre-CI commit, for the run this chunk's verdict reads` (basis: `git log --format='%h %s' 00c73fd..HEAD`,
one line). The tree beside it at this report: `evidence/operator-pass.md` (the sections written after the
commit) and `research.md` (four lines indented, `inputs#I6`).

Every coordinate of text this chunk added is in the last section, `## New text, by line`; no line range of new
text is stated above it.

## Changes (structured — detectors read this)

- **Files:** 21 source, test and schema files (basis: `git diff --name-only 00c73fd` less `.andromeda/`,
  `viola-0.1.0/` and `.claude/`, 21 lines; `gate.py scope`: changed 21 · listed 21).
  - New (3): `src/cmd/revive.rs`, `tests/cli_revive.rs`, `tests/chaos_revive.rs`.
  - Modified (18): `crates/viola-state/src/{snapshot,replay,strict,liveness}.rs`,
    `crates/viola-state/tests/state_replay.rs`, `crates/viola-agent-claude/src/lib.rs`,
    `src/cmd/{run,mod,hook}.rs`, `src/main.rs`, `src/run/{dialog,snapshot,wheel}.rs`,
    `src/bin/viola-fake-agent.rs`, `schemas/diag-line.v1.json`, `tests/support/{home,outer_pty}.rs`,
    `tests/cli_fake_agent.rs`.
  - Chunk evidence (new, 10 files, basis `ls evidence`): `red-green.md`, `live-revive.md`, `live-sessions.ndjson`,
    `operator-pass.md`, and the rig scripts `live-pty.py`, `live-ledger.py`, `live-drive.py` (byte copies of
    chunk 2026-10-09-inner-cr-and-crlf-in-a-sent-text's), `live-setup.sh`, `live-wait.py`, `live-keys.py`.
- **Symbols / APIs:**
  - CLI verb, new: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name>
    --list` (`--list` conflicts with `--id`, `--fork` and the trailing arguments). Exit 0 when the revived
    child's wrapper ends as `run`'s does; exit 1 for a preflight refusal; exit 2 for a clap usage error, a
    malformed `--id` included. It takes no `--json`.
  - Preflight of the start arm, in this order, each refusal one `human::refuse` pair on stderr and one
    `process-exit` line (`subject` `self`, `exit_code` 1) in `run-<name>.ndjson`: (1) `strict-modes-failed`,
    (2) `already-live` (`run`'s own collision check and its two landed pairs, unchanged), (3) `no-session` (two
    pairs: no logged session; an `--id` the log does not hold), (4) `cwd-missing`. A later reading is not taken
    once an earlier one refuses. The four pairs are the plan's wording, accepted by the founder (`inputs#I5`,
    point 4).
  - The start arm logs as process `run` (`ObsProcess::Run`, the `run-<name>.ndjson` role file, `run`'s detail
    sink); `role_of` in `src/main.rs` names `revive` beside `run` (`Role::Other`). `--list` opens no process
    log and writes no line: its two refusals (`strict-modes-failed`, `no-session`) are the stderr pair and exit
    1 only.
  - A passed preflight launches program `claude` (looked up by name on the reviving process's own `PATH`,
    from its own current directory, by `run`'s resolver) with child arguments `--plugin-dir <dir>` (the
    wrapper's, first), `--resume <id>`, `--fork-session` when `--fork` is given, then the words after `--`. The
    child's spawn directory is the snapshot's recorded `cwd`; program resolution and the version gate never
    read that directory. The id is the newest logged one of the session-id shape, or `--id` when the log holds
    it. A revived start's first record is `wheel{holder:"driver", cause:"start"}`, as every start's (the
    founder: a revived start keeps it, `inputs#I5`, point 3).
  - `viola-state`, new public items: `snapshot::InstanceSnapshot.cwd: Option<String>` (omitted on write when
    absent; `SNAPSHOT_V` stays 1); `replay::{SessionLink, SessionChain, session_chain}` (one pass over
    `events.ndjson` through `events::read_from`, one link per `session-start` line whose `agent_session_id` is
    a string, with the pass's `Skipped`; it writes and logs nothing); `strict::check_instance(home,
    instance_dir)` (the home, `instances/`, the instance directory, its `snapshot.json` and `events.ndjson`,
    each that exists, in that order).
  - `viola-agent-claude`, new public items: `PROGRAM` (`claude`), `RESUME_FLAG` (`--resume`),
    `FORK_SESSION_FLAG` (`--fork-session`), `is_session_id` (36 characters, hyphens at offsets 8, 13, 18, 23,
    ASCII hex elsewhere), `resume_args(id, fork)`.
  - `src/cmd/run.rs`: `start` takes a `Launch { name, program, args, spawn_dir }` from its caller in place of
    `RunArgs`; `run` builds it from its arguments and the current directory. `start_state` writes the spawn
    directory into the first snapshot's `cwd` when it is valid UTF-8, and no `cwd` otherwise. New
    `open_wrapper_log` holds what `run` did before its start (log init, own start line, the persistent names).
    Made `pub(super)`: `Launch`, `Launched`, `Started`, `start`, `collision_check`, `refused`, `pump_child`,
    `open_wrapper_log`. Remaining-caller facts: `start` has two product callers now (`run`, `revive`) and the
    span-order unit test; `collision_check` is called from `start` and from revive's preflight, so a revive
    that passes runs it twice.
  - `read_snapshot_or_replay` has its first product caller (revive's third reading); the four other snapshot
    readers (`src/cmd/client.rs`, `run.rs` `collision_check`, `hook.rs` twice) still call `read_snapshot`.
  - Fake agent (test-only), two argv options, the ninth and tenth: `--resume <id>` and `--fork-session`.
    Under `--resume` the SessionStart fired at launch is the recorded `SessionStart.default` payload with
    `source` set to `resume` and `session_id` set to the id given (with `--fork-session` beside it, to the
    compiled id `0f0e0d0c-0b0a-4908-8706-050403020100`), the trailing newline kept. Without `--resume` every
    payload is the recorded bytes; `--fork-session` alone changes nothing. Its helper `payload` now sets the
    fields its caller names (it set `prompt` alone).
  - No channel method, listener, env var, config key, test seam, event kind, process value or harness command
    was added.
- **Crates / modules:** new module `src/cmd/revive.rs` in the root bin. No crate added or removed.
- **Dependencies:** none added or bumped (`Cargo.toml` and `Cargo.lock` unmoved against `00c73fda87ef`, the
  preservation guard entry, exit 0).
- **Schema / config:** `schemas/diag-line.v1.json`: the `process-exit` `detail` enum gains `cwd-missing` and
  `no-session` (9 values → 11). The snapshot's `data` gains the optional key `cwd`. No config key.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - Root wait count: 23 sites in 17 files → **26 sites in 19 files** (basis: `grep -rn 'Instant::now() +
    WITHIN' tests`, 26 lines, and `grep -rln`, 19 files, on the final tree at 14:53Z; the three new sites are in
    `tests/support/home.rs`, `tests/chaos_revive.rs` and `tests/cli_revive.rs`). Stated in architecture (`23
    sites`, 1 hit) and in the key file `contracts/test-plan/5-command-implementation.md` (1 hit).
  - Fake agent options: 8 → 10. `process-exit` details: 9 → 11. CLI verbs: one more, `revive`. Snapshot data
    fields: 11 → 12 (`cwd`). Root test files: two more.
  - Relied-on CLI shapes with no ledger row: one more, `--resume <id>` reopening the same session with a
    SessionStart of source `resume` (the founder, `inputs#I3`: no row now, owed on a route entry). The
    seventeen rows stand; no stamp moved.
  - Unit tests 1540 → 1624 (basis: the unit entry's document); CI tests 1931 / 1898 / 1902 → 2023 / 1998 /
    2002 on `windows-2025` / `macos-latest` / `ubuntu-latest` (basis: `evidence/operator-pass.md`).
- **Dev-tool versions:** none — `claude` re-read at 2.1.287 by path on the dev host (`claude --version`,
  14:47:42Z). Read beside it: the bare name `claude` on the dev host's own `PATH` resolves to 2.1.289, a
  version the home has no stamp for (`evidence/live-revive.md`).
- **Harness / gate surface:** no `agent-run` script, harness command, CI step or status shape changed. Root
  test support gained: `OuterPty::{kill, shown}`; `Wrapper::{boot_in, revive, kill, shown}`; `claude_dir`,
  `path_with`, `revived_agent_flags`, `revive_command`, `holder_gone` in `tests/support/home.rs`.
- **Cross-project / external claims:**
  - CI: run `ci#38061685124` on sha `0fad11c9c62d`, event `push`, `run_attempt` 1, conclusion `success`, 15 of
    15 checks (`ci.py conclusion --sha HEAD --wait 1800`: `verdict: green`; `evidence/operator-pass.md`). The
    sha is the record: this wrap's commit adds to that tree.
  - The `claude` CLI, 2.1.287, on the Linux dev host (three live starts on the founder's number, `inputs#I5`;
    basis `evidence/live-revive.md`): `viola revive` logged a `session-start` of cause `resume` with the first
    life's id, the child's `cwd` the recorded directory; `/compact` logged cause `compact` with the same id;
    the same id resumed by hand from another directory was found, ran there and loaded that directory's
    `CLAUDE.md`. The real SessionStart payload of source `resume` holds 10 key names, the fake agent's
    rewritten one 5 (`cwd`, `hook_event_name`, `session_id`, `source`, `transcript_path`); the five more are
    `context_tokens`, `estimated_cache_write_usd`, `prompt_cache_likely_expired`, `scratchpad_dir`,
    `seconds_since_last_response`. The `compact` payload holds 8 (the five, `model`, `prompt_id`,
    `scratchpad_dir`). `viola send` of `/compact` exits 13 `not-delivered` / `no-prompt-submitted` after 10 s
    while the command runs. No reading became a ledger row or a fixture.
  - The session-memory study `refs/session-memory-options.md` (tracked here): its P6 `/compact` half and its P7
    are read above; its P4 and P12 are not.
  - Inputs (`inputs.py verify`, 15:08Z: 6 entries, unchanged 1, drifted 0, vanished 0, broken 0, n/a 5,
    unparsed 0): `I1` · message, the operator's take-up direction · copy · n/a (a message has no live source);
    `I2` · the memory file `tests-kill-only-own-processes.md` · copy · unchanged; `I3` · message, the three P4
    forks · copy · n/a; `I4` · message, the P5 review · copy · n/a; `I5` · message, the founder's four answers
    with the implement invocation · copy · n/a, cited by this report (`verify` printed it `UNCITED` before the
    report existed); `I6` · message, the operator's two messages to this wrap · copy · n/a, snapped at this
    wrap and cited by this report. Added at the wrap's resume (15:18Z, after that reading): `I7` · message, the
    operator's invocation resuming this wrap at Phase 2 with the route directions · copy · n/a, cited below;
    `I8` · message, the operator's seven answers at this wrap's Phase 2 halt · copy · n/a, cited below (added
    after the fan-out read this report).
- **Reverted / negative API facts:** none. The three neutralising edits of the controls in
  `evidence/red-green.md` were each read back as removed before its green run.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - plan.md step 9, step 11 and the acceptance bullet "(tests) The kill-and-revive case … waits for
    gone-by-pid-and-start-time and an unconnectable endpoint": they name `wait_endpoint_gone`, whose Unix rule
    is the socket file absent (`tests/support/home.rs` `unconnectable`). A killed wrapper removes nothing, so on
    Unix its socket file stays and that wait can never end (`viola-channel` `server.rs`: the next bind takes
    over a leftover socket file). The case waits on `holder_gone` instead: on Unix a connect to the path is
    refused or the path is absent, on Windows the pipe is not found. The stop path keeps the old rule. Evidence:
    the kill-and-revive case green on the three CI runners.
- **Expected amendments (from plan):** thirteen entries. Search basis for every line: `str.count` of the token
  over each master body and `grep -rl -F` over `.andromeda/registries/` (15:09Z); hits as `master n`.
  1. architecture §Occupied Resources → Binary, subcommands and exit codes (the `revive` subcommand, the two
     exit-1 details, the fake agent's ninth and tenth options) — carried: Symbols / APIs, Schema / config.
     `squatted-name`: architecture 2, obs-plan 4, design-system 1, no key file. `tag-turn-screen`:
     architecture 1, test-plan 2. `revive`: architecture 1, test-plan 2, obs-plan 1.
  2. architecture §Standard Contracts → Instance snapshot (`cwd?`, a field no event carries) and Snapshot
     envelope (revive is the replay's first reader) — carried: Symbols / APIs. `pending_dialog`: architecture
     4. `no reader takes`: architecture 1.
  3. architecture §Established Decisions [PTY] (the child's cwd has a second source for a revived start) —
     carried: Symbols / APIs. `current directory`: architecture 1, security-plan 1.
  4. architecture §Cross-cutting Patterns → Capability ledger as the single gate (`--resume <id>` a relied-on
     shape with no row, owed on a route entry; the founder, `inputs#I3`) — carried: Counts / qualifiers moved.
     `no row yet`: architecture 1. `--resume`: 0 hits in every master.
  5. architecture §Infrastructure Patterns → Project directory structure (`src/cmd/revive.rs`, the two root
     test files) and §Occupied Resources → Filesystem (the root wait count) — carried: Files, Counts /
     qualifiers moved. `23 sites`: architecture 1 and the key file
     `contracts/test-plan/5-command-implementation.md`. `chaos_torn_append`: test-plan 2, architecture 0.
  6. security-plan §Input Validation, Child executable resolution row (the revived child's cwd is the recorded
     one; ratified by the founder as a boundary widening, `inputs#I3`) — carried: Symbols / APIs. `Child
     executable resolution`: security-plan 1.
  7. security-plan §Authentication & Authorization, `~/.viola/` access control row (revive runs the
     strict-modes check on its own read); §Input Validation, Own state files on read row, and §Threat Model
     Summary (the replay has a caller); a row for the session id as an argv value — carried: Symbols / APIs.
     `Strict-modes check`: security-plan 1. `Own state files on read`: security-plan 1. `no caller`:
     security-plan 1.
  8. test-plan §7 Fake agent (under `--resume` the fake agent sets `source` and `session_id`; the operator,
     `inputs#I3`) — carried: Symbols / APIs. `tag-turn-screen`: test-plan 2 (the option list's sites).
  9. test-plan §4 viola-state (a product reader takes the replay), §6 Exit-cause matrix (the revive causes), §3
     `run` step 2 (the root wait count) — carried: Symbols / APIs, Counts / qualifiers moved. `Exit-cause
     matrix`: test-plan 1. `23 sites`: test-plan body 0, its key file 1.
  10. obs-plan §6 `detail` code catalog (`cwd-missing`, `no-session`), §7 Panic hooks (revive's start arm is
      role `run`; `--list` writes no process log), §3 intro (`refuse` has a second calling verb), §4 Edge flows
      E5 (a product run writes the two lines) — carried: Symbols / APIs, Schema / config. `squatted-name`:
      obs-plan 4. `E5`: obs-plan 4, test-plan 2, a11y-plan 3. `state-recovered`: obs-plan 7 and the key files
      `contracts/obs-plan/log-format-json-schema.md`, `contracts/test-plan/log-format.md`,
      `contracts/a11y-plan/focus-management-test-harness.md`. `process-exit`: five key files (three under
      `contracts/obs-plan/`, two under `contracts/test-plan/`).
  11. design-system §Surface: cli → Component Patterns (the revive refusals and hints, the exit-1 phraseology
      row) and → Navigation Pattern (the verb and its help group) — carried: Symbols / APIs; the four pairs'
      texts are in `src/cmd/revive.rs` `Refusal::pair`. `already live`: design-system 4. `Navigation Pattern`:
      design-system 2.
  12. layout-templates §Surface: cli · Output structure (a `viola revive` structure: `--list` rows, the
      refusals) and — `viola --help` (the verb's group) — carried: Symbols / APIs. `already live`:
      layout-templates 3. A `--list` row is `<ts>  <cause>  <id>`, two spaces between fields, one line per
      logged session in log order, no header, no colour.
  13. a11y-plan §1 A11y Scope Summary, CLI verbs (the list names `revive`) — carried: Symbols / APIs. `CLI
      verbs`: a11y-plan 1.
  - The plan's ledger-note line on `matrix#v1-41` was written at phase P5 (the phase run dir's
    `note-v1-41.md`); no ledger note is owed to P7.3.
- **Coverage of new surfaces:**
  - `viola revive <name>` (the start arm) → validation mechanism✓ (the name parser; `--id` the closed shape at
    clap and a member of the log; strict-modes before either file's value is used; the recorded directory an
    existing directory) · instrumentation log✓ (`process-start` / `process-exit` with a closed `detail`,
    `run`'s spans, one `state-recovered` on the replay arm) · PII redacted✓ (no pair holds the recorded
    directory, a pid or a logged id; the canary-named directory case; CI's secret scan `success`) · tests
    unit/integ (and three live starts) · a11y ✓ (one `hint:` line last, typed exits 1 and 2; nothing written
    while the child holds the terminal) · tokens n/a (no colour)
  - `viola revive <name> --list` → validation mechanism✓ (strict-modes first; only ids of the session-id shape
    are printed; the `ts` through `escape_message`) · instrumentation n/a (by plan it opens no process log) ·
    PII n/a (ids and causes the instance's own log holds; no path) · tests unit/integ · a11y ✓ (ASCII, no ESC
    byte) · tokens n/a
  - snapshot `data.cwd` → validation mechanism✓ (read only after strict-modes; used as the spawn directory
    alone) · instrumentation n/a · PII redacted✓ (a host path in the 0600 snapshot only; on no log line, error
    body or stdout) · tests unit/integ · a11y n/a · tokens n/a
  - `viola_state::strict::check_instance` → validation n/a · instrumentation n/a · PII n/a · tests unit (five
    widened modes, `cfg(unix)`; an owner-only tree; an absent tree; a path that cannot be statted) and integ
    (two `cfg(unix)` root cases); ✗ for a widened DACL on an instance tree on Windows (the crate's Windows
    cases widen the ledger paths only) · a11y n/a · tokens n/a
  - `viola_state::replay::session_chain` → validation mechanism✓ (`read_from`'s frame bound and three counts) ·
    instrumentation n/a (it logs nothing by design) · PII n/a · tests unit/integ · a11y n/a · tokens n/a
  - `viola_agent_claude::{is_session_id, resume_args}` → validation mechanism✓ · instrumentation n/a · PII n/a ·
    tests unit · a11y n/a · tokens n/a
  - fake agent `--resume` / `--fork-session` (test-only) → validation n/a · instrumentation n/a · PII n/a ·
    tests unit/integ · a11y n/a · tokens n/a

## Deviations from intent

- **The endpoint wait after a kill.** The plan names `wait_endpoint_gone`; the kill path uses a connect probe
  (`holder_gone`). Justification: Spec claims disproved, above.
- **Cases beyond the plan's list**, all in listed files: `revive_list_refuses_state_another_user_could_write`
  (`cfg(unix)`; `--list` calls the check at a site of its own, with its own red and green reading, control C in
  `evidence/red-green.md`), `revive_list_of_a_name_never_started_is_no_session_and_opens_no_log`, and inline
  cases for the clap arguments, the `--list` rows and `role_of`.
- **The fork case stops its second life cleanly** (Ctrl-C, exit 0) where the kill-and-revive case ends through
  the drop guard as planned: a clean stop writes the start arm's coverage profile.
- **A bounded wait on the removal of the recorded directory** in
  `revive_whose_recorded_directory_is_gone_is_cwd_missing`: a directory a process still holds cannot be removed
  on Windows. That hold was assumed, not measured, and the loop has no reading of ever running; the case
  passed on `windows-2025`.
- **The name parser** is `send::parse_name`, the one the other verbs share, where the plan cites `run.rs`'s
  private copy. **The hints** are built from `viola-agent-claude`'s constants, so `src/cmd/revive.rs` holds no
  `claude` or `--resume` literal.
- **`run`'s log setup** moved into `open_wrapper_log`, shared with revive, in the listed file `src/cmd/run.rs`.
- **Step 12, the live step:** the model alias `haiku` was passed to every live child; the payload's key names
  were read through a second plugin folder of the rig's own after `--`; start 1 was stopped, not killed.
- Scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 21 · listed 21 · recorded 0
  (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 88`, 15:08:21Z, base
  `00c73fda`). The read first printed `UNPARSED 4`: `research.md` lines 171 to 174, the sweep record's four
  column-0 lines inside `## Files to modify`, added at P4 after the lists check. On the operator's word at the
  halt (`inputs#I6`: "Indent them … Whitespace only, no word changed") the four lines were indented by two
  spaces, 8 bytes, `git diff -w` empty for the file.

## Decisions & corrections

- The founder's four answers to the review card, live in the overseer dialog, relayed by the operator
  (`inputs#I5`): (1) `session-live` is not built, a CARRY on "The board: viola list" with P4 and P12; (2) three
  live `claude` starts for this chunk only, on the headless rig, each ledgered before it is made; (3) a revived
  start keeps `wheel` `driver`, cause `start`; (4) the four refusal pairs stand as the plan words them.
- The operator at this wrap (`inputs#I6`): Phase 1 only, then a resume point naming Phase 2; the route
  directions come with the resume. And the indent of the four research lines.
- The operator at the resume (`inputs#I7`): the wrap runs whole from Phase 2 to the commit; each route item
  the resume point lists goes to its real first consumer, recommended-first, every option priced, on the
  founder's words `inputs#I3` and `inputs#I5`; the two findings with no owner each get an owner proposed in the
  same dialog.
- The operator at the Phase 2 halt (`inputs#I8`, written here after the fan-out had read this report): the
  session id as a child argv value lands as fact with the founder's word owed and no ratification written now;
  the sixth panic exemption (`revive --list` opens no process log) is recorded and a log is owed with
  `--list`'s `--json`; the owed `--resume` row and the `/compact` finding ride "Paste newline ledger row";
  the Unix items split by consumer; revive's `--json` rides "CLI machine contract"; the help group is `setup`,
  beside `run`. One correction came with it: this wrap's card said the kill-and-revive case does not assert
  that the child ended. It does, through `Wrapper::kill` in `tests/support/home.rs` (the listing's row
  561-586): a wait on the recorded child's pid and start time, bounded by `WITHIN`, failing with "the child
  outlived its killed wrapper". The card had read `tests/chaos_revive.rs` alone.
- The plan's route-step items stand for P5 as listed in plan.md's last note: the `session-live` CARRY; the
  owner of the owed `--resume` ledger row; the revive causes on "Exit-cause code catalogue"; revive's `--json`
  on "CLI machine contract"; the Unix pin of a child ending with its killed wrapper (now read green on both
  Unix runners with the fake agent); the founder's wheel answer.
- Found at implement, with no owner yet: the real resume payload's five more key names against the fake
  agent's; `viola send` of `/compact` refused `no-prompt-submitted` while the command runs; on the dev host
  the bare name `claude` is 2.1.289, so a revive typed there with no `PATH` change runs an unstamped CLI.
- Guard and permission events of this session: the bash guard refused a heredoc with a file target once and
  a `cd` outside a subshell once, each re-issued once in the form it passes; one multi-edit of the fake agent
  went through an inline python heredoc before that; the write guard refused two scratch files under `target/`
  (made by the rig script `live-setup.sh` instead); the removal of `target/rev-live-911840/` was denied by the
  permission layer and not done another way.
- Sweep hazards: the key files stand one directory deeper than the registry's top level, under
  `.andromeda/registries/contracts/<master>/`, so an owner map keyed on the top-level names reads every key
  file as unowned (30 files, this report's first site count). A count of test names carrying one of the unit
  filter's tokens reads 71 where the filter selects 70: one `state_replay` integration case's name carries
  `session_chain`.
- Chosen where research was silent: test shapes from `tests/cli_instance_state.rs` (a refused start) and
  `tests/chaos_torn_append.rs` (a chaos case over a real home).

## Outcome

Acceptance criteria, each against the diff:

- (arch) verb module, the resume shapes in `viola-agent-claude` alone, no dependency — **met for this chunk's
  code, with one limit**: `"--resume"` and `"--fork-session"` stand as product literals in
  `crates/viola-agent-claude/src/lib.rs` only (basis: `grep -rn -F` over `src crates`; the other hits are the
  test-only fake agent and tests). The program name `claude` also stands at `src/cmd/verify.rs:105`, verify's
  default program, which predates the chunk and is unmoved under the preservation guard; the criterion's
  "nowhere else in product code" does not hold for that line. Unlinked to the matrix: raised for P2.
- (arch) snapshot `cwd` under `v` 1, absent without its key, none from the replay — met (`snapshot_cwd` cases;
  `state_replay`).
- (arch) after a kill and a revive the log is longer, its earlier bytes unchanged, the new `session-start`
  cause `resume` with the first id — met (`chaos_revive`, three runners).
- (arch) a `run` start unchanged but for `cwd`; first record `wheel{driver,start}`; `viola-core` untouched —
  met (the guard; the unit case on `start_state`; `chaos_revive`; the smoke's boot).
- (security) a revive over state another user could write starts no child, `strict-modes-failed` — met on Unix
  (`strict_instance` cases; the root case with control B). Windows has no widened-tree case.
- (security) a malformed `--id` is exit 2, an unknown one `no-session` before any spawn, the id one argv
  element of a direct spawn — met.
- (security) no refusal line, hint or `--list` row holds the recorded directory or a pid — met (the canary
  case; the pairs' unit table).
- (security) the snapshot is written by the revived wrapper alone; the chain read and the replay write no file
  — met (`state_replay`; the unchanged log prefix).
- (obs) one `process-exit` per refusal with its closed detail, no child start after it — met (`cli_revive`).
- (obs) one `state-recovered` line, `snapshot-replayed`, valid against the schema — met.
- (obs) G2 and schema-check exit 0 over the homes the tests leave — met on CI (the steps `success` on three
  runners); locally both read green but a local run removes each test home, so neither read these cases' lines.
- (layouts) the collision pair on a live name, `hint: viola list` last, exit 1, empty stdout — met (also read
  live).
- (layouts) each other refusal one `unable:` and one `hint:` line last, hints distinct, none naming `viola
  release` — met.
- (design) `--list` static lines, ASCII, two spaces, no ESC, empty stderr; refusals without ESC — met.
- (a11y) a passed revive writes no line of its own while the child holds the terminal — met (`chaos_revive`).
- (a11y) each refusal ends with one `hint:` line and a typed exit (1, or 2 for usage) — met.
- (tests) the kill-and-revive case kills only its own wrapper, waits for gone by pid and start time and for an
  endpoint no one answers, reads past the offset, no sleep and no retry; the second life's receipt `cwd` is the
  first directory — met, the endpoint rule being `holder_gone`'s (Deviations).
- (tests) without `--resume` the fake agent's payloads are the recorded bytes — met (`contract_fake_agent_drift`
  green inside pre-push; `fixtures` unmoved).
- (tests) `run --unit` and `pre-push` exit 0 with `"ok":true`, the CI read `verdict: green` — met:
  `ci#38061685124`.
- No capability claimed; `v1-41` advanced and not proven — as planned (`matrix.py show --chunk`: claimed 0).

Gates, by `run`, from the block's second firing on the final tree (14:55Z) and the operator pass:

- `cargo fmt --all --check` — green · exit 0
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green · exit 0
- `bash scripts/agent-run.sh run --unit` — green · exit 0 · `"ok":true` (1624 of 1624)
- `bash scripts/agent-run.sh run --unit --filter 'test(/snapshot_cwd|…|preflight_order/)'` — green (70 of 70)
- `bash scripts/agent-run.sh run --integration --filter 'binary(cli_revive) | binary(chaos_revive) |
  binary(state_replay) | binary(cli_fake_agent)'` — green (55 of 55)
- `git diff --quiet 00c73fda87ef -- crates/viola-channel …` (the preservation guard) — green · exit 0
- `bash scripts/agent-run.sh cleanup --session p-rev-smoke` — green
- `bash scripts/agent-run.sh boot --session p-rev-smoke --instance builder` — green · `"ok":true`
- `bash scripts/agent-run.sh status --session p-rev-smoke` — green · `state:"ready"`
- `bash scripts/g2-zero-panics.sh` — green · `g2: clean`
- `bash scripts/agent-run.sh schema-check` — green · 154 files, 2142 lines, no failure
- `bash scripts/agent-run.sh cleanup --session p-rev-smoke` — green · `processes_gone:true`,
  `endpoint_gone:true`
- `bash scripts/agent-run.sh pre-push` — green · `"ok":true`, `"stage":"linux-tests"`, coverage 2002 of 2002,
  no breach; fired a third time as the pass's step 1, green
- `python -X utf8 …/gate.py hygiene` (`leg = 'operator'`) — fired as written at 14:56:51Z and again at
  14:57:26Z: `hygiene: clean`, exit 0 (`evidence/operator-pass.md`)
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (`leg = 'operator'`) — fired once
  through `--operator`: green · exit 0 · history moved `00c73fda→0fad11c9` on the two remote-tracking refs, a
  fast-forward
- `python -X utf8 …/ci.py conclusion --sha HEAD --wait 1800` (`leg = 'operator'`) — fired as written: exit 0 ·
  `verdict: green` · checks 15/15 · `ci#38061685124`, attempt 1
- Smoke: the plan's smoke entries above, not re-driven. The revive itself booted three times on the real CLI
  (`evidence/live-revive.md`).

Watches: none — the entry folded no `watch:`.

Outcome basis: the operator pass ran (`0fad11c`, the only commit since the base `00c73fd`); the verdicts above
rest on its final state and on the final HEAD's CI run recorded in `evidence/operator-pass.md`. The implement
conversation is in this window; its report is the basis for what only it holds (the deviations, the census).
No source or test file changed after the block's second firing.

Process hygiene, from implement's census and re-measured here at 15:10:07Z over the host's process list: the
gate block, pre-push and the smoke session — terminated; control B's unguarded revive and its fake child —
terminated by the test's drop guard; two rehearsal hosts with the fake agent — terminated, exit 0; three live
hosts with `claude` 2.1.287 — terminated, exit 0, none killed. 0 rig hosts, 0 processes of the product build,
0 under the repository's `target/`. This wrap's code-graph refresh, fired at Setup, ended by itself (exit 0).
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 00c73fda (the parent of the oldest pre-CI commit 0fad11c9) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-agent-claude/src/lib.rs — added 77 line(s) in 2 range(s)
added: 115-146 · 643-687
- 124-135 @126 «pub fn is_session_id(id: &str) -> bool {»
  - 127-134 «id.len() == 36»
- 137-145 @139 «pub fn resume_args(id: &str, fork: bool) -> Vec<OsString> {»
  - 141-143 «if fork {»
  - 643-668 @663 «fn session_id_shape_is_thirty_six_hex_characters_with_four_hyphens(»
  - 670-686 @671 «fn resume_args_are_the_flag_the_id_and_the_fork_flag_when_forking() {»
### crates/viola-state/src/liveness.rs — added 1 line(s) in 1 range(s)
added: 91
### crates/viola-state/src/replay.rs — added 184 line(s) in 3 range(s)
added: 75-113 · 187 · 449-592
- 75-82 @77 «pub struct SessionLink {»
- 84-89 @86 «pub struct SessionChain {»
- 91-112 @93 «pub fn session_chain(instance_dir: &Path) -> Result<SessionChain, StateError> {»
  - 96-109 «for line in lines.by_ref() {»
  - 450-456 «fn link(cause: &str, id: &str) -> SessionLink {»
  - 458-473 @459 «fn session_chain_of_an_empty_or_an_absent_log_holds_no_link() {»
  - 475-500 @477 «fn session_chain_holds_every_logged_session_in_log_order_with_its_cause() {»
  - 502-526 @503 «fn session_chain_holds_no_link_for_a_session_start_whose_id_is_null() {»
  - 528-557 @530 «fn session_chain_reads_a_missing_ts_and_a_cause_that_is_no_string_as_unknown() {»
  - 559-585 @560 «fn session_chain_counts_a_torn_last_line_and_an_unknown_kind_and_links_neither() {»
  - 587-592 @588 «fn session_chain_of_a_log_that_cannot_be_read_is_an_error() {»
### crates/viola-state/src/snapshot.rs — added 39 line(s) in 3 range(s)
added: 58-61 · 177 · 307-340
  - 307-319 @308 «fn snapshot_cwd_is_written_under_v_1_and_reads_back() {»
  - 330-339 «fn snapshot_cwd_reads_as_absent_without_its_key_and_beside_an_unknown_one(»
### crates/viola-state/src/strict.rs — added 116 line(s) in 4 range(s)
added: 8 · 10-11 · 33-54 · 715-805
- 36-47 @39 «pub fn check_instance(home: &Path, instance_dir: &Path) -> Result<(), Refused> {»
  - 40-46 «check_existing(&[»
  - 716-749 @718 «fn instance_viola_wrote(root: &Path) -> (PathBuf, PathBuf) {»
  - 751-757 @752 «fn strict_instance_of_a_tree_viola_wrote_passes() {»
  - 759-767 @760 «fn strict_instance_of_a_home_with_no_instance_directory_passes() {»
  - 769-783 @772 «fn strict_instance_of_a_path_that_cannot_be_statted_is_unreadable() {»
  - 785-805 @792 «fn strict_instance_refuses_a_widened_mode(#[case] which: &str, #[case] mode: u32) {»
### crates/viola-state/tests/state_replay.rs — added 65 line(s) in 3 range(s)
added: 13-16 · 34-42 · 186-237
- 13-16 «use viola_state::replay::{»
- 187-211 @190 «fn state_replay_a_snapshot_cut_short_yields_no_cwd_and_is_left_untouched() {»
  - 196-201 «OpenOptions::new()»
  - 205-208 «assert!(»
- 213-237 @214 «fn state_replay_the_session_chain_of_a_two_session_log_is_both_in_order() {»
  - 218-227 «let read: Vec<(&str, &str)> = links»
  - 229-234 «assert!(»
### schemas/diag-line.v1.json — added 1 line(s) in 1 range(s)
added: 82
### src/bin/viola-fake-agent.rs — added 143 line(s) in 25 range(s)
added: 35-36 · 74-75 · 116-117 · 143-156 · 304-311 · 314-316 · 358-359 · 373 · 389 · 410 · 415 · 487-488 · 491-492 · 518
       561 · 737-740 · 814 · 880-882 · 911-915 · 952-953 · 1112 · 1114-1115 · 1117 · 1121 · 1126-1205
  - 143-155 @145 «fn resume_fields(&self) -> Vec<(&'static str, &str)> {»
  - 309-311 «if set.is_empty() {»
  - 1126-1134 @1127 «fn recorded_session_start() -> Vec<u8> {»
  - 1136-1172 @1137 «fn fake_resume_sets_the_source_and_the_given_id_on_the_recorded_session_start() {»
  - 1174-1193 @1175 «fn fake_resume_with_fork_session_reports_the_compiled_fork_id() {»
  - 1195-1204 @1197 «fn fake_resume_absent_leaves_the_recorded_bytes() {»
### src/cmd/hook.rs — added 1 line(s) in 1 range(s)
added: 730
### src/cmd/mod.rs — added 16 line(s) in 3 range(s)
added: 7 · 41-42 · 88-100
### src/cmd/revive.rs — new file · 526 line(s)
- 18-35 @19 «pub(crate) struct ReviveArgs {»
- 37-43 «fn parse_session_id(raw: &str) -> Result<String, String> {»
  - 38-42 «if is_session_id(raw) {»
- 45-53 @48 «enum Refusal {»
- 55-95 «impl Refusal {»
  - 56-63 @57 «const fn detail(self) -> &'static str {»
  - 65-89 @67 «fn pair(self, name: &str) -> (String, String) {»
  - 91-94 «fn write(self, name: &ViolaName) {»
- 97-110 «pub(crate) fn revive(home: &Path, args: ReviveArgs) -> anyhow::Result<ExitCode> {»
  - 98-100 «if args.list {»
  - 102-105 «let started = match preflight(home, args)? {»
  - 106-109 «match started {»
- 112-132 @115 «fn preflight_order<R, S, D>(»
  - 121-123 «if let Some(refusal) = strict_modes() {»
  - 124-126 «if let Some(refusal) = holder() {»
  - 127-130 «let session = match session()? {»
- 134-171 @135 «fn preflight(home: &Path, args: ReviveArgs) -> anyhow::Result<Result<Launch, Started>> {»
  - 139-142 «let refuse = |refusal: Refusal| {»
  - 143-158 «let passed = preflight_order(»
  - 159-162 «let ((id, _), spawn_dir) = match passed {»
  - 165-170 «Ok(Ok(Launch {»
- 173-185 @175 «fn pick_session(links: &[SessionLink], asked: Option<&str>) -> Result<String, Refusal> {»
  - 176-179 «let mut ids = links»
  - 180-183 «match asked {»
- 187-198 @189 «fn recorded_dir(recovered: &Recovered) -> Option<PathBuf> {»
  - 190-197 «match recovered {»
- 200-218 @202 «fn list(home: &Path, name: &ViolaName) -> anyhow::Result<ExitCode> {»
  - 205-208 «if check_instance(&home, &instance_dir).is_err() {»
  - 210-213 «if rows.is_empty() {»
  - 214-216 «for row in rows {»
- 220-234 @221 «fn list_rows(links: &[SessionLink]) -> Vec<String> {»
  - 222-233 «links»
- 236-242 @237 «fn cause_word(cause: &str) -> &str {»
  - 238-241 «match cause {»
- 244-526 @245 «mod tests {»
  - 299-322 «fn preflight_order_the_first_refusal_wins_and_no_later_reading_is_taken(»
  - 324-339 @326 «fn preflight_order_a_session_reading_that_fails_is_an_error_and_ends_the_readings() {»
  - 341-347 «fn link(cause: &str, id: &str) -> SessionLink {»
  - 368-379 «fn preflight_order_third_reading_picks_the_asked_or_the_newest_logged_session(»
  - 381-396 «fn snapshot(cwd: Option<&Path>) -> Recovered {»
  - 398-419 @399 «fn preflight_order_fourth_reading_takes_only_a_snapshot_whose_cwd_is_a_directory() {»
  - 446-458 «fn preflight_order_refusals_have_their_fixed_pair_and_closed_detail(»
  - 460-482 @461 «fn list_rows_are_ts_cause_and_id_two_spaces_apart_in_log_order() {»
  - 484-488 @485 «struct Line {»
  - 490-494 «fn parse(words: &[&str]) -> Result<ReviveArgs, clap::error::ErrorKind> {»
  - 496-507 @497 «fn revive_args_take_a_name_an_id_the_fork_flag_and_the_words_after_the_dashes() {»
  - 522-525 @523 «fn revive_args_refuse_as_usage(#[case] words: &[&str], #[case] kind: clap::error::ErrorKind) {»
### src/cmd/run.rs — added 135 line(s) in 29 range(s)
added: 137-148 · 150-168 · 170 · 179 · 184 · 196 · 204-210 · 212 · 215 · 222-223 · 239-240 · 253 · 261 · 267 · 275 · 281
       283-284 · 289 · 293-294 · 316 · 323 · 434-436 · 444 · 460 · 512 · 948-1011 · 1030 · 1032-1034 · 1036
- 137-147 @139 «pub(super) struct Launch {»
  - 152-159 «let launch = Launch {»
  - 160-163 «match start(home, &launch, &persistent)? {»
  - 948-974 @950 «fn first_snapshot(instance_dir: &Path, spawn_dir: &Path) -> InstanceSnapshot {»
  - 976-1000 @979 «fn snapshot_cwd_of_a_start_is_the_spawn_directory() {»
  - 1002-1010 @1004 «fn snapshot_cwd_of_a_start_in_a_directory_that_is_not_utf8_is_absent() {»
### src/main.rs — added 12 line(s) in 3 range(s)
added: 88-90 · 99-102 · 320-324
### src/run/dialog.rs — added 1 line(s) in 1 range(s)
added: 456
### src/run/snapshot.rs — added 1 line(s) in 1 range(s)
added: 71
### src/run/wheel.rs — added 1 line(s) in 1 range(s)
added: 792
### tests/chaos_revive.rs — new file · 151 line(s)
- 18-20 «use support::home::{»
- 27-33 @28 «fn past(instance_dir: &Path, offset: u64) -> Vec<Value> {»
  - 29-32 «read_from(instance_dir, offset)»
- 35-40 «fn session_starts(records: &[Value]) -> Vec<&Value> {»
  - 36-39 «records»
- 42-51 @43 «fn wait_session_start_hooks(receipt: &Path, count: usize) {»
  - 44-50 «fake::wait_for(receipt, "the SessionStart hook's receipt", |lines| {»
- 53-55 «fn canonical(path: &Path) -> PathBuf {»
- 57-151 @58 «fn revive_after_a_killed_wrapper_resumes_the_logged_session_in_the_recorded_directory(»
  - 68-73 «let first = Wrapper::boot_in(»
  - 76-78 «let logged = wait_events(&dir, "the first life's session-start", |l| {»
  - 81-84 «let first_id = session_starts(&logged)[0]["data"]["agent_session_id"]»
  - 95-99 «while session_starts(&past(&dir, offset)).is_empty() {»
  - 105-108 «assert_eq!(»
  - 117-120 «let cwds: Vec<PathBuf> = of_kind(&lines[receipted..], "cwd")»
  - 124-127 «assert_eq!(»
  - 129-133 «let listed = Running::over(»
  - 136-140 «let rows: Vec<Vec<&str>> = listed»
  - 141-144 «assert_eq!(»
  - 147-150 «assert!(»
### tests/cli_fake_agent.rs — added 61 line(s) in 1 range(s)
added: 484-544
- 484-507 @486 «fn launch_session_start(extra: &[&str]) -> (Vec<u8>, Vec<u8>) {»
  - 490-495 «let mut args = vec![»
  - 500-504 «let recorded = std::fs::read(»
- 509-518 @510 «fn resumed(recorded: &[u8], session_id: &str) -> Vec<u8> {»
- 520-531 @524 «fn fake_agent_resume_reports_the_given_session_on_the_recorded_session_start() {»
- 533-543 @536 «fn fake_agent_fork_session_reports_the_compiled_fork_id() {»
### tests/cli_revive.rs — new file · 382 line(s)
- 18-20 «use support::home::{»
- 33-34 «const NO_SESSION: &str = "unable: builder has no logged session to resume\n\»
- 35-36 «const UNKNOWN_ID: &str = "unable: builder has no logged session with that id\n\»
- 37-39 «const CWD_MISSING: &str = "unable: builder's recorded directory is missing\n\»
- 40-43 @41 «const STRICT_MODES: &str = "unable: builder's state files can be written by another user\n\»
- 45-49 @46 «fn revive(home: &TestHome, dir: &Path, args: &[&str]) -> Ran {»
- 51-53 «fn role_file(home: &Path) -> PathBuf {»
- 55-57 «fn role_lines(home: &Path) -> Vec<Value> {»
- 59-82 @61 «fn assert_refused(home: &Path, before: usize, detail: &str) {»
  - 64-67 «let exits: Vec<&Value> = added»
  - 76-81 «assert!(»
- 84-95 @86 «fn first_life(stamped: StampedHome, dir: &Path) -> Wrapper {»
  - 90-92 «wait_events(&wrapper.instance_dir(), "the session-start record", |l| {»
- 97-106 @98 «fn wait_session_start_hooks(receipt: &Path, count: usize) {»
  - 99-105 «fake::wait_for(receipt, "the SessionStart hook's receipt", |lines| {»
- 108-113 @109 «fn made_dir(home: &TestHome, name: &str) -> PathBuf {»
- 115-123 @117 «fn stopped_life(stamped: StampedHome) -> (StampedHome, PathBuf, usize) {»
- 125-135 @127 «fn remove_dir_once_free(dir: &Path) {»
  - 130-134 «while fs::remove_dir_all(dir).is_err() {»
- 137-139 «fn instance_dir(home: &TestHome) -> PathBuf {»
- 141-147 «fn starts(home: &TestHome) -> usize {»
  - 142-145 «of_kind(»
- 149-166 @150 «fn revive_on_a_live_name_prints_the_run_collision_pair(booted_wrapper: Wrapper) {»
  - 155-159 «let ran = revive(»
- 168-177 @169 «fn revive_of_a_name_never_started_is_no_session(stamped_home: StampedHome) {»
- 179-190 @180 «fn revive_with_an_id_the_log_does_not_hold_is_no_session(stamped_home: StampedHome) {»
- 192-203 @193 «fn revive_with_a_malformed_id_is_a_usage_error(stamped_home: StampedHome) {»
  - 199-202 «assert!(»
- 205-230 @208 «fn revive_whose_recorded_directory_is_gone_is_cwd_missing(stamped_home: StampedHome) {»
  - 226-229 «assert!(»
- 232-276 @235 «fn revive_of_a_snapshot_with_no_cwd_is_cwd_missing(stamped_home: StampedHome) {»
  - 240-245 «OpenOptions::new()»
  - 256-259 «let recovered: Vec<&Value> = lines[before..]»
  - 270-275 «for line in &lines[before..] {»
- 278-299 @280 «fn revive_refuses_state_another_user_could_write(stamped_home: StampedHome) {»
  - 293-298 «assert!(»
- 301-326 @302 «fn revive_list_prints_the_logged_chain(stamped_home: StampedHome) {»
  - 315-318 «assert!(»
  - 321-325 «assert_eq!(»
- 328-336 @329 «fn revive_list_of_a_name_never_started_is_no_session_and_opens_no_log(stamped_home: StampedHome) {»
- 338-358 @342 «fn revive_list_refuses_state_another_user_could_write(stamped_home: StampedHome) {»
  - 353-357 «assert_eq!(»
- 360-382 @363 «fn revive_with_fork_reports_the_fork_session(stamped_home: StampedHome) {»
  - 370-372 «let lines = wait_events(&log, "the revived session-start", |l| {»
  - 374-377 «let started: Vec<&Value> = lines[before..]»
### tests/support/home.rs — added 167 line(s) in 10 range(s)
added: 378 · 389 · 392-433 · 441 · 443 · 460 · 465 · 555-661 · 683-693 · 697
  - 392-395 @393 «pub fn boot_in(stamped: StampedHome, name: &str, dir: &Path, extra: &[&str]) -> Self {»
  - 397-430 @400 «pub fn revive(»
  - 556-559 @557 «pub fn shown(&self) -> Vec<u8> {»
  - 561-586 @564 «pub fn kill(self) -> StampedHome {»
- 589-603 @593 «pub fn claude_dir(scratch: &Path) -> PathBuf {»
  - 596-601 «if !claude.is_file() {»
- 605-613 @606 «pub fn path_with(dir: &Path) -> String {»
  - 609-612 «std::env::join_paths(dirs)»
- 615-629 @618 «pub fn revived_agent_flags(home: &Path, name: &str, trusted_root: &Path) -> Vec<OsString> {»
- 631-646 @633 «pub fn revive_command(home: &TestHome, dir: &Path, args: &[&str]) -> std::process::Command {»
  - 635-644 «command»
  - 652-655 @653 «{»
  - 656-661 @657 «{»
### tests/support/outer_pty.rs — added 10 line(s) in 1 range(s)
added: 100-109
  - 100-103 @101 «pub fn kill(&mut self) {»
  - 105-108 @106 «pub fn shown(&self) -> Vec<u8> {»
