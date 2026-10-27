proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 8
    section: "## Occupied Resources"
    change: "Replace «It rebuilds the path by the same rule only when the snapshot is missing or unreadable.» with: Each reads the snapshot through the replay (`read_snapshot_or_replay`), and only a snapshot read whole yields an endpoint, because a snapshot rebuilt from the log carries no `endpoint`, `pid` or `started_at`. On a replay, an absence or an error, `live_endpoint` (the CLI frame verbs and `mcp`'s frame tools) returns none. On a replay or an absence, hook `connect` is unreachable (fail open; SessionEnd's direct append as before). `run`'s collision check takes its caller's one read: on a replay it decides from the beat alone (`already-live` within `STALE_AFTER`, else free), and it reads an absent or unreadable snapshot as free (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "occupied-resources#1.1: the path rebuild on a missing snapshot replaced by what each replay reader now decides (`live_endpoint` none, hook `connect` unreachable, collision check from the beat)"
    rationale: "Report Changes, Symbols: `live_endpoint` reads `read_snapshot_or_replay` and only `Recovered::Snapshot` proceeds (liveness.rs 92-94); hook `connect` (224-226) maps `Replayed` / `Absent` to `NotSent::Unreachable`; `collision_check` (425-432) reads `Replayed` from the beat and an error or `Absent` as free; Count 5 (`read_snapshot(` product callers 3 → 0)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 14
    section: "## Occupied Resources"
    change: "Append: Before that check, and before anything under the home is read or opened, each of these processes runs the strict-modes instance check (`viola_state::strict::check_instance` on the home and `instances/<name>`). The CLI frame verbs run it through the shared `client::refuse_untrusted` (`send` after its text read): exit 21 with the instance pair, or under `--json` the `instance-unreachable` document. The hook event and dialog paths run it through `trusted`: exit 0 at once, with no `config.json` read, no stdin read and no frame. `mcp`'s frame tools run it before `live_endpoint`: `instance-unreachable` with `detail: null`. A refusal writes no frame and no line (chunk 2026-10-26-home-and-code-bearing-file-integrity; the founder's word, `inputs#I3` Q2)."
    sidecar: "occupied-resources#2.1: the strict-modes instance check added ahead of the server check for every frame-writing process"
    rationale: "Report Changes, Symbols: `refuse_untrusted` called first by the six verbs and `link_call`; hook.rs `canonical` / `trusted` before obs init; tools.rs `frame` (337-339) runs `check_instance` before `live_endpoint`; Retired: gaps (1) and (4)–(8)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 65
    section: "## Occupied Resources"
    change: "Append: `hook` takes it through `instance_of` → `canonical` → `trusted` (`check_instance`) before anything under the home is read. A `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so `viola hook` writes nothing (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "occupied-resources#10.1: a `VIOLA_DIR` naming no existing instance directory leaves the hook silent"
    rationale: "Report Changes, Symbols (`src/cmd/hook.rs`) and Deviation 5: a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so the hook writes nothing."
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 103
    section: "## Occupied Resources"
    change: "In the opening `run` clause read: `run` (whose start, like `viola revive`'s start arm, runs the home-tree check `check_home_tree` through `checked_exe` / `refuse_tree` before `open_wrapper_log`; a refused check opens no process log and writes no line). Widen the clause «a refused `link` / `unlink` instance check opens no role file and writes no line» to: a refused instance check of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` or `unlink` (the shared `client::refuse_untrusted`) opens no role file and writes no line (chunk 2026-10-26-home-and-code-bearing-file-integrity, `inputs#I3` Q2)."
    sidecar: "occupied-resources#15.1: `run`'s home-tree refusal and the six frame verbs' instance refusal join the sites that write no line"
    rationale: "Report Count 20: occupied-resources.md:103 @c215 and @c1095 → extend; Symbols: `run` (166-172) and `revive`'s start call `checked_exe` / `refuse_tree` before `open_wrapper_log`; `refuse_untrusted` opens nothing under the home and writes no line."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 104
    section: "## Occupied Resources"
    change: "Before `verify`'s `process-start` / `process-exit` pair add: `verify` first runs `check_home_tree(home, None, None)` (the home, `ledger/`, `bin/`, `plugin/`) before its `cli` log, and a refusal opens no role file and writes no line (no `process-start`, no spawn pair, no step line or summary) (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "occupied-resources#15.1: `verify`'s home-tree refusal added as a site that writes no line"
    rationale: "Report Changes, Symbols (`src/cmd/verify.rs` 64-70: refused → `refuse_home()`, exit 1, no step line, no summary, no line); Count 20 lists `verify` among the new no-line sites."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 105
    section: "## Occupied Resources"
    change: "In the `mcp.ndjson` clause read «none for a frame tool whose instance check (`check_instance`, before `live_endpoint`) or server check fails». In the `hook-<name>.ndjson` clause add: the event and dialog paths write nothing for a refused or absent instance (`trusted` comes before `read_diagnostics_level` and obs init, so there is no role file, no `hook-invoked` and no `hook-decision`), as the statusline arm already did (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "occupied-resources#15.1: the `mcp` frame tools' and the hook event/dialog paths' refused instance check write no line"
    rationale: "Report Count 20: occupied-resources.md:105 re-read for the new sites; Symbols: tools.rs 337-339 (no frame, no line); hook.rs: a refused or absent instance returns exit 0 at once with no role file and no line."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 105
    section: "## Occupied Resources"
    change: "Add: a snapshot rebuilt from the log writes one `state-recovered` line from each reader moved onto the replay at chunk 2026-10-26-home-and-code-bearing-file-integrity, in its own process's role file: `live_endpoint` (the CLI frame verbs' `cli-<name>.ndjson`, `mcp`'s frame tools' `mcp.ndjson`), hook `connect` and statusline `serve` (`hook-<name>.ndjson`) and `run`'s collision check (`run-<name>.ndjson`). `revive`'s preflight reads once for both its collision check and its recorded-directory reading, so one replay writes one line."
    sidecar: "occupied-resources#15.1: the four readers moved onto the replay join the `state-recovered` emitters"
    rationale: "Report Count 6: occupied-resources.md:104 re-read for the four new readers; Count 5: replay product readers 2 → 6; Coverage: the four readers on the replay → instrumentation `state-recovered`; Deviation 4: one replay, one line in `revive`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 105
    section: "## Occupied Resources"
    change: "Add: `open_role_file` and `write_detail` (`src/obs.rs`) run `strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything. A symlink is refused; a folder takes the read rule; a file takes the read rule on Unix and the write rule on Windows; an absent path passes. A refusal is a failed open that ends at `std::io::sink`, and the detail line is dropped (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "occupied-resources#15.1: the diagnostics opens' symlink and modes check before any narrowing"
    rationale: "Report Changes, Symbols (`src/obs.rs` `check_diagnostics` 69-76; `strict::check_diagnostics` 127-145); Coverage: diagnostics opens."
    fact: F15
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 323
    section: "## Occupied Resources"
    change: "End the line: Content-bearing detail stays in the instance detail files `instances/<ViolaName>/diagnostics/detail-<role>.ndjson` (owner-only, one `write` per line, format fixed by `schemas/diag-detail.v1.json`)."
    sidecar: "architecture.md §Occupied Resources `diagnostics/`: the instance detail file named"
    rationale: "Report Count 12: the instance detail file is not named at architecture.md:323; the directive's outcome item (a) (`inputs#I5`); Outcome: `v1-17`'s by-construction clause relies on §Occupied Resources naming the detail file."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 105
    section: "## Occupied Resources"
    change: "End the entity: Content-bearing detail stays in the instance detail files `instances/<ViolaName>/diagnostics/detail-<role>.ndjson` (Filesystem, `instances/<ViolaName>/`)."
    sidecar: "occupied-resources#15.1: the instance detail file named, as on the body line"
    rationale: "The same sentence the body's :323 holds closes this entity; Report Count 12."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 111
    section: "## Occupied Resources"
    change: "Replace «Four readers check these files before they use them (`viola list`, `viola revive`, `hook statusline`, and `viola link` / `viola unlink` sharing one check).» with: These readers check these files before they use them: `viola list`; `viola revive`; `viola run`'s start (through `check_home_tree` with its name); `viola hook`'s statusline, event and dialog paths (sharing `trusted`); `viola mcp`'s frame tools; and the CLI frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (sharing `client::refuse_untrusted`) (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "occupied-resources#16.1: the readers that check the instance files widened from four to every entry point"
    rationale: "Report Changes, Symbols: `refuse_untrusted` at the six verbs and `link_call`; hook.rs `trusted` for both arms; tools.rs 337-339; `checked_exe` runs `check_home_tree(home, Some(name), …)`; Retired: the eight dated gaps."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 111
    section: "## Occupied Resources"
    change: "After the `viola list` sentence add: `scan`'s own home check is the write rule, but `check_instance` holds the home to the read rule. So a 0750 home passes a board with no instance, and is refused (`ScanRefused::Instance`) once it holds one. `board.rs` is unchanged, and its owner is a P5 route pin (proposed: a CARRY on «Unix endpoint and home hardening»)."
    sidecar: "occupied-resources#16.1: `viola list`'s home-check inconsistency recorded with its owner pin"
    rationale: "Report Count 14 (board-scan: the inconsistency is the new fact, `board.rs` unchanged); the directive's outcome item (c) (`inputs#I5`); Route asks for P5."
    fact: F20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 112
    section: "## Occupied Resources"
    change: "Replace revive's set «over the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order» with: over the home (read rule), `instances/` (write rule), the instance directory (read rule), its `diagnostics/` (read rule, new in the set), `snapshot.json` (write rule) and `events.ndjson` (write rule on Unix, read rule on Windows), checking only entries that exist and a folder before what it holds."
    sidecar: "occupied-resources#16.1: `check_instance`'s set widened (`diagnostics/` added; the home, the instance dir and `diagnostics/` to the read rule)"
    rationale: "Report Changes, Symbols (`viola_state::strict`: `check_instance` now reads the home (READ), `instances/` (write), the instance directory (READ), `diagnostics/` (READ), `snapshot.json` (write), `events.ndjson` (Unix write; Windows READ)); Count 2."
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 112
    section: "## Occupied Resources"
    change: "Replace the call-site sentence with: `check_instance` has these three product call sites (`src/cmd/hook.rs` `trusted`, shared by the statusline, event and dialog paths; `src/cmd/revive.rs`; and `src/cmd/client.rs` `refuse_untrusted`, shared by `link` / `unlink` and the six frame verbs; the rule: `check_instance(` under `src/`). Outside that rule, `viola-mcp`'s `tools.rs` `frame` calls it, and `run` reaches its set through `viola_state::strict::check_home_tree`."
    sidecar: "occupied-resources#16.1: the three `check_instance` call sites' files moved (statusline.rs → hook.rs, link.rs → client.rs)"
    rationale: "Report Changes, remaining-caller facts: `check_instance(` under `src/` 3 → 3, files moved (`src/cmd/hook/statusline.rs` → `src/cmd/hook.rs`, `src/cmd/link.rs` → `src/cmd/client.rs`, `src/cmd/revive.rs`); Count 2: occupied-resources.md:112 @c2225 / @c2397 stale."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 112
    section: "## Occupied Resources"
    change: "After the `viola link` / `viola unlink` sentence add these entry points. The six frame verbs `send` (after its text read), `wait`, `last`, `answer`, `pause` and `release` run the same check first through `client::refuse_untrusted`, with `link`'s refusal: exit 21, the instance pair or under `--json` the `instance-unreachable` document, nothing under the home opened, no line, and the mode or ACL left as found. The hook event and dialog paths run it through `trusted` before `read_diagnostics_level` and obs init: exit 0, with no `config.json` read, role file, stdin read, line, frame or SessionEnd append; the two `--capture` arms stay ahead of it. `viola mcp`'s frame tools run it before `live_endpoint`: `instance-unreachable` with `detail: null`, no frame and no line. `viola run` and `viola revive`'s start run `check_home_tree` before `open_wrapper_log`: exit 1 with the home pair, or the instance pair for an instance-level refusal, and no log, line or child (chunk 2026-10-26-home-and-code-bearing-file-integrity; the founder's words `inputs#I3` Q2, Q3)."
    sidecar: "occupied-resources#16.1: the six frame verbs, the hook event/dialog paths, the `mcp` frame tools and `run`'s start join the checked entry points, each refusing with no line"
    rationale: "Report Count 20: occupied-resources.md:112 @c446 / @c1336 → extend (six verbs); Symbols: `refuse_untrusted`, hook.rs, tools.rs 337-339, `checked_exe` / `refuse_tree`; Decisions: `inputs#I3` Q2 (no line) and Q3 (link's instance pair)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 112
    section: "## Occupied Resources"
    change: "Add: `viola_state::strict` holds a read rule beside the write rule. On Unix it refuses any group or other bit (`mode & 0o077`). On Windows it refuses any allow ACE, inherit-only ACEs included, that grants a right in `READ_RIGHTS` = `0x9000_0001` (`FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to a SID `trusted_sid` rejects. `Refused` gains `Readable` and `Symlink`, each with a fixed message (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "occupied-resources#16.1: the read rule and its two new refusals recorded"
    rationale: "Report Changes, Symbols (`unix_read_verdict` 209-217, `windows_read_verdict` 279-296, `READ_RIGHTS`, `Refused` + `Readable` / `Symlink`); Count 3."
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 112
    section: "## Occupied Resources"
    change: "Add: `check_home_tree(home, name?, key?)` (`viola_state::strict`) checks the home (read rule). With a name it then runs `check_instance`'s set. It then checks `ledger/`, `bin/` and `plugin/`. With a key it also checks `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and `.claude-plugin/` (write rule). Only entries that exist are checked. A refusal is `TreeRefused::HomeLevel` or `TreeRefused::InstanceLevel`. `run`'s start (`checked_exe`, with its name and the key of the bytes it read once) and `verify` (no name, no key) call it."
    sidecar: "occupied-resources#16.1: the home-tree check recorded"
    rationale: "Report Changes, Symbols (`check_home_tree` 91-125, `TreeRefused` 83-89; `run.rs` `checked_exe` 189-200; verify.rs 64-70); Count 4 (the new names stand in no master)."
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 113
    section: "## Occupied Resources"
    change: "Replace «No case widens the DACL of an instance tree on Windows yet; that case is owed to the working-route entry …Home and code-bearing file integrity….» with: On Windows, `cfg(windows)` cases widen it: an instance tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and a plain home made before viola is refused naming the entry. All are green on `windows-2025` (ci#40403389629, chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "occupied-resources#16.1: the owed Windows instance-tree DACL case is now built and green"
    rationale: "Report, Spec claims disproved by measurement: occupied-resources.md:113 «No case widens the DACL of an instance tree on Windows yet» is now covered by the `cfg(windows)` cases (strict.rs 708-830), green on ci#40403389629."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 113
    section: "## Occupied Resources"
    change: "Add: measured on `windows-2025`, a home made with `create_dir_all` before viola is refused `Writable`, the refused entry an inherited `S-1-5-32-545` (Users) write ACE. A home a test made first is one the product must refuse (ci#40398601594, `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`)."
    sidecar: "occupied-resources#16.1: the Windows refusal of a home made before viola, as measured"
    rationale: "Report, Spec claims disproved by measurement (ci#40398601594, job 121838141094) and Deviation 6 (CARRY 13's answer)."
    fact: F17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 113
    section: "## Occupied Resources"
    change: "Add: measured on Windows, a path under a regular file stats `NotFound`, so the strict-modes check skips it as absent and the catch-site line stands there. On Unix the same home is a strict-modes refusal (`verify` exit 1 with the home pair; the frame verbs exit 21 with the instance pair), so a test that plants a regular file above a checked path owes a per-OS oracle."
    sidecar: "occupied-resources#16.1: the per-OS reading of a home under a regular file"
    rationale: "Report Count 13 and Decisions (Measured on Windows: a path under a regular file stats `NotFound`); Deviation 8 (`verify_under_a_regular_file_refuses_the_home`, per-OS oracle)."
    fact: F18
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 126
    section: "## Occupied Resources"
    change: "After «The ONE writer is `viola_state::stamps::update_stamps`, called only by `viola verify`» add: it runs `strict::check_stamps(home)` before it creates `ledger/` and again under the lock before the capped read. A refusal (`StampsError::StrictModes`; its `State` arm the former `StateError`) reads and writes nothing, and `verify` maps it to the home pair, exit 1. `verify` has already checked the home, `ledger/`, `bin/` and `plugin/` through `check_home_tree` before its `cli` log (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "occupied-resources#18.1: `update_stamps`'s strict-modes check before the create and under the lock"
    rationale: "Report Changes, Symbols (`viola_state::stamps`: `StampsError` 37-44, `check_stamps` before `create_private_dir(ledger/)` and under the lock; verify.rs 64-70, 288-296); Count 17 (the single-writer claim stays true); Deviation 1."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 171
    section: "## Occupied Resources"
    change: "Add: measured on `windows-2025`, an inheritable `icacls` grant (`(OI)(CI)`) on a seeded test home's `bin/` (or on the home) spreads through `seed_conpty`'s hard links into the shared `target/conpty-seed/<key>/` companion, and widens every seeded home until it is removed. A test that widens a home tree above `bin/<key>/conpty/` therefore starts that home unseeded, and `seed_conpty` makes the home and the seed dir through `create_private_dir` (ci#40398601594, ci#40400887151)."
    sidecar: "occupied-resources#25.1: the hard-link propagation hazard of seeded test homes and its unseeded rule"
    rationale: "Report, Spec claims disproved by measurement (the hard-link propagation, `evidence/operator-pass.md`); Deviation 10 (`run_refuses_a_widened_home_tree_before_its_log` starts unseeded); Harness surface (`seed_conpty` uses `create_private_dir`)."
    fact: F19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 185
    section: "## Occupied Resources"
    change: "After «takes the protected user + SYSTEM DACL» add: measured on `windows-2025`, the protected owner-only DACL of a home viola makes outside the profile reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`, where canonical SDDL spells the runner's local Administrator (RID 500) `LA`, so a SID oracle against it fails (read back through `icacls /save`; ci#40400887151, chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "occupied-resources#27.1: the measured protected DACL of a viola-made folder outside the profile"
    rationale: "Report, Cross-project / external claims (measured on `windows-2025`, ci#40400887151's log) and Decisions (canonical SDDL spells RID 500 `LA`; `Get-Acl` printed nothing, `icacls /save` read it back)."
    fact: F16
note: "F19's natural home, the `target/conpty-seed/<key>/` entity (body :370), and the ConPTY companions line (body :319: the `strict-modes-failed` fallback word and the owner/DACL leg) are not in this group, so they go to the new-detail pass or group 1. F4–F8 and F10–F14 belong to conventions, established-decisions and standard-contracts, and F21 is a route fact. The owner of the `viola list` inconsistency (F20) is P5's pin."
