proposals:
  - detector: D-retired
    severity: warning
    section: '§Standard Contracts'
    change: 'In CLI `--json` output the exit-21 line reads `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` (exit 21), `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` (`viola_core::UnreachableCause`), written by `client::unreachable_document`; `v` stays 1, `error` stays `instance-unreachable`, and the MCP `isError` result is the same less `v`, its text still `refused: instance-unreachable`.'
    sidecar: 'exit-21 document: `detail` null retired, one closed cause per document on CLI --json and MCP.'
    rationale: 'Report Retired names the exit-21 document `detail: null`; Symbols / APIs gives the document, the five causes and the unchanged MCP text; Expected amendments lists architecture.md:226.'
    basis: 'crates/viola-core/src/lib.rs:181-190'
  - detector: D-retired
    severity: warning
    section: '§Standard Contracts'
    change: 'The exit-1 `<code>` line covers `viola revive --json` refusing its strict-modes check, its preflight or the start it shares with `run`, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; `already-live` is a live holder or a fresh replayed beat and `stale-heartbeat` a stale holder; a `pump_child` error prints no document; the sentence that a post-preflight refusal prints no document and that its document is owed to the working-route entry "Exit-cause code catalogue" is removed, the `--list` clause stays.'
    sidecar: 'revive --json post-preflight gap closed: ten exit-1 codes, stale-heartbeat split from already-live, the owed-document sentence removed.'
    rationale: 'Report Retired names the `revive --json` post-preflight gap and `already-live` as a stale holder refusal; Symbols / APIs lists the ten codes and states the old sentence is no longer true; Expected amendments lists architecture.md:228.'
    basis: 'src/cmd/revive.rs:119-134'
  - detector: D-retired
    severity: warning
    section: '§Standard Contracts'
    change: 'The `viola plugin install --json` line says its refused strict-modes check prints the `instance-unreachable` document with `detail` `strict-modes-failed` (exit 21) through `client::unreachable_document`, no longer through `client::refused_document`.'
    sidecar: 'plugin install: the refused strict-modes document names its cause and its new writer.'
    rationale: 'Report Symbols / APIs: `unreachable_document` is the one writer of the exit-21 document, `src/cmd/plugin.rs` writes `StrictModes` through it, and `refused_document` stays exit 1 only.'
    basis: 'src/cmd/client.rs:222-226'
    dependent-of: D-retired
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts'
    change: 'CLI `--json` output gains a line: `{"v":1,"error":"internal-error","detail":null}` (exit 1), a `cli` verb at the `main` catch site with `--json` among the arguments before a `--` (`json_asked`), printed on stdout by `cmd::internal_error_document` in place of `error: internal error`, stderr empty; without `--json` the human line is unchanged, and `run`, the start of `revive`, `mcp` and `hook` stay silent there.'
    sidecar: 'registered the catch-site internal-error document under --json.'
    rationale: 'Report Symbols / APIs, the `cli` catch site under `--json`, adds a machine document that the CLI `--json` registry does not list.'
    basis: 'src/main.rs:154-161'
  - detector: D-arch-resources
    severity: warning
    section: '§Infrastructure Patterns → Build system'
    change: 'The `human` callers clause says the `main` catch site prints `error: internal error` for every `cli` verb without `--json`, and under `--json` the `internal-error` document on stdout in its place (`cmd::internal_error_document`), stderr empty.'
    sidecar: 'Build system: the catch-site line is scoped to the human mode.'
    rationale: 'The contract states the line for every `cli` verb; report Counts `catch-site` says a site stating the output under `--json` too takes the document, and the human line stands byte for byte.'
    basis: 'src/cmd/mod.rs:99-102'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Infrastructure Patterns → Project directory structure'
    change: 'The `client.rs` comment says a refused check under `--json` prints the `instance-unreachable` document with `detail` `strict-modes-failed`, names the closed `Unreachable` as `Dead` · `Unwrapped` · `Lost` · `ServerUnverified` in place of `instance-dead` · `server-verify-failed`, and adds `unreachable_document`, the one writer of the exit-21 document, `refused_document` staying exit 1 only.'
    sidecar: 'Project directory structure: client.rs carries the four Unreachable variants and unreachable_document.'
    rationale: 'Report Symbols / APIs: `Unreachable` is now `Dead` · `Unwrapped` · `Lost` · `ServerUnverified`, `Dead(&static str)` is gone, and `unreachable_document` is new.'
    basis: 'src/cmd/client.rs:114-122'
  - detector: D-arch-resources
    severity: warning
    section: '§Infrastructure Patterns → Project directory structure'
    change: 'The `viola-core` comment adds `UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, each word from `as_str`, no serde path).'
    sidecar: 'Project directory structure: viola-core lists UnreachableCause.'
    rationale: 'Report Symbols / APIs lands `viola_core::UnreachableCause`, a new public type of a crate whose contents the comment enumerates.'
    basis: 'crates/viola-core/src/lib.rs:181-190'
  - detector: D-arch-resources
    severity: warning
    section: '§Infrastructure Patterns → Project directory structure'
    change: 'The `viola-state` comment adds `liveness::without_endpoint` beside `liveness::live_endpoint`: with no live endpoint a directory reads `InstanceDead` and anything else `UnwrappedName`, opening nothing and writing no line, shared by the CLI frame verbs and viola-mcp frame tools.'
    sidecar: 'Project directory structure: viola-state lists liveness::without_endpoint.'
    rationale: 'Report Symbols / APIs lands `viola_state::liveness::without_endpoint`, called from `src/cmd/client.rs` and `crates/viola-mcp/src/tools.rs`, a new public read beside the one the comment names.'
    basis: 'crates/viola-state/src/liveness.rs:107-115'
note: 'D-arch-decisions holds: report Dependencies reads none added or bumped and Crates reads none added or removed. The exit-21 proposal rests on the founder word inputs#I2 Q1. The remaining sites the report lists for architecture (conventions.md, established-decisions.md, occupied-resources.md, standard-contracts.md) are standing detail groups and so the cascade edits them.'
