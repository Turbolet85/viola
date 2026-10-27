proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources (Filesystem, `instances/<ViolaName>/`)"
    change: "The closing sentence names every checker and the widened set: `viola list`, `viola revive`, `viola run`'s start (`strict::check_home_tree` with the name), `hook statusline`, `viola hook`'s event and dialog paths, `viola link` / `viola unlink`, the six CLI frame verbs `send` · `wait` · `last` · `answer` · `pause` · `release` (`client::refuse_untrusted`) and `viola mcp`'s frame tools check these files before they use them, ahead of any log; `check_instance` holds the home, the instance directory and its `diagnostics/` to the read rule (Unix no group or other bit; Windows no read right to another SID), `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists, a folder before what it holds; a refused check opens nothing under the home and writes no line."
    sidecar: "§Occupied Resources instances/: the instance strict-modes check's checkers extended (run start, hook event/dialog, six CLI frame verbs, MCP frame tools) and its set widened to the read rule on the home, the instance dir and diagnostics/ (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols: `check_instance`'s set widened (home, instance dir and `diagnostics/` READ; Windows `events.ndjson` READ), `refuse_untrusted` called first by the six frame verbs, hook.rs `trusted` before obs init, tools.rs `frame` checks before `live_endpoint`, run.rs `checked_exe`; Counts 2 and 20; the founder's no-line form (inputs#I3 Q2)."
  - detector: D-arch-resources
    severity: warning
    section: "§Conventions (CLI exit codes)"
    change: "`21` instance unreachable, and a refused strict-modes check of `viola list`, `viola plugin install`, `viola link`, `viola unlink` or a CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`: the instance pair, or the `instance-unreachable` document under `--json`); `1` keeps the start, revive-preflight and verify refusals, a refused home-tree check of `run`, `revive`'s start and `verify` among them."
    sidecar: "§Conventions exit codes: exit 21's strict-modes list gains the six CLI frame verbs; exit 1 names the home-tree refusals of run, revive's start and verify."
    rationale: "Report Changes, Symbols: `client::refuse_untrusted` returns `Some(21)` with `human::instance_state_pair` or `refused_document(«instance-unreachable»)` for send/wait/last/answer/pause/release; run/revive `refuse_tree` and verify `refuse_home` exit 1; Count 8 (instance-pair) and Count 9 (home-pair)."
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources (Filesystem, `bin/<version>-<hash>/viola(.exe)`)"
    change: "Add: before any log, `viola run` and `revive`'s start check the home tree (`strict::check_home_tree`): the home to the read rule, then the instance set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy and `plugin/<key>/` with its `hooks/` and `.claude-plugin/` to the write rule, each that exists; a refusal exits 1 with the home pair (the instance pair for an instance-level refusal), opens no log and spawns no child, and `run` pins the exe bytes it checked (`pin::pin_bytes`, keyed by `pin_key`); `viola verify` checks the home, `ledger/`, `bin/` and `plugin/` before its `cli` log (exit 1, the home pair)."
    sidecar: "§Occupied Resources bin/: the home-tree strict-modes check (check_home_tree) of run, revive's start and verify over the pinned copy and plugin dirs registered."
    rationale: "Report Changes, Symbols: run.rs `checked_exe` / `refuse_tree` (`TreeRefused` HomeLevel → home pair, InstanceLevel → instance pair) called before `open_wrapper_log` by run and revive's start; verify.rs `check_home_tree(&home, None, None)` before its cli log; pin.rs `pin_key` / `pin_bytes`; Count 4 (the new tree checks stand in no master) and Count 9."
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources (Filesystem, `bin/<version>-<hash>/conpty/`)"
    change: "At every start `pin_companions` first runs the strict-modes check (`strict::check_present`, the Windows owner and DACL) on `bin/<key>/conpty/` and on each companion that exists, then holds each with `FILE_SHARE_READ` only and fully re-hashes it against its compiled SHA-256 pin until the spawn; a refused check or a mismatch leaves the file as found, writes nothing, never loads it, never refuses the start, and runs the child on the inbox ConPTY (`sideload_fallback` `strict-modes-failed` or `hash-mismatch`)."
    sidecar: "§Occupied Resources conpty/: the companions' owner/DACL check before the held open and the `strict-modes-failed` fallback word added beside the re-hash."
    rationale: "Report Changes, Symbols: pin.rs `pin_companions` runs `strict::check_present` before the held open, refusal `PinError::StrictModes`, mapped by run.rs `sideload_outcome` to `strict-modes-failed`; Schema/config: diag-line `sideload_fallback` gains `strict-modes-failed`; Retired item (3) (companions under the re-hash alone); Counts 7 and 18 (architecture.md:319 stale)."
    basis: "architecture.md:319"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources (Filesystem, `diagnostics/`)"
    change: "Content-bearing detail stays in `instances/<ViolaName>/diagnostics/detail-<role>.ndjson`; every open of a home-level role file or an instance detail file first runs `strict::check_diagnostics` on the existing `diagnostics/` folder and the file (a symlink refused; the folder to the read rule; the file to the read rule on Unix and the write rule on Windows; an absent path passes), before anything is narrowed, a refusal being a failed open (the writer ends at a sink, a detail line dropped)."
    sidecar: "§Occupied Resources diagnostics/: the instance detail file named and the diagnostics opens' symlink and modes check registered (directive inputs#I5 item a)."
    rationale: "Report Count 12 (the detail file not named at architecture.md:323) and the directive's outcome item (a) (inputs#I5); Changes, Symbols: obs.rs `check_diagnostics(dir, file)` runs `strict::check_diagnostics` in `open_role_file` and `write_detail` before `ensure_private_dir`, a refusal an `io::Error`; strict.rs `check_diagnostics` semantics; detail file name as at architecture.md:412."
    basis: "architecture.md:323"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts (Snapshot envelope)"
    change: "Append: the replay's product readers are `viola revive`, `viola list`, `liveness::live_endpoint` (the CLI frame verbs and `viola mcp`'s frame tools), `viola hook`'s `connect`, `hook statusline` and `run`'s collision check (`revive`'s preflight reads once for both its collision check and its recorded directory); a replayed snapshot carries no `endpoint`, `pid`, `started_at` or `statusline_command`, so `live_endpoint` and the hook's `connect` read it as unreachable, `hook statusline` runs nothing, and the collision check judges liveness from the beat alone (a beat within `STALE_AFTER` refuses `already-live`); each replay logs one `state-recovered` line, and no product code under `src/` calls `read_snapshot` directly."
    sidecar: "§Standard Contracts Snapshot envelope: the replay's product readers 2 → 6 and what a replayed snapshot means to each (the founder's boundary-widening ratification, inputs#I3 Q1)."
    rationale: "Report Count 5 (replay product readers 2 → 6; `read_snapshot(` product callers 3 → 0); Changes, Symbols: liveness.rs 92-94, hook.rs `connect` 224-226, statusline.rs `serve` 128-132, run.rs `collision_check` 425-432; Retired: `read_snapshot` as a product reader under `src/`; inputs#I3 Q1."
  - detector: D-arch-resources
    severity: warning
    section: "§Infrastructure Patterns → Project directory structure"
    change: "viola-state's line «the classified snapshot read, the log replay and the session chain (`replay.rs`; read by `viola revive` and `board::read`)» names every replay reader: `viola revive`, `board::read`, `liveness::live_endpoint`, `viola hook`'s `connect`, `hook statusline`'s `serve` and `run`'s collision check."
    sidecar: "Project directory structure: replay.rs's readers extended to the four readers moved onto read_snapshot_or_replay."
    rationale: "Report Count 5 and Changes, Symbols (liveness, hook `connect`, statusline `serve`, run `collision_check` now read `read_snapshot_or_replay`); the same claim as the Snapshot envelope proposal, restated in the keyed contract."
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Infrastructure Patterns → Project directory structure"
    change: "viola-state's «the instance strict-modes check (`strict::check_instance`)» becomes «the strict-modes checks (`strict`: the write rule and the read rule (`unix_read_verdict`, `windows_read_verdict`, `READ_RIGHTS`), `check_instance`, `check_home_tree` → `TreeRefused`, `check_diagnostics`, `check_present`, `check_stamps`)»."
    sidecar: "Project directory structure: viola-state strict's new checks and the read rule registered."
    rationale: "Report Count 4 («the four new names stand in no master — a fact to register») and Changes, Symbols (strict.rs: `Rule`, `unix_read_verdict` 209-217, `windows_read_verdict` 279-296, `check_home_tree` 91-125, `check_diagnostics` 127-145, `check_present` 160-163, `TreeRefused` 83-89; stamps.rs `check_stamps` before `create_private_dir`)."
  - detector: D-arch-resources
    severity: warning
    section: "§Infrastructure Patterns → Project directory structure"
    change: "client.rs's entry gains `refuse_untrusted`: the instance strict-modes check (`check_instance` on the absolute home and `instances/<name>`) that every CLI frame verb (send, wait, last, answer, pause, release) and `link` / `unlink` run first, before `connect`: refused, exit 21 with the instance pair or the `instance-unreachable` document under `--json`, nothing opened under the home and no line."
    sidecar: "Project directory structure: client.rs's refuse_untrusted, shared by the six frame verbs and link/unlink, registered."
    rationale: "Report Changes, Symbols: `src/cmd/client.rs` new `pub(crate) fn refuse_untrusted` (20-41), called first by send, wait, last, answer, pause, release, with `link_call` moved onto it; remaining-caller facts (`src/cmd/link.rs` → `src/cmd/client.rs`)."
  - detector: D-arch-resources
    severity: warning
    section: "§Infrastructure Patterns → Project directory structure"
    change: "hook/statusline.rs's «the instance check» is hook.rs's shared `canonical` / `trusted`, which the hook's event and dialog paths also take before any `config.json` read, role file, stdin read or line (a refused or absent instance exits 0 at once); the hook's `connect` reads the snapshot through the replay, a replayed one unreachable."
    sidecar: "Project directory structure: the hook instance check moved into hook.rs and shared by the event, dialog and statusline arms."
    rationale: "Report Changes, Symbols: `src/cmd/hook.rs` `canonical` (75-81) and `trusted` (83-86) moved from statusline.rs, shared by both arms, taken before `read_diagnostics_level` / `viola_obs_init`; remaining-caller facts (`src/cmd/hook/statusline.rs` → `src/cmd/hook.rs`)."
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources (Environment variables, `VIOLA_NAME`)"
    change: "Its absence, a `VIOLA_DIR` naming an instance directory that does not exist, or an instance its strict-modes check refuses makes every hook event and dialog path a silent exit 0: no `config.json` read, no stdin read, no line, no frame and no SessionEnd append (the hidden `hook --capture` arms, taken first, read no `VIOLA_*` at all, [Hook Contract])."
    sidecar: "§Occupied Resources VIOLA_NAME: a VIOLA_DIR naming no directory, or a refused instance, silences the hook as an absent VIOLA_NAME does (directive inputs#I5 item b)."
    rationale: "Report Changes, Symbols (hook.rs: `instance_of(…).and_then(canonical).filter(trusted)` before obs init; a `VIOLA_DIR` naming no existing directory canonicalises to no session); Deviation 5; Count 23 and the directive's outcome item (b) (inputs#I5)."
note: "D-arch-decisions: no violation (Dependencies none; no new library or runtime; the replay readers are the founder's ratified widening, inputs#I3 Q1). No body passage states a retired dated gap (the macOS server-check gap at §Standard Contracts Instance snapshot is not retired). Stale rows in existing detail categories are the cascade's: established-decisions.md:79, :110, :207, :17, :225, :213 (Count 13); standard-contracts.md:25, :109; occupied-resources.md:103, :112, :113; conventions.md:23. Count 14 (viola list's home check) is a P5 route owner, not an architecture change. update_stamps's pre-create check (Count 17) left at the body's ledger/stamps.json bullet as true per the report's disposition."
