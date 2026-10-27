# Apply — architecture · part 4 of 5

## Verdicts
- fanout-architecture--cascade-g2.md 1 · applied · .andromeda/architecture-detail/occupied-resources.md:8#1
- fanout-architecture--cascade-g2.md 2 · folded into fanout-architecture--cascade-g2.md 15
- fanout-architecture--cascade-g2.md 3 · applied · .andromeda/architecture-detail/occupied-resources.md:67+#1
- fanout-architecture--cascade-g2.md 4 · applied · .andromeda/architecture-detail/occupied-resources.md:103#1, .andromeda/architecture-detail/occupied-resources.md:103#2
- fanout-architecture--cascade-g2.md 5 · applied · .andromeda/architecture-detail/occupied-resources.md:104#1
- fanout-architecture--cascade-g2.md 6 · applied · .andromeda/architecture-detail/occupied-resources.md:105#1, .andromeda/architecture-detail/occupied-resources.md:105#2
- fanout-architecture--cascade-g2.md 7 · applied · .andromeda/architecture-detail/occupied-resources.md:107+#1
- fanout-architecture--cascade-g2.md 8 · applied · .andromeda/architecture-detail/occupied-resources.md:107+#2
- fanout-architecture--cascade-g2.md 10 · applied · .andromeda/architecture-detail/occupied-resources.md:105#3
- fanout-architecture--cascade-g2.md 11 · applied · .andromeda/architecture-detail/occupied-resources.md:111#1
- fanout-architecture--cascade-g2.md 12 · applied · .andromeda/architecture-detail/occupied-resources.md:115+#5
- fanout-architecture--cascade-g2.md 13 · applied · .andromeda/architecture-detail/occupied-resources.md:112#1
- fanout-architecture--cascade-g2.md 14 · applied · .andromeda/architecture-detail/occupied-resources.md:112#2
- fanout-architecture--cascade-g2.md 15 · applied · .andromeda/architecture-detail/occupied-resources.md:115+#3
- fanout-architecture--cascade-g2.md 16 · applied · .andromeda/architecture-detail/occupied-resources.md:115+#1
- fanout-architecture--cascade-g2.md 17 · applied · .andromeda/architecture-detail/occupied-resources.md:115+#2
- fanout-architecture--cascade-g2.md 18 · applied · .andromeda/architecture-detail/occupied-resources.md:113#1
- fanout-architecture--cascade-g2.md 20 · applied · .andromeda/architecture-detail/occupied-resources.md:115+#4
- fanout-architecture--cascade-g2.md 21 · applied · .andromeda/architecture-detail/occupied-resources.md:126#1
- fanout-architecture--cascade-g2.md 22 · applied · .andromeda/architecture-detail/occupied-resources.md:175+#1
- fanout-architecture--cascade-g2.md 23 · applied · .andromeda/architecture-detail/occupied-resources.md:188+#1
- cascade-compare-architecture.md 5 · applied · .andromeda/architecture-detail/occupied-resources.md:37+#1

## Edits
- line: .andromeda/architecture-detail/occupied-resources.md:8
  old: «It rebuilds the path by the same rule only when the snapshot is missing or unreadable.»
  new: «Each reads the snapshot through `viola_state::replay::read_snapshot_or_replay`, and only a snapshot read whole yields an `endpoint`, because one rebuilt from the log carries no `endpoint`, `pid` or `started_at`: on a replay, an absence or an error `live_endpoint` (the CLI frame verbs and `mcp`'s frame tools) returns none; on a replay or an absence hook `connect` is unreachable (fail open; SessionEnd appends directly); `run`'s collision check takes its caller's one read and decides a replay from the heartbeat alone (`already-live` within `STALE_AFTER`, else free) and an absence or an error as free. None rebuilds the path.»
- after: .andromeda/architecture-detail/occupied-resources.md:37
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-binaries -->»
  add: «»
  add: «- `send`, `wait`, `last`, `answer`, `pause` and `release` exit 21 when their instance check refuses, as `link` / `unlink` do (human: the instance-state `unable:` / `hint:` pair on stderr; `--json`: the `{"v":1,"error":"instance-unreachable","detail":null}` document on stdout). `verify` exits 1 with the home-state pair, before its `cli` log and any step line, when `check_home_tree` refuses its home, `ledger/`, `bin/` or `plugin/`, or `update_stamps` refuses `ledger/stamps.json`. `run` and `revive`'s start exit 1 before any log when the home-tree check refuses (`revive --json` keeps its `strict-modes-failed` document). How a home under a regular file reads on each OS: Filesystem, `instances/<ViolaName>/`, below.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/occupied-resources.md:67
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-env-vars -->»
  add: «»
  add: «- `hook`'s event, dialog and statusline paths (not its two `--capture` arms) take `VIOLA_DIR` through `instance_of`, then `canonical`, then `trusted` (`check_instance`), before anything under the home is read: a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so `viola hook` exits 0 and writes nothing.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/occupied-resources.md:103
  old: «once its strict-modes check has passed; a refused check, on either arm, opens no process log and writes no line)»
  new: «once its strict-modes check has passed; `run`'s start and `revive`'s start arm also run the home-tree check through `checked_exe` / `refuse_tree` before `open_wrapper_log`; a refused check, on either `revive` arm or `run`'s start, opens no process log and writes no line)»
