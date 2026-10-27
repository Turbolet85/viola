# Apply — security-plan · part 1 of 4

## Verdicts
- fanout-security-plan.md 1 · applied · .andromeda/security-plan.md:114#1
- fanout-security-plan.md 2 · applied · .andromeda/security-plan.md:118#1
- fanout-security-plan.md 3 · applied · .andromeda/security-plan.md:142#1
- fanout-security-plan--cascade-g2.md 7 · folded into fanout-security-plan.md 3
- fanout-security-plan.md 4 · applied · .andromeda/security-plan.md:154#1
- fanout-security-plan--cascade-g2.md 12 · folded into fanout-security-plan.md 4
- fanout-security-plan.md 5 · applied · .andromeda/security-plan.md:439#1
- fanout-security-plan--cascade-g2.md 27 · folded into fanout-security-plan.md 5
- fanout-security-plan.md 6 · applied · .andromeda/security-plan.md:170#1
- fanout-security-plan--cascade-g2.md 16 · folded into fanout-security-plan.md 6
- fanout-security-plan--cascade-g1.md 1 · applied · .andromeda/security-plan-detail/threat-model-summary.md:56#1
- fanout-security-plan--cascade-g1.md 2 · applied · .andromeda/security-plan-detail/threat-model-summary.md:87+#1
- fanout-security-plan--cascade-g1.md 3 · applied · .andromeda/security-plan-detail/threat-model-summary.md:96+#1
- fanout-security-plan--cascade-g1.md 4 · applied · .andromeda/security-plan-detail/threat-model-summary.md:111+#1
- fanout-security-plan--cascade-g1.md 5 · applied · .andromeda/security-plan-detail/threat-model-summary.md:124#1
- fanout-security-plan--cascade-g1.md 6 · applied · .andromeda/security-plan-detail/threat-model-summary.md:126#1

## Edits
- line: .andromeda/security-plan.md:114
  old: «the client verifies the server against its one snapshot read: on Linux and Windows the connected peer's pid (Windows over an SQOS identification-level open) and that process's start time, a mismatch or a check that cannot be made sending no frame, never falling back to a default connect; the snapshot read without the strict-modes check is a dated gap owed to "Home and code-bearing file integrity", except for CLI `link` / `unlink`, which read it only after the instance's strict-modes check and take no gap, and the client euid check»
  new: «the client runs the instance's strict-modes check, a refusal writing no frame and no line, and only then verifies the server against its one snapshot read: on Linux and Windows the connected peer's pid (Windows over an SQOS identification-level open) and that process's start time, a mismatch, a snapshot replayed from the log (no endpoint, pid or start time) or a check that cannot be made sending no frame, never falling back to a default connect; the client euid check»
- line: .andromeda/security-plan.md:118
  old: «or a home another user can read, in every process that reads a trusted file, with its dated interim gaps.»
  new: «a home, instance directory or `diagnostics/` another user can read (Unix any group or other bit; Windows an allow ACE granting a read right to another SID, inherit-only included) or a symlinked `diagnostics/` folder or file, in every process that reads a trusted file, before its first read or open under the home.»
- line: .andromeda/security-plan.md:142
  old: «`statusline_command` or ledger read, except the ratified dated interim gaps until the Epoch 6 entries land;»
  new: «`statusline_command` or ledger read, `ledger/stamps.json` checked before `ledger/` is created and again under its lock, a refusal reading and writing nothing;»
- line: .andromeda/security-plan.md:154
  old: «read only after the home strict-modes check, except the ratified dated gaps;»
  new: «read only after the home strict-modes check; a snapshot that fails to parse or carries a newer `v` is replayed from the log by its readers, and a replayed one carries no `endpoint`, `pid`, `started_at` or `statusline_command`, so no frame is written and no command runs from it;»
- line: .andromeda/security-plan.md:170
  old: «the companions are loaded only after a full SHA-256 re-hash under a read-share-only handle, by absolute path, and any failure degrades to the inbox ConPTY, recorded only as codes, never a refusal.»
  new: «the companions are loaded only after the owner/DACL strict-modes check of `bin/<version>-<hash>/conpty/` and each companion, made before the held open, and a full SHA-256 re-hash under a read-share-only handle, by absolute path, and any failure, a refused check included, degrades to the inbox ConPTY, recorded only as codes, never a refusal.»
- line: .andromeda/security-plan.md:439
  old: «except the dated gaps: `hook.event`, `hook.dialog`, the CLI `send`, `wait` / `last`, `answer`, `pause` / `release` frames and `viola mcp`'s tool frames verify the server against a snapshot read without the strict-modes check, owed to "Home and code-bearing file integrity", and every client's euid check»
  new: «except the dated gaps: every client's euid check»
