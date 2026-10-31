proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 35
    section: '## Conventions'
    change: 'In the `viola run` collision clause, replace `detail:"already-live"`, stderr `unable: <name> is already live` / `hint: viola list`, or `unable: <name> is still running but not answering` with the stale hint with: a live holder `detail:"already-live"`, stderr `unable: <name> is already live` / `hint: viola list`; a stale holder `detail:"stale-heartbeat"`, `unable: <name> is still running but not answering` with the stale hint'
    sidecar: 'Conventions: a stale holder now refuses `stale-heartbeat`, `already-live` kept for a live holder.'
    rationale: 'Report **Retired** names `already-live` as a stale holder''s refusal; **Symbols / APIs**, A stale holder''s code: `collision_check` prints and logs `stale-heartbeat` for a stale holder; **Counts**, `stale-live` lists conventions.md:35 as changed.'
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 36
    section: '## Conventions'
    change: 'In the preflight readings, replace `detail:"already-live"` (`run`''s own collision check and its two pairs, unchanged) with `detail:"already-live"` or, for a stale holder, `detail:"stale-heartbeat"` (`run`''s own collision check and its two pairs, unchanged), and add `stale-heartbeat` to the `--json` code list after `already-live`.'
    sidecar: 'Conventions: the revive preflight names `stale-heartbeat` beside `already-live`, in its readings and its document codes.'
    rationale: 'Report **Symbols / APIs**: the revive codes now include `stale-heartbeat`, and the collision check `revive` shares with `run` prints it for a stale holder; **Expected amendments** lists conventions.md:36 under `stale-live`.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 36
    section: '## Conventions'
    change: 'Delete the clause , while a refusal `run`''s start makes after a passed preflight prints no document (owed to "Exit-cause code catalogue"; chunk 2026-10-17-cli-machine-contract) so that the sentence ends at the same log lines; the start refusals'' documents are the new item following line 52.'
    sidecar: 'Conventions: the `revive --json` post-preflight gap closed; its owed-to line retired.'
    rationale: 'Report **Retired** names the `revive --json` post-preflight gap; **Symbols / APIs**, `viola revive --json`: the sentence is no longer true; **Counts**, `revive-gap` and `title`.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 38
    section: '## Conventions'
    change: 'After no dispatch arm prints it add: (under `--json` before a `--`, read by `src/main.rs` `json_asked`, the catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout in its place, `internal_error_document`, stderr empty; `run`, `revive`''s start, `mcp` and `hook` stay silent there)'
    sidecar: 'Conventions: the `cli` catch site prints the `internal-error` document under `--json`.'
    rationale: 'Report **Symbols / APIs**, The `cli` catch site under `--json`; **Counts**, `catch-site` lists conventions.md:38 as a site stating the output under `--json`, which takes the document.'
    basis: src/main.rs:154-161
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 46
    section: '## Conventions'
    change: 'Replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the `plugin install` and `link` / `unlink` clauses, the same exit and document, stand.'
    sidecar: 'Conventions: exit 21 on the board, `plugin install` and `link` / `unlink` strict-modes refusals carries `detail:"strict-modes-failed"`.'
    rationale: 'Report **Retired** names the exit-21 document''s `detail: null`; **Symbols / APIs**: `refuse_untrusted` and `src/cmd/plugin.rs` write `StrictModes`, and `list.rs` builds `strict-modes-failed`, the board''s only refusal.'
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 64
    section: '## Conventions'
    change: 'Replace the verb prints the same `{"v":1,"error":"instance-unreachable","detail":null}` under `--json` with the verb prints `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` under `--json`, and replace the cause has no exit code, `--json` code or `RefusalReason` of its own with the cause has no exit code or `RefusalReason` of its own, its `--json` word the `detail` `server-verify-failed`.'
    sidecar: 'Conventions: a server that is not the recorded wrapper now reads `detail:"server-verify-failed"` under `--json`.'
    rationale: 'Report **Symbols / APIs**, The exit-21 document moved: `server-verify-failed` is a closed cause, no exit moved and no `RefusalReason` added; **Retired**, the exit-21 document''s `detail: null`.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 70
    section: '## Conventions'
    change: 'Replace `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`.'
    sidecar: 'Conventions: the frame verbs'' strict-modes refusal carries `detail:"strict-modes-failed"`.'
    rationale: 'Report **Retired**, the exit-21 document''s `detail: null`; **Symbols / APIs**: `refuse_untrusted` writes `StrictModes` through `unreachable_document`.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 76
    section: '## Conventions'
    change: 'Replace (`instance-unreachable` with `detail: null`; with (`instance-unreachable` with `detail` its cause, one closed word as on the CLI, the result less `v`;'
    sidecar: 'Conventions: MCP''s `instance-unreachable` result carries the cause in `detail`.'
    rationale: 'Report **Symbols / APIs**, The exit-21 document moved: MCP''s `isError` result is the same less `v`, its text still `refused: instance-unreachable`; **Retired**, the exit-21 document''s `detail: null`.'
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 83
    section: '## Conventions'
    change: 'Replace returns the `instance-unreachable` `isError` result, text with returns the `instance-unreachable` `isError` result with `detail: "server-verify-failed"`, text'
    sidecar: 'Conventions: MCP''s failed server check names `server-verify-failed`.'
    rationale: 'Report **Symbols / APIs**, MCP: in `tools.rs` `frame` a failed `server_verified` is `ServerVerify`, with no frame and no line.'
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 89
    section: '## Conventions'
    change: 'Replace `isError` result with `detail: null`, text with `isError` result with `detail: "strict-modes-failed"`, text'
    sidecar: 'Conventions: MCP''s refused instance check carries `detail: "strict-modes-failed"`.'
    rationale: 'Report **Symbols / APIs**, MCP: in `tools.rs` `frame` a refused `check_instance` is `StrictModes`; **Retired**, the exit-21 document''s `detail: null`.'
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 52
    section: '## Conventions'
    change: 'New item following line 52: - `viola revive --json` prints one exit-1 document `{"v":1,"error":"<code>","detail":null}` for every refusal of the start it shares with `run`, stderr empty: `refused_with` prints `client::refused_document` in place of the pair for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and for a program not found `internal-error` (no human pair), all before `spawn_child`; `run` itself passes `json` false and its human pairs are unmoved. An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` under `--json` prints the `internal-error` document and returns the same error, so the catch site still logs its chain and its line; a `pump_child` error prints no document, the child having held the terminal. The codes of `viola revive --json`: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error` (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'Conventions: `viola revive --json` start refusals each print a document; the ten codes listed.'
    rationale: 'Report **Symbols / APIs**, `run`''s start under `--json` and `viola revive --json`. Its own item, because the exit-code block :33-46 stands at 7545 B of the 8 KB entity bound and line 36 at 1857 B of the 2 000 line bound.'
    basis: src/cmd/revive.rs:119-134
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 58
    section: '## Conventions'
    change: 'New item following line 58: - `viola verify`''s refusals each carry their code on the `process-exit` it writes under a parsed `VIOLA_NAME` (`Ended::Refused`): `batch-script-child` · `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`, beside the standing `interrupted`. A refused home tree stays lineless, a failing row is no refusal and carries no code, the fixture''s `<file>` enters no code, and `verify` takes no `--json` (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'Conventions: `viola verify`''s refusal codes on its `process-exit` line.'
    rationale: 'Report **Symbols / APIs**, `verify`''s codes; **Schema / config**: the `process-exit` `detail` enum admits them; **Counts**, `verify-codes`.'
    basis: schemas/diag-line.v1.json:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 70
    section: '## Conventions'
    change: 'New item following line 70: - Exit `21`''s `--json` document names its cause, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `v` still 1 and `error` unmoved, one closed word per cause (`viola_core::UnreachableCause`, no serde path): `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`. `client::unreachable_document` writes it (`refuse_untrusted` and `src/cmd/plugin.rs` through it), and `src/cmd/list.rs`''s `refused_document` builds the board''s only refusal, `strict-modes-failed`. No exit moved and no `RefusalReason` or exit code was added (the founder''s own word, chunk 2026-10-31-exit-cause-code-catalogue, `inputs#I2` Q1).'
    sidecar: 'Conventions: the exit-21 document''s five closed causes.'
    rationale: 'Report **Symbols / APIs**, The exit-21 document moved, `viola_core::UnreachableCause` and `src/cmd/client.rs`; **Decisions & corrections**, the founder''s `inputs#I2` Q1.'
    basis: crates/viola-core/src/lib.rs:181-190
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 70
    section: '## Conventions'
    change: 'New item following the exit-21 document item: - A CLI frame verb''s cause: the strict-modes check first, then no live endpoint reads `viola_state::liveness::without_endpoint` (`symlink_metadata`: a directory `instance-dead`; anything else, absent, a regular file or a symlink, `unwrapped-name`; it opens nothing and writes no line; the delegate''s word, PROVISIONAL, chunk 2026-10-31-exit-cause-code-catalogue, `inputs#I2` Q2), a failed connect `instance-dead`, a closed or reset connection `endpoint-lost`, a server not the recorded wrapper `server-verify-failed`. Its `process-exit` reads `during:"call"` for `endpoint-lost` alone and `during:"connect"` for `instance-dead`, `unwrapped-name` and `server-verify-failed`. A frame verb against a stale instance still reads `instance-dead` (`live_endpoint` gives none for a stale wrapper): `stale-heartbeat` is exit 1''s cause alone.'
    sidecar: 'Conventions: how a CLI frame verb reads its exit-21 cause, and each cause''s `during`.'
    rationale: 'Report **Symbols / APIs**: `without_endpoint`, `src/cmd/client.rs` `answer_of`, A process-exit `during`, A stale holder''s code; **Decisions & corrections**, the delegate''s `inputs#I2` Q2, PROVISIONAL.'
    basis: crates/viola-state/src/liveness.rs:107-115
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 89
    section: '## Conventions'
    change: 'New item following line 89: - An MCP `instance-unreachable` result names its cause in `detail` (`result::unreachable(UnreachableCause)`), its text still `refused: instance-unreachable`: in `frame` a refused `check_instance` is `strict-modes-failed`, no `live_endpoint` is `without_endpoint`''s reading, a failed `server_verified` is `server-verify-failed`; in `of_reply` a failed connect is `instance-dead` and a closed or reset connection `endpoint-lost`; in `of_board` the board''s refusal is `strict-modes-failed`. No frame and no line on any of them, `isError` the record; JSON-RPC errors are unmoved and `-32603` keeps its fixed body (chunk 2026-10-31-exit-cause-code-catalogue, `inputs#I2` Q1 and Q4).'
    sidecar: 'Conventions: how an MCP tool reads its `instance-unreachable` cause.'
    rationale: 'Report **Symbols / APIs**, MCP; **Coverage of new surfaces**, MCP `detail` causes: no line by `inputs#I2` Q4, `isError` the record.'
    basis: crates/viola-core/src/lib.rs:181-190
note: 'D-arch-resources and D-arch-decisions fired nothing here (no new IPC, env var, crate or dependency); the exit-21 proposals rest on the founder''s inputs#I2 Q1 on which the report says the wrap halts; the body''s .andromeda/architecture.md:226 and :228 hold the same retired claims outside this file.'
