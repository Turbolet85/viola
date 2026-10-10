# Fan-out results — wrap of 2026-10-10-viola-revive

Seven doc-agents, one batch, the prompt of `amendment-flow.md` sent verbatim with repo-relative paths.
Each return was taken from the agent's hand-back by script, never retyped; the entity probe read 0 before
and after the decode on all seven. Per document: the verdict line, each proposal's disposition with the
check that decided it, then the parsed list as returned (its leading note lines kept as YAML comments).

## architecture

Verdict: 22 proposals (D-arch-resources 18, D-arch-decisions 4). Stripped: five closing notes, not proposals — dependencies clean; the report's "snapshot data fields 11 → 12" and the contract's 14 keys count different things and architecture states no count; the key file enumerates no root test file, so expected amendment 5's "two root test files" has no site; the "one endpoint per `viola run`" sites are not falsified by the report and are left; every coordinate is a row of the report's listing.
Numbered R1 to R22 in the order of the list.
- R1 Subcommands bullet → apply · check 1 routine (Accurate this-chunk addition; plan expected amendment 1; the bullet enumerates every verb, so "Registry over-reach" does not govern)
- R2 Stack table CLI parser row → apply · dependent of R1
- R3 directory tree, `src/cmd/` → apply · routine (expected amendment 5)
- R4 directory tree, `human.rs` callers → apply · routine
- R5 fake agent options 8 → 10 → apply · routine (expected amendment 1); the count's rule is the enumerated list beside it, and both sites (here and test-plan §7, T8) move in one apply
- R6 exit 1, revive's refusals → apply · routine (expected amendment 1)
- R7 `role_of` sentence → apply · routine; read by the wrap at `src/main.rs` `role_of`: `revive` returns `Role::Other` beside `run`
- R8 `diagnostics/` producers → apply · routine
- R9 flags passed to the child → escalate · check 1, two rules with opposite verdicts: "Accurate this-chunk addition" (routine) and "Boundary widening" (escalate). The discriminating fact: the id is a value read from the instance's log and placed on the child's command line, a crossing that was not shown to the founder as a widening (`inputs#I3` shows him the recorded cwd only). Resolved with S6: applied as fact, the founder's word owed and no ratification written (the operator's answer at the Phase 2 halt, `inputs#I8`, answer 1).
- R10 Instance snapshot `cwd?` → apply · routine (expected amendment 2); part of the recorded-cwd widening the founder ratified (R19)
- R11 fields no event carries → apply · dependent of R10
- R12 Snapshot envelope, first reader → apply · routine (expected amendment 2)
- R13 owed-reader sentence → apply · dependent of R12
- R14 directory tree, `viola-state/` → apply · dependent of R12
- R15 [Database / State Store], `read_from` callers → apply · routine
- R16 Filesystem, the instance check → apply · routine; the Windows gap is named with its route owner (P5)
- R17 root wait count and `holder_gone` → apply · routine (expected amendment 5; playbook "A count in a master that carries no rule": the rule stands beside the count and both sites, here and the test-plan key file, move in one apply); check 6: with T10 it disposes the report's disproved plan claim
- R18 capability ledger, second shape with no row → apply · the founder's recorded word settles it (`inputs#I3`, answer 1); the owning entry is "Paste newline ledger row" (the operator's answer at the Phase 2 halt, `inputs#I8`, answer 3)
- R19 [PTY], the child's cwd → apply · Boundary widening: escalated and resolved on the founder's ratification on disk, given after this widening was shown to him as one (`inputs#I3`, answer 3; playbook "Boundary widening — what ratifies it"). Recorded in the sidecar as his, the overseer and the operator as relay.
- R20 Design Philosophy, no daemon → apply · routine; read by the wrap: `revive` calls `start` (the report's listing, `src/cmd/revive.rs` 97-110), the one path that binds the endpoint
- R21 [Session Liveness] → apply · routine
- R22 [CLI Conventions], no `--json` → apply · routine; the owning entry is "CLI machine contract" (`inputs#I8`, answer 6)

### Parsed list, as returned

```yaml
proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Binary, subcommands and exit codes (the "Subcommands:" bullet)
    change: >-
      Add `revive` to the subcommand list, with its shapes: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` (the start arm) and `viola revive <name> --list` (`--list` conflicts with `--id`, `--fork` and the trailing arguments); it takes no `--json`; exit 0 when the revived child's wrapper ends as `run`'s does, 1 for a preflight refusal, 2 for a clap usage error (a malformed `--id` included).
    sidecar: >-
      2026-10-10-viola-revive: registered the `revive` subcommand and its two argument shapes in Occupied Resources → Binary, subcommands and exit codes.
    rationale: >-
      Report Changes → Symbols / APIs lands the CLI verb `viola revive`; the Subcommands bullet lists `run` … `plugin install` and does not hold it (the report's own search found `revive` once in architecture, in the Snapshot envelope's owed-reader sentence).
    basis: src/cmd/revive.rs:18-35
  - detector: D-arch-resources
    severity: warning
    section: §Stack and Technologies → CLI parser row (Role column)
    change: >-
      The row's subcommand list reads `run · revive · send · wait · last · list · answer · hook · mcp · ui · verify · pause · release · link · unlink · plugin install`.
    sidecar: >-
      2026-10-10-viola-revive: the Stack table's CLI parser row names `revive` among the subcommands.
    rationale: >-
      Same verb list as the Occupied Resources Subcommands bullet, restated in the Stack table; report Changes → Symbols / APIs adds the verb and "CLI verbs: one more, `revive`" (Counts / qualifiers moved).
    basis: src/cmd/revive.rs:18-35
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: >-
      The `src/cmd/` comment's module list gains `revive` ("one module per subcommand: run, revive, send, …"), with a note that `revive.rs` holds the four-reading preflight, the `--list` rows and no `claude` / `--resume` literal (its child shapes come from `viola-agent-claude`).
    sidecar: >-
      2026-10-10-viola-revive: Project directory structure lists the new root-bin module `src/cmd/revive.rs`.
    rationale: >-
      Report Changes → Crates / modules: "new module `src/cmd/revive.rs` in the root bin"; the key file's `cmd/` line enumerates one module per subcommand and stops at `plugin`. Report Deviations: the hints are built from `viola-agent-claude`'s constants.
    basis: src/cmd/revive.rs:97-110
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: >-
      The `src/human.rs` comment's caller list gains `revive` (its refusals are `human::refuse` pairs): "called by `run`, `revive`, `verify`, `send`, `wait`, `last`, `answer`, `pause`, `release` and the `main` catch site".
    sidecar: >-
      2026-10-10-viola-revive: `human.rs`'s caller list in Project directory structure names `revive`.
    rationale: >-
      Report Changes → Symbols / APIs: each revive refusal is "one `human::refuse` pair on stderr"; the key file's `human.rs` line enumerates the calling verbs without it.
    basis: src/cmd/revive.rs:91-94
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Binary, subcommands and exit codes (the "Test-only binaries" bullet)
    change: >-
      "eight argv options" becomes ten, and the list gains the ninth and tenth: `--resume <id>` (the SessionStart fired at launch is the recorded `SessionStart.default` payload with `source` set to `resume` and `session_id` set to the id given, the trailing newline kept) and `--fork-session` (beside `--resume`, `session_id` is the compiled id `0f0e0d0c-0b0a-4908-8706-050403020100`; alone it changes nothing); without `--resume` every payload is the recorded bytes.
    sidecar: >-
      2026-10-10-viola-revive: the fake agent's option count moved 8 → 10 (`--resume <id>`, `--fork-session`).
    rationale: >-
      Report Changes → Symbols / APIs ("Fake agent (test-only), two argv options, the ninth and tenth") and Counts / qualifiers moved ("Fake agent options: 8 → 10"); the bullet states "eight argv options" and enumerates eight.
    basis: src/bin/viola-fake-agent.rs:143-155
  - detector: D-arch-resources
    severity: warning
    section: §Conventions → CLI exit codes (the `1` bullet)
    change: >-
      Add `viola revive`'s preflight refusals to exit 1, in their fixed order, a later reading never taken once an earlier one refuses: (1) `strict-modes-failed` (`unable: <name>'s state files can be written by another user`), (2) `already-live` (`run`'s own collision check and its two pairs, unchanged), (3) `no-session` (two pairs: `unable: <name> has no logged session to resume`; `unable: <name> has no logged session with that id` for an `--id` the log does not hold), (4) `cwd-missing` (`unable: <name>'s recorded directory is missing`); each is one `unable:` / `hint:` pair on stderr (texts in `src/cmd/revive.rs` `Refusal::pair`, no recorded directory, pid or logged id in any) and one `process-exit{subject:"self", exit_code:1, detail}` in `run-<name>.ndjson`; `revive --list` has two (`strict-modes-failed`, `no-session`), the stderr pair and exit 1 only.
    sidecar: >-
      2026-10-10-viola-revive: exit 1 gained revive's four preflight refusals; `process-exit` details `no-session` and `cwd-missing` are new (9 → 11 in `schemas/diag-line.v1.json`).
    rationale: >-
      Report Changes → Symbols / APIs (the preflight order and the four pairs, accepted by the founder, inputs#I5 point 4) and Schema / config (`detail` enum gains `cwd-missing` and `no-session`); the `1` bullet enumerates every exit-1 refusal with its detail and stderr pair and holds none of revive's.
    basis: src/cmd/revive.rs:55-95
  - detector: D-arch-resources
    severity: warning
    section: §Conventions → CLI exit codes (the `1` bullet, the `role_of` sentence)
    change: >-
      "`role_of` files every first word but `run`, `hook` and a leading `-` flag under `cli`" becomes "every first word but `run`, `revive`, `hook` and a leading `-` flag" — `role_of` names `revive` beside `run` (`Role::Other`), and revive's start arm logs as process `run`; the parenthetical list of `cli` verbs is unchanged.
    sidecar: >-
      2026-10-10-viola-revive: `role_of` no longer files `revive` under `cli`; the exit-1 bullet's role sentence says so.
    rationale: >-
      Report Changes → Symbols / APIs: "`role_of` in `src/main.rs` names `revive` beside `run` (`Role::Other`)"; the sentence as written puts every first word other than `run` and `hook` under `cli`.
    basis: src/main.rs
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem → `diagnostics/`
    change: >-
      The producers sentence names revive: `run-<name>.ndjson` is written by `run` and by `viola revive`'s start arm, which logs as process `run` (`ObsProcess::Run`, `run`'s detail sink) — a refused revive writes one `process-exit{subject:"self", exit_code:1}` with its closed `detail` there; `revive --list` opens no process log and writes no line.
    sidecar: >-
      2026-10-10-viola-revive: `run-<name>.ndjson` has a second producer, revive's start arm; `--list` writes no process log.
    rationale: >-
      Report Changes → Symbols / APIs: "The start arm logs as process `run` (`ObsProcess::Run`, the `run-<name>.ndjson` role file, `run`'s detail sink) … `--list` opens no process log and writes no line"; the bullet lists the role files' producers as `run`, `hook` and seven `cli` verbs.
    basis: src/cmd/revive.rs:134-171
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Claude Code integration names (the "Flag passed to the child" bullet)
    change: >-
      Flags passed to the child: `--plugin-dir <viola home>/plugin/<version>-<hash>` first on every start; a revived start launches program `claude` (`viola_agent_claude::PROGRAM`, looked up by name on the reviving process's own `PATH`, from its own current directory, by `run`'s resolver) and adds `--resume <id>` (`RESUME_FLAG`), `--fork-session` when `--fork` is given (`FORK_SESSION_FLAG`), then the words after `--` (`resume_args(id, fork)`); the id is one argv element, the newest logged one of the session-id shape or `--id` when the log holds it (`is_session_id`: 36 characters, hyphens at offsets 8, 13, 18, 23, ASCII hex elsewhere).
    sidecar: >-
      2026-10-10-viola-revive: registered the revived child's argv (`--resume <id>`, `--fork-session`) and `viola-agent-claude`'s new public items beside `--plugin-dir`.
    rationale: >-
      Report Changes → Symbols / APIs lands the child arguments and the new `viola-agent-claude` public items `PROGRAM`, `RESUME_FLAG`, `FORK_SESSION_FLAG`, `is_session_id`, `resume_args`; the bullet registers `--plugin-dir` as the one flag viola passes (the report's search: `--resume`, 0 hits in every master).
    basis: crates/viola-agent-claude/src/lib.rs:124-145
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Instance snapshot
    change: >-
      `data` gains the optional key `cwd?` (`snapshot::InstanceSnapshot.cwd: Option<String>`): the child's spawn directory, written by `start_state` into the first snapshot when it is valid UTF-8 and omitted otherwise; additive, `v` stays 1; a host path that lives only in the 0600 snapshot (on no log line, error body or stdout); its one reader is `viola revive`, which uses it as the revived child's spawn directory alone.
    sidecar: >-
      2026-10-10-viola-revive: the instance snapshot's `data` gained the optional `cwd` (the spawn directory), no `v` bump.
    rationale: >-
      Report Changes → Symbols / APIs (`InstanceSnapshot.cwd`, "omitted on write when absent; `SNAPSHOT_V` stays 1") and Schema / config ("The snapshot's `data` gains the optional key `cwd`"); the contract's `data` list does not hold it.
    basis: src/cmd/run.rs:137-147
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Snapshot envelope
    change: >-
      The "Fields no event carries" list gains `cwd`: (`endpoint`, `pid`, `started_at`, `pinned_bin`, `statusline_command`, `cli_version`, `cli_verified`, `child_pid`, `pending_dialog`, `cwd`) — a replay yields no `cwd`, so a revive over a replayed snapshot is refused `cwd-missing`.
    sidecar: >-
      2026-10-10-viola-revive: `cwd` joined the snapshot fields no event carries (absent after a replay).
    rationale: >-
      The same field list restated in the envelope paragraph; report Outcome: "snapshot `cwd` under `v` 1, absent without its key, none from the replay — met", and the plan's expected amendment 2 ("`cwd?`, a field no event carries").
    basis: tests/cli_revive.rs:232-276
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Snapshot envelope
    change: >-
      "the fallback is library code in `viola-state` and no reader takes it yet" becomes: the fallback has one product reader, `viola revive`'s third preflight reading, the first product caller of `replay::read_snapshot_or_replay` (one `state-recovered` line, `snapshot-replayed`, on the replay arm); and the paragraph registers the new `replay::{SessionLink, SessionChain, session_chain}`: one pass over `events.ndjson` through `events::read_from`, one link per `session-start` line whose `agent_session_id` is a string, with the pass's `Skipped`; it writes and logs nothing.
    sidecar: >-
      2026-10-10-viola-revive: the replay fallback has its first product reader (revive); `replay::session_chain` registered.
    rationale: >-
      Report Changes → Symbols / APIs: "`read_snapshot_or_replay` has its first product caller (revive's third reading)" and the new `viola-state` public items; the paragraph says no reader takes the fallback yet.
    basis: crates/viola-state/src/replay.rs:91-112
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Snapshot envelope
    change: >-
      The closing sentence "The first reader of the replay is owed to the working-route entry "viola revive"" becomes: the first reader of the replay landed with chunk 2026-10-10-viola-revive (revive's preflight); `read_snapshot` keeps its results and its four product callers (`src/cmd/client.rs`, `run.rs` `collision_check`, `hook.rs` twice) still read every such snapshot as absent.
    sidecar: >-
      2026-10-10-viola-revive: the Snapshot envelope's owed-reader sentence is retired; the reader landed.
    rationale: >-
      The same "no reader yet" claim worded as a debt; report Changes: revive is the first product caller, and "the four other snapshot readers … still call `read_snapshot`".
    basis: src/cmd/revive.rs:187-198
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: >-
      The `viola-state/` comment "the classified snapshot read and the log replay (`replay.rs`; no caller yet)" becomes "the classified snapshot read, the log replay and the session chain (`replay.rs`; read by `viola revive`), the instance strict-modes check (`strict::check_instance`)".
    sidecar: >-
      2026-10-10-viola-revive: Project directory structure no longer says `replay.rs` has no caller.
    rationale: >-
      The "no caller yet" claim restated in the key file's tree; report Changes: first product caller of `read_snapshot_or_replay`, new `replay::session_chain` and `strict::check_instance`.
    basis: crates/viola-state/src/replay.rs:91-112
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Established Decisions → [Database / State Store]
    change: >-
      "The one reader so far (`viola_state::events::read_from`: the wrapper's `wait` scan and `last` rebuild)" gains its new caller: `replay::session_chain` (revive's preflight and `revive --list`) also reads through `read_from` and carries the pass's `Skipped` counts; the counts are still logged nowhere and shown on no surface (`--list` prints rows only).
    sidecar: >-
      2026-10-10-viola-revive: `events::read_from` has a third use, the session chain read by revive.
    rationale: >-
      Report Changes → Symbols / APIs: `session_chain` is "one pass over `events.ndjson` through `events::read_from` … with the pass's `Skipped`; it writes and logs nothing"; the decision enumerates `read_from`'s uses as the `wait` scan and the `last` rebuild.
    basis: crates/viola-state/src/replay.rs:91-112
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem → `instances/<ViolaName>/…`
    change: >-
      Add the read-side check: `viola revive` (both the start arm and `--list`) runs `viola_state::strict::check_instance(home, instance_dir)` — the home, `instances/`, the instance directory, its `snapshot.json` and `events.ndjson`, each that exists, in that order — before either file's value is used; a refusal is exit 1 `strict-modes-failed` and no child starts. Proven on Unix (five widened modes); no widened-DACL case exists for an instance tree on Windows.
    sidecar: >-
      2026-10-10-viola-revive: registered `strict::check_instance`, the strict-modes check revive runs over an instance tree before reading it.
    rationale: >-
      Report Changes → Symbols / APIs (new public `strict::check_instance`) and Coverage of new surfaces (unit and integ on `cfg(unix)`; "✗ for a widened DACL on an instance tree on Windows"); architecture's only strict-modes sites are the stamps read and the Stack row, none for instance files.
    basis: crates/viola-state/src/strict.rs:36-47
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem (the test-only `viola-root-watch` bullet)
    change: >-
      "`Instant::now() + WITHIN` reads 23 sites in 17 files under `tests/`, as measured at chunk 2026-10-10-self-healing-state's report" becomes "26 sites in 19 files under `tests/`, as measured at chunk 2026-10-10-viola-revive's report on the Linux dev host's final tree (the three new sites in `tests/support/home.rs`, `tests/chaos_revive.rs` and `tests/cli_revive.rs`)"; and beside `wait_endpoint_gone` (the stop path, unchanged) the bullet names `holder_gone`, the wait after a killed wrapper: on Unix a connect to the path is refused or the path is absent, on Windows the pipe is not found — a killed wrapper leaves its socket file, so `wait_endpoint_gone`'s Unix rule (the file absent) can never end there.
    sidecar: >-
      2026-10-10-viola-revive: root wait count 23 sites / 17 files → 26 / 19; `holder_gone` is the endpoint wait after a kill.
    rationale: >-
      Report Changes → Counts / qualifiers moved (basis `grep -rn 'Instant::now() + WITHIN' tests`, 26 lines, 19 files, 14:53Z) and Spec claims disproved by measurement (the kill path waits on `holder_gone`; the stop path keeps the old rule).
    basis: tests/support/home.rs
  - detector: D-arch-resources
    severity: warning
    section: §Cross-cutting Patterns → Capability ledger as the single gate for CLI-specific behaviour
    change: >-
      After the LF-in-paste sentence add a second relied-on shape with no row: `viola revive` relies on `claude --resume <id>` reopening the same session and firing a SessionStart of source `resume` with that id (as measured on 2.1.287 on the Linux dev host at chunk 2026-10-10-viola-revive, `evidence/live-revive.md`, three live starts; under the fake agent on the three CI OSes otherwise); it has no ledger row and no `viola verify` probe, none is landed now by the founder's word (inputs#I3), and the row is owed on a route entry. "One shape `send` does rely on has no row yet" reads as two relied-on shapes without a row, one of `send`'s and one of `revive`'s; the seventeen rows stand and no stamp moved.
    sidecar: >-
      2026-10-10-viola-revive: `--resume <id>` is a relied-on CLI shape with no ledger row, owed on a route entry (founder, inputs#I3).
    rationale: >-
      Report Changes → Counts / qualifiers moved: "Relied-on CLI shapes with no ledger row: one more, `--resume <id>` reopening the same session with a SessionStart of source `resume` (the founder, `inputs#I3`: no row now, owed on a route entry). The seventeen rows stand; no stamp moved." The pattern names exactly one such shape.
    basis: tests/chaos_revive.rs:57-151
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [PTY]
    change: >-
      "The child's cwd is set explicitly to viola's current directory" holds for a `run` start; a revived start (`viola revive`) sets it to the instance snapshot's recorded `cwd` instead (`Launch.spawn_dir`; `run` builds its `Launch` from the current directory, revive from the snapshot), refused `cwd-missing` when that is absent or no existing directory. Program resolution and the version gate never read the recorded directory: the program is looked up on the reviving process's own `PATH` from its own current directory.
    sidecar: >-
      2026-10-10-viola-revive: [PTY] — the child's cwd has a second source, the snapshot's recorded `cwd`, for a revived start.
    rationale: >-
      Report Changes → Symbols / APIs: "The child's spawn directory is the snapshot's recorded `cwd`; program resolution and the version gate never read that directory" and "`start` takes a `Launch { name, program, args, spawn_dir }`"; the locked sentence gives viola's current directory as the only source (ratified as a boundary widening by the founder, inputs#I3; plan expected amendment 3).
    basis: src/cmd/run.rs:137-147
  - detector: D-arch-decisions
    severity: warning
    section: §Design Philosophy → "No daemon; the disk is the shared truth."
    change: >-
      "Every `viola run` owns its own local-socket endpoint, every other verb is a separate process that owns no endpoint (short-lived, except the long-running `mcp` and `ui`)" becomes: every wrapper — a `viola run`, or a `viola revive` whose preflight passed, both through the one `start` — owns its own local-socket endpoint; every other verb (and `revive --list`, and a refused revive) is a separate process that owns no endpoint (short-lived, except the long-running `mcp` and `ui`).
    sidecar: >-
      2026-10-10-viola-revive: the no-daemon principle names revive's start arm as the second way a wrapper starts.
    rationale: >-
      Report Changes → Symbols / APIs: "`start` has two product callers now (`run`, `revive`)", revive exits "when the revived child's wrapper ends as `run`'s does", and Outcome: "the snapshot is written by the revived wrapper alone". The sentence as written makes `revive` a short-lived verb with no endpoint. Inferred from the shared `start` (the report does not spell out the bind for revive); the orchestrator should confirm before applying.
    basis: src/cmd/revive.rs:97-110
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Session Liveness]
    change: >-
      Beside "A `viola run` whose `ViolaName` is `live` or `stale` refuses to start" add: `viola revive` refuses the same way with `run`'s own collision check and its two pairs (`already-live`), as the second of its four preflight readings (strict-modes → collision → logged session → recorded directory), then starts through the same `start` as `run` (`Launch { name, program, args, spawn_dir }`), so a revive that passes runs `collision_check` twice; a revived start resumes the gone instance in place — its `events.ndjson` appended to, earlier bytes unchanged, no launch replay — and its first record is `wheel{holder:"driver", cause:"start"}` like every start's (the founder, inputs#I5 point 3).
    sidecar: >-
      2026-10-10-viola-revive: [Session Liveness] records revive as the second caller of the start path and its preflight order.
    rationale: >-
      Report Changes → Symbols / APIs (preflight order; "`start` has two product callers now"; "`collision_check` is called from `start` and from revive's preflight, so a revive that passes runs it twice"; the first record) and Outcome (the log longer, earlier bytes unchanged); the decision describes the start path and its refusal for `run` alone.
    basis: src/cmd/revive.rs:112-132
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Conventions]
    change: >-
      Record the open gap: `viola revive` takes no `--json` (its refusals are the stderr pair and the typed exit only); its machine form is owed to the working-route entry "CLI machine contract".
    sidecar: >-
      2026-10-10-viola-revive: `revive` ships without `--json`; owed on "CLI machine contract".
    rationale: >-
      Report Changes → Symbols / APIs: "It takes no `--json`", and Decisions & corrections lists "revive's `--json` on "CLI machine contract"" among the route-step items; [CLI Conventions] locks "`--json` for agents" and Project Intent says new capabilities arrive as subcommands "exposed consistently on the CLI (`--json`)".
    basis: src/cmd/revive.rs:18-35

# Notes for the orchestrator (not proposals):
# - D-arch-decisions, dependencies: clean. Report Dependencies: "none added or bumped"; no crate added or removed; §Inherited Defaults needs no edit.
# - Count mismatch to check before applying the Instance snapshot edit: the report says "Snapshot data fields: 11 → 12 (`cwd`)", while architecture's Instance snapshot `data` list holds 14 keys today (15 with `cwd?`). Architecture states no count, so nothing there is stale, but the two tallies count different things.
# - The plan's expected amendment 5 names "the two root test files" for Project directory structure. Neither the key file nor the body enumerates root test files (`tests/` lists only `cmd/`, `snapshots/`, `support/`), so there is no site to amend and no proposal.
# - Not proposed, left as wording that names `viola run` as the wrapper role and is not falsified by the report: "one endpoint per `viola run`" (Stack Wrapper IPC row, [Message Broker / IPC], Occupied Resources IPC endpoints, Inherited Defaults IPC), "set by `viola run` in the child" (`VIOLA_DIR`, `VIOLA_BIN`, `PATH`, the per-session settings override) and "The `VIOLA_*` variables are wrapper-to-child plumbing set by `viola run` only" (Config management). A revived wrapper goes through the same `start`, but the report states no env or settings fact for revive, so these were not turned into proposals. If the Design Philosophy proposal is applied, these sites carry the same actor and may want the same "wrapper (`run` or a passed `revive`)" wording.
# - Every line number above is a row range from the report's "New text, by line"; where the report gives none (`src/main.rs`, `tests/support/home.rs` for the wait count) the file is named without a line. No architecture.md line number is cited.
```

## security-plan

Verdict: 8 proposals (D-security-input 7, D-security-auth 1; D-security-deps none). Nothing stripped.
Numbered S1 to S8 in the order of the list.
- S1 Child executable resolution row → apply · Boundary widening: escalated and resolved on the founder's ratification on disk, given after it was shown to him as a widening (`inputs#I3`, answer 3); recorded as his, the overseer and the operator as relay (expected amendment 6)
- S2 Threat Model, child spawning → apply the `revive` entry point · dependent of S1; its clause on `--resume <id>` applied with S6, the founder's word owed
- S3 Data classification, the snapshot's `cwd` → apply · dependent of S1 (the stored directory is the ratified widening's own value)
- S4 Own state files on read row → apply · routine (expected amendment 7); read by the wrap: `read_snapshot_or_replay` reaches `read_snapshot_classified`, so neither is caller-less
- S5 Threat Model, filesystem trust boundary → apply · dependent of S4
- S6 new row, session id as a child argv value → escalate · check 1, two rules with opposite verdicts, as R9. Expected amendment 7 asks for the row; the plan names only amendment 6 as a widening. Resolved at the Phase 2 halt: the row lands as fact and says it is not ratified; the founder is shown the crossing after this wrap and his word is recorded as his at the next (`inputs#I8`, answer 1).
- S7 CLI arguments / stdin row → apply · routine
- S8 `~/.viola/` access control row → apply · routine (expected amendment 7)

### Parsed list, as returned

```yaml
proposals:
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → Child executable resolution (`run`, `claude agents`) row"
    change: >-
      Retitle the row to name `revive` beside `run`, and replace its last sentence ("The child's cwd is set explicitly to the spawner's current directory.") with: `viola run` sets the child's cwd explicitly to the spawner's current directory and writes that directory into the first snapshot's `cwd` when it is valid UTF-8 (no `cwd` otherwise). `viola revive` launches the fixed program `claude` (`viola_agent_claude::PROGRAM`), looked up by name on the reviving process's own PATH from its own current directory by `run`'s resolver, and sets the child's cwd to the snapshot's recorded `cwd`, a second source: read only after the strict-modes check, used as the spawn directory alone, and required to be an existing directory (an absent `cwd` or a missing directory is `cwd-missing`, exit 1, no child started). Program resolution and the version gate never read that directory. A boundary widening ratified by the founder (`inputs#I3`, chunk 2026-10-10-viola-revive).
    sidecar: >-
      2026-10-10-viola-revive: Child executable resolution row - a revived child's cwd is the snapshot's recorded `cwd` (read after strict-modes, an existing directory or `cwd-missing`), no longer always the spawner's current directory; founder-ratified widening (`inputs#I3`).
    rationale: >-
      Report Changes, Symbols / APIs: "The child's spawn directory is the snapshot's recorded `cwd`; program resolution and the version gate never read that directory", and `start_state` "writes the spawn directory into the first snapshot's `cwd` when it is valid UTF-8, and no `cwd` otherwise". Coverage of new surfaces, snapshot `data.cwd`: "read only after strict-modes; used as the spawn directory alone". Expected amendment 6 names this row and records the founder's ratification as a boundary widening. The row's closing sentence is now false for one of two product callers of `start` (`run`, `revive`). The validation itself is present; the drift is the doc's claim.
    basis: "src/cmd/revive.rs:187-198 (`recorded_dir`); src/cmd/run.rs:137-147 (`Launch`, with `spawn_dir`)"
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface → Child process spawning and PATH resolution (Entry point)"
    change: >-
      Add an entry-point bullet beside `run`'s: `revive` launches `claude`, looked up by name on the reviving process's own PATH from its own current directory by `run`'s resolver, as a direct spawn with child arguments `--plugin-dir <dir>` (the wrapper's, first), `--resume <id>`, `--fork-session` when `--fork` is given, then the words after `--`; the child's spawn directory is the snapshot's recorded `cwd`, which program resolution and the version gate never read.
    sidecar: >-
      2026-10-10-viola-revive: Threat Model child-spawning vector names `revive` as a second spawner of the wrapped child (fixed program `claude` by PATH, recorded `cwd` as spawn directory).
    rationale: >-
      Duplicate site of the actor claim the primary changes: this vector enumerates who spawns the wrapped child and names only `run` ("`run` resolves the child executable ...", "`run` puts the pinned copy's folder first on the child's PATH"). Report Changes, Symbols / APIs: "`start` has two product callers now (`run`, `revive`)" and "A passed preflight launches program `claude` (looked up by name on the reviving process's own `PATH`, from its own current directory, by `run`'s resolver) with child arguments ...". Primary: the Child executable resolution row proposal. Related finding in the report (Decisions & corrections, no owner yet): on the dev host the bare name `claude` resolves to 2.1.289, an unstamped version, which the existing trust-boundary sentence ("Whatever the calling process's PATH resolves to") already covers.
    basis: "src/cmd/revive.rs:134-171 (`preflight`, building the `Launch`); crates/viola-agent-claude/src/lib.rs:137-145 (`resume_args`)"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Data classification → operational metadata (Where)"
    change: >-
      The snapshot bullet should read: Snapshots hold `pid`, `child_pid`, `agent_session_id`, `started_at` and the optional `cwd` (the start's spawn directory, an absolute host path that can carry the OS username; it lives in the 0600 snapshot only, on no log line, error body or stdout, and is omitted on write when absent).
    sidecar: >-
      2026-10-10-viola-revive: Data classification - the instance snapshot gains the optional `cwd` (a host path, 0600 snapshot only).
    rationale: >-
      Duplicate site of the snapshot-content claim the primary relies on: this bullet lists what a snapshot holds and classes the only PII as "the OS username in paths"; the snapshot now stores a path. Report Changes: `snapshot::InstanceSnapshot.cwd: Option<String>` (omitted on write when absent; `SNAPSHOT_V` stays 1); Schema / config: "The snapshot's `data` gains the optional key `cwd`"; Coverage, snapshot `data.cwd`: "a host path in the 0600 snapshot only; on no log line, error body or stdout". Primary: the Child executable resolution row proposal.
    basis: "src/cmd/run.rs:976-1000 (`snapshot_cwd_of_a_start_is_the_spawn_directory`)"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → Own state files on read row"
    change: >-
      Two sentences change. (1) The `events.ndjson` reader's callers: `viola_state::events::read_from` now serves the wrapper's `wait` scan and `last` rebuild and `viola_state::replay::session_chain`, revive's one pass that keeps one link per `session-start` line whose `agent_session_id` is a string, returns the pass's `Skipped`, and writes and logs nothing. (2) Replace "they are library code with no caller yet, so no product reader's handling of an absent, unreadable or newer snapshot changed" with: the log replay has one product caller, `viola revive`'s preflight through `read_snapshot_or_replay`, taken only after `viola_state::strict::check_instance` has passed; it reads through the same bounds and writes no file (the snapshot is written by the revived wrapper alone), and the snapshot's `cwd` it yields is used as the spawn directory alone. The four other snapshot readers (`src/cmd/client.rs`, `run.rs` `collision_check`, `hook.rs` twice) still call `read_snapshot`, so their handling of an absent, unreadable or newer snapshot is unchanged. Revive is not one of the row's interim strict-modes exceptions.
    sidecar: >-
      2026-10-10-viola-revive: Own state files on read row - the log replay has its first product caller (`viola revive`, after `check_instance`); `session_chain` is a third caller of `events::read_from`; the "no caller yet" sentence is retired.
    rationale: >-
      Report Changes, Symbols / APIs: "`read_snapshot_or_replay` has its first product caller (revive's third reading); the four other snapshot readers (`src/cmd/client.rs`, `run.rs` `collision_check`, `hook.rs` twice) still call `read_snapshot`", and `replay::{SessionLink, SessionChain, session_chain}` "(one pass over `events.ndjson` through `events::read_from` ... with the pass's `Skipped`; it writes and logs nothing)". Coverage: `session_chain` validation is "`read_from`'s frame bound and three counts"; Outcome: "the chain read and the replay write no file - met". Expected amendment 7 names this row. The mandated validation (`MAX_FRAME` line cap, strict-modes before the read) is present; the row's "no caller yet" claim and its two-caller parenthetical are stale. One limit for the orchestrator: the report states the caller of `read_snapshot_or_replay` only; it does not say whether `viola_state::snapshot::read_snapshot_classified` is reached on that path, so the "no caller yet" wording for that one function needs the orchestrator's own read before it is kept or dropped.
    basis: "crates/viola-state/src/replay.rs:91-112 (`session_chain`); src/cmd/revive.rs:112-132 (`preflight_order`)"
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface → Filesystem state under `~/.viola/` (Trust boundary)"
    change: >-
      Replace "stands as library code; no product reader takes it yet" with: the replay of the log for a snapshot that fails to parse, or carries a newer `v`, has one product reader, `viola revive`'s preflight (`read_snapshot_or_replay`, after the strict-modes check on the instance tree); no other product reader takes it (Standard Contracts: Snapshot envelope).
    sidecar: >-
      2026-10-10-viola-revive: Threat Model filesystem trust boundary - the log replay is no longer caller-less; `viola revive` is its one product reader.
    rationale: >-
      Second occurrence of the claim the Own-state-files primary retires, worded differently ("no product reader takes it yet"). Report Changes, Symbols / APIs: "`read_snapshot_or_replay` has its first product caller (revive's third reading)". Expected amendment 7 lists "§Threat Model Summary (the replay has a caller)" with the search hit `no caller`: security-plan 1; this second wording carries no `no caller` token, so a single-site apply on the row would leave it standing. Primary: the Own state files on read row proposal.
    basis: "src/cmd/revive.rs:134-171 (`preflight`)"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → new row: Session id as a child argv value (`viola revive`)"
    change: >-
      Add a row. What to validate: the id handed to the child after `--resume` is the newest logged `agent_session_id` of the session-id shape, or the `--id` value when the instance's log holds it (an `--id` the log does not hold, or a log with no session, is `no-session`: exit 1 before any spawn). The shape is closed, `viola_agent_claude::is_session_id`: 36 characters, hyphens at offsets 8, 13, 18 and 23, ASCII hex elsewhere; a malformed `--id` is a clap usage error (exit 2). The id is one argv element of a direct spawn, never a shell, built only by `viola_agent_claude::resume_args(id, fork)` (`--resume <id>`, then `--fork-session` when `--fork` is given), after the wrapper's `--plugin-dir <dir>` and before the words after `--`. `revive --list` prints only ids of that shape, its `ts` through `escape_message`. No refusal line or hint holds the recorded directory, a pid or a logged id. Stack reference: `viola-agent-claude` `is_session_id` / `resume_args` (the only product home of the `--resume` and `--fork-session` literals); `src/cmd/revive.rs` `parse_session_id` / `pick_session`; `viola_state::replay::session_chain`.
    sidecar: >-
      2026-10-10-viola-revive: new Input Validation row - the logged or `--id` session id as a `claude --resume` argv value: closed 36-character shape, log membership, one argv element of a direct spawn.
    rationale: >-
      New external-input surface with no §Input Validation row: a value that entered through hook stdin, was stored in `events.ndjson` and is now deserialized back and placed on a child's command line. Report Changes, Symbols / APIs: `is_session_id` "(36 characters, hyphens at offsets 8, 13, 18, 23, ASCII hex elsewhere)", `resume_args(id, fork)`, "The id is the newest logged one of the session-id shape, or `--id` when the log holds it"; exit 2 for "a malformed `--id`". Outcome (security): "a malformed `--id` is exit 2, an unknown one `no-session` before any spawn, the id one argv element of a direct spawn - met". Expected amendment 7 asks for "a row for the session id as an argv value" (`--resume`: 0 hits in every master). The validation is present in the report; §Input Validation has no row that mandates it, so the boundary is unrecorded.
    basis: "crates/viola-agent-claude/src/lib.rs:124-135 (`is_session_id`); crates/viola-agent-claude/src/lib.rs:137-145 (`resume_args`); src/cmd/revive.rs:37-43 (`parse_session_id`); src/cmd/revive.rs:173-185 (`pick_session`)"
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → CLI arguments / stdin row"
    change: >-
      Add after the `pause` / `release` arguments sentence: **`revive` arguments:** `<name>` through `ViolaName::try_new` (the verbs' shared name parser, `send::parse_name`); `--id <ID>` through a clap value parser that accepts only the session-id shape (anything else is a usage error, exit 2); `--list` conflicts with `--id`, `--fork` and the trailing arguments; the words after `--` go to the child as argv elements of a direct spawn; the verb takes no `--json`. Both arms run the strict-modes check on the instance tree (`viola_state::strict::check_instance`) before any snapshot or log value is used, so `viola revive` is not among this row's interim gaps.
    sidecar: >-
      2026-10-10-viola-revive: CLI arguments / stdin row - `viola revive` arguments (`<name>` by the shared name parser, `--id` a closed-shape clap parser, `--list` conflicts, trailing child words).
    rationale: >-
      The row enumerates each verb's argument validation (`wait` / `last`, `answer`, `pause` / `release`) and carries no entry for the new verb. Report Changes, Symbols / APIs: "CLI verb, new: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name> --list` (`--list` conflicts with `--id`, `--fork` and the trailing arguments) ... exit 2 for a clap usage error, a malformed `--id` included. It takes no `--json`". Coverage of new surfaces, start arm: "the name parser; `--id` the closed shape at clap and a member of the log; strict-modes before either file's value is used". Deviations: "The name parser is `send::parse_name`, the one the other verbs share". Names go through the mandated `ViolaName::try_new` path; nothing is unvalidated, the row is incomplete.
    basis: "src/cmd/revive.rs:18-35 (`ReviveArgs`); src/cmd/revive.rs:37-43 (`parse_session_id`)"
  - detector: D-security-auth
    severity: warning
    section: "§Authentication & Authorization → `~/.viola/` access control row (Strict-modes check)"
    change: >-
      Add `viola revive` to the entry-point set and to the failure outcomes: `viola revive` runs `viola_state::strict::check_instance(home, instance_dir)` as the first reading of its start arm's preflight and at a site of its own in `--list`, over the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists, in that order, before any value of either file is used; a refusal is `strict-modes-failed`: exit 1, one `unable:` / `hint:` pair on stderr, no child started, and on the start arm one `process-exit{subject:"self", exit_code:1, detail:"strict-modes-failed"}` line in `run-<name>.ndjson` (`--list` opens no process log and writes no line). It is not one of the dated interim gaps. The check is pinned by `cfg(unix)` cases; no Windows case widens the DACL of an instance tree yet.
    sidecar: >-
      2026-10-10-viola-revive: `~/.viola/` access control row - `viola revive` joins the strict-modes entry points through `strict::check_instance` (home, `instances/`, instance dir, `snapshot.json`, `events.ndjson`); refusal `strict-modes-failed`, exit 1.
    rationale: >-
      No off-spec auth library or secret source: the report adds no dependency, no token, key, cookie or credential, and revive's access control is the plan's own OS-level check, so the flow matches §Authentication and §Secret Management. The drift is the row's closed list of entry points ("at the start of `run`, `ui`, `mcp`, every `hook <event>` ..., and every CLI client verb that resolves an instance's `endpoint`") and of outcomes, neither of which names the new reader of `snapshot.json`. Report Changes, Symbols / APIs: `strict::check_instance(home, instance_dir)` "(the home, `instances/`, the instance directory, its `snapshot.json` and `events.ndjson`, each that exists, in that order)"; preflight order "(1) `strict-modes-failed`"; "`--list` opens no process log and writes no line: its two refusals (`strict-modes-failed`, `no-session`) are the stderr pair and exit 1 only". Outcome (security): "a revive over state another user could write starts no child, `strict-modes-failed` - met on Unix ... Windows has no widened-tree case"; Coverage marks the Windows widened-DACL instance-tree case as missing. Expected amendment 7 names this row (`Strict-modes check`: security-plan 1). No other occurrence of a claim is retired by this addition, so it carries no dependent proposal.
    basis: "crates/viola-state/src/strict.rs:36-47 (`check_instance`); src/cmd/revive.rs:200-218 (`list`); src/cmd/revive.rs:112-132 (`preflight_order`)"
```

## design-system

Verdict: 0 proposals. Stripped: a note that D-design-tokens found every `tokens` flag `n/a` and no `hardcoded` flag, and that the report's expected amendment 11 names content the document does not hold, outside this detector.
- Expected amendment 11 → raised by the orchestrator (check 5), routine: the report substantiates it (Symbols / APIs; the pairs' texts at the listing row `src/cmd/revive.rs` 65-89). Sites: cli pattern 2 (the refusals and hints), pattern 5 (a `revive` bullet), the exit-1 phraseology row, the stderr rule's fixed-message clause, the detail-code sentence, Navigation Pattern (the verb list and the help group, the group `setup`, beside `run`, on the operator's answer, `inputs#I8`, answer 7). Two more sites came from the cascade sweep: the expression table's cli 0.0 row and the no-colour ban.

### Parsed list, as returned

```yaml
proposals: []
# D-design-tokens: no drift. The report's Coverage of new surfaces lists seven surfaces; every `tokens` flag is `n/a`
# (`viola revive <name>` start arm: "tokens n/a (no colour)"; `--list`: "tokens n/a"; snapshot `data.cwd`,
# `strict::check_instance`, `replay::session_chain`, `is_session_id` / `resume_args`, fake agent options: "tokens n/a").
# No flag reads `hardcoded✗`. The Outcome's design criterion agrees: "`--list` static lines, ASCII, two spaces, no ESC,
# empty stderr; refusals without ESC — met", which matches §Spacing's CLI rule (two spaces between columns) and the
# §Surface: cli colour bans.
#
# Outside this detector, not proposed here: the report's Expected amendments item 11 names design-system
# §Surface: cli → Component Patterns (the revive refusals and hints, the exit-1 phraseology row) and → Navigation
# Pattern (the verb `revive` and its help group). That is content the doc does not yet hold (the cli verb list and
# the `viola --help` groups name no `revive`; the exit-1 row and the start-refusal list name only `viola run` and
# `viola verify` refusals), but it is not a token-usage violation, so D-design-tokens yields no proposal for it.
# An empty list here means "no token drift", not "design-system owes nothing this chunk".
```

## layout-templates

Verdict: 6 proposals (D-layout-surface). Stripped: a note that `revive` has 0 hits in the document, that the report names neither the help group nor the hint texts, and which sections were swept and left.
Numbered L1 to L6 in the order of the list.
- L1 Output structure, `viola revive` → apply · routine (expected amendment 12); hint texts from the listing row `src/cmd/revive.rs` 65-89
- L2 Primary screens → apply · dependent of L1
- L3 `viola --help` group → apply · `setup`, beside `run`: the operator's answer (`inputs#I8`, answer 7; the report names none)
- L4 Line form, fixed-message exceptions → apply · dependent of L1
- L5 `--json` of every data verb → apply · routine; states the fact (no `--json`), the owner "CLI machine contract" (`inputs#I8`, answer 6)
- L6 Expression level → apply · dependent of L1

### Parsed list, as returned

```yaml
# drift-detector: layout-templates · chunk 2026-10-10-viola-revive · detector D-layout-surface
# Measured: `revive` has 0 hits in .andromeda/layout-templates.md; `already live` has 3 (the report's own count, Expected amendments 12).
# Not measured (the report gives neither, and no source file was opened): the `--help` group that holds `revive`, and the four `hint:` texts.
# Swept and left standing (still true for revive, no proposal): §Surface: cli → Component — Primary content block 2 → Hint line
#   (`unable: <name> is already live` takes `hint: viola list`; no hint names `viola release`), → Component — Header / banner
#   ("`viola run` prints nothing"), → Component — Footer / terminator, and the web-spa IA notes' verb list.
proposals:
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Output structure — `viola revive` (new subsection, after Output structure — `viola run`)"
    change: "Add a `viola revive` output structure: (a) `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` that passes preflight prints nothing of its own while the revived child holds the terminal, as `viola run`; (b) `viola revive <name> --list` prints on stdout one line per logged session in log order, `<ts>  <cause>  <id>`, two spaces between fields, no header, no colour, ASCII, empty stderr; (c) each refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr, empty stdout, no path, pid or logged id: `unable: <name> is already live` / `hint: viola list` (run's collision pair), `unable: <name> has no logged session to resume`, `unable: <name> has no logged session with that id`, `unable: <name>'s recorded directory is missing`, `unable: <name>'s state files can be written by another user`, hints distinct and none naming `viola release`; `--list` has two of them (state files, no logged session); a malformed `--id` is clap's usage error, exit 2; the verb takes no `--json`."
    sidecar: "2026-10-10 · 2026-10-10-viola-revive · D-layout-surface · added §Surface: cli → Output structure — `viola revive` (`--list` rows, the exit-1 refusal pairs, silent passthrough on a passed start)."
    rationale: "Report Changes → Symbols / APIs adds the CLI verb `viola revive` with a `--list` arm and four preflight refusals, each a `human::refuse` pair on stderr; Expected amendments 12 names this exact section and gives the row shape (`<ts>  <cause>  <id>`, two spaces, log order, no header, no colour). The document holds no `revive` wireframe (0 hits), so the new user-facing surface is undocumented. The `unable:` wordings are the test constants the report lists (worded for `builder`); the `hint:` texts are not in the report — the orchestrator reads them from `Refusal::pair`."
    basis: "tests/cli_revive.rs:33-43 (the four `unable:` constants) · src/cmd/revive.rs:65-89 (`Refusal::pair`, hint texts) · src/cmd/revive.rs:220-234 (`list_rows`)"
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Primary screens (commands)"
    change: "Add a bullet: `viola revive <name>` resumes a dead instance in place (its newest logged session, in the recorded directory) and is passthrough like `viola run`, printing nothing while the child runs; `viola revive <name> --list` prints the instance's logged sessions, one static line each."
    sidecar: "2026-10-10 · 2026-10-10-viola-revive · D-layout-surface · §Surface: cli → Primary screens (commands) names `viola revive` and its `--list` arm."
    rationale: "The command list is the surface inventory the detector's invariant points at; it enumerates every verb and stops at `run`, `--help` and `ui`. Report Changes → Symbols / APIs: the verb is new this chunk; Counts / qualifiers moved: `CLI verbs: one more, revive`."
    basis: "src/cmd/revive.rs:18-35 (`ReviveArgs`)"
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Output structure — `viola --help`"
    change: "The grouped verb table names `revive` in its help group. The report does not say which of board / traffic / wheel / handoff / setup holds it — the orchestrator reads the group from the binary's help definition before applying."
    sidecar: "2026-10-10 · 2026-10-10-viola-revive · D-layout-surface · §Surface: cli → Output structure — `viola --help` lists `revive` in its verb group."
    rationale: "The wireframe's five group lines are a closed list of verbs and omit the new one. Expected amendments 12 names this site (`viola --help` (the verb's group)); Expected amendments 11 pairs it with design-system's Navigation Pattern (the verb and its help group). No group name appears anywhere in the report."
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Component — Primary content block 2: refusal lines and the `unable` column → Line form (stderr)"
    change: "The fixed-message exceptions are the exit-1 start refusals of `viola run` and of `viola revive` (the shared `unable: <name> is already live` pair plus revive's no-logged-session, unknown-id, recorded-directory-missing and state-files-writable pairs; `--list` takes two of them) and `viola verify`'s exit-1 refusals — each an `unable: <text>` line and its own `hint:` line with no path or pid."
    sidecar: "2026-10-10 · 2026-10-10-viola-revive · D-layout-surface · Line form's fixed-message exceptions now name `viola revive`'s exit-1 refusal pairs beside run's and verify's."
    rationale: "This sentence restates, as a closed list, which verbs print the `unable: <text>` form: run and verify only. Report Changes → Symbols / APIs: revive's preflight refuses with four details (`strict-modes-failed`, `already-live`, `no-session` with two pairs, `cwd-missing`), each one stderr pair and exit 1; Outcome (layouts): each refusal one `unable:` and one `hint:` line last. Left as is, the list contradicts the new Output structure block. One of the three `already live` sites the report counts."
    basis: "tests/cli_revive.rs:33-43 · src/cmd/revive.rs:45-53 (`enum Refusal`)"
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Component — Primary navigation (verb structure) → Global flag `--home`"
    change: "`--json` is the machine view of every data verb except `viola revive`, which takes no `--json` (its `--list` rows are the human form only)."
    sidecar: "2026-10-10 · 2026-10-10-viola-revive · D-layout-surface · Primary navigation notes that `viola revive` takes no `--json`."
    rationale: "The bullet claims `--json` for every data verb. Report Changes → Symbols / APIs: `It takes no --json`, while `--list` prints data rows on stdout; Decisions & corrections keeps revive's `--json` as an open route item on `CLI machine contract`. With the `--list` structure documented, the universal claim no longer holds as worded. Lower confidence than the others: it turns on reading `--list` as a data verb."
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Expression level (this surface)"
    change: "0.0 for `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` and the `viola run` / `viola revive` passthrough: no colour, no non-ASCII glyph and no cursor control."
    sidecar: "2026-10-10 · 2026-10-10-viola-revive · D-layout-surface · Expression level's 0.0 list names the `viola revive` passthrough beside `viola run`'s."
    rationale: "The 0.0 list names `viola run` as the one passthrough. Report Coverage of new surfaces: the start arm writes nothing while the child holds the terminal, tokens n/a (no colour); Outcome (a11y): a passed revive writes no line of its own while the child holds the terminal. A second passthrough verb exists and the list omits it."
    dependent-of: D-layout-surface
```

## test-plan

Verdict: 13 proposals (D-tests-coverage; D-tests-framework and D-tests-obs-harness clean). Stripped: three opening notes with those two clean verdicts and the count of real violations.
Numbered T1 to T13 in the order of the list.
- T1 §4 viola-state, `check_instance` and the Windows gap → apply · check 1 routine by "Sequencing deferral": the missing widened-DACL case gets its owner as a route annotation at P5 (a CARRY on "Home and code-bearing file integrity")
- T2 §6 Exit-cause matrix → apply · routine (expected amendment 9); the owners "Exit-cause code catalogue" (the founder's 2026-10-01 CARRY) and "CLI machine contract" (`inputs#I8`, answer 6)
- T3 §5 strict-modes bullet → apply · dependent of T2
- T4 §4 Snapshot envelope bullet → apply · routine (expected amendment 9)
- T5 §1 chaos-test trigger → apply · dependent of T4
- T6 §6 Scenario E5, as landed → apply · dependent of T4
- T7 §7 Fake agent, the payload → apply · routine (expected amendment 8; the operator's word, `inputs#I3`, answer 2)
- T8 §7 Fake agent, Modes 8 → 10 → apply · dependent of T7; one apply with R5
- T9 key file, root wait count → apply · routine (expected amendment 9); one apply with R17
- T10 key file, the kill path's `holder_gone` → apply · routine; check 6: disposes the disproved plan claim
- T11 key file, `cleanup` after a force-kill → reject the body edit · check 4: what the harness reads after a force-killed wrapper on Unix is an inference, and the report states no harness command changed. The wrap read `crates/viola-e2e/src/harness/cleanup.rs` (`endpoint_gone` is one of the three fields `ok` needs) and took no reading of a run. Routed to P5 as a CARRY carrying a labelled hypothesis.
- T12 §1 coverage scope, CLI verbs → apply · routine
- T13 §1 surfaces, cli → apply · dependent of T12

### Parsed list, as returned

```yaml
# D-tests-framework: clean - the report's gates run through `scripts/agent-run.sh run --unit` / `--integration --filter` / `pre-push` (nextest via the harness), fmt and clippy as specified; Dependencies: none added or bumped.
# D-tests-obs-harness: clean - the report states no agent-run script, harness command, CI step, status shape, event kind or process value changed; the two new `process-exit` `detail` values are not enumerated in test-plan §3 Log format or in obs-plan §3's key files, so §3 and obs-plan §3 still agree.
# D-tests-coverage: one real violation (first proposal); the rest are test-plan statements of what is covered / owed that the report's Changes retire (its Expected amendments 8 and 9 among them), filed under the nearest detector.
proposals:
  - detector: D-tests-coverage
    severity: escalate
    section: §4 Unit Test Strategy → What unit tests cover → viola-state (after the Unix and Windows strict-modes bullets)
    change: >-
      Add a bullet: the instance check `strict::check_instance(home, instance_dir)` reads the home, `instances/`, the instance directory, its `snapshot.json` and `events.ndjson`, each that exists, in that order. Unit cases `strict_instance_*`: five widened modes (`cfg(unix)`), a tree viola wrote passes, a home with no instance directory passes, a path that cannot be statted is unreadable; two `cfg(unix)` root cases in `tests/cli_revive.rs` hold the refusal on `viola revive` and on `viola revive --list`. Not covered: a widened DACL on an instance tree on Windows (the crate's Windows cases widen the ledger paths only) - owed.
    sidecar: >-
      2026-10-10-viola-revive: §4 viola-state records `strict::check_instance` as landed and names the missing Windows widened-DACL instance-tree case as owed.
    rationale: >-
      The one new path the report itself flags uncovered: Coverage of new surfaces, `viola_state::strict::check_instance` - "tests unit ... and integ (two `cfg(unix)` root cases); [fail] for a widened DACL on an instance tree on Windows", and Outcome (security) "met on Unix ... Windows has no widened-tree case". Test-plan §1 (filesystem trigger: "group/world-writable home or instance dir → refused ... Windows DACL owner / ACE cases where the runner allows") and the multi-os-compat trigger (OS-branch tests cover "Windows DACL / strict-modes vs Unix modes") mandate it. Escalated because a body edit only records the gap: the test needs an owner (the resume point lists "a widened DACL on an instance tree" as not measured, no owner).
    basis: crates/viola-state/src/strict.rs:751-805 (the unit cases), crates/viola-state/src/strict.rs:36-47, tests/cli_revive.rs:278-299, tests/cli_revive.rs:338-358
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy → Non-path suites → Exit-cause matrix
    change: >-
      Add a third list, "Exit 1 (`revive` refuses to start) causes", in preflight order, a later reading not taken once an earlier one refuses: `strict-modes-failed` (state another user could write), `already-live` (`run`'s own collision check and its two landed pairs, unchanged; `hint: viola list` last on a live name), `no-session` (two pairs: no logged session; an `--id` the log does not hold), `cwd-missing` (the recorded directory gone, or a snapshot with no `cwd`). As landed these are cases of root `tests/cli_revive.rs`, not rows of `cross_exit_causes.rs`: each asserts exit 1, one `process-exit` line (`subject` `self`, `exit_code` 1) with its closed `detail` in `run-<name>.ndjson`, one `unable:` line and one `hint:` line last with distinct hints, none naming `viola release`, and no `--json` document, because `revive` takes no `--json`; `--list`'s two refusals (`strict-modes-failed`, `no-session`) are the stderr pair and exit 1 with no log line; a malformed `--id` is the clap usage exit 2. Joining the rstest table and a `--json` document are route items (the plan names "Exit-cause code catalogue" and "CLI machine contract").
    sidecar: >-
      2026-10-10-viola-revive: §6 Exit-cause matrix gains the `viola revive` exit-1 causes as landed in `tests/cli_revive.rs`, outside `cross_exit_causes.rs` and without a `--json` document.
    rationale: >-
      Report Expected amendment 9 ("§6 Exit-cause matrix (the revive causes)"). Changes → Symbols / APIs adds four preflight refusals at exit 1 and "It takes no `--json`"; Schema / config adds `cwd-missing` and `no-session` to the `process-exit` `detail` enum (9 → 11). The matrix mandates one negative test per exit-1 cause in the `cross_exit_causes.rs` table, each asserting the `--json` document; the new causes are tested at another site and cannot assert a `--json` document. Outcome: (obs) "one `process-exit` per refusal with its closed detail" and (layouts) hints distinct - met in `cli_revive`. Decisions & corrections lists the revive causes on "Exit-cause code catalogue" and revive's `--json` on "CLI machine contract" as the plan's route items.
    basis: tests/cli_revive.rs:149-299, src/cmd/revive.rs:45-95, src/cmd/revive.rs:112-132
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns covered → On-disk (the strict-modes refusals bullet)
    change: >-
      "strict-modes refusals: exit 1 for `run`/`ui`/`mcp`, hook 0 with no output, CLI 21" becomes: exit 1 for `run`/`ui`/`mcp` and for `viola revive` (its start arm with one `process-exit` of detail `strict-modes-failed`; `--list` with the stderr pair alone, no log line), hook 0 with no output, CLI 21 (revive apart).
    sidecar: >-
      2026-10-10-viola-revive: §5 On-disk strict-modes bullet names `viola revive` (both arms) as an exit-1 refusal.
    rationale: >-
      Second occurrence of the claim the matrix proposal retires (exit 1 on strict-modes is `run`/`ui`/`mcp` only, every CLI verb 21). Changes → Symbols / APIs: revive's first preflight reading is `strict-modes-failed`, exit 1; `--list`'s two refusals "are the stderr pair and exit 1 only".
    basis: tests/cli_revive.rs:278-299, tests/cli_revive.rs:338-358
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover → viola-state (the Snapshot envelope bullet)
    change: >-
      Replace "Library code: no product reader takes the replay yet." with: `viola revive` is the read-or-replay function's first product caller (`read_snapshot_or_replay`, revive's third preflight reading; its one `state-recovered` line on the replay arm is read in `tests/cli_revive.rs`); the four other snapshot readers (`src/cmd/client.rs`, `run.rs` `collision_check`, `hook.rs` twice) still call `read_snapshot`.
    sidecar: >-
      2026-10-10-viola-revive: §4 viola-state - the replay has its first product reader, `viola revive`; the other four snapshot readers still call `read_snapshot`.
    rationale: >-
      Report Expected amendment 9 ("§4 viola-state (a product reader takes the replay)"). Changes → Symbols / APIs: "`read_snapshot_or_replay` has its first product caller (revive's third reading); the four other snapshot readers ... still call `read_snapshot`". Outcome (obs): "one `state-recovered` line, `snapshot-replayed`, valid against the schema - met".
    basis: tests/cli_revive.rs:232-276
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → Coverage triggers → chaos-test (the corrupt `snapshot.json` bullet)
    change: >-
      Replace "and which reader takes it is owed, first to the route entry "viola revive"" with: and `viola revive` is its first product reader (chunk 2026-10-10-viola-revive); the replay gives no `cwd`, a field no event carries.
    sidecar: >-
      2026-10-10-viola-revive: §1 chaos-test trigger - the replay's reader is no longer owed; `viola revive` takes it.
    rationale: >-
      Same retired claim as the §4 Snapshot envelope bullet, restated in the trigger list. Changes → Symbols / APIs (first product caller); Outcome (arch): snapshot `cwd` "none from the replay - met (`snapshot_cwd` cases; `state_replay`)".
    basis: crates/viola-state/tests/state_replay.rs:187-211
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy → Scenario E5 - snapshot corruption → replay recovery (the "As landed" paragraph)
    change: >-
      Replace "The replay stands as library code, covered below the surface by `crates/viola-state/tests/state_replay.rs`. The scenario needs `link` ..., a budget pause ..., `list --json` ... and a reader that takes the replay (first "viola revive")." with: the replay has one product reader, `viola revive` (chunk 2026-10-10-viola-revive), and stays covered below the surface by `crates/viola-state/tests/state_replay.rs`; `run`'s `collision_check` still calls `read_snapshot`, so step 3's `viola run builder` does not take it. The scenario still needs `link` (the route entry "Session links"), a budget pause ("Budget governor") and `list --json` ("The board: viola list"). Beside the two signals already set against what landed: a replayed snapshot carries no `cwd`, revive's fourth reading takes only a snapshot whose `cwd` is a directory (`cwd-missing` otherwise), and a revived start's first record is `wheel{holder:"driver", cause:"start"}` too.
    sidecar: >-
      2026-10-10-viola-revive: §6 E5 "As landed" - the replay's reader exists (`viola revive`); the scenario stays owed for `link`, the budget pause and `list --json`, and `run` still reads `read_snapshot`.
    rationale: >-
      Third occurrence of the "no reader takes the replay yet" claim. Changes → Symbols / APIs: first product caller; `collision_check` still calls `read_snapshot`; "A revived start's first record is `wheel{holder:\"driver\", cause:\"start\"}`, as every start's (the founder ... `inputs#I5`, point 3)"; preflight (4) `cwd-missing`. The scenario's "owed, not built" heading stays true: the report claims no E5 build.
    basis: tests/cli_revive.rs:232-276, src/cmd/revive.rs:187-198, crates/viola-state/tests/state_replay.rs:187-211
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fake agent (the hook-commands bullet, the payload on stdin)
    change: >-
      Replace "for UserPromptSubmit only the top-level `prompt` key is set (...), and no other payload is built" with: for UserPromptSubmit only the top-level `prompt` key is set (the object re-serialised, the fixture's trailing newline kept, so the recorded prompt yields the recorded bytes); under `--resume <id>` the SessionStart fired at launch is the recorded `SessionStart.default` payload with `source` set to `resume` and `session_id` set to the id given - with `--fork-session` beside it, to the compiled id `0f0e0d0c-0b0a-4908-8706-050403020100` - the trailing newline kept; no other payload is built, and without `--resume` every payload is the recorded bytes (`--fork-session` alone changes nothing).
    sidecar: >-
      2026-10-10-viola-revive: §7 Fake agent - under `--resume` the launch SessionStart carries `source` `resume` and the given (or, with `--fork-session`, the compiled) `session_id`; without it every payload stays the recorded bytes.
    rationale: >-
      Report Expected amendment 8 ("§7 Fake agent (under `--resume` the fake agent sets `source` and `session_id`; the operator, `inputs#I3`)"). Changes → Symbols / APIs, Fake agent bullet: "Its helper `payload` now sets the fields its caller names (it set `prompt` alone)". Coverage: fake agent `--resume` / `--fork-session` tests unit/integ; Outcome (tests): "without `--resume` the fake agent's payloads are the recorded bytes - met (`contract_fake_agent_drift` green inside pre-push)".
    basis: src/bin/viola-fake-agent.rs:143-155, src/bin/viola-fake-agent.rs:1136-1204, tests/cli_fake_agent.rs:520-543
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fake agent (the Modes bullet)
    change: >-
      "eight argv options (no env) for verify's four interactive runs and their tests" becomes "ten argv options (no env): eight for verify's four interactive runs and their tests" with the list unchanged, then: and two for `viola revive`'s tests, the ninth and tenth, `--resume <id>` (the launch SessionStart reports source `resume` and that id) and `--fork-session` (beside `--resume`, the compiled fork id; alone it changes nothing). Beside "no shape is invented before a recorded fixture" add: the `--resume` SessionStart is set on the operator's word (`inputs#I3`) with no recorded fixture of source `resume`; a live reading on `claude` 2.1.287 holds 10 key names where the rewritten payload holds 5 (`cwd`, `hook_event_name`, `session_id`, `source`, `transcript_path`), the five more being `context_tokens`, `estimated_cache_write_usd`, `prompt_cache_likely_expired`, `scratchpad_dir`, `seconds_since_last_response`; no reading became a fixture or a ledger row.
    sidecar: >-
      2026-10-10-viola-revive: §7 Fake agent Modes - eight argv options → ten (`--resume <id>`, `--fork-session`); the resume SessionStart's shape is set without a recorded fixture, the live 10-key reading noted.
    rationale: >-
      Second site of the fake agent's surface (the option count), which would contradict the payload bullet after a single-site apply. Changes → Symbols / APIs: "Fake agent (test-only), two argv options, the ninth and tenth"; Counts / qualifiers moved: "Fake agent options: 8 → 10"; Cross-project / external claims: the real payload of source `resume` holds 10 key names, the fake agent's 5, "No reading became a ledger row or a fixture"; Decisions & corrections lists that difference as found at implement with no owner yet.
    basis: src/bin/viola-fake-agent.rs:143-155, tests/cli_fake_agent.rs:520-543
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 → 5-command implementation (`run` step 2, the root-chain paragraph)
    change: >-
      "(`Instant::now() + WITHIN` reads 23 sites in 17 files under `tests/`, as measured at chunk 2026-10-10-self-healing-state's report on the Linux dev host's tree; ...)" becomes: reads 26 sites in 19 files under `tests/`, as measured at chunk 2026-10-10-viola-revive's report on its final tree (the three new sites in `tests/support/home.rs`, `tests/chaos_revive.rs` and `tests/cli_revive.rs`); the pointer to architecture §Occupied Resources → Filesystem for the count's rule stays.
    sidecar: >-
      2026-10-10-viola-revive: §3 5-command implementation - root wait count 23 sites in 17 files → 26 sites in 19 files.
    rationale: >-
      Report Expected amendment 9 ("§3 `run` step 2 (the root wait count)"). Counts / qualifiers moved: "Root wait count: 23 sites in 17 files → 26 sites in 19 files (basis: `grep -rn 'Instant::now() + WITHIN' tests`, 26 lines, and `grep -rln`, 19 files, on the final tree at 14:53Z ...). Stated in architecture (`23 sites`, 1 hit) and in the key file `contracts/test-plan/5-command-implementation.md` (1 hit)". Test-plan's body holds no other statement of the count (0 hits); architecture's site is its own detector's.
  - detector: D-tests-coverage
    severity: warning
    section: §3 → 5-command implementation (`run` step 2, after the `Wrapper::stop` / `stop_keep` sentence)
    change: >-
      Add: the kill path (`Wrapper::kill`, used by `tests/chaos_revive.rs`) waits on `holder_gone` in place of `wait_endpoint_gone` - on Unix a connect to the path is refused or the path is absent, on Windows the pipe is not found - because a killed wrapper removes nothing: on Unix its socket file stays (the next bind takes over a leftover socket file), so a wait on the file being absent never ends after a kill (as measured at chunk 2026-10-10-viola-revive; the kill-and-revive case green on the three CI runners). The stop path keeps the `unconnectable` rule.
    sidecar: >-
      2026-10-10-viola-revive: §3 5-command implementation - root test support gains a kill path whose endpoint wait is the connect probe `holder_gone`; the stop path's `wait_endpoint_gone` rule unchanged.
    rationale: >-
      Changes → Spec claims disproved by measurement: the plan named `wait_endpoint_gone` after a kill, "whose Unix rule is the socket file absent ... A killed wrapper removes nothing, so on Unix its socket file stays and that wait can never end ... The case waits on `holder_gone` instead ... The stop path keeps the old rule". Harness / gate surface: root test support gained `Wrapper::{boot_in, revive, kill, shown}` and `holder_gone`. This key file is where the root chain's endpoint wait is specified, and it states only the file-absent rule, the reading the plan took.
    basis: tests/support/home.rs:561-586, tests/support/outer_pty.rs:100-103, tests/chaos_revive.rs:57-151
  - detector: D-tests-coverage
    severity: escalate
    section: §3 → 5-command implementation (`cleanup`, the Verification bullet)
    change: >-
      After "A force-kill is reported but does not by itself fail cleanup." add: on Unix a killed wrapper leaves its socket file (as measured on the root tests at chunk 2026-10-10-viola-revive), so whether step 4's Unix rule (the socket path no longer exists) holds after a force-killed wrapper is not measured for the harness.
    sidecar: >-
      2026-10-10-viola-revive: §3 5-command implementation `cleanup` - the force-kill sentence scoped: the Unix endpoint-gone rule after a force-killed wrapper is unmeasured for the harness.
    rationale: >-
      Another occurrence of the mechanism the kill-path proposal retires (a killed wrapper's endpoint is gone by the Unix socket file being absent): cleanup step 2 force-kills survivors, step 4 requires the Unix socket path gone, "Any `false` field means exit 1", yet the Verification bullet says a force-kill does not by itself fail cleanup. The report measured the leftover socket file on the root tests only and states no harness command changed, so what the harness reads after a force-kill is an inference, not a measurement - escalated for the operator to scope the sentence or owe the measurement.
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → Coverage scope (the "CLI verbs and the `--json` / exit-code contract" entity)
    change: >-
      The verb list gains `revive` (`run · revive · send · wait · last · list · answer · pause · release · link · unlink · ui · verify · plugin install`), with: `revive` takes no `--json`; its exits are 0 (the revived child's wrapper ends as `run`'s does), 1 (a preflight refusal) and 2 (a clap usage error, a malformed `--id` included).
    sidecar: >-
      2026-10-10-viola-revive: §1 coverage scope - the CLI verbs entity names `revive` (no `--json`; exits 0, 1, 2).
    rationale: >-
      Changes → Symbols / APIs: "CLI verb, new: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name> --list` ... It takes no `--json`"; Counts / qualifiers moved: "CLI verbs: one more, `revive`". The entity's closed verb list is the coverage scope the new verb's tests (`tests/cli_revive.rs`, `tests/chaos_revive.rs`) belong to, and it implies a `--json` contract revive does not have.
    basis: src/cmd/revive.rs:18-35, tests/cli_revive.rs:192-203
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → Surfaces under test → cli (`viola` verbs)
    change: >-
      The surface's verb list gains `revive` beside `run` (launch semantics), with a note on Signal: `revive` has the human signal only - exit 0 / 1 / 2, on a refusal one `unable:` line and a last-line `hint:` on stderr with empty stdout, and for `--list` one `<ts>  <cause>  <id>` row per logged session on stdout, ASCII, no ESC byte - and no JSON document.
    sidecar: >-
      2026-10-10-viola-revive: §1 surfaces - the cli verb surface names `revive` and its human-only signal.
    rationale: >-
      Second statement of the closed CLI verb list in test-plan (the same 13 verbs, here with "`--home <per-test temp>` and `--json`" as the driver). Changes → Symbols / APIs (the verb, no `--json`); Expected amendment 12 gives the `--list` row shape; Outcome (design): "`--list` static lines, ASCII, two spaces, no ESC, empty stderr; refusals without ESC - met".
    basis: tests/cli_revive.rs:301-326, src/cmd/revive.rs:220-234
    dependent-of: D-tests-coverage
```

## obs-plan

Verdict: 15 proposals (D-obs-instrumentation 11, D-obs-stack 4; D-obs-pii clean). Stripped: two opening notes with the clean D-obs-pii verdict and the D-obs-stack summary.
Numbered O1 to O15 in the order of the list.
- O1 §6 detail catalog, exit 1 → apply · routine (expected amendment 10)
- O2 §6 Filesystem refusals → apply · dependent of O1
- O3 §4 Scenario 1, on failure → apply · dependent of O1
- O4 §1 Critical paths → apply · dependent of O1 (playbook "Verbatim upstream copy kept current")
- O5 §1 Telemetry triggers → apply · dependent of O1 (same rule)
- O6 §4 Edge flows E5 → apply · routine (expected amendment 10)
- O7 §7 Panic hooks, role sentence → apply · routine (expected amendment 10); read at `src/main.rs` `role_of`
- O8 §4 Instrumentation per surface → apply · dependent of O7
- O9 §1 Telemetry surfaces → apply · dependent of O7
- O10 §10 Bounded exemptions, a sixth → escalate · check 1, no rule and the change is structural: the list of cases where a panic writes no line grows by one. The fact is the report's (`--list` opens no process log) and the plan's; the consequence for a panic is the detector's inference from §7. Resolved at the Phase 2 halt: recorded as the sixth exemption, standing only until `--list` opens the `cli` role log with its `--json`, a CARRY on "CLI machine contract" (`inputs#I8`, answer 2).
- O11 §2 Agent-readable invariants → apply with O10 · dependent
- O12 §3 intro, `refuse` has a second caller → apply · routine (expected amendment 10)
- O13 §11 Logs → apply · dependent of O12
- O14 §3 OTel SDK init, the `run` anchor → apply · dependent of O12
- O15 §3 Logging stack, shared writers → apply · dependent of O12

### Parsed list, as returned

```yaml
# D-obs-pii: evaluated clean, no proposal. Report Coverage of new surfaces: no refusal pair, hint, `--list` row or log line holds the recorded directory, a pid or a logged id; snapshot `data.cwd` is a host path in the 0600 snapshot only; CI secret scan `success`.
# D-obs-stack: Dependencies "none added or bumped"; the logger and sink are `run`'s own (`open_wrapper_log`). The drift is in §3's actor claims only.
proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: "§6 Log Coverage → `detail` code catalog (the `process-exit` exit 1 line)"
    change: >-
      The `process-exit` exit 1 line lists: `already-live` (a `live` or a `stale` holder of the name, from `viola run` or from `viola revive`, which prints `run`'s own collision pair; a `gone` holder is taken over with no exit-1 line), `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `internal-error`, and from `viola revive`'s start-arm preflight `strict-modes-failed` (the instance's state files could be written by another user), `no-session` (no logged session, or an `--id` the log does not hold) and `cwd-missing` (the snapshot records no directory, or one that is not an existing directory). The preflight reads in the order strict-modes → holder → session → directory and takes no later reading once one refuses; each refusal is one `process-exit{subject:"self", exit_code:1, detail}` in `run-<name>.ndjson` with no child started. `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 11 values.
    sidecar: "§6 `detail` code catalog, `process-exit` exit 1: adds `no-session` and `cwd-missing` (revive's start-arm preflight) and names `strict-modes-failed` and `already-live` as revive exit-1 causes too; the schema's `process-exit` `detail` enum is 11 values (was 9)."
    rationale: >-
      Report Changes → Symbols / APIs: the start arm's preflight refuses in the order `strict-modes-failed`, `already-live`, `no-session` (two pairs), `cwd-missing`, each with one `process-exit` line (`subject` `self`, `exit_code` 1) in `run-<name>.ndjson`. Schema / config: the `process-exit` `detail` enum gains `cwd-missing` and `no-session` (9 values → 11). The catalog is closed and lists neither, and it places `strict-modes-failed` on exit 21 only. Expected amendments 10 names this site.
    basis: "src/cmd/revive.rs:56-63 (`Refusal::detail`); schemas/diag-line.v1.json:82; tests/cli_revive.rs:59-82 (`assert_refused`)"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§6 Log Coverage → `detail` code catalog (the Filesystem refusals (Vector 7) line)"
    change: >-
      `strict-modes-failed` is carried on `process-exit` exit 21, on `process-exit` exit 1 from `viola revive`'s start-arm preflight (`viola_state::strict::check_instance` over the home, `instances/`, the instance directory, its `snapshot.json` and `events.ndjson`, each that exists), on `hook-decision`, and on `parse-rejected{parser:"ledger-stamps"}` from `run`'s version gate, at `warn`, with no path. `viola revive <name> --list` makes the same check and refuses with the stderr pair and exit 1 only, with no log line.
    sidecar: "§6 Filesystem refusals: `strict-modes-failed` also rides `process-exit` exit 1 from revive's preflight (`check_instance`); `--list`'s refusal writes no line."
    rationale: >-
      Same claim as the primary, restated as a carrier list: the line enumerates where `strict-modes-failed` appears and names exit 21, `hook-decision` and `parse-rejected` only. Report Symbols / APIs: revive's first preflight reading refuses `strict-modes-failed` with `exit_code` 1, and new `strict::check_instance(home, instance_dir)` checks the five paths in that order; `--list`'s two refusals are the stderr pair and exit 1 only.
    basis: "crates/viola-state/src/strict.rs:36-47 (`check_instance`); src/cmd/revive.rs:121-123"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Scenario: `viola run` start sequence to child spawn → Required log fields (the \"On failure\" bullet)"
    change: >-
      On failure, `process-exit{subject:"self", exit_code:1, detail}` with `detail` ∈ `already-live|squatted-name|pinned-hash-mismatch|batch-script-child` (Founder Direction 4; no path or pid). `viola revive` enters the same sequence through `start` (its second product caller) after its own preflight, whose refusals add `detail` ∈ `strict-modes-failed|no-session|cwd-missing` (and `already-live` from the same `collision_check`) before any child is started; a revive that passes its preflight runs `collision_check` twice, once in the preflight and once inside `start`.
    sidecar: "§4 Scenario 1, Required log fields: revive's start arm shares the sequence through `start`; its preflight adds exit-1 details `strict-modes-failed|no-session|cwd-missing`; `collision_check` runs twice on a passing revive."
    rationale: >-
      Same claim as the primary: the scenario fixes the exit-1 `detail` set of a start at four values. Report Symbols / APIs: `start` takes a `Launch` and has two product callers now (`run`, `revive`); `collision_check` is called from `start` and from revive's preflight, so a revive that passes runs it twice; the start arm writes `run`'s spans and one `process-exit` per refusal (Coverage of new surfaces, Outcome (obs)).
    basis: "src/cmd/revive.rs:112-132 (`preflight_order`); src/cmd/run.rs:137-147 (`Launch`)"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§1 Obs Scope Summary → Critical paths (must-trace) → `viola run` start sequence to child spawn → Required log fields"
    change: >-
      Exit-1 causes use a distinct `detail` code: already-live, squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, and for a start made by `viola revive` also strict-modes-failed, no-session and cwd-missing. No path or pid appears in the detail.
    sidecar: "§1 Critical paths, start sequence: the exit-1 cause list gains revive's `strict-modes-failed`, `no-session`, `cwd-missing`."
    rationale: >-
      Same claim as the primary, in §1's own wording (§1 is kept current; a hit inside it is judged like a body amendment). Report Symbols / APIs and Schema / config: revive's start arm logs as process `run` and adds two exit-1 details plus `strict-modes-failed` on exit 1.
    basis: "src/cmd/revive.rs:56-63"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§1 Obs Scope Summary → Telemetry triggers → creator-explicit-telemetry (Founder Direction 4, cross-plan audit)"
    change: >-
      Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s strict-mode fail, no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid.
    sidecar: "§1 Telemetry triggers, Founder Direction 4: the exit-1 cause list gains revive's three preflight causes; strict-mode fail is an exit-1 cause on revive."
    rationale: >-
      Same claim as the primary, restated without its tokens: the trigger enumerates the causes behind exit 1 and binds strict-mode fail to exit 21 only. Report Symbols / APIs: four preflight refusals at exit 1, the pairs accepted by the founder (`inputs#I5`, point 4); the canary-named directory case shows no pair or detail holds the recorded directory or a pid.
    basis: "src/cmd/revive.rs:65-89 (`Refusal::pair`)"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Edge flows → E5 (snapshot corruption)"
    change: >-
      Replace the last sentence ("It is library code: no product process calls it yet … (first the working-route entry \"viola revive\")") with: Its first product caller is `viola revive`'s start arm (the preflight's third reading, chunk 2026-10-10-viola-revive): a revive over an unreadable snapshot writes one `state-recovered{detail:"snapshot-replayed"}` line in `run-<name>.ndjson`, and the replay yields no recorded directory, so that revive ends `process-exit{exit_code:1, detail:"cwd-missing"}`. The four other snapshot readers (`src/cmd/client.rs`, `run`'s `collision_check`, `src/cmd/hook.rs` twice) still call `read_snapshot` and write no `state-recovered` line; `replay::session_chain`, the pass revive reads its session ids from, logs nothing by design.
    sidecar: "§4 Edge flows E5: `read_snapshot_or_replay` has a product caller, `viola revive`'s start arm; a product run now writes the snapshot `state-recovered` details in `run-<name>.ndjson` (was: library code, no product caller)."
    rationale: >-
      Report Symbols / APIs: `read_snapshot_or_replay` has its first product caller (revive's third reading); the four other snapshot readers still call `read_snapshot`; `session_chain` writes and logs nothing. Outcome (obs): one `state-recovered` line, `snapshot-replayed`, valid against the schema — met; (arch) snapshot `cwd` … none from the replay. E5 still says no product process calls it and no product run writes the two details. Expected amendments 10 names this site.
    basis: "tests/cli_revive.rs:232-276 (`revive_of_a_snapshot_with_no_cwd_is_cwd_missing`); src/cmd/revive.rs:187-198 (`recorded_dir`)"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§7 Error Capture & Reporting → Panic hooks → Main-thread catch site (every role)"
    change: >-
      The role-from-argv sentence reads: the first argument that is neither a global flag nor a flag's value and that names a role subcommand (`run`, `hook`, `mcp`, `ui`; `role_of` in `src/main.rs` names `revive` beside `run`; any other verb is `cli`). Add: `viola revive`'s start arm logs as process `run` (`ObsProcess::Run`: the `run-<name>.ndjson` role file and `run`'s detail sink), opened by the `open_wrapper_log` it shares with `run`; `viola revive <name> --list` opens no process log and writes no line, so its two refusals (`strict-modes-failed`, `no-session`) are the stderr pair and exit 1 only.
    sidecar: "§7 Panic hooks, main-thread catch site: `role_of` names `revive` beside `run`; revive's start arm is role `run` (`run-<name>.ndjson`, `run`'s detail sink); `revive --list` opens no process log."
    rationale: >-
      Report Symbols / APIs: the start arm logs as process `run` (`ObsProcess::Run`, the `run-<name>.ndjson` role file, `run`'s detail sink); `role_of` in `src/main.rs` names `revive` beside `run` (`Role::Other`); `--list` opens no process log and writes no line. §7 lists the role subcommands as `run`, `hook`, `mcp`, `ui` with every other verb `cli`, which would read `revive` as `cli`. Expected amendments 10 names this site.
    basis: "src/main.rs (role_of; the report gives no line for it); src/cmd/revive.rs:200-218 (`list`); tests/cli_revive.rs:328-336"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Instrumentation per surface → row \"cli (short-lived verbs)\""
    change: >-
      After "`process-start` / `process-exit{exit_code, detail}` into `cli-<name>.ndjson` when an instance resolves (D-06)" add: except `viola revive <name> --list`, which by plan opens no process log and writes no line (§7 Panic hooks); `viola revive`'s start arm is not a `cli` verb — it logs as process `run` (the `viola run` row).
    sidecar: "§4 Instrumentation per surface, short-lived verbs: `revive --list` writes no process log by plan; revive's start arm belongs to the `viola run` row."
    rationale: >-
      Same claim the §7 change retires (every verb outside `run`/`hook`/`mcp`/`ui` is a `cli` verb that logs to `cli-<name>.ndjson` once an instance resolves), restated in the surface table. Report Coverage of new surfaces: `viola revive <name> --list` → instrumentation n/a (by plan it opens no process log); Deviations: `revive_list_of_a_name_never_started_is_no_session_and_opens_no_log`.
    basis: "tests/cli_revive.rs:328-336"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§1 Obs Scope Summary → Telemetry surfaces → cli (short-lived verbs) (the verb list and its Exporter bullet)"
    change: >-
      The short-lived verb list gains `revive --list`. The Exporter bullet gains: `viola revive <name> --list` opens no process-log file and writes no line, by plan; `viola revive`'s start arm is the long-lived wrapper surface below (`process="run"`, `diagnostics/run-<name>.ndjson`), not a short-lived verb.
    sidecar: "§1 Telemetry surfaces, short-lived verbs: `revive --list` listed, with no process-log file by plan; revive's start arm belongs to the `viola run` wrapper surface."
    rationale: >-
      Same claim as the §7 primary, in §1: the surface enumerates the short-lived verbs and says each writes JSON-per-line to `diagnostics/` when an instance is resolved. Report Counts / qualifiers moved: CLI verbs, one more, `revive`; Symbols / APIs: `--list` opens no process log; the start arm logs as process `run`.
    basis: "src/cmd/revive.rs:97-110 (`revive`: the `--list` branch at 98-100)"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → Zero unlogged panics → Bounded exemptions"
    change: >-
      Add a sixth exemption: `viola revive <name> --list`, which by plan opens no process log and so has no role file; a panic there writes no line (as in exemptions 2 and 5). `viola revive`'s start arm is not exempt: its role file is `run-<name>.ndjson`.
    sidecar: "§10 Bounded exemptions: adds `revive --list` (no process log, so no role file); revive's start arm is covered as role `run`."
    rationale: >-
      The list is headed "no file exists, so no line can be written" and is exhaustive; the report adds one more no-file case (Symbols / APIs: `--list` opens no process log and writes no line). The panic consequence is not stated in the report: it follows from §7's own rule that the panic hook writes nothing while no role file is open. Proposed so the §7 statement does not stand beside an exemption list that omits it.
    basis: "src/cmd/revive.rs:200-218 (`list`)"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§2 Telemetry Strategy → Agent-readable invariants (the \"Every panic after init step 4\" bullet)"
    change: >-
      The parenthetical list of bounded exemptions reads: the pre-init window, `cli` without an instance, `hook` without `VIOLA_NAME`, a failed step-4 role-file open, the no-obs-init `hook --capture` arm, and `revive --list`, which opens no process log.
    sidecar: "§2 Agent-readable invariants: the bounded-exemption list names `revive --list` beside the five in §10."
    rationale: >-
      The bullet restates the §10 exemption list inline; if §10 gains the `revive --list` case and this copy does not, the two lists differ. Rests on the same report fact (`--list` opens no process log) and on the same inference as the §10 proposal; apply both or neither.
    dependent-of: D-obs-instrumentation
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract (intro, the `run` bullet of the harness pattern)"
    change: >-
      `run` writes nothing to the terminal except the child's own output, with one exception before any child is spawned: a start refusal writes exactly two fixed stderr lines (no path or pid) through `src/human.rs` `refuse` → `write_refusal`, one `write_all` on the locked stderr, a closed pipe swallowed — the root bin's only human-stderr writer, called by `run` and by `revive`. `run`'s refusals are a `.cmd`/`.bat` child, a live or stale name, a tampered pinned copy and a squatted endpoint; `viola revive` (its start arm logs as process `run`) adds state files another user could write, no logged session or an `--id` the log does not hold, and a missing recorded directory, and on a live name prints `run`'s own collision pair; `revive --list` writes the pair for its two refusals and nothing else on stderr. While the child runs, neither writes anything of its own.
    sidecar: "§3 intro: `refuse` / `write_refusal` is called by `run` and `revive` (was: only by `run`); the pre-spawn refusal list gains revive's strict-modes, no-session and cwd-missing pairs."
    rationale: >-
      Report Symbols / APIs: each revive preflight refusal is one `human::refuse` pair on stderr; `--list`'s two refusals are the stderr pair; Deviations: the hints are built from `viola-agent-claude`'s constants; Outcome (a11y): a passed revive writes no line of its own while the child holds the terminal. §3's intro says `refuse` is "called only by `run`" and closes the start-refusal list at four causes. Expected amendments 10 names this site (`refuse` has a second calling verb). No logger, sink or dependency moved (Dependencies: none added or bumped).
    basis: "src/cmd/revive.rs:91-94 (`Refusal::write`); src/cmd/revive.rs:65-89 (`Refusal::pair`); tests/cli_revive.rs:149-166"
  - detector: D-obs-stack
    severity: warning
    section: "§11 Obs Anti-Patterns → Logs (the `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` ban)"
    change: >-
      In the `#[allow]` carve-out sentence, "`run`'s pre-spawn start refusals write through `src/human.rs`" becomes "`run`'s and `revive`'s pre-spawn refusals (and `revive --list`'s) write through `src/human.rs` (`refuse` / `write_refusal`, one `write_all` on the locked stderr), which uses no print macro and carries no `#[allow]`".
    sidecar: "§11 Logs, print-macro ban: the `src/human.rs` writer serves `revive`'s refusals as well as `run`'s."
    rationale: >-
      The ban restates the §3 intro's mechanism and names `run` as its one actor. The sentence stays true for `run` but reads as the complete set of callers; with the §3 intro amended to two callers, this copy would still name one. Report Symbols / APIs: revive's refusals are `human::refuse` pairs; gates: clippy `-D warnings` green, so no print macro or `#[allow]` was added for them.
    basis: "src/cmd/revive.rs:91-94"
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: "§3 → OTel SDK init (Init order → Per-role anchors, the `run` anchor)"
    change: >-
      `run`: steps 1–6 finish **before** `run.collision_check`, so exit-1 causes are logged. The panic hook is live before `pty.spawn`. `viola revive`'s start arm is this role: it runs the same log init and own start line through `open_wrapper_log` (shared with `run`) before its preflight, so its four exit-1 refusals — the strict-modes one, read ahead of the collision check, included — are logged in `run-<name>.ndjson`; its instance is its own `ViolaName` argument. `viola revive <name> --list` opens no process log.
    sidecar: "§3 OTel SDK init, per-role anchors: the `run` anchor covers `viola revive`'s start arm (`open_wrapper_log` before the preflight); `revive --list` opens no process log."
    rationale: >-
      The anchor ties the `run` role's init to `viola run` alone and to `run.collision_check` as the first exit-1 cause. Report Symbols / APIs: new `open_wrapper_log` holds what `run` did before its start (log init, own start line, the persistent names) and is shared with revive (Deviations); revive's first refusal, `strict-modes-failed`, precedes the collision check and still gets its `process-exit` line.
    basis: "src/cmd/revive.rs:134-171 (`preflight`); src/cmd/run.rs (`open_wrapper_log`; the report gives no line for it)"
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: "§3 → Logging stack (Format, the `run-<name>.ndjson` and `ui-<port>.ndjson` shared-writer bullet)"
    change: >-
      `run-<name>.ndjson` and `ui-<port>.ndjson`, briefly: a second `viola run <name>` or a `viola revive <name>` on a live or `stale` name, or `viola ui` on a live port, opens the same file at init step 4. It appends its `process-start` and its exit-1 `process-exit` (for example `detail:"already-live"`, Scenario 1) next to the holder's lines. A `gone` name is taken over by `run` with no exit-1 line; `viola revive` on a `gone` name appends its own lines to the same file and either starts the child or ends with an exit-1 `process-exit` (`strict-modes-failed`, `no-session`, `cwd-missing`). The concurrent-append check covers this collision path too (D-29).
    sidecar: "§3 Logging stack, shared writers of `run-<name>.ndjson`: `viola revive <name>` is a writer of the role file beside a second `viola run <name>`."
    rationale: >-
      The bullet enumerates which processes append to `run-<name>.ndjson` and names a second `viola run` only. Report Symbols / APIs: revive's start arm writes the `run-<name>.ndjson` role file, and on a live name refuses `already-live` through `run`'s own collision check with its two landed pairs; Deviations: `run`'s log setup moved into `open_wrapper_log`, shared with revive.
    basis: "tests/cli_revive.rs:149-166 (`revive_on_a_live_name_prints_the_run_collision_pair`)"
    dependent-of: D-obs-stack
```

## a11y-plan

Verdict: 11 proposals (D-a11y-surface; D-a11y-obs-schema clean). Stripped: three opening notes with the clean D-a11y-obs-schema verdict and that no help-group label is proposed.
Numbered A1 to A11 in the order of the list.
- A1 §1 CLI verbs → apply · routine (expected amendment 13)
- A2 §8 Error recovery → apply, narrowed · the CLI bullet's general form stands; one sentence names revive's exit-1 pairs and points to design-system cli pattern 2, the home of the fixed-message form (it already lists `run`'s and `verify`'s)
- A3 §1 TUI passthrough entity → apply · routine
- A4 §1 Boundary-only vendor zones → apply · dependent of A3
- A5 §1 Surface cli, Notes → apply · dependent of A3
- A6 §1 Surface tui → apply, the heading · dependent of A3; the surface's Notes name `run` as the process, which holds for a revived start
- A7 §6 CLI equivalent → apply · dependent of A3
- A8 key file, Keyboard test harness → apply · dependent of A3
- A9 §11 Strategy → apply · dependent of A3
- A10 §11 Keyboard → apply · dependent of A3
- A11 §11 Visual → apply · dependent of A3

### Parsed list, as returned

```yaml
# D-a11y-obs-schema: no drift. The report's only schema change is two new `process-exit` `detail` values in `schemas/diag-line.v1.json`; no event kind or process value was added, and a11y rows carry no `detail` and validate against `e2e-web/schemas/a11y-row.v1.json`, never `diag-line.v1.json` (key file structured-violation-json-schema.md).
# D-a11y-surface: drift. `viola revive` is a new CLI verb the body names nowhere. Two violated occurrences (the CLI verbs entity, the TUI passthrough entity); the rest are restatements of those two claims.
# Not proposed: the `--help` group labels (board / traffic / wheel / handoff / setup). The report says the verb has a help group but does not name it or state a new label.
proposals:
  - detector: D-a11y-surface
    severity: warning
    section: "§1 A11y Scope Summary → CLI and terminal entities → CLI verbs"
    change: "The entity's verb list gains `revive`, with its modes stated: `revive` takes no `--json`; `viola revive <name> --list` prints static ASCII rows (`<ts>  <cause>  <id>`, no header, no colour, no ESC byte); each start-arm preflight refusal is one `unable:` line and one `hint:` line last on stderr with exit 1, and a clap usage error exits 2; a passed start is the TUI passthrough entity below."
    sidecar: "§1 CLI verbs: the list names `revive` (no `--json`; `--list` static ASCII rows; exit-1 refusal pairs ending in `hint:`; exit 2 for usage) — chunk 2026-10-10-viola-revive."
    rationale: "Report Changes → Symbols / APIs adds the CLI verb `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `--list`, and says it takes no `--json`; Expected amendments entry 13 names this section (`CLI verbs`: a11y-plan 1 hit); Coverage of new surfaces marks both arms a11y ✓ (one `hint:` line last, typed exits 1 and 2; `--list` ASCII, no ESC byte). The body's list does not hold the verb, so the new surface is uncovered in the plan."
    basis: "src/cmd/revive.rs:18-35"
  - detector: D-a11y-surface
    severity: warning
    section: "§8 Cognitive Accessibility → Error recovery (the CLI pattern bullet and the cli verification bullet)"
    change: "Beside the `unable  <reason>  <detail>` refusals, the CLI has the exit-1 refusal pair — one `unable:` sentence line, then one `hint:` line last on stderr: `viola revive`'s preflight refusals (`strict-modes-failed`, `already-live`, `no-session`, `cwd-missing`; `already-live` prints `run`'s collision pair unchanged) and the two of `--list`; a clap usage error, a malformed `--id` included, exits 2. The root cases in `tests/cli_revive.rs` pin the pairs and the exits; they are outside the trycmd refusal layout and the driver exit-code set this paragraph lists."
    sidecar: "§8 Error recovery: the exit-1 `unable:` / `hint:` refusal pair of `viola revive` recorded beside the `unable <reason> <detail>` layout, with exit 2 for usage — chunk 2026-10-10-viola-revive."
    rationale: "With `revive` in the CLI verbs entity, this paragraph's claim that CLI refusals are `unable  <reason>  <detail>` with the driver exit codes it lists no longer covers every listed verb. Report Symbols / APIs: each revive refusal is one `human::refuse` pair on stderr, exit 1, exit 2 for a clap usage error. Outcome: '(a11y) each refusal ends with one `hint:` line and a typed exit (1, or 2 for usage) — met'. The pair texts are the constants in `tests/cli_revive.rs`."
    basis: "tests/cli_revive.rs:33-43"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§1 A11y Scope Summary → CLI and terminal entities → `viola run` TUI passthrough"
    change: "The entity is the wrapper's TUI passthrough, entered by `viola run` and by a passed `viola revive` (its start arm goes through `run`'s `start` with child program `claude` and logs as process `run`). The three boundary clauses bind both verbs; under revive the one asserted by a case of its own is that a passed revive writes no line of its own while the child holds the terminal (`tests/chaos_revive.rs`), and the other two rest on the shared start."
    sidecar: "§1 TUI passthrough entity: a passed `viola revive` enters the same boundary as `viola run`; its zero-own-lines clause is read by `chaos_revive` — chunk 2026-10-10-viola-revive."
    rationale: "Report Symbols / APIs: a passed preflight launches `claude` through `run`'s `start` (`start` has two product callers now, `run` and `revive`), and the start arm logs as process `run`. Outcome: '(a11y) a passed revive writes no line of its own while the child holds the terminal — met (`chaos_revive`)'. The body scopes the passthrough boundary to `viola run` by name, so the second verb that hosts the `claude` TUI is uncovered. The report gives no revive-specific case for the keystroke or the focus / mouse / resize clause, so the change claims none."
    basis: "tests/chaos_revive.rs:57-151"
  - detector: D-a11y-surface
    severity: warning
    section: "§1 A11y Scope Summary → Boundary-only vendor zones"
    change: "The only vendor-content boundary is the wrapper-hosted `claude` TUI above, entered by `viola run` and by a passed `viola revive`."
    sidecar: "§1 Boundary-only vendor zones: the vendor boundary named as the wrapper-hosted TUI of `viola run` and a passed `viola revive` — chunk 2026-10-10-viola-revive."
    rationale: "Restates the claim that the passthrough is `viola run`'s alone ('The only vendor-content boundary is the `viola run` TUI above'). Report Symbols / APIs: a passed revive launches `claude` through the same start."
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§1 A11y Scope Summary → A11y surfaces & assistive tech reach → Surface: cli → Notes (required behaviour)"
    change: "There is no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`; `viola revive --list` rows and the revive refusal pairs carry no ESC byte."
    sidecar: "§1 Surface cli Notes: the no-SGR list names a passed `viola revive`; `--list` rows and revive refusals carry no ESC — chunk 2026-10-10-viola-revive."
    rationale: "Restates the viola-is-silent-under-`viola run` claim without the second start verb. Report Outcome: '(design) `--list` static lines, ASCII, two spaces, no ESC, empty stderr; refusals without ESC — met' and '(a11y) a passed revive writes no line of its own while the child holds the terminal — met'."
    basis: "tests/chaos_revive.rs:57-151"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§1 A11y Scope Summary → A11y surfaces & assistive tech reach → Surface: tui"
    change: "The surface is the wrapper passthrough of `viola run` and of a passed `viola revive` (boundary-only). Service identity tagging stays `process:\"run\"` for both, because revive's start arm logs as process `run`; the Notes line reads that viola's diagnostics for a start by either verb go to `diagnostics/` (the `run-<name>.ndjson` role file), never to the terminal, and that `viola revive --list` opens no process log."
    sidecar: "§1 Surface tui: heading and Notes cover a passed `viola revive` beside `viola run`; `process:\"run\"` holds for both — chunk 2026-10-10-viola-revive."
    rationale: "The surface heading and its Notes name `viola run` and `run` alone. Report Symbols / APIs: the start arm logs as process `run` (`ObsProcess::Run`, the `run-<name>.ndjson` role file, `run`'s detail sink); `role_of` in `src/main.rs` names `revive` beside `run`; `--list` opens no process log and writes no line."
    basis: "src/main.rs"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§6 Visual Design Verification → State color tokens (not-color-alone) → CLI equivalent"
    change: "viola emits zero SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`; the revive refusal pairs and `--list` rows carry no ESC byte."
    sidecar: "§6 CLI equivalent: the zero-SGR list names a passed `viola revive` — chunk 2026-10-10-viola-revive."
    rationale: "Same claim as the §1 cli Notes, restated in §6 with `viola run` as the only wrapped start. Report Outcome: '(a11y) a passed revive writes no line of its own while the child holds the terminal — met (`chaos_revive`)'; '(design) … no ESC … refusals without ESC — met'. The Windows ConPTY-host sentence that follows is measured under `viola run` only and stays as written."
    basis: "tests/chaos_revive.rs:57-151"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§3 → Keyboard test harness"
    change: "In Tooling, after the zero-viola-bytes clause: a revived start has a reading of its own — the kill-and-revive case in `tests/chaos_revive.rs` asserts that a passed `viola revive` writes no line of its own while the child holds the terminal, green on the three CI runners. The keystroke clause and the focus / mouse / resize clause have no revive-specific case."
    sidecar: "§3 Keyboard test harness (Tooling): the zero-own-lines reading of a passed `viola revive` (`chaos_revive`) recorded; no revive-specific case for the other two clauses — chunk 2026-10-10-viola-revive."
    rationale: "The keyed contract lists the boundary cases for the tui surface and holds none for the new start verb. Report Outcome: '(a11y) a passed revive writes no line of its own while the child holds the terminal — met (`chaos_revive`)'; Spec claims disproved: the kill-and-revive case green on the three CI runners; Harness / gate surface: test support gained `OuterPty::{kill, shown}` and `Wrapper::{boot_in, revive, kill, shown}`."
    basis: "tests/chaos_revive.rs:57-151"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§11 A11y Anti-Patterns → Strategy"
    change: "NEVER claim WCAG conformance for the cli or the wrapper-hosted TUI (`viola run`, a passed `viola revive`)."
    sidecar: "§11 Strategy: the no-conformance-claim ban names the wrapper-hosted TUI of both start verbs — chunk 2026-10-10-viola-revive."
    rationale: "The ban restates the passthrough as the `viola run` TUI. Report Symbols / APIs: a passed revive hosts the same `claude` TUI through `run`'s `start`."
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§11 A11y Anti-Patterns → Keyboard"
    change: "NEVER block, refuse or delay a human keystroke in a wrapped start (`viola run`, or a passed `viola revive`) past the current atomic paste; the rest of the ban stays as written."
    sidecar: "§11 Keyboard: the keystroke ban covers a passed `viola revive` beside `viola run` — chunk 2026-10-10-viola-revive."
    rationale: "The ban scopes the human-keystroke rule to `viola run` by name. Report Symbols / APIs: `start` has two product callers now (`run`, `revive`) and `pump_child` is shared (`pub(super)`). This widens the norm only: the report holds no revive-specific keystroke case, and the change asserts none."
    basis: "src/cmd/run.rs"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§11 A11y Anti-Patterns → Visual"
    change: "NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`."
    sidecar: "§11 Visual: the no-SGR ban names a passed `viola revive` — chunk 2026-10-10-viola-revive."
    rationale: "Third restatement of the zero-SGR list with `viola run` as the only wrapped start. Report Outcome: '(a11y) a passed revive writes no line of its own while the child holds the terminal — met'; '(design) … refusals without ESC — met'."
    basis: "tests/chaos_revive.rs:57-151"
    dependent-of: D-a11y-surface
```

## Validate

Checks over the whole set (75 proposals, 7 documents). Outcome: 74 applied (R9, S6, O10 and O11 after the halt), 1 rejected at check 4 (T11), 0 rejected for a source the report does not carry; 3 escalations, all resolved at the Phase 2 halt (`inputs#I8`).
- Opening rule (a coordinate the report does not carry): 0 rejected. Every `basis` range was checked against the report's last section: a row's range, a child row, or a span from one row's first number to another's last in the same file.
- Check 2, cross-contradiction: none. R22, T2 and L5 name the same owed `--json`; R5 and T8, R17 and T9 move the same counts.
- Check 3, intent: the report's deviations each carry a justification; the scope record is empty. One acceptance criterion is met with a limit (the program name `claude` at `src/cmd/verify.rs:105` predates the chunk): no master states "nowhere else", and the line is routed to P5 as a CARRY on the next entry that touches verify.
- Check 4, absence: the key file enumerates no root test file (the wrap's read: the six lines holding `cmd`, `tests/`, `chaos` or `cli_`), so the two new root test files have no site there.
- Check 5, expected amendments: thirteen entries, each matched (1: R1 R5 R6 · 2: R10 R11 R12 R13 · 3: R19 · 4: R18 · 5: R3 R17 · 6: S1 · 7: S8 S4 S5 S6 · 8: T7 T8 · 9: T4 T2 T9 · 10: O1 O7 O12 O6 · 11: raised by the orchestrator · 12: L1 L3 · 13: A1). No row of `citation-dispositions.md` reads `claim false`.
- Check 6, disproved claims: one entry (the plan's `wait_endpoint_gone` after a kill), disposed by R17 and T10.
