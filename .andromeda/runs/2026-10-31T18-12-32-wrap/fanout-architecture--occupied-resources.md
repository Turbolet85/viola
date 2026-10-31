proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 30
    section: '## Occupied Resources'
    change: 'In the `revive` sentence, replace "under `--json` a refused strict-modes check or a preflight refusal is the exit-1 `{"v":1,"error":"<code>","detail":null}` document (one of the five codes, §Standard Contracts) with stderr empty" with "under `--json` every refusal of the start it shares with `run` is one exit-1 `{"v":1,"error":"<code>","detail":null}` document with stderr empty, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error` (§Standard Contracts), a `claude` not on `PATH` and an anyhow error of the start reading `internal-error`; a `pump_child` error prints no document, the child having held the terminal"; and replace "1 for a refused strict-modes check or a preflight refusal" with "1 for a refused strict-modes check, a preflight refusal or a refusal of the start".'
    sidecar: 'Occupied Resources, Binaries: `revive --json` prints one exit-1 document for every refusal of its start, ten codes where five stood; the post-preflight gap is closed.'
    rationale: 'Report Retired names the `revive --json` post-preflight gap; Symbols / APIs `viola revive --json` lists the ten codes and says a `pump_child` error prints no document. "One of the five codes" no longer holds.'
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 42
    section: '## Occupied Resources'
    change: 'Replace "`--json`: the `{"v":1,"error":"instance-unreachable","detail":null}` document on stdout" with "`--json`: the `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document on stdout".'
    sidecar: 'Occupied Resources, Binaries: a frame verb''s refused instance check reads `detail` `strict-modes-failed` in its exit-21 document.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols / APIs: `refuse_untrusted` writes `StrictModes` through `unreachable_document`. Expected amendments list this hit.'
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 148
    section: '## Occupied Resources'
    change: 'At @c1741, in the `viola list` refusal, replace "`--json`: `{"v":1,"error":"instance-unreachable","detail":null}` on stdout" with "`--json`: `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout".'
    sidecar: 'Occupied Resources, Filesystem: the board''s refusal reads `detail` `strict-modes-failed`.'
    rationale: 'Report Counts: `occupied-resources.md:148` doc-null @c1741 changed; Symbols / APIs: `src/cmd/list.rs` `refused_document` builds `strict-modes-failed`, the board''s only refusal.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 149
    section: '## Occupied Resources'
    change: 'At @c1362, in the `viola link` / `viola unlink` refusal, replace "`--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document on stdout" with "`--json` the `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document on stdout"; the exit-1 `strict-modes-failed` document of `viola revive` earlier in the line stands.'
    sidecar: 'Occupied Resources, Filesystem: a refused `link` / `unlink` instance check reads `detail` `strict-modes-failed`.'
    rationale: 'Report Counts: `occupied-resources.md:149` doc-null @c1362 changed; `client::refuse_untrusted`, shared by `link` / `unlink`, writes `StrictModes`.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 168
    section: '## Occupied Resources'
    change: 'Replace "`viola mcp`''s frame tools before `live_endpoint` (`instance-unreachable` with `detail: null`, no frame)" with "`viola mcp`''s frame tools before `live_endpoint` (`instance-unreachable` with `detail: "strict-modes-failed"`, no frame)".'
    sidecar: 'Occupied Resources, Filesystem: MCP''s refused instance check reads `detail` `strict-modes-failed`.'
    rationale: 'Report Symbols / APIs, MCP: in `tools.rs` `frame` a refused `check_instance` is `StrictModes`. Expected amendments list this hit.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 122
    section: '## Occupied Resources'
    change: 'At @c926, replace "exit 21 with `detail:"instance-dead"` + `during`, or `detail:"server-verify-failed"` + `during:"connect"` when the serving process is not the recorded wrapper (then no `channel-request` line), at WARN" with "exit 21 with its cause in `detail`: `instance-dead`, `unwrapped-name` or `server-verify-failed` (the last when the serving process is not the recorded wrapper, then no `channel-request` line) + `during:"connect"`, or `endpoint-lost` + `during:"call"`, at WARN".'
    sidecar: 'Occupied Resources, Filesystem: `instance-dead` no longer carries `during:"call"`; a wrapper lost with the call open logs `endpoint-lost`, and a name never wrapped logs `unwrapped-name`.'
    rationale: 'Report Retired names `instance-dead` · `during` `call`; Symbols / APIs "A process-exit `during`"; Counts dead-call: `occupied-resources.md:122` changed.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 32
    section: '## Occupied Resources'
    change: 'New record in its own `<!-- detail: values · res-binaries -->` group: - The `cli` catch site under `--json`: where `prints_internal_error` is true and `src/main.rs` `json_asked` finds `--json` among the arguments before a `--` (read from argv as `role_of` is), it prints `{"v":1,"error":"internal-error","detail":null}` on stdout (`cmd::internal_error_document`) in place of `error: internal error`, stderr then empty. Without `--json` the line is byte for byte as it stood; `run`, `revive`''s start, `mcp` and `hook` stay silent there.'
    sidecar: 'Occupied Resources, Binaries: the catch site''s `--json` document registered beside the verbs'' exit 1.'
    rationale: 'Report Symbols / APIs, "The `cli` catch site under `--json`". Line 30 names exit 1 through the catch site for `list` and `plugin install` and states no output, so it stays true and the new form is a new record.'
    basis: src/cmd/mod.rs:99
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 44
    section: '## Occupied Resources'
    change: 'New record in its own `<!-- detail: values · res-binaries -->` group: - Exit 21''s `detail` is its cause, the closed `viola_core::UnreachableCause` (`v` still 1, `error` still `instance-unreachable`, no exit moved; the founder''s own word, chunk 2026-10-31-exit-cause-code-catalogue `inputs#I2` Q1): `strict-modes-failed` a refused check · `instance-dead` the instance directory stands with no live endpoint, or the connect failed · `unwrapped-name` no directory there (absent, a regular file, a symlink) · `endpoint-lost` the wrapper lost with the call open · `server-verify-failed` the serving process is not the recorded wrapper. With no endpoint the cause is `viola_state::liveness::without_endpoint`''s `symlink_metadata` reading, which opens nothing and writes no line, the strict-modes check first at every caller; that the directory decides is the delegate''s word, PROVISIONAL (`inputs#I2` Q2). `client::unreachable_document` is the one writer of the document, and `viola list` builds `strict-modes-failed` as its board''s only refusal. MCP''s `isError` result is the same body less `v`, its text still `refused: instance-unreachable`, with no frame and no line. A frame verb against a stale instance reads `instance-dead`.'
    sidecar: 'Occupied Resources, Binaries: exit 21''s five closed causes registered, with their writer and MCP''s form.'
    rationale: 'Report Symbols / APIs: "The exit-21 document moved", `viola_core::UnreachableCause`, `without_endpoint`, `unreachable_document`, MCP; Decisions & corrections give the authority of Q1 and of the PROVISIONAL Q2.'
    basis: crates/viola-core/src/lib.rs:181
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 126
    section: '## Occupied Resources'
    change: 'New record in its own `<!-- detail: values · res-filesystem -->` group: - A stale holder''s code: `collision_check` (`src/cmd/run.rs`) prints and logs `stale-heartbeat` for a `stale` holder, and `already-live` for a live holder and a fresh replayed beat. The `run.collision_check` span''s `outcome` stays `already-live` for a live or a stale holder, its closed set saying a holder stands. Stale heartbeat is exit 1''s cause alone.'
    sidecar: 'Occupied Resources, Filesystem: `stale-heartbeat` registered as a stale holder''s code, the span''s `outcome` unmoved.'
    rationale: 'Report Symbols / APIs, "A stale holder''s code"; Retired names `already-live` as a stale holder''s refusal. No passage of this file states that refusal: line 8''s `already-live` is the fresh replayed beat, which stands. Schema / config adds the word to the `process-exit` `detail` enum.'
    basis: schemas/diag-line.v1.json:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 132
    section: '## Occupied Resources'
    change: 'New record in its own `<!-- detail: values · res-filesystem -->` group: - `verify`''s refusal codes: the `process-exit` `verify` writes under a parsed `VIOLA_NAME` carries its `Ended::Refused` code (`src/cmd/verify.rs`) in `detail`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture`, beside the standing `interrupted`. A refused home tree stays lineless; a failing row stays no refusal and no code; the fixture''s `<file>` enters no code; `verify` takes no `--json`.'
    sidecar: 'Occupied Resources, Filesystem: `verify`''s `process-exit` refusal codes registered.'
    rationale: 'Report Symbols / APIs, "`verify`''s codes", and Schema / config; lines 122-124 name only `interrupted` for that exit.'
    basis: schemas/diag-line.v1.json:78
note: 'Not proposed: line 42 says `verify` exits "before its `cli` log and any step line" when `update_stamps` refuses, yet the report has `verify` log `strict-modes-failed` with a refused home tree lineless, and its new witness `verify_window_a_widened_stamps_file_refuses_with_its_code` suggests the stamps refusal is the line; the report does not say which refusal writes it, so the orchestrator should read that clause.'
