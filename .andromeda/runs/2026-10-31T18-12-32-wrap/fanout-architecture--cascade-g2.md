proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture.md
    near: 226
    section: '## Standard Contracts'
    change: 'The exit-21 bullet now reads `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` (exit 21), `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, in place of `"detail":null`.'
    sidecar: 'Standard Contracts, CLI --json output: the exit-21 document carries its cause in `detail`, one form per cause.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols / APIs, The exit-21 document moved, gives the form and the five closed causes.'
    basis: .andromeda/architecture.md:226
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture.md
    near: 228
    section: '## Standard Contracts'
    change: 'The exit-1 bullet now says `viola revive --json` prints this document for every refusal of its start, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`, still written by `client::refused_document` (codes only: no path, pid or id), stderr empty; the `--list --json` sentence stands; the closing sentence, that a refusal `run`''s start makes after a passed preflight prints no document and its document is owed to the working-route entry "Exit-cause code catalogue", is removed.'
    sidecar: 'Standard Contracts, CLI --json output: revive --json documents every refusal of its start, ten codes, the owed pointer spent.'
    rationale: 'Report Retired names the `revive --json` post-preflight gap; Symbols / APIs, `viola revive --json`, lists the ten codes and says the sentence is no longer true.'
    basis: .andromeda/architecture.md:228
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 229
    section: '## Standard Contracts'
    change: 'The `viola plugin install --json` bullet now ends: its refused strict-modes check prints the `instance-unreachable` document with `detail` `strict-modes-failed` (exit 21) through `client::unreachable_document`.'
    sidecar: 'Standard Contracts, CLI --json output: plugin install''s refusal is written by client::unreachable_document, with its cause.'
    rationale: 'Report Symbols / APIs: `unreachable_document` is the one writer of the exit-21 document, `src/cmd/plugin.rs` writes `StrictModes` through it, and `refused_document` stays exit 1''s.'
    basis: src/cmd/client.rs:222
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 99
    section: '## Standard Contracts'
    change: 'The MCP `list` clause now reads: returns the document''s `ok`, or `instance-unreachable` with `detail` `strict-modes-failed` for the board''s refusal, its only one; the rest of the line stands.'
    sidecar: 'Standard Contracts, channel methods: MCP list''s board refusal names its cause.'
    rationale: 'Report Symbols / APIs: MCP `of_board` reads the board''s refusal as `StrictModes`, and `src/cmd/list.rs` builds `strict-modes-failed`, the board''s only refusal.'
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 111
    section: '## Standard Contracts'
    change: 'The refused strict-modes check of `viola link` and `viola unlink` now prints exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout (exit 21), in place of `"detail":null`; the rest of the line stands.'
    sidecar: 'Standard Contracts, CLI --json output: link and unlink''s strict-modes refusal reads strict-modes-failed.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Counts, `doc-null`: a strict-modes site takes `strict-modes-failed`. The check, its order and its empty stderr did not move.'
    basis: .andromeda/architecture-detail/standard-contracts.md:111
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 111
    section: '## Standard Contracts'
    change: 'New bullet: - The exit-21 document''s `detail` is one of five closed causes, `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`, with `v` still 1 and `error` still `instance-unreachable` (chunk 2026-10-31-exit-cause-code-catalogue, the founder''s own word, its `inputs#I2` Q1).'
    sidecar: 'Standard Contracts, CLI --json output: the five closed exit-21 causes.'
    rationale: 'Report Symbols / APIs, The exit-21 document moved: five closed causes, `v` still 1, `error` unmoved, no exit or `RefusalReason` added.'
    fact: F1
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 117
    section: '## Standard Contracts'
    change: 'The six frame verbs'' refused strict-modes check now prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout under `--json` (exit 21), in place of the same document with `"detail":null`; `client::refuse_untrusted`, its order and the empty stderr stand.'
    sidecar: 'Standard Contracts, CLI --json output: the frame verbs'' strict-modes refusal reads strict-modes-failed.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols / APIs: `refuse_untrusted` writes `StrictModes` through `unreachable_document`.'
    basis: .andromeda/architecture-detail/standard-contracts.md:117
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 117
    section: '## Standard Contracts'
    change: 'New bullet: - `client::unreachable_document` is the one writer of the exit-21 CLI document; `refuse_untrusted` and `src/cmd/plugin.rs` write `StrictModes` through it, and `src/cmd/list.rs`''s `refused_document` builds `strict-modes-failed` as a `Value` for `board_document`, the board''s only refusal.'
    sidecar: 'Standard Contracts, CLI --json output: the exit-21 document''s one writer.'
    rationale: 'Report Symbols / APIs, the `src/cmd/client.rs` item: `unreachable_document`, its callers, and `list.rs`''s board document.'
    basis: src/cmd/client.rs:222
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 117
    section: '## Standard Contracts'
    change: 'New bullet: - `Launch` carries `json: bool` (`run` passes false, `revive` passes `args.json`), and `refused_with` prints `refused_document` in place of the human pair for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and a program not found as `internal-error`, all before `spawn_child`; the human pairs stand byte for byte.'
    sidecar: 'Standard Contracts, CLI --json output: Launch.json and refused_with, the start''s documents.'
    rationale: 'Report Symbols / APIs, `run`''s start under `--json`.'
    basis: src/cmd/run.rs:422
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 117
    section: '## Standard Contracts'
    change: 'New bullet: - `viola revive --json` prints one exit-1 document for every refusal of its start; an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `internal-error` and returns the same error, so the catch site still logs its chain and its line, and a `pump_child` error prints no document because the child has held the terminal.'
    sidecar: 'Standard Contracts, CLI --json output: revive --json documents its start''s refusals and anyhow errors, pump_child excepted.'
    rationale: 'Report Symbols / APIs, `viola revive --json`; Reverted facts: a `Started`-time document from `main` was rejected for the `pump_child` reason.'
    basis: src/cmd/revive.rs:119
    fact: F11
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 30
    section: '## Occupied Resources'
    change: 'The `revive` clause now reads: under `--json` every refusal of its start is the exit-1 `{"v":1,"error":"<code>","detail":null}` document (one of the ten codes, §Standard Contracts) with stderr empty, and its exit list reads 1 for a refused strict-modes check, a preflight refusal or a refusal of the start after a passed preflight; the `--list --json` clause and the rest of the line stand.'
    sidecar: 'Occupied Resources, binaries: revive --json documents every refusal of its start, ten codes.'
    rationale: 'Report Retired names the `revive --json` post-preflight gap; Symbols / APIs, `viola revive --json`, lists the ten codes. The `list` and `plugin install` exits on this line stand: no exit moved.'
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 42
    section: '## Occupied Resources'
    change: 'The `--json` form of the frame verbs'' refused instance check now reads the `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document on stdout, in place of `"detail":null`; the `verify`, `run` and `revive` sentences stand.'
    sidecar: 'Occupied Resources, binaries: the frame verbs'' refused check reads strict-modes-failed.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Counts, `doc-null`: a strict-modes site takes `strict-modes-failed`. A refused home tree stays lineless and `revive --json` keeps its `strict-modes-failed` document.'
    basis: .andromeda/architecture-detail/occupied-resources.md:42
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 122
    section: '## Occupied Resources'
    change: 'The `cli-<name>.ndjson` frame verbs'' clause now reads exit 21 with `detail:"instance-dead"` + `during:"connect"`, in place of `detail:"instance-dead"` + `during`; the `server-verify-failed` clause stands.'
    sidecar: 'Occupied Resources, filesystem: instance-dead on process-exit is during connect only.'
    rationale: 'Report Retired names `instance-dead` · `during` `call`; Counts, `dead-call`, lists this line, offset c926.'
    basis: .andromeda/architecture-detail/occupied-resources.md:122
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 122
    section: '## Occupied Resources'
    change: 'After «with no pid, start time, path or endpoint,» insert: exit 21 with `detail:"unwrapped-name"` + `during:"connect"`, or `detail:"endpoint-lost"` + `during:"call"` for a wrapper lost with the call open,'
    sidecar: 'Occupied Resources, filesystem: unwrapped-name and endpoint-lost on process-exit, each with its during.'
    rationale: 'Report Symbols / APIs, A process-exit `during`: `endpoint-lost` is `during:"call"`; `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during:"connect"`.'
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 124
    section: '## Occupied Resources'
    change: 'New continuation line after this one: `schemas/diag-line.v1.json`''s `process-exit` `detail` also admits `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`, `program-not-found`, `version-unreadable` and `unclean-fixture` (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'Occupied Resources, filesystem: six more process-exit detail words admitted by the line schema.'
    rationale: 'Report Schema / config: the `process-exit` `detail` enum gains these six; `channel-response` and `hook-decision` did not move.'
    basis: schemas/diag-line.v1.json:78
    fact: F14
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 148
    section: '## Occupied Resources'
    change: 'The `viola list` refusal''s `--json` form now reads `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout, in place of `"detail":null`; the rest of the line stands.'
    sidecar: 'Occupied Resources, filesystem: the board''s refusal reads strict-modes-failed.'
    rationale: 'Report Counts, the over-2 000-char row: `occupied-resources.md:148` doc-null @c1741 changed; Symbols / APIs: `strict-modes-failed` is the board''s only refusal.'
    basis: .andromeda/architecture-detail/occupied-resources.md:148
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 149
    section: '## Occupied Resources'
    change: 'The `viola link` / `viola unlink` refusal''s `--json` form now reads the `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document on stdout, in place of `"detail":null`; `revive`''s `strict-modes-failed` exit-1 document on this line stands.'
    sidecar: 'Occupied Resources, filesystem: link and unlink''s refusal reads strict-modes-failed.'
    rationale: 'Report Counts, the over-2 000-char row: `occupied-resources.md:149` doc-null @c1362 changed; `refuse_untrusted` writes `StrictModes`.'
    basis: .andromeda/architecture-detail/occupied-resources.md:149
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 168
    section: '## Occupied Resources'
    change: 'The `viola mcp` frame-tools clause now reads (`instance-unreachable` with `detail: "strict-modes-failed"`, no frame), in place of `detail: null`; the rest of the line stands.'
    sidecar: 'Occupied Resources, filesystem: MCP''s refused instance check names strict-modes-failed.'
    rationale: 'Report Retired names `detail: null`; Symbols / APIs, MCP: a refused `check_instance` is `StrictModes`, no frame and no line.'
    basis: .andromeda/architecture-detail/occupied-resources.md:168
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 34
    section: '## Cross-cutting Patterns'
    change: 'New sub-bullet after this line:   - Every exit-21 cause and exit-1 word passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`, carrying no path, pid or upstream text (chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'Cross-cutting Patterns, sanitised error surfaces: the new cause and exit-1 words are code words, each a named row.'
    rationale: 'Report Outcome: every new word passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`; Coverage of new surfaces: no path, pid or upstream text.'
    fact: F17
note: F16 has no picked entity to carry it (no test-binary row among the picks), so it is the new-detail pass's, and the stale-holder, `without_endpoint` and `verify` code facts (F3, F9, F13) are established-decisions'; the picked standard-contracts lines 111 and 117 hold neither the five-code exit-1 list nor the "prints no document" sentence the task places in contract-cli-json-output, so if the detail holds them on another line, that line was not picked and no proposal here edits it.