- line: .andromeda/security-plan-detail/threat-model-summary.md:56
  old: «The snapshot is read without the strict-modes check (owed to "Home and code-bearing file integrity"), except by CLI `link` / `unlink`, which read it only after `check_instance` passed on the driven instance, and macOS»
  new: «Each first runs the instance's strict-modes check (`check_instance`: the CLI verbs through `client::refuse_untrusted`, `viola hook` before its obs init, the `viola mcp` tools before `live_endpoint`), a refusal sending no frame, and then reads the snapshot through `read_snapshot_or_replay`, a replayed or absent snapshot giving no endpoint and so no frame; macOS»
- after: .andromeda/security-plan-detail/threat-model-summary.md:87
  anchor: «    - `hook statusline` runs `statusline_command` from `snapshot.json` through a shell. That snapshot is found through `VIOLA_DIR`, so its integrity is set by filesystem permissions on the viola home.»
  add: «    - Every hook path (event, dialog and `hook statusline`) runs the instance's strict-modes check on the canonicalised `VIOLA_DIR` instance before anything under the home is read or opened, the `--capture` arms ahead of it; a refused or absent instance exits 0 with no `config.json` read, no role file, no stdin read, no line and no frame. `hook statusline` reads the snapshot through `read_snapshot_or_replay`, and a replayed snapshot carries no `statusline_command`, so nothing runs.»
- after: .andromeda/security-plan-detail/threat-model-summary.md:96
  anchor: «    - Tool input schemas (schemars) and the `ViolaName` newtype check the input.»
  add: «    - Before its frame, each of `send`, `wait`, `last` and `answer` runs the instance's strict-modes check (`check_instance` on the home and the instance directory) ahead of the liveness pre-check and the server check; a refusal is `instance-unreachable` with `detail: null`, no frame and no line.»
- after: .andromeda/security-plan-detail/threat-model-summary.md:111
  anchor: «  - **Trust boundary:** Same-user invocation, clap parsing, the `ViolaName` nutype, and tolerant `config.json` parsing.»
  add: «    - Before its first read or open under the home, `run`, `revive`'s start and `verify` hold the home tree to the strict-modes check (`check_home_tree`; a refusal exit 1, no log opened), and the frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` hold the instance to it (`check_instance`; a refusal exit 21), so a state tree chosen by `--home` or `VIOLA_DIR` that another user can write, or whose home, instance directory or `diagnostics/` another user can read, is refused.»
- line: .andromeda/security-plan-detail/threat-model-summary.md:124
  old: «has two product readers, `viola revive`'s preflight and `viola list` (`viola_state::board::read`, chunk 2026-10-11-the-board-viola-list), each taking it after the strict-modes check on the instance's files; no other product reader takes it»
  new: «is taken by these product readers: `viola revive`'s preflight, `viola list` (`viola_state::board::read`), `viola_state::liveness::live_endpoint` (the CLI frame verbs and the `viola mcp` tools), `viola hook`'s `connect`, `hook statusline`'s `serve` and `run`'s collision check, each after the strict-modes check on the instance's files; a replayed snapshot carries no `endpoint`, `pid`, `started_at` or `statusline_command`, so it yields no frame and runs no command»
- line: .andromeda/security-plan-detail/threat-model-summary.md:126
  old: «Nothing checks integrity beyond the content hash in the folder name.»
  new: «Beyond the content hash in the folder name, `run` and `revive`'s start hold the home tree to the strict-modes check (Unix owner and mode, Windows owner and DACL) before the wrapper log opens (`check_home_tree`: the home, the instance's files, `ledger/`, `bin/`, `plugin/`, `bin/<version>-<hash>/`, the pinned `viola(.exe)`, `plugin/<version>-<hash>/`, its `hooks/` and `.claude-plugin/`), and on Windows `pin_companions` checks `bin/<version>-<hash>/conpty/` and each companion before the held open; `marketplace/` is off the integrity set.»

## Patterns
[[pattern]]
id = "sec1-01"
fixed = "the snapshot read without the strict-modes check is a dated gap"

[[pattern]]
id = "sec1-02"
fixed = "with its dated interim gaps"

[[pattern]]
id = "sec1-03"
fixed = "except the ratified dated interim gaps"

[[pattern]]
id = "sec1-04"
fixed = "except the ratified dated gaps"

[[pattern]]
id = "sec1-05"
fixed = "verify the server against a snapshot read without the strict-modes check"

[[pattern]]
id = "sec1-06"
fixed = "The snapshot is read without the strict-modes check"
ignore_case = true

[[pattern]]
id = "sec1-07"
fixed = "has two product readers"

[[pattern]]
id = "sec1-08"
fixed = "checks integrity beyond the content hash"
ignore_case = true
