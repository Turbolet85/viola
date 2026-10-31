proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 37
    section: '## Input Validation'
    change: 'In the **Frame verbs** sentence, the `--json` form of a refused `client::refuse_untrusted` reads exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` in place of the `detail":null` document; exit 21, the instance pair in human mode, nothing opened under the home and no line all stand.'
    sidecar: 'Input Validation, CLI arguments row: a refused frame-verb strict-modes check under `--json` now names its cause, `detail` `strict-modes-failed`, not null (chunk 2026-10-31-exit-cause-code-catalogue, `inputs#I2` Q1).'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols says a strict-modes site now writes `strict-modes-failed` and Expected amendments lists `input-validation.md:37`.'
    basis: .andromeda/security-plan-detail/input-validation.md:37
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 37
    section: '## Input Validation'
    change: 'In the `viola mcp` sentence, a refusal of `check_instance` returns `instance-unreachable` with `detail` `strict-modes-failed`, in place of `detail: null`, with no frame and no line.'
    sidecar: 'Input Validation, CLI arguments row: the MCP frame tools'' strict-modes refusal now carries `detail` `strict-modes-failed`, the CLI body less `v` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Symbols, MCP: in `tools.rs` `frame` a refused `check_instance` is `StrictModes`, no frame and no line; Retired names `detail: null`.'
    basis: .andromeda/security-plan-detail/input-validation.md:37
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 37
    section: '## Input Validation'
    change: 'New line after 37: **The exit-21 cause (chunk 2026-10-31-exit-cause-code-catalogue):** the `detail` of `instance-unreachable` on CLI `--json` and MCP is one of the closed `viola_core::UnreachableCause` words `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, with no serde path: every word is written by the two clients (`client::unreachable_document`, the one writer of the CLI document, and `viola_mcp` `result::unreachable`), and none holds a path, a pid or upstream text. The cause in `detail` is the founder''s own answer (`inputs#I2` Q1 of that chunk, through the overseer''s dialog).'
    sidecar: 'Input Validation, CLI arguments row: new record of the closed `UnreachableCause` set and its two writers (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Symbols names `viola_core::UnreachableCause`, no serde path, and Coverage reads validation closed enum and PII no path, pid or upstream text; D-security-input holds, this records the control.'
    basis: crates/viola-core/src/lib.rs:181-190
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 37
    section: '## Input Validation'
    change: 'New line after 37: **No endpoint:** where `live_endpoint` gives none, the strict-modes check having stayed first at every caller, `viola_state::liveness::without_endpoint` reads the instance directory through `symlink_metadata`: a directory is `instance-dead`, anything else (absent, a regular file, a symlink) `unwrapped-name`; it opens nothing and writes no line. Its callers are `Unreachable::without_endpoint` in `src/cmd/client.rs` and `frame` in `crates/viola-mcp/src/tools.rs`. The reading by the directory is the delegate''s word, PROVISIONAL (the overseer, `inputs#I2` Q2 of chunk 2026-10-31-exit-cause-code-catalogue).'
    sidecar: 'Input Validation, CLI arguments row: new record of `without_endpoint`, the no-endpoint reading by the instance directory, PROVISIONAL (`inputs#I2` Q2).'
    rationale: 'Report Symbols names `without_endpoint` as a new filesystem read behind the strict-modes check; Expected amendments for security-plan carries the unwrapped reading as `delegate the overseer`, PROVISIONAL, so the record must keep that word.'
    basis: crates/viola-state/src/liveness.rs:107-115
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 38
    section: '## Input Validation'
    change: 'In the `revive` arguments, remove the clause `a refusal run''s start makes after a passed preflight prints no document (owed to the route entry "Exit-cause code catalogue");`: that refusal now prints its one exit-1 document.'
    sidecar: 'Input Validation, CLI arguments row: the `revive --json` post-preflight gap is closed and its owed-to clause removed (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Retired names the `revive --json` post-preflight gap, and Symbols says the sentence "a refusal `run`''s start makes after a passed preflight prints no document" is no longer true; `revive-gap` and `title` move with it.'
    basis: .andromeda/security-plan-detail/input-validation.md:38
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 38
    section: '## Input Validation'
    change: 'In the `revive` arguments, `a refused strict-modes check or a preflight refusal` becomes `every refusal of the start it shares with run`, and the closed `<code>` list reads `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`.'
    sidecar: 'Input Validation, CLI arguments row: `revive --json` start codes grow by five, `stale-heartbeat` among them (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Symbols, `viola revive --json`, gives the ten codes and every refusal of the shared start; `start-codes-rev` moves with `revive-gap`, and Retired names `already-live` as a stale holder''s refusal.'
    basis: .andromeda/security-plan-detail/input-validation.md:38
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 38
    section: '## Input Validation'
    change: 'New line after 38: **`revive --json`, a start that errs (chunk 2026-10-31-exit-cause-code-catalogue):** an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `refused_document("internal-error")` and returns the same error, so the catch site still logs its chain and its line; a `pump_child` error prints no document, the child having held the terminal, and viola writes nothing past it. A program not found is `internal-error` too.'
    sidecar: 'Input Validation, CLI arguments row: new record of `revive --json`''s `internal-error` document and the `pump_child` exception (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Symbols, `run`''s start under `--json` and `viola revive --json`, state both; placed on its own line because line 38 stands near the 2 000-char entity limit.'
    basis: src/cmd/revive.rs:119-134
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 40
    section: '## Input Validation'
    change: 'New line after 40: **`--json` at the catch site (chunk 2026-10-31-exit-cause-code-catalogue):** `json_asked` in `src/main.rs` reads `--json` among the arguments before a `--` from argv, as `role_of` is read, into a bool. Where `prints_internal_error` is true it prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr then empty; without `--json` the line stands byte for byte, and `run`, `revive`''s start, `mcp` and `hook` stay silent there.'
    sidecar: 'Input Validation, CLI arguments row: new record of `json_asked`, the argv read outside clap at the `cli` catch site (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Symbols, the `cli` catch site under `--json`: a new read of an external input outside the clap derive this row describes, closed to a bool with a fixed document, so D-security-input holds and the row should name it.'
    basis: src/main.rs:154-161
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 75
    section: '## Input Validation'
    change: 'After `the two fixed unable:/hint: stderr lines with no path or pid`, add: under `viola revive --json` the one `batch-script-child` document stands in place of the pair (`refused_with`), stderr empty; `run`, which passes no `--json`, keeps the pair byte for byte.'
    sidecar: 'Input Validation, Child executable resolution row: `batch-script-child` prints its document under `revive --json` in place of the pair (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Symbols, `run`''s start under `--json`: `refused_with` prints `refused_document` in place of the pair for `batch-script-child`, before `spawn_child`; another occurrence of the retired post-preflight gap, this row stating the pair alone.'
    basis: src/cmd/run.rs:422-431
    dependent-of: D-retired
note: 'D-security-auth and D-security-deps read no violation (no auth or crypto moved, no dependency added); `.andromeda/security-plan.md:413` in the body still reads `instance-unreachable` with `detail: null`, listed by the report under Error Handling and outside this file.'