- line: .andromeda/architecture-detail/occupied-resources.md:103
  old: «a refused `link` / `unlink` instance check opens no role file and writes no line»
  new: «a refused instance check of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` or `unlink` (`client::refuse_untrusted`) opens no role file and writes no line»
- line: .andromeda/architecture-detail/occupied-resources.md:104
  old: «`verify`'s `process-start`/`process-exit{subject:"self"}` pair, with eight spawn pairs between them»
  new: «`verify`, once `check_home_tree` has passed over its home, `ledger/`, `bin/` and `plugin/` (a refusal opens no role file and writes no line), writes its `process-start`/`process-exit{subject:"self"}` pair, with eight spawn pairs between them»
- line: .andromeda/architecture-detail/occupied-resources.md:105
  old: «(none for a frame tool whose server check fails)»
  new: «(none for a frame tool whose instance check or server check fails, and a refused instance check writes no line at all)»
- line: .andromeda/architecture-detail/occupied-resources.md:105
  old: «with `corr:null, decision_emitted:false`); `hook`'s drift reports»
  new: «with `corr:null, decision_emitted:false`; a refused or absent instance writes no line on any path, its check (`trusted`) coming before `read_diagnostics_level` and obs init); `hook`'s drift reports»
- line: .andromeda/architecture-detail/occupied-resources.md:105
  old: «Content-bearing detail stays in `instances/<ViolaName>/diagnostics/`.»
  new: «Content-bearing detail stays in the instance detail files `instances/<ViolaName>/diagnostics/detail-<role>.ndjson` (the instance files, below).»
- after: .andromeda/architecture-detail/occupied-resources.md:107
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- A snapshot the replay rebuilds from the log writes one `state-recovered` line into the role file of the process that read it (`cli-<name>.ndjson`, `mcp.ndjson`, `hook-<name>.ndjson` or `run-<name>.ndjson`), from each replay reader: `live_endpoint` (the CLI frame verbs and `mcp`'s frame tools), hook `connect`, statusline `serve` and `run`'s collision check, beside `viola revive` and `viola list`. `viola revive`'s preflight reads the snapshot once for both its collision check and its recorded-directory reading, so one replay writes one line.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/occupied-resources.md:107
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- `open_role_file` and `write_detail` (`src/obs.rs`) run `strict::check_diagnostics` on an existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything: a symbolic link is refused, a folder takes the read rule, a file takes the read rule on Unix and the write rule on Windows, and an absent path passes. A refusal is a failed open: the writer ends at `std::io::sink`, and a detail line is dropped.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/occupied-resources.md:111
  old: «Four readers check these files before they use them (`viola list`, `viola revive`, `hook statusline`, and `viola link` / `viola unlink` sharing one check).»
  new: «These entry points check these files before they use them: `viola list`; `viola revive`; `viola run`'s start (through `check_home_tree` with its name); `viola hook`'s statusline, event and dialog paths (sharing `trusted`); `viola mcp`'s frame tools; and the CLI frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (sharing `client::refuse_untrusted`).»
- line: .andromeda/architecture-detail/occupied-resources.md:112
  old: «over the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order»
  new: «over the home (the read rule), `instances/` (the write rule), the instance directory (the read rule), its `diagnostics/` (the read rule), its `snapshot.json` (the write rule) and its `events.ndjson` (the write rule on Unix, the read rule on Windows), each that exists and a folder before what it holds»
- line: .andromeda/architecture-detail/occupied-resources.md:112
  old: «`check_instance` has these three product call sites (`src/cmd/hook/statusline.rs`, `src/cmd/revive.rs` and `src/cmd/link.rs`, the one site both link verbs share; the rule: `check_instance(` under `src/`).»
  new: «`check_instance` has these three product call sites (`src/cmd/hook.rs` `trusted`, shared by the statusline, event and dialog paths; `src/cmd/revive.rs`; and `src/cmd/client.rs` `refuse_untrusted`, shared by `link` / `unlink` and the six frame verbs; the rule: `check_instance(` under `src/`). Outside that rule `viola-mcp`'s `tools.rs` `frame` calls it, and `run`'s start reaches its set through `viola_state::strict::check_home_tree`.»
- line: .andromeda/architecture-detail/occupied-resources.md:113
  old: «No case widens the DACL of an instance tree on Windows yet; that case is owed to the working-route entry "Home and code-bearing file integrity".»
  new: «On Windows `cfg(windows)` cases prove it: an instance tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and a plain home made before viola is refused, naming its inherited Users (`S-1-5-32-545`) write entry (as measured at ci#40403389629, `windows-2025`).»
- after: .andromeda/architecture-detail/occupied-resources.md:115
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- `viola_state::strict` holds two rules. The read rule is the write rule plus, on Unix, a refusal of any group or other bit (`mode & 0o077`) and, on Windows, a refusal of any allow ACE, an inherit-only one included, that grants a right in `READ_RIGHTS` (`0x9000_0001`: `FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to a SID `trusted_sid` rejects (`unix_read_verdict`, `windows_read_verdict`, `check_read`). `Refused` holds `Readable` for a read-rule refusal and `Symlink` for a symbolic link `check_diagnostics` meets, each with a fixed message.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/occupied-resources.md:115
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- `viola_state::strict::check_home_tree(home, name, key)` checks the home (the read rule); with a name, `check_instance`'s set; then `ledger/`, `bin/` and `plugin/`; with a key, `bin/<key>/`, the pinned `viola` (`viola.exe` on Windows), `plugin/<key>/`, its `hooks/` and its `.claude-plugin/` (the write rule). It checks only entries that exist and refuses as `TreeRefused::HomeLevel` or `TreeRefused::InstanceLevel`. `run`'s start and `viola revive`'s start call it through `checked_exe` (the instance's name and the `pin_key` of the exe bytes read once), and `verify` calls it with no name and no key.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/occupied-resources.md:115
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- The other checked entry points run the instance check first, each refusing with nothing under the home opened, no line written and the mode or ACL left as found: the CLI frame verbs `send` (after its text read), `wait`, `last`, `answer`, `pause` and `release` through `client::refuse_untrusted`, which `link` / `unlink` share, before any snapshot value is read (their exits: Binaries, above); `viola hook`'s event and dialog paths through `trusted`, before `read_diagnostics_level` and obs init, with its two `--capture` arms ahead of it (exit 0, with no `config.json` read, role file, stdin read, frame or SessionEnd append); `viola mcp`'s frame tools before `live_endpoint` (`instance-unreachable` with `detail: null`, no frame); and `viola run`'s and `viola revive`'s starts through `check_home_tree` before `open_wrapper_log` (the home pair, or the instance pair for an instance-level refusal, and no child).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/occupied-resources.md:115
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- A path under a regular file stats `NotFound` on Windows, so the strict-modes check skips it as absent and the `cli` catch-site line stands there; on Unix the same home is a strict-modes refusal (`verify` exits 1 with the home pair, the frame verbs exit 21 with the instance pair). A test that plants a regular file above a checked path therefore needs a per-OS oracle (as measured at ci#40398601594, `windows-2025`).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/occupied-resources.md:115
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- `viola list`'s two home checks disagree: `board::scan`'s own `check_path` holds the home to the write rule while `check_instance` holds it to the read rule, so a 0750 home passes a board with no instance and is refused (`ScanRefused::Instance`) once it holds one. Their reconciliation is owed to a CARRY on the working-route entry "Unix endpoint and home hardening".»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/occupied-resources.md:126
  old: «called only by `viola verify`: it holds the `.lock` for the read-modify-write»
  new: «called only by `viola verify`: it runs `strict::check_stamps(home)` before it creates `ledger/` and again under the lock before the capped read, a refusal (`StampsError::StrictModes`) reading and writing nothing and `verify` printing the home pair with exit 1; it holds the `.lock` for the read-modify-write»
- after: .andromeda/architecture-detail/occupied-resources.md:175
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-repository -->»
  add: «»
  add: «- An inheritable `icacls` grant (`(OI)(CI)`) on a seeded test home's `bin/`, or on the home, reaches through `seed_conpty`'s hard links into the shared `target/conpty-seed/<key>/` companion and widens every seeded home until it is removed; a test that widens a home tree above `bin/<key>/conpty/` therefore starts that home unseeded, and `seed_conpty` makes the home and the seed dir through `viola_state::fs::create_private_dir` (as measured on `windows-2025` at ci#40398601594 and ci#40400887151).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/occupied-resources.md:188
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-repository -->»
  add: «»
  add: «- The protected owner-only DACL of a home viola makes outside the profile reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)` on `windows-2025`, read back through `icacls /save`: canonical SDDL spells the runner's user, the local Administrator (RID 500), `LA`, so an oracle matching its SID string fails (as measured at ci#40400887151).»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "arch4-01"
fixed = 'rebuilds the path by the same rule only when the snapshot is missing or unreadable'

[[pattern]]
id = "arch4-02"
fixed = 'Four readers check these files before they use them'

[[pattern]]
id = "arch4-03"
fixed = 'its `snapshot.json` and its `events.ndjson`, each that exists and in that order'

[[pattern]]
id = "arch4-04"
fixed = '`src/cmd/hook/statusline.rs`, `src/cmd/revive.rs` and `src/cmd/link.rs`'

[[pattern]]
id = "arch4-05"
fixed = 'No case widens the DACL of an instance tree on Windows yet'
